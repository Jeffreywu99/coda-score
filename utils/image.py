"""Image processing utilities for music score preprocessing."""

import cv2
import numpy as np
from pathlib import Path


def pdf_to_images(pdf_path: Path, output_dir: Path, dpi: int = 300) -> list[Path]:
    """Convert PDF pages to PNG images.

    Args:
        pdf_path: Path to PDF file.
        output_dir: Output directory for PNG files.
        dpi: Resolution in DPI (300 recommended for scores).

    Returns:
        List of output PNG paths.
    """
    import fitz  # PyMuPDF

    output_dir.mkdir(parents=True, exist_ok=True)
    doc = fitz.open(str(pdf_path))
    output_paths = []

    for page_num in range(len(doc)):
        page = doc[page_num]
        pix = page.get_pixmap(dpi=dpi)
        output_path = output_dir / f"page_{page_num + 1:03d}.png"
        pix.save(str(output_path))
        output_paths.append(output_path)

    doc.close()
    return output_paths


def deskew(image: np.ndarray, max_angle: float = 5.0) -> np.ndarray:
    """Deskew an image by detecting horizontal lines and correcting rotation.

    Args:
        image: Input image (BGR or grayscale).
        max_angle: Maximum correction angle in degrees.

    Returns:
        Deskewed image.
    """
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image.copy()
    edges = cv2.Canny(gray, 50, 150, apertureSize=3)
    lines = cv2.HoughLinesP(edges, 1, np.pi / 180, threshold=100, minLineLength=100, maxLineGap=10)

    if lines is None:
        return image

    angles = []
    for line in lines:
        x1, y1, x2, y2 = line[0]
        angle = np.degrees(np.arctan2(y2 - y1, x2 - x1))
        if abs(angle) < max_angle:
            angles.append(angle)

    if not angles:
        return image

    median_angle = np.median(angles)
    if abs(median_angle) < 0.1:  # Skip negligible rotation
        return image

    h, w = image.shape[:2]
    center = (w // 2, h // 2)
    rotation_matrix = cv2.getRotationMatrix2D(center, median_angle, 1.0)
    border_value = (255, 255, 255) if len(image.shape) == 3 else 255
    return cv2.warpAffine(image, rotation_matrix, (w, h), borderValue=border_value)


def adaptive_threshold(image: np.ndarray) -> np.ndarray:
    """Apply adaptive thresholding to clean up scanned scores.

    Handles yellowed paper, uneven lighting, and faded ink.

    Args:
        image: Input image (BGR or grayscale).

    Returns:
        Cleaned image (grayscale).
    """
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image.copy()

    # CLAHE for contrast enhancement
    clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
    enhanced = clahe.apply(gray)

    # Adaptive threshold (inverted: black ink on white paper)
    binary = cv2.adaptiveThreshold(
        enhanced, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY_INV, 15, 8
    )
    # Invert back
    return cv2.bitwise_not(binary)


def normalize_score_image(
    image: np.ndarray,
    target_size: tuple[int, int] = (1024, 1024),
    padding_color: int = 255,
) -> np.ndarray:
    """Resize and pad a score image to a square target size.

    Preserves aspect ratio and centers the image on a white background.

    Args:
        image: Input image (any size).
        target_size: Target (width, height).
        padding_color: Pixel value for padding (255 = white).

    Returns:
        Normalized image at target_size.
    """
    h, w = image.shape[:2]
    target_w, target_h = target_size

    # Scale to fit (with 5% margin)
    scale = min(target_w / w, target_h / h) * 0.95
    new_w = int(w * scale)
    new_h = int(h * scale)

    resized = cv2.resize(image, (new_w, new_h), interpolation=cv2.INTER_AREA)

    # Create padded canvas
    if len(image.shape) == 3:
        canvas = np.full((target_h, target_w, image.shape[2]), padding_color, dtype=np.uint8)
    else:
        canvas = np.full((target_h, target_w), padding_color, dtype=np.uint8)

    y_offset = (target_h - new_h) // 2
    x_offset = (target_w - new_w) // 2
    canvas[y_offset : y_offset + new_h, x_offset : x_offset + new_w] = resized

    return canvas
