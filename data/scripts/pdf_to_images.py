"""Convert PDF scores to individual page images."""

import sys
from pathlib import Path

import click

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from utils.image import pdf_to_images


@click.command()
@click.argument("pdf_path", type=click.Path(exists=True, path_type=Path))
@click.option("--output-dir", "-o", type=click.Path(path_type=Path), help="Output directory (default: same name as PDF)")
@click.option("--dpi", default=300, help="Resolution in DPI")
def main(pdf_path: Path, output_dir: Path | None, dpi: int):
    """Convert a PDF score to individual page PNG images."""
    if output_dir is None:
        output_dir = pdf_path.parent / pdf_path.stem

    print(f"Converting: {pdf_path.name}")
    print(f"Output: {output_dir}")
    print(f"DPI: {dpi}")

    paths = pdf_to_images(pdf_path, output_dir, dpi)
    print(f"\nGenerated {len(paths)} pages:")
    for p in paths:
        print(f"  {p.name}")


if __name__ == "__main__":
    main()
