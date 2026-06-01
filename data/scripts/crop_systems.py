"""Split orchestral score pages into individual system crops.

Each system (group of staves bracketed together) becomes one training sample.
Systems are padded to a square for SDXL training, preserving aspect ratio.
"""

import sys
from pathlib import Path

import cv2
import click
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from utils.image import normalize_score_image


def find_content_bbox(image: np.ndarray, threshold: int = 240) -> tuple[int, int, int, int]:
    """Find bounding box of non-white content, excluding edge noise."""
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image
    mask = gray < threshold

    # Erode to remove isolated noise dots, then dilate to reconnect
    kernel = np.ones((5, 5), np.uint8)
    mask = cv2.morphologyEx(mask.astype(np.uint8), cv2.MORPH_CLOSE, kernel)

    if not mask.any():
        return 0, 0, image.shape[1], image.shape[0]

    rows = np.any(mask, axis=1)
    cols = np.any(mask, axis=0)
    rmin, rmax = np.where(rows)[0][[0, -1]]
    cmin, cmax = np.where(cols)[0][[0, -1]]

    return cmin, rmin, cmax + 1, rmax + 1


def find_staff_rows(gray: np.ndarray, min_line_length: int = 200) -> np.ndarray:
    """Detect individual staff lines using row darkness profile.

    Returns boolean array of shape (height,) where True = staff line pixel row.
    """
    h, w = gray.shape
    # Row darkness: fraction of pixels below threshold
    row_darkness = np.mean(gray < 128, axis=1)

    # Staff lines are horizontal lines of high darkness
    # Use local maxima detection
    is_staff = np.zeros(h, dtype=bool)
    for row in range(1, h - 1):
        # A staff line has higher darkness than neighbors
        if row_darkness[row] > 0.15 and row_darkness[row] > row_darkness[row - 1] and row_darkness[row] >= row_darkness[row + 1]:
            is_staff[row] = True

    # Merge lines that are very close (within 3px = same staff line)
    merged = np.zeros(h, dtype=bool)
    i = 0
    while i < h:
        if is_staff[i]:
            # Find end of this cluster
            j = i
            while j < h and is_staff[j]:
                j += 1
            # Take the middle row of the cluster
            mid = (i + j - 1) // 2
            if j - i <= 4:  # Cluster <= 4px is a single staff line
                merged[mid] = True
            i = j
        else:
            i += 1

    return merged


def group_into_systems(
    staff_rows: np.ndarray, min_staves_per_system: int = 2, max_gap: int = 50
) -> list[tuple[int, int]]:
    """Group detected staff lines into systems (bracketed groups of staves).

    A system is a group of 5-line staves separated by consistent small gaps.
    Systems are separated by larger gaps.

    Returns list of (top_row, bottom_row) for each system.
    """
    # Find staves (groups of 5 closely-spaced staff lines)
    staff_positions = np.where(staff_rows)[0]
    if len(staff_positions) < 10:  # Need at least 2 staves (10 lines)
        return []

    # Cluster staff lines into individual staves (5 lines each)
    staves = []  # list of (center_y, top_y, bottom_y)
    i = 0
    while i < len(staff_positions):
        cluster = [staff_positions[i]]
        j = i + 1
        while j < len(staff_positions) and staff_positions[j] - staff_positions[j - 1] < 15:
            cluster.append(staff_positions[j])
            j += 1

        # Each staff should have ~5 lines. Take groups of ~5.
        if len(cluster) >= 3 and len(cluster) <= 8:
            staves.append((int(np.mean(cluster)), cluster[0] - 2, cluster[-1] + 2))
        i = j

    if len(staves) < 2:
        return []

    # Group staves into systems based on gap between staves
    # Staves within a system have small gaps; between systems have larger gaps
    gaps = [staves[k + 1][0] - staves[k][0] for k in range(len(staves) - 1)]
    if not gaps:
        return []

    median_gap = np.median(gaps)
    system_gap_threshold = median_gap * 1.8  # 80% larger than typical staff gap

    systems = []
    sys_start = 0
    for k in range(len(staves) - 1):
        if gaps[k] > system_gap_threshold:
            # System break
            if k - sys_start + 1 >= min_staves_per_system:
                systems.append((staves[sys_start][1], staves[k][2]))
            sys_start = k + 1

    # Last system
    if len(staves) - sys_start >= min_staves_per_system:
        systems.append((staves[sys_start][1], staves[-1][2]))

    return systems


@click.command()
@click.argument("input_dir", type=click.Path(exists=True, path_type=Path))
@click.argument("output_dir", type=click.Path(path_type=Path))
@click.option("--size", default=1024, help="Target square size in pixels")
@click.option("--margin", default=15, help="Margin pixels around each system crop")
@click.option("--min-systems", default=1, help="Min systems per page to keep")
def main(input_dir: Path, output_dir: Path, size: int, margin: int, min_systems: int):
    """Split orchestral score pages into per-system square crops.

    Reads PNGs from INPUT_DIR, crops to content area, splits into systems,
    and saves each system as a 1024x1024 PNG in OUTPUT_DIR.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    image_files = sorted(input_dir.glob("*.png"))

    if not image_files:
        print(f"No PNG files found in {input_dir}")
        return

    print(f"Processing {len(image_files)} pages -> system crops")

    total_systems = 0
    total_pages = 0
    skipped = 0

    for img_path in image_files:
        image = cv2.imread(str(img_path))
        if image is None:
            print(f"  SKIP {img_path.name}: cannot read")
            continue

        # Step 1: Crop to content (removes white margins + QR codes)
        x1, y1, x2, y2 = find_content_bbox(image)
        cropped = image[y1:y2, x1:x2]
        gray = cv2.cvtColor(cropped, cv2.COLOR_BGR2GRAY)

        # Step 2: Detect staff lines and group into systems
        staff_rows = find_staff_rows(gray)
        systems = group_into_systems(staff_rows)

        if len(systems) < min_systems:
            skipped += 1
            continue

        total_pages += 1

        # Step 3: Extract each system as a square crop
        for sys_idx, (sys_top, sys_bottom) in enumerate(systems):
            # Add margin
            sys_top = max(0, sys_top - margin)
            sys_bottom = min(cropped.shape[0], sys_bottom + margin)

            system_crop = cropped[sys_top:sys_bottom, :]

            # Skip if too small
            if system_crop.shape[0] < 50 or system_crop.shape[1] < 100:
                continue

            # Normalize to square
            square = normalize_score_image(system_crop, target_size=(size, size))

            out_name = f"{img_path.stem}_sys{sys_idx:02d}.png"
            cv2.imwrite(str(output_dir / out_name), square)
            total_systems += 1

    print(f"\nDone: {total_systems} system crops from {total_pages}/{len(image_files)} pages")
    if skipped:
        print(f"  Skipped {skipped} pages (< {min_systems} systems)")

    print(f"Output: {output_dir}")


if __name__ == "__main__":
    main()
