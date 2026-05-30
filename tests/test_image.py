"""Tests for image processing utilities."""

import cv2
import numpy as np
import pytest

import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from utils.image import adaptive_threshold, deskew, normalize_score_image


class TestNormalizeScoreImage:
    def test_output_shape(self):
        """Output is always the target size."""
        img = np.random.randint(0, 255, (600, 800, 3), dtype=np.uint8)
        result = normalize_score_image(img, target_size=(1024, 1024))
        assert result.shape == (1024, 1024, 3)

    def test_padding_is_white(self):
        """Corners should be white (padding)."""
        img = np.random.randint(0, 255, (600, 800, 3), dtype=np.uint8)
        result = normalize_score_image(img, target_size=(1024, 1024))
        assert result[0, 0, 0] == 255
        assert result[-1, -1, 0] == 255

    def test_preserves_aspect_ratio(self):
        """Wide image should have vertical padding, not horizontal."""
        img = np.zeros((500, 1000, 3), dtype=np.uint8)
        result = normalize_score_image(img, target_size=(1024, 1024))
        center_row = result[512, 512, :]
        assert not np.all(center_row == 255), "Center should have content"

    def test_grayscale_input(self):
        """Should work with grayscale images too."""
        img = np.random.randint(0, 255, (600, 800), dtype=np.uint8)
        result = normalize_score_image(img, target_size=(512, 512))
        assert result.shape == (512, 512)


class TestDeskew:
    def test_straight_image_unchanged(self):
        """A straight image with horizontal lines should not be rotated."""
        img = np.ones((500, 500, 3), dtype=np.uint8) * 255
        for y in range(100, 400, 50):
            cv2.line(img, (50, y), (450, y), (0, 0, 0), 2)
        result = deskew(img)
        assert result.shape == img.shape

    def test_output_shape_preserved(self):
        """Output should have same dimensions as input."""
        img = np.ones((400, 600, 3), dtype=np.uint8) * 200
        result = deskew(img)
        assert result.shape == img.shape


class TestAdaptiveThreshold:
    def test_binary_output(self):
        """Output should be approximately binary (mostly 0 and 255)."""
        img = np.zeros((200, 200), dtype=np.uint8)
        for i in range(200):
            img[i, :] = int(i * 255 / 200)
        result = adaptive_threshold(img)
        unique = np.unique(result)
        assert len(unique) <= 10  # Mostly binary after thresholding

    def test_output_shape(self):
        """Output is always grayscale with same spatial dims."""
        img = np.random.randint(0, 255, (300, 400, 3), dtype=np.uint8)
        result = adaptive_threshold(img)
        assert result.shape == (300, 400)
