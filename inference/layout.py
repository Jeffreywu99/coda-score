"""Procedural music score layout generator for ControlNet conditioning.

Generates blank staff line layouts using OpenCV. No music theory involved.
The output is a simple structural sketch: horizontal lines grouped into
staff systems, with margins and basic page layout.

Used as ControlNet conditioning input: the diffusion model receives the
staff line structure and fills in the musical content entirely through
learned visual patterns.
"""

import cv2
import numpy as np
import click
from pathlib import Path


def generate_staff_layout(
    width: int = 1024,
    height: int = 1024,
    num_systems: int = 8,
    lines_per_staff: int = 5,
    line_spacing: int = 8,
    margin_top: int = 80,
    margin_bottom: int = 60,
    margin_left: int = 60,
    margin_right: int = 40,
    line_thickness: int = 2,
    line_color: int = 0,
    bg_color: int = 255,
) -> np.ndarray:
    """Generate a blank music staff layout image."""
    canvas = np.full((height, width), bg_color, dtype=np.uint8)

    usable_height = height - margin_top - margin_bottom
    staff_height = (lines_per_staff - 1) * line_spacing
    system_spacing = usable_height // num_systems

    for sys_idx in range(num_systems):
        y_start = margin_top + sys_idx * system_spacing

        for line_idx in range(lines_per_staff):
            y = y_start + line_idx * line_spacing
            cv2.line(canvas, (margin_left, y), (width - margin_right, y), line_color, line_thickness)

        # Barline at left edge
        cv2.line(canvas, (margin_left, y_start), (margin_left, y_start + staff_height),
                 line_color, line_thickness + 1)

    return canvas


def generate_piano_grand_staff_layout(
    width: int = 1024,
    height: int = 1024,
    num_systems: int = 6,
    line_spacing: int = 8,
    gap_between_staves: int = 24,
    margin_top: int = 80,
    margin_bottom: int = 60,
    margin_left: int = 60,
    margin_right: int = 40,
) -> np.ndarray:
    """Generate a piano score layout with grand staff (treble + bass per system)."""
    canvas = np.full((height, width), 255, dtype=np.uint8)

    usable_height = height - margin_top - margin_bottom
    system_spacing = usable_height // num_systems

    for sys_idx in range(num_systems):
        y_base = margin_top + sys_idx * system_spacing

        # Upper staff (treble)
        for line_idx in range(5):
            y = y_base + line_idx * line_spacing
            cv2.line(canvas, (margin_left, y), (width - margin_right, y), 0, 2)

        # Lower staff (bass)
        y_lower_base = y_base + 4 * line_spacing + gap_between_staves
        for line_idx in range(5):
            y = y_lower_base + line_idx * line_spacing
            cv2.line(canvas, (margin_left, y), (width - margin_right, y), 0, 2)

        # Brace
        brace_top = y_base
        brace_bottom = y_lower_base + 4 * line_spacing
        cv2.line(canvas, (margin_left, brace_top), (margin_left, brace_bottom), 0, 3)

    return canvas


def generate_graphic_notation_layout(
    width: int = 1024,
    height: int = 1024,
    num_regions: int = 4,
    margin: int = 60,
) -> np.ndarray:
    """Generate a layout for graphic notation using rectangular regions."""
    canvas = np.full((height, width), 255, dtype=np.uint8)

    usable_h = height - 2 * margin
    usable_w = width - 2 * margin
    band_height = usable_h // num_regions

    for i in range(num_regions):
        y = margin + i * band_height
        cv2.rectangle(canvas, (margin, y), (margin + usable_w, y + band_height - 10), 180, 1)
        center_y = y + band_height // 2
        cv2.line(canvas, (margin, center_y), (margin + usable_w, center_y), 200, 1)

    return canvas


@click.command()
@click.option("--type", "layout_type", type=click.Choice(["staff", "piano", "graphic"]),
              default="staff", help="Layout type")
@click.option("--width", default=1024, help="Image width")
@click.option("--height", default=1024, help="Image height")
@click.option("--systems", default=8, help="Number of staff systems")
@click.option("--output", default="outputs/layout.png", help="Output path")
def main(layout_type, width, height, systems, output):
    """Generate a blank score layout for ControlNet conditioning."""
    if layout_type == "staff":
        layout = generate_staff_layout(width=width, height=height, num_systems=systems)
    elif layout_type == "piano":
        layout = generate_piano_grand_staff_layout(width=width, height=height, num_systems=systems)
    else:
        layout = generate_graphic_notation_layout(width=width, height=height, num_regions=systems)

    output_path = Path(output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    cv2.imwrite(str(output_path), layout)
    print(f"Generated {layout_type} layout -> {output_path} ({width}x{height})")


if __name__ == "__main__":
    main()
