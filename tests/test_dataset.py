"""Tests for ScoreDataset and caption generation."""

import json
import sys
from pathlib import Path

import numpy as np
import pytest
from PIL import Image

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from models.utils.caption import ScoreMetadata, generate_caption, STYLE_TEMPLATES
from models.utils.dataset import ScoreDataset


def _create_test_images(tmp_path: Path, count: int = 5) -> Path:
    """Create dummy test images in a temp directory."""
    img_dir = tmp_path / "images"
    img_dir.mkdir()
    for i in range(count):
        img = Image.fromarray(np.random.randint(0, 255, (512, 512, 3), dtype=np.uint8))
        img.save(img_dir / f"page_{i + 1:03d}.png")
    return img_dir


def _create_test_metadata(tmp_path: Path, img_dir: Path) -> Path:
    """Create test metadata JSON."""
    meta_file = tmp_path / "metadata.json"
    metadata = [
        {
            "filename": png.name,
            "composer": "Test Composer",
            "period": "post_war",
            "techniques": ["serialism"],
            "instrumentation": "piano",
            "notation_type": "traditional",
            "trigger_word": "in the style of Test Composer",
        }
        for png in sorted(img_dir.glob("*.png"))
    ]
    with open(meta_file, "w") as f:
        json.dump(metadata, f)
    return meta_file


class TestCaptionGeneration:
    def test_basic_caption(self):
        meta = ScoreMetadata(composer="Boulez", trigger_word="in the style of Boulez")
        caption = generate_caption(meta)
        assert "in the style of Boulez" in caption
        assert "contemporary classical music score" in caption
        assert "black ink on white paper" in caption

    def test_caption_with_techniques(self):
        meta = ScoreMetadata(
            composer="Boulez",
            period="post_war",
            techniques=["serialism", "integral_serialism"],
            instrumentation="piano",
            trigger_word="in the style of Boulez",
        )
        caption = generate_caption(meta)
        assert "serial" in caption.lower()
        assert "piano" in caption
        assert "post-war" in caption

    def test_style_templates_exist(self):
        assert "boulez" in STYLE_TEMPLATES
        assert "xenakis" in STYLE_TEMPLATES
        assert "graphic_notation" in STYLE_TEMPLATES

    def test_auto_trigger_from_composer(self):
        meta = ScoreMetadata(composer="Xenakis")
        caption = generate_caption(meta)
        assert "in the style of Xenakis" in caption


class TestScoreDataset:
    def test_loads_images(self, tmp_path):
        img_dir = _create_test_images(tmp_path)
        meta_file = _create_test_metadata(tmp_path, img_dir)
        ds = ScoreDataset(img_dir, meta_file, resolution=256)

        assert len(ds) == 5
        sample = ds[0]
        assert sample["pixel_values"].shape == (3, 256, 256)
        assert "caption" in sample
        assert "filename" in sample

    def test_caption_contains_trigger(self, tmp_path):
        img_dir = _create_test_images(tmp_path)
        meta_file = _create_test_metadata(tmp_path, img_dir)
        ds = ScoreDataset(img_dir, meta_file, resolution=256)

        sample = ds[0]
        assert "Test Composer" in sample["caption"]

    def test_works_without_metadata(self, tmp_path):
        img_dir = _create_test_images(tmp_path)
        default_meta = ScoreMetadata(composer="Unknown", trigger_word="test score")
        ds = ScoreDataset(img_dir, resolution=256, default_metadata=default_meta)

        assert len(ds) == 5
        assert "test score" in ds[0]["caption"]

    def test_empty_dir_raises(self, tmp_path):
        empty_dir = tmp_path / "empty"
        empty_dir.mkdir()
        with pytest.raises(ValueError, match="No PNG"):
            ScoreDataset(empty_dir, resolution=256)
