"""Data augmentation transforms for music score images.

Augmentations are specifically designed for scanned music scores:
- Slight rotation: simulates scan misalignment (+-2 degrees)
- Brightness/contrast: simulates different scan qualities
- No horizontal flip: music is NOT left-right symmetric!
- No color jitter: scores are monochrome
"""

import torchvision.transforms as T


def get_train_transforms(resolution: int = 768) -> T.Compose:
    """Training transforms with score-appropriate augmentation."""
    return T.Compose([
        T.Resize(resolution, interpolation=T.InterpolationMode.LANCZOS),
        T.CenterCrop(resolution),
        T.RandomApply([T.RandomRotation(degrees=2, fill=255)], p=0.3),
        T.RandomApply([T.ColorJitter(brightness=0.08, contrast=0.08)], p=0.3),
        T.ToTensor(),
        T.Normalize([0.5], [0.5]),  # [-1, 1] for diffusion models
    ])


def get_val_transforms(resolution: int = 768) -> T.Compose:
    """Validation transforms (deterministic, no augmentation)."""
    return T.Compose([
        T.Resize(resolution, interpolation=T.InterpolationMode.LANCZOS),
        T.CenterCrop(resolution),
        T.ToTensor(),
        T.Normalize([0.5], [0.5]),
    ])
