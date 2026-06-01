"""Preprocess score images for LoRA training."""

import sys
from pathlib import Path

import cv2
import click

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from utils.image import deskew, adaptive_threshold, normalize_score_image


@click.command()
@click.argument("input_dir", type=click.Path(exists=True, path_type=Path))
@click.argument("output_dir", type=click.Path(path_type=Path))
@click.option("--size", default=1024, help="Target square size in pixels")
@click.option("--do-deskew/--no-deskew", default=True, help="Correct scan rotation")
@click.option("--do-threshold/--no-threshold", default=False, help="Apply adaptive threshold (removes color)")
@click.option("--do-grayscale/--no-grayscale", default=False, help="Convert to grayscale")
def main(input_dir: Path, output_dir: Path, size: int, do_deskew: bool, do_threshold: bool, do_grayscale: bool):
    """Preprocess score images for LoRA training.

    Reads PNGs from INPUT_DIR, applies preprocessing, saves to OUTPUT_DIR.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    image_files = sorted(input_dir.glob("*.png"))

    if not image_files:
        print(f"No PNG files found in {input_dir}")
        return

    print(f"Processing {len(image_files)} images -> {size}x{size}")
    print(f"  Deskew: {do_deskew} | Threshold: {do_threshold} | Grayscale: {do_grayscale}")

    success = 0
    for img_path in image_files:
        image = cv2.imread(str(img_path))
        if image is None:
            print(f"  SKIP {img_path.name}: cannot read")
            continue

        if do_deskew:
            image = deskew(image)
        if do_threshold:
            image = adaptive_threshold(image)
        if do_grayscale:
            image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        image = normalize_score_image(image, target_size=(size, size))

        output_path = output_dir / img_path.name
        cv2.imwrite(str(output_path), image)
        success += 1

    print(f"\nDone: {success}/{len(image_files)} images processed -> {output_dir}")


if __name__ == "__main__":
    main()
