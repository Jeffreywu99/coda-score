"""Batch labeling tool for score image annotation."""

import json
import sys
from pathlib import Path

import click

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))


@click.command()
@click.argument("image_dir", type=click.Path(exists=True, path_type=Path))
@click.argument("output_file", type=click.Path(path_type=Path))
@click.option("--composer", required=True, help="Composer name (e.g., 'Pierre Boulez')")
@click.option("--work", default="", help="Work title")
@click.option("--period", default="post_war", type=click.Choice(["early_modern", "post_war", "late_20th", "contemporary"]))
@click.option("--techniques", default="serialism", help="Comma-separated technique tags")
@click.option("--instrumentation", default="piano", help="Instrumentation (e.g., 'piano', 'string quartet')")
@click.option("--notation-type", default="traditional", type=click.Choice(["traditional", "proportional", "graphic", "mixed", "text_score"]))
@click.option("--trigger", default="", help="LoRA trigger word (auto-generated if empty)")
def main(image_dir, output_file, composer, work, period, techniques, instrumentation, notation_type, trigger):
    """Generate annotation JSON for all PNG images in a directory.

    All images in the directory get the same metadata (suitable for
    single-composer LoRA training).
    """
    images = sorted(image_dir.glob("*.png"))
    if not images:
        print(f"No PNG files in {image_dir}")
        return

    tech_list = [t.strip() for t in techniques.split(",")]
    trigger_word = trigger or f"in the style of {composer}"

    annotations = [
        {
            "filename": img.name,
            "composer": composer,
            "work_title": work,
            "period": period,
            "techniques": tech_list,
            "instrumentation": instrumentation,
            "notation_type": notation_type,
            "trigger_word": trigger_word,
        }
        for img in images
    ]

    output_file = Path(output_file)
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, "w") as f:
        json.dump(annotations, f, indent=2, ensure_ascii=False)

    print(f"Generated {len(annotations)} annotations -> {output_file}")
    print(f"  Composer: {composer}")
    print(f"  Trigger: {trigger_word}")


if __name__ == "__main__":
    main()
