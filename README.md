# coda-score v0.2.0

**Graphical Modern Music Score Generation System**

A diffusion model-based system for generating contemporary academic music score images. Uses SDXL + LoRA fine-tuning to learn visual notation styles of 20th/21st century composers purely from score images — no symbolic music representation involved.

## Core Thesis

Modern music notation (graphic notation, proportional notation, extended techniques, New Complexity) constitutes an independent visual grammar that can be learned by diffusion models without any music theory.

## Generation Modes

| Mode | Use Case | How |
|------|----------|-----|
| txt2img | Graphic notation | Text prompt + LoRA → pure diffusion |
| img2img | Traditional notation | IMSLP reference image + LoRA → style transfer |
| ControlNet | Mixed notation | Procedural staff layout + LoRA → guided generation |

All modes are image-level operations. No symbolic music tools (LilyPond, MusicXML) involved.

## Setup

```bash
# Install uv
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"

# Install dependencies
uv sync --extra dev

# Verify CUDA
uv run python -c "import torch; print(f'CUDA: {torch.cuda.is_available()}, GPU: {torch.cuda.get_device_name(0) if torch.cuda.is_available() else \"N/A\"}')"
```

## Quick Start

```bash
# Pure diffusion (graphic notation)
uv run python inference/generate.py --mode txt2img --lora weights/lora/boulez.safetensors

# img2img from reference (traditional notation)
uv run python inference/generate.py --mode img2img --ref-image references/score.png --lora weights/lora/boulez.safetensors

# ControlNet guided (mixed notation)
uv run python inference/generate.py --mode controlnet --layout-image layouts/staff.png --lora weights/lora/boulez.safetensors
```

## Project Status

Phase 0: Environment setup + minimal validation (in progress)
