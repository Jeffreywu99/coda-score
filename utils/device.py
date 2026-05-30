"""Device management for CUDA/MPS/CPU."""

import torch


def get_device() -> torch.device:
    """Get the best available device."""
    if torch.cuda.is_available():
        return torch.device("cuda")
    if hasattr(torch.backends, "mps") and torch.backends.mps.is_available():
        return torch.device("mps")
    return torch.device("cpu")


def get_dtype(device: torch.device) -> torch.dtype:
    """Get optimal dtype for device."""
    if device.type in ("cuda", "mps"):
        return torch.float16
    return torch.float32


def get_vram_gb() -> float:
    """Get GPU VRAM in GB (0 if no GPU)."""
    if torch.cuda.is_available():
        return torch.cuda.get_device_properties(0).total_memory / 1024**3
    return 0.0


def print_device_info() -> None:
    """Print device information."""
    device = get_device()
    print(f"Device: {device}")
    if device.type == "cuda":
        props = torch.cuda.get_device_properties(0)
        print(f"  GPU: {props.name}")
        print(f"  VRAM: {props.total_memory / 1024**3:.1f} GB")
        print(f"  Compute: sm_{props.major}{props.minor}")
    allocated = torch.cuda.memory_allocated(device) / 1024**3 if device.type == "cuda" else 0
    print(f"  Allocated: {allocated:.2f} GB")
