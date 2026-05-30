"""PyTorch Dataset for music score images with metadata-driven captions."""

import json
from pathlib import Path

from PIL import Image
from torch.utils.data import Dataset

from .caption import ScoreMetadata, generate_caption
from .augment import get_train_transforms, get_val_transforms


class ScoreDataset(Dataset):
    """Dataset for music score images.

    Loads PNG images from a directory and pairs them with metadata-driven
    captions for LoRA training. If no metadata file is provided, uses
    default_metadata for all images.

    Directory layout:
        image_dir/
        +-- page_001.png
        +-- page_002.png
        +-- ...

    Metadata JSON format:
        [
            {
                "filename": "page_001.png",
                "composer": "Boulez",
                "period": "post_war",
                "techniques": ["serialism"],
                "instrumentation": "piano",
                "notation_type": "traditional",
                "trigger_word": "in the style of Boulez"
            }
        ]
    """

    def __init__(
        self,
        image_dir: str | Path,
        metadata_file: str | Path | None = None,
        resolution: int = 768,
        split: str = "train",
        default_metadata: ScoreMetadata | None = None,
    ):
        self.image_dir = Path(image_dir)
        self.resolution = resolution
        self.split = split
        self.default_metadata = default_metadata

        self.transform = (
            get_train_transforms(resolution) if split == "train" else get_val_transforms(resolution)
        )

        self.image_paths = sorted(self.image_dir.glob("*.png"))
        if not self.image_paths:
            raise ValueError(f"No PNG images found in {image_dir}")

        self.metadata: dict[str, ScoreMetadata] = {}
        if metadata_file and Path(metadata_file).exists():
            with open(metadata_file) as f:
                for item in json.load(f):
                    filename = item.pop("filename")
                    self.metadata[filename] = ScoreMetadata(**item)

    def __len__(self) -> int:
        return len(self.image_paths)

    def __getitem__(self, idx: int) -> dict:
        img_path = self.image_paths[idx]
        image = Image.open(img_path).convert("RGB")

        meta = self.metadata.get(img_path.name, self.default_metadata)
        caption = generate_caption(meta) if meta else "contemporary classical music score, black ink on white paper"

        pixel_values = self.transform(image)

        return {
            "pixel_values": pixel_values,
            "caption": caption,
            "filename": img_path.name,
        }
