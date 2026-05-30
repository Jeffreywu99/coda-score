# Phase 0: 环境搭建 + 多模式生成最小验证 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 搭建 Python + PyTorch + Diffusers 环境，训练第一个 Boulez 风格 LoRA，验证三种图像级生成模式能否产出有风格特征的现代乐谱图像。

**Architecture:** 纯图形化生成——SDXL 基座 + 风格 LoRA，三种生成模式按记谱类型选择：
1. **纯扩散 (txt2img)**：文本提示 + LoRA → 从噪声生成完整图像。用于图形记谱。
2. **img2img**：以 IMSLP 公共领域乐谱为参考起点 + LoRA 风格化。用于传统五线谱。
3. **ControlNet**：程序化生成空白五线谱布局 → ControlNet 引导 + LoRA 填充内容。用于混合记谱。

所有模式都是图像级操作，不经过任何符号化音乐工具（无 LilyPond、无 MusicXML）。论文核心论点：扩散模型可脱离音乐理论、纯视觉学习现代记谱语法。

**Tech Stack:** Python 3.12, PyTorch 2.7+ (CUDA 12.8), HuggingFace Diffusers, PEFT, bitsandbytes, uv, OpenCV, Pillow, W&B

**Hardware Strategy:**
- **RTX 5060 8GB** (当前): 环境搭建、SDXL 推理验证、数据管线、512² LoRA 概念验证
- **RTX 5080 16GB** (待购): 768²/1024² 正式 LoRA 训练、完整验证

---

## File Structure

```
coda-score/
├── docs/
│   ├── plan-v2.0.md
│   ├── phase0-report.md              # Phase 0 结果报告
│   └── superpowers/plans/
├── data/
│   ├── scripts/
│   │   ├── pdf_to_images.py          # PDF → PNG
│   │   ├── preprocess.py             # 图像预处理
│   │   ├── label.py                  # 元数据标注 CLI
│   │   └── split_dataset.py          # 训练/验证集划分
│   ├── raw/                          # 原始 PDF/扫描（gitignore）
│   │   └── boulez/
│   ├── processed/                    # 处理后图像（gitignore）
│   │   └── boulez/
│   └── metadata/
│       ├── style_taxonomy.yaml       # 风格分类体系
│       └── annotations/
│           └── boulez.json
├── models/
│   ├── train_lora.py                 # LoRA 训练脚本（核心）
│   ├── configs/
│   │   └── lora_boulez.yaml
│   └── utils/
│       ├── __init__.py
│       ├── dataset.py                # ScoreDataset
│       ├── caption.py                # Caption 模板
│       └── augment.py                # 数据增强
├── inference/
│   ├── generate.py                   # CLI 推理工具（三种模式）
│   ├── validate.py                   # 验证对比脚本
│   └── layout.py                     # 程序化生成五线谱布局图
├── utils/
│   ├── __init__.py
│   ├── image.py                      # 图像处理
│   └── device.py                     # 设备管理
├── references/                       # img2img 参考图像（gitignore）
├── notebooks/
├── study/                            # 保留：v1.x 知识库
├── weights/lora/                     # 模型权重（gitignore）
├── outputs/                          # 生成结果（gitignore）
├── pyproject.toml
├── .gitignore
└── README.md
```

---

## Task 1: Project Migration（v1.x → v2.0）

**Files:**
- Modify: `.gitignore`
- Delete: `src/`, `src-tauri/`, `index.html`, `package.json`, `package-lock.json`, `.github/workflows/build.yml`, `.github/workflows/ci.yml`
- Keep: `study/`, `docs/`

- [ ] **Step 1: 创建 v1-archive 分支保存现有代码**

```bash
cd D:\vibe-coding-app\coda-score
git checkout -b v1-archive
git checkout main
```

- [ ] **Step 2: 从 main 分支删除 v1.x 文件**

```bash
# 删除 Tauri 前端和后端
rm -rf src/ src-tauri/ index.html package.json package-lock.json
# 删除 v1.x CI/CD
rm -f .github/workflows/build.yml .github/workflows/ci.yml
# 提交
git add -A
git commit -m "chore: archive v1.x, prepare for v2.0 rewrite"
```

- [ ] **Step 3: 创建 v2.0 目录骨架**

```bash
mkdir -p data/{scripts,raw/boulez,processed/boulez,metadata/annotations}
mkdir -p models/{configs,utils}
mkdir -p inference
mkdir -p utils
mkdir -p notebooks
mkdir -p weights/lora
mkdir -p outputs
```

- [ ] **Step 4: 创建 .gitignore**

```gitignore
# Data (large files)
data/raw/
data/processed/

# Model weights
weights/

# Outputs
outputs/

# Reference images for img2img
references/

# Python
__pycache__/
*.pyc
*.pyo
.pytest_cache/
.venv/

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Jupyter
.ipynb_checkpoints/

# uv
uv.lock
```

- [ ] **Step 5: 创建空 `__init__.py` 文件**

```bash
touch utils/__init__.py models/__init__.py models/utils/__init__.py
```

- [ ] **Step 6: 创建 README.md**

```markdown
# coda-score v2.0

**Graphical Modern Music Score Generation System**

A diffusion model-based system for generating contemporary academic music score images. Uses SDXL + LoRA fine-tuning to learn visual notation styles of 20th/21st century composers purely from score images — no symbolic music representation involved.

## Core Thesis

Modern music notation (graphic notation, proportional notation, extended techniques, New Complexity) constitutes an independent visual grammar that can be learned by diffusion models without any music theory.

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
# Generate a score image (base SDXL)
uv run python inference/generate.py --prompt "contemporary classical music score, black ink on white paper"

# Generate with LoRA style
uv run python inference/generate.py --lora weights/lora/coda-score-boulez/coda-score-boulez.safetensors --prompt "contemporary classical music score, in the style of Boulez, serial composition, for piano"
```

## Project Status

Phase 0: Environment setup + minimal validation (in progress)
```

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "chore: create v2.0 project structure with README"
```

---

## Task 2: Python Environment Setup

**Files:**
- Create: `pyproject.toml`

- [ ] **Step 1: 安装 uv**

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
# 重启终端后验证
uv --version
```

- [ ] **Step 2: 创建 pyproject.toml**

```toml
[project]
name = "coda-score"
version = "0.2.0"
description = "Graphical modern music score generation via diffusion models"
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    # Core ML — RTX 5060/5080 Blackwell (sm_120) needs torch >= 2.7.0 + CUDA 12.8
    "torch>=2.7.0",
    "torchvision>=0.22.0",
    "diffusers>=0.32.0",
    "transformers>=4.48.0",
    "accelerate>=1.2.0",
    "peft>=0.14.0",
    "safetensors>=0.5.0",
    # 8-bit optimizer for memory-constrained training
    "bitsandbytes>=0.45.0",
    # Image processing
    "opencv-python>=4.10.0",
    "Pillow>=11.0.0",
    # Data & config
    "pyyaml>=6.0",
    "click>=8.1.0",
    "tqdm>=4.67.0",
    # PDF → image conversion
    "pymupdf>=1.25.0",
    # Experiment tracking
    "wandb>=0.19.0",
    # Super resolution (Phase 2)
    # "realesrgan>=0.3.0",
    # Web UI (Phase 2)
    # "gradio>=5.0.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0",
    "ruff>=0.9.0",
    "jupyter>=1.1.0",
]

[project.scripts]
coda-generate = "inference.generate:main"
coda-train = "models.train_lora:main"
coda-label = "data.scripts.label:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.ruff]
line-length = 100
target-version = "py312"

[tool.ruff.lint]
select = ["E", "F", "I", "W"]

[tool.pytest.ini_options]
testpaths = ["tests"]
```

- [ ] **Step 3: 创建虚拟环境并安装依赖**

```powershell
cd D:\vibe-coding-app\coda-score
uv sync --extra dev
```

- [ ] **Step 4: 安装 PyTorch with CUDA 12.8（如果默认安装不支持 sm_120）**

```powershell
# 先检查默认安装的 PyTorch 是否支持 Blackwell
uv run python -c "import torch; print(torch.cuda.get_device_capability(0) if torch.cuda.is_available() else 'no cuda')"

# 如果报错 sm_120 或 capability 不是 (12, 0)，手动安装 CUDA 12.8 版本：
uv pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu128
```

> ⚠️ **国内用户：** 如果下载 HuggingFace 模型慢，设置镜像：
> ```powershell
> $env:HF_ENDPOINT = "https://hf-mirror.com"
> ```

- [ ] **Step 5: 验证完整环境**

```powershell
uv run python -c "
import torch
import diffusers
import peft
import transformers

print('=== Environment Check ===')
print(f'PyTorch:      {torch.__version__}')
print(f'Diffusers:    {diffusers.__version__}')
print(f'PEFT:         {peft.__version__}')
print(f'Transformers: {transformers.__version__}')
print(f'CUDA:         {torch.cuda.is_available()}')

if torch.cuda.is_available():
    name = torch.cuda.get_device_name(0)
    vram = torch.cuda.get_device_properties(0).total_mem / 1024**3
    cap = torch.cuda.get_device_capability(0)
    print(f'GPU:          {name}')
    print(f'VRAM:         {vram:.1f} GB')
    print(f'Compute cap:  {cap[0]}.{cap[1]}')

    # Quick compute test
    x = torch.randn(512, 512, device='cuda')
    y = torch.mm(x, x)
    print(f'Compute test: OK ({y.shape})')
else:
    print('WARNING: CUDA not available!')
print('===========================')
"
```

Expected:
```
=== Environment Check ===
PyTorch:      2.7.x
Diffusers:    0.32.x
PEFT:         0.14.x
Transformers: 4.48.x
CUDA:         True
GPU:          NVIDIA GeForce RTX 5060
VRAM:         8.0 GB
Compute cap:  12.0
Compute test: OK (torch.Size([512, 512]))
===========================
```

- [ ] **Step 6: Commit**

```bash
git add pyproject.toml
git commit -m "chore: setup Python project with uv and pyproject.toml"
```

---

## Task 3: SDXL Basic Inference Validation

**Goal:** 在 RTX 5060 8GB 上跑通 SDXL 推理，确认图像生成管线可用，记录显存占用。

**Files:**
- Create: `utils/device.py`
- Create: `inference/generate.py`

- [ ] **Step 1: 创建 `utils/device.py`**

```python
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
        return torch.cuda.get_device_properties(0).total_mem / 1024**3
    return 0.0


def print_device_info() -> None:
    """Print device information."""
    device = get_device()
    print(f"Device: {device}")
    if device.type == "cuda":
        props = torch.cuda.get_device_properties(0)
        print(f"  GPU: {props.name}")
        print(f"  VRAM: {props.total_mem / 1024**3:.1f} GB")
        print(f"  Compute: sm_{props.major}{props.minor}")
    allocated = torch.cuda.memory_allocated(device) / 1024**3 if device.type == "cuda" else 0
    print(f"  Allocated: {allocated:.2f} GB")
```

- [ ] **Step 2: 创建 `inference/generate.py`**

```python
"""CLI tool for SDXL score image generation — supports three modes.

Modes:
  txt2img   — Pure diffusion from text prompt + LoRA (default)
  img2img   — Style transfer from a reference score image + LoRA
  controlnet — Guided generation using a structural layout + LoRA

All modes are image-level operations. No symbolic music tools involved.
"""

import sys
from pathlib import Path

import click
import torch
from PIL import Image

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from utils.device import get_device, get_dtype, get_vram_gb, print_device_info


def load_pipeline(mode: str, device, dtype, vram_gb: float, lora_path=None, lora_weight=0.8):
    """Load the appropriate pipeline for the given mode."""
    if mode == "controlnet":
        from diffusers import StableDiffusionXLControlNetPipeline, ControlNetModel, AutoencoderKL

        controlnet = ControlNetModel.from_pretrained(
            "diffusers/controlnet-canny-sdxl-1.0", torch_dtype=dtype
        )
        vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", torch_dtype=dtype)
        pipe = StableDiffusionXLControlNetPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            controlnet=controlnet, vae=vae, torch_dtype=dtype,
        )
    elif mode == "img2img":
        from diffusers import StableDiffusionXLImg2ImgPipeline

        pipe = StableDiffusionXLImg2ImgPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=dtype, variant="fp16",
        )
    else:  # txt2img
        from diffusers import StableDiffusionXLPipeline

        pipe = StableDiffusionXLPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=dtype, variant="fp16",
        )

    # Memory optimizations
    if vram_gb < 12:
        print(f"  VRAM {vram_gb:.0f}GB < 12GB → CPU offload + VAE tiling")
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    # Load LoRA
    if lora_path and Path(lora_path).exists():
        print(f"  LoRA: {Path(lora_path).name} (weight: {lora_weight})")
        pipe.load_lora_weights(lora_path, adapter_name="style")
        pipe.set_adapters(["style"], adapter_weights=[lora_weight])

    return pipe


@click.command()
@click.option("--mode", type=click.Choice(["txt2img", "img2img", "controlnet"]), default="txt2img",
              help="Generation mode")
@click.option("--prompt", default="contemporary classical music score, in the style of Boulez, for piano, serial composition, black ink on white paper",
              help="Text prompt")
@click.option("--negative-prompt", default="blurry, low quality, colored, photograph, watermark",
              help="Negative prompt")
@click.option("--lora", default=None, help="Path to LoRA .safetensors")
@click.option("--lora-weight", default=0.8, help="LoRA weight (0.0-1.0)")
@click.option("--ref-image", default=None, help="Reference image for img2img mode")
@click.option("--strength", default=0.55, help="img2img denoising strength (0.0=keep original, 1.0=pure noise)")
@click.option("--layout-image", default=None, help="Layout/edge image for ControlNet mode")
@click.option("--controlnet-scale", default=0.8, help="ControlNet conditioning scale")
@click.option("--steps", default=30, help="Inference steps")
@click.option("--cfg", default=7.0, help="CFG guidance scale")
@click.option("--seed", default=42, help="Random seed (-1 for random)")
@click.option("--width", default=1024, help="Image width (multiple of 8)")
@click.option("--height", default=1024, help="Image height (multiple of 8)")
@click.option("--output", default="outputs/generated.png", help="Output path")
@click.option("--batch", default=1, help="Number of images")
def main(mode, prompt, negative_prompt, lora, lora_weight, ref_image, strength,
         layout_image, controlnet_scale, steps, cfg, seed, width, height, output, batch):
    """Generate music score images using SDXL with three modes.

    \b
    Examples:
      # Pure diffusion (graphic notation)
      coda-generate --mode txt2img --lora weights/lora/boulez.safetensors

      # img2img from IMSLP reference (traditional notation)
      coda-generate --mode img2img --ref-image references/scriabin_page.png --strength 0.55

      # ControlNet with procedural layout (mixed notation)
      coda-generate --mode controlnet --layout-image layouts/staff_layout.png
    """
    device = get_device()
    dtype = get_dtype(device)
    vram = get_vram_gb()
    print_device_info()

    print(f"\nMode: {mode}")
    pipe = load_pipeline(mode, device, dtype, vram, lora, lora_weight)

    for i in range(batch):
        current_seed = seed + i if seed >= 0 else torch.randint(0, 2**32, (1,)).item()
        generator = torch.Generator(device=device).manual_seed(current_seed)

        print(f"\nGenerating {i + 1}/{batch} — seed {current_seed}")
        print(f"  Prompt: {prompt[:80]}...")

        if mode == "img2img":
            if not ref_image:
                print("ERROR: --ref-image required for img2img mode")
                return
            init_image = Image.open(ref_image).convert("RGB").resize((width, height))
            print(f"  Reference: {ref_image} (strength: {strength})")
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt, image=init_image,
                strength=strength, num_inference_steps=steps, guidance_scale=cfg,
                generator=generator,
            ).images[0]

        elif mode == "controlnet":
            if not layout_image:
                print("ERROR: --layout-image required for controlnet mode")
                return
            import cv2
            import numpy as np
            layout = cv2.imread(layout_image, cv2.IMREAD_GRAYSCALE)
            layout = cv2.resize(layout, (width, height))
            edges = cv2.Canny(layout, 50, 150)
            edge_image = Image.fromarray(edges)
            print(f"  Layout: {layout_image} (scale: {controlnet_scale})")
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt, image=edge_image,
                controlnet_conditioning_scale=controlnet_scale,
                num_inference_steps=steps, guidance_scale=cfg,
                width=width, height=height, generator=generator,
            ).images[0]

        else:  # txt2img
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt,
                num_inference_steps=steps, guidance_scale=cfg,
                width=width, height=height, generator=generator,
            ).images[0]

        output_path = Path(output)
        if batch > 1:
            output_path = output_path.parent / f"{output_path.stem}_{i:03d}{output_path.suffix}"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        image.save(output_path)
        print(f"  Saved: {output_path}")

    print("\nDone.")


if __name__ == "__main__":
    main()
```

- [ ] **Step 3: 运行 SDXL 基础推理（1024×1024）**

```powershell
uv run python inference/generate.py --output outputs/test_base_1024.png
```

> 如果 1024×1024 因显存不足失败（OOM），脚本会自动启用 CPU offload。如果仍然 OOM，用低分辨率重试：
> ```powershell
> uv run python inference/generate.py --width 512 --height 512 --output outputs/test_base_512.png
> ```

- [ ] **Step 4: 运行低分辨率推理（512×512）作为备份**

```powershell
uv run python inference/generate.py --width 512 --height 512 --output outputs/test_base_512.png
```

- [ ] **Step 5: 记录显存使用情况**

在推理过程中，打开另一个终端：

```powershell
nvidia-smi -l 2
```

记录到下表：

| 指标 | 值 |
|------|-----|
| 空闲显存 | ___GB |
| 1024² 推理峰值 | ___GB |
| 512² 推理峰值 | ___GB |
| 推理后残留 | ___GB |

- [ ] **Step 6: 检查生成图像质量**

打开 `outputs/test_base_1024.png`（或 `test_base_512.png`）。这是 SDXL 原生生成的乐谱——应该有类似乐谱的视觉（五线谱、音符轮廓），但细节不正确（乱码文字、错误符号数量等）。**这是预期的基线。**

- [ ] **Step 7: Commit**

```bash
git add utils/device.py utils/__init__.py inference/generate.py
git commit -m "feat: add SDXL inference CLI with memory optimization"
```

---

## Task 4: Data Pipeline — Image Processing

**Goal:** PDF → 标准化训练图像的完整预处理管线。

**Files:**
- Create: `utils/image.py`
- Create: `data/scripts/pdf_to_images.py`
- Create: `data/scripts/preprocess.py`
- Create: `tests/test_image.py`

- [ ] **Step 1: 创建 `utils/image.py`**

```python
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
```

- [ ] **Step 2: 创建 `data/scripts/pdf_to_images.py`**

```python
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
```

- [ ] **Step 3: 创建 `data/scripts/preprocess.py`**

```python
"""Preprocess score images for LoRA training."""

import sys
from pathlib import Path

import cv2
import click

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from utils.image import deskew, adaptive_threshold, normalize_score_image


@click.command()
@click.argument("input_dir", type=click.Path(exists=True, path_type=Path))
@click.argument("output_dir", type=click.Path(path_type=Path))
@click.option("--size", default=1024, help="Target square size in pixels")
@click.option("--deskew/--no-deskew", default=True, help="Correct scan rotation")
@click.option("--threshold/--no-threshold", default=False, help="Apply adaptive threshold (removes color)")
@click.option("--grayscale/--color", default=False, help="Convert to grayscale")
def main(input_dir: Path, output_dir: Path, size: int, deskew: bool, threshold: bool, grayscale: bool):
    """Preprocess score images for LoRA training.

    Reads PNGs from INPUT_DIR, applies preprocessing, saves to OUTPUT_DIR.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    image_files = sorted(input_dir.glob("*.png"))

    if not image_files:
        print(f"No PNG files found in {input_dir}")
        return

    print(f"Processing {len(image_files)} images → {size}x{size}")
    print(f"  Deskew: {deskew} | Threshold: {threshold} | Grayscale: {grayscale}")

    success = 0
    for img_path in image_files:
        image = cv2.imread(str(img_path))
        if image is None:
            print(f"  SKIP {img_path.name}: cannot read")
            continue

        if deskew:
            image = deskew(image)
        if threshold:
            image = adaptive_threshold(image)
        if grayscale:
            image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        image = normalize_score_image(image, target_size=(size, size))

        output_path = output_dir / img_path.name
        cv2.imwrite(str(output_path), image)
        success += 1

    print(f"\nDone: {success}/{len(image_files)} images processed → {output_dir}")


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: 创建 `tests/test_image.py`**

```python
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
        # Check center row has content, top/bottom rows are white
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
```

- [ ] **Step 5: 运行测试**

```powershell
uv run pytest tests/test_image.py -v
```

Expected: 7 tests pass.

- [ ] **Step 6: Commit**

```bash
git add utils/image.py data/scripts/pdf_to_images.py data/scripts/preprocess.py tests/test_image.py
git commit -m "feat: add image preprocessing pipeline (PDF→PNG, deskew, threshold, normalize)"
```

---

## Task 5: Data Pipeline — Dataset, Caption & Augmentation

**Goal:** PyTorch Dataset 类、风格分类体系、Caption 模板、数据增强。

**Files:**
- Create: `data/metadata/style_taxonomy.yaml`
- Create: `models/utils/caption.py`
- Create: `models/utils/augment.py`
- Create: `models/utils/dataset.py`
- Create: `data/scripts/label.py`
- Create: `tests/test_dataset.py`

- [ ] **Step 1: 创建 `data/metadata/style_taxonomy.yaml`**

```yaml
# coda-score Style Taxonomy
# Multi-dimensional annotation schema for modern music score images

periods:
  early_modern:
    label: "Early Modern"
    years: "1900-1945"
  post_war:
    label: "Post-War Avant-Garde"
    years: "1945-1970"
  late_20th:
    label: "Late 20th Century"
    years: "1970-2000"
  contemporary:
    label: "Contemporary"
    years: "2000-"

techniques:
  - serialism
  - integral_serialism
  - spectralism
  - stochastic
  - algorithmic
  - minimalism
  - post_minimalism
  - graphic_notation
  - text_score
  - proportional_notation
  - extended_techniques
  - microtonality
  - new_complexity
  - aleatoric
  - indeterminate
  - electronic
  - spatial_music
  - pointillism
  - collage
  - polystylism

instrumentation:
  solo: [piano, violin, cello, flute, clarinet, guitar, voice, percussion]
  chamber: [duo, trio, quartet, quintet, ensemble]
  large: [orchestral, choral, wind_ensemble]
  electronic: [tape, live_electronics, mixed]

notation_types:
  - traditional
  - proportional
  - graphic
  - text_score
  - mixed
  - tablature

page_types: [full_page, system, fragment, cover]
source_types: [scan, render, photo]
```

- [ ] **Step 2: 创建 `models/utils/caption.py`**

```python
"""Caption template generation for LoRA training.

Captions condition the diffusion model during training. The trigger word
activates the LoRA style; the rest describes visual characteristics.
"""

from dataclasses import dataclass, field


@dataclass
class ScoreMetadata:
    """Metadata for a single score image page."""

    composer: str
    work_title: str = ""
    period: str = ""
    techniques: list[str] = field(default_factory=list)
    instrumentation: str = ""
    notation_type: str = "traditional"
    page_type: str = "full_page"
    source_type: str = "scan"
    trigger_word: str = ""


PERIOD_LABELS = {
    "early_modern": "early 20th century",
    "post_war": "post-war avant-garde",
    "late_20th": "late 20th century",
    "contemporary": "contemporary",
}

TECHNIQUE_LABELS = {
    "serialism": "serial composition",
    "integral_serialism": "integral serialism",
    "spectralism": "spectral music",
    "stochastic": "stochastic composition",
    "algorithmic": "algorithmic composition",
    "graphic_notation": "graphic notation",
    "extended_techniques": "extended techniques",
    "microtonality": "microtonal notation",
    "new_complexity": "new complexity style",
    "minimalism": "minimalist composition",
    "proportional_notation": "proportional notation",
    "aleatoric": "aleatoric composition",
}

NOTATION_LABELS = {
    "traditional": "traditional staff notation",
    "proportional": "proportional notation",
    "graphic": "graphic notation",
    "mixed": "mixed notation",
    "text_score": "text score",
}


def generate_caption(meta: ScoreMetadata) -> str:
    """Generate a training caption from score metadata.

    The caption is a comma-separated description that conditions the
    diffusion model. It always starts with a base descriptor and ends
    with quality tags.

    Args:
        meta: Score metadata for this image.

    Returns:
        Caption string.
    """
    parts = ["contemporary classical music score"]

    # Trigger word or composer name
    if meta.trigger_word:
        parts.append(meta.trigger_word)
    elif meta.composer:
        parts.append(f"in the style of {meta.composer}")

    # Period
    if meta.period in PERIOD_LABELS:
        parts.append(PERIOD_LABELS[meta.period])

    # Techniques (max 3 to avoid overloading)
    tech_labels = [TECHNIQUE_LABELS.get(t, t) for t in meta.techniques[:3]]
    if tech_labels:
        parts.append(", ".join(tech_labels))

    # Instrumentation
    if meta.instrumentation:
        parts.append(f"for {meta.instrumentation}")

    # Notation type
    if meta.notation_type in NOTATION_LABELS:
        parts.append(NOTATION_LABELS[meta.notation_type])

    # Quality descriptors (always included)
    parts.append("high quality music engraving")
    parts.append("black ink on white paper")

    return ", ".join(parts)


# Pre-defined metadata templates for common styles
STYLE_TEMPLATES: dict[str, ScoreMetadata] = {
    "boulez": ScoreMetadata(
        composer="Pierre Boulez",
        period="post_war",
        techniques=["serialism", "integral_serialism"],
        instrumentation="piano",
        notation_type="traditional",
        trigger_word="in the style of Boulez",
    ),
    "xenakis": ScoreMetadata(
        composer="Iannis Xenakis",
        period="post_war",
        techniques=["stochastic", "algorithmic"],
        instrumentation="orchestral",
        notation_type="mixed",
        trigger_word="in the style of Xenakis",
    ),
    "graphic_notation": ScoreMetadata(
        composer="",
        period="post_war",
        techniques=["graphic_notation"],
        notation_type="graphic",
        trigger_word="graphic notation score",
    ),
    "ferneyhough": ScoreMetadata(
        composer="Brian Ferneyhough",
        period="late_20th",
        techniques=["new_complexity", "extended_techniques"],
        instrumentation="solo",
        notation_type="traditional",
        trigger_word="in the style of Ferneyhough",
    ),
}
```

- [ ] **Step 3: 创建 `models/utils/augment.py`**

```python
"""Data augmentation transforms for music score images.

Augmentations are specifically designed for scanned music scores:
- Slight rotation: simulates scan misalignment (±2°)
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
```

- [ ] **Step 4: 创建 `models/utils/dataset.py`**

```python
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
        ├── page_001.png
        ├── page_002.png
        └── ...

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

        # Find images
        self.image_paths = sorted(self.image_dir.glob("*.png"))
        if not self.image_paths:
            raise ValueError(f"No PNG images found in {image_dir}")

        # Load metadata
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
```

- [ ] **Step 5: 创建 `data/scripts/label.py`**

```python
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

    print(f"Generated {len(annotations)} annotations → {output_file}")
    print(f"  Composer: {composer}")
    print(f"  Trigger: {trigger_word}")


if __name__ == "__main__":
    main()
```

- [ ] **Step 6: 创建 `tests/test_dataset.py`**

```python
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
```

- [ ] **Step 7: 运行所有测试**

```powershell
uv run pytest tests/ -v
```

Expected: 所有测试通过（约 11 tests）。

- [ ] **Step 8: Commit**

```bash
git add models/utils/ data/metadata/style_taxonomy.yaml data/scripts/label.py tests/test_dataset.py
git commit -m "feat: add ScoreDataset, caption templates, augmentation, and batch labeler"
```

---

## Task 6: Data Collection — Boulez Piano Works (50 pages)

**Goal:** 收集并预处理至少 50 页 Boulez 钢琴作品乐谱图像。

> ⚠️ 这一步需要你手动操作。我只能提供工具、建议和追踪。

- [ ] **Step 1: 获取 Boulez 钢琴作品乐谱**

**优先获取清单（按数据量排序）：**

| 作品 | 预估页数 | 获取方式 | 视觉特征 |
|------|---------|---------|----------|
| *Deuxième Sonate* (1948) | ~48 页 | Henle/Universal Edition 购买 | 序列主义经典，密度高 |
| *12 Notations* (1945) | ~12 页 | 图书馆/购买 | 早期12音，较规整 |
| *Première Sonate* (1946) | ~20 页 | 图书馆/购买 | 爆发力强 |

**目标：*Deuxième Sonate* (48页) 即可开始，补 *12 Notations* 凑 60 页更理想。**

- [ ] **Step 2: 将 PDF 放入 raw 目录**

```
data/raw/boulez/
├── boulez_sonate2.pdf
└── boulez_12notations.pdf
```

- [ ] **Step 3: PDF 转图像**

```powershell
uv run python data/scripts/pdf_to_images.py data/raw/boulez/boulez_sonate2.pdf -o data/raw/boulez/sonate2/
uv run python data/scripts/pdf_to_images.py data/raw/boulez/boulez_12notations.pdf -o data/raw/boulez/12notations/
```

- [ ] **Step 4: 手动检查并清理**

打开 `data/raw/boulez/sonate2/` 和 `12notations/` 中的图像：
- 删除空白页、版权页、目录页、注释页
- 删除严重模糊/倾斜/有污渍的页面
- 确保保留的都是乐谱页面
- **目标：至少 50 页清晰乐谱**

- [ ] **Step 5: 预处理**

```powershell
uv run python data/scripts/preprocess.py data/raw/boulez/sonate2/ data/processed/boulez/ --size 1024
uv run python data/scripts/preprocess.py data/raw/boulez/12notations/ data/processed/boulez/ --size 1024
```

- [ ] **Step 6: 生成标注文件**

```powershell
uv run python data/scripts/label.py data/processed/boulez/ data/metadata/annotations/boulez.json --composer "Pierre Boulez" --work "Piano Works" --techniques "serialism,integral_serialism" --instrumentation "piano"
```

- [ ] **Step 7: 验证数据集加载**

```powershell
uv run python -c "
from models.utils.dataset import ScoreDataset
ds = ScoreDataset('data/processed/boulez/', 'data/metadata/annotations/boulez.json', resolution=512)
print(f'Dataset size: {len(ds)}')
sample = ds[0]
print(f'Image shape: {sample[\"pixel_values\"].shape}')
print(f'Caption: {sample[\"caption\"]}')
assert len(ds) >= 50, f'Need ≥50 images, got {len(ds)}'
print('✅ Dataset validation passed!')
"
```

- [ ] **Step 8: Commit 标注和工具（不含图像数据）**

```bash
git add data/scripts/label.py data/metadata/annotations/boulez.json data/metadata/style_taxonomy.yaml
git commit -m "feat: add batch labeler and Boulez annotation template"
```

---

## Task 7: LoRA Training Script

**Goal:** 基于 Diffusers + PEFT 的 SDXL LoRA 训练脚本。

**Files:**
- Create: `models/configs/lora_boulez.yaml`
- Create: `models/train_lora.py`

- [ ] **Step 1: 创建 `models/configs/lora_boulez.yaml`**

```yaml
# LoRA training config for Boulez piano works
# Phase 0: Concept validation on RTX 5060 8GB at 512x512
# Phase 1 (RTX 5080 16GB): switch resolution to 768 or 1024

model:
  base: "stabilityai/stable-diffusion-xl-base-1.0"
  variant: "fp16"

lora:
  rank: 32          # 16-64, scores have simple texture
  alpha: 16         # typically half of rank
  dropout: 0.05
  target_modules:
    - "to_k"
    - "to_q"
    - "to_v"
    - "to_out.0"

training:
  # RTX 5060 8GB → 512; RTX 5080 16GB → 768 or 1024
  resolution: 512
  batch_size: 1
  gradient_accumulation_steps: 4
  learning_rate: 2.0e-4
  lr_scheduler: "constant_with_warmup"
  lr_warmup_steps: 100
  max_steps: 2000
  mixed_precision: "fp16"
  optimizer: "adamw_8bit"    # bitsandbytes 8-bit AdamW
  gradient_checkpointing: true
  max_grad_norm: 1.0
  seed: 42

data:
  train_dir: "data/processed/boulez/"
  metadata_file: "data/metadata/annotations/boulez.json"
  trigger_word: "in the style of Boulez"

output:
  dir: "weights/lora/"
  name: "coda-score-boulez"
  save_every_n_steps: 500

validation:
  enabled: true
  every_n_steps: 500
  resolution: 512
  steps: 30
  cfg: 7.0
  prompts:
    - "contemporary classical music score, in the style of Boulez, for piano, serial composition, black ink on white paper"
    - "modern avant-garde piano score, in the style of Boulez, dense atonal notation, complex rhythms"
    - "contemporary classical music score, black ink on white paper, traditional engraving"
```

- [ ] **Step 2: 创建 `models/train_lora.py`**

> 这个训练脚本基于 HuggingFace Diffusers 官方 LoRA 训练示例，针对乐谱图像做了定制化。完整代码较长（~400行），以下为核心结构：

```python
"""SDXL LoRA training script for music score style transfer.

Trains a LoRA adapter on SDXL's UNet using music score images.
The base SDXL model weights are frozen; only the LoRA parameters
(~20-40M params, < 200MB) are trained.

Usage:
    uv run python models/train_lora.py --config models/configs/lora_boulez.yaml
"""

import logging
import sys
from pathlib import Path

import click
import torch
import torch.nn.functional as F
import yaml
from accelerate import Accelerator
from accelerate.utils import set_seed
from diffusers import AutoencoderKL, DDPMScheduler, UNet2DConditionModel
from diffusers.optimization import get_scheduler
from diffusers.training_utils import cast_training_params
from peft import LoraConfig
from peft.utils import get_peft_model_state_dict
from safetensors.torch import save_file
from torch.utils.data import DataLoader
from tqdm.auto import tqdm
from transformers import AutoTokenizer, CLIPTextModel, CLIPTextModelWithProjection

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from models.utils.caption import ScoreMetadata
from models.utils.dataset import ScoreDataset

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")


def load_models(base_model: str, variant: str = "fp16"):
    """Load frozen SDXL components."""
    dtype = torch.float16 if variant == "fp16" else torch.float32

    noise_scheduler = DDPMScheduler.from_pretrained(base_model, subfolder="scheduler")
    tokenizer_one = AutoTokenizer.from_pretrained(base_model, subfolder="tokenizer", use_fast=False)
    tokenizer_two = AutoTokenizer.from_pretrained(base_model, subfolder="tokenizer_2", use_fast=False)
    text_encoder_one = CLIPTextModel.from_pretrained(base_model, subfolder="text_encoder", torch_dtype=dtype)
    text_encoder_two = CLIPTextModelWithProjection.from_pretrained(base_model, subfolder="text_encoder_2", torch_dtype=dtype)
    vae = AutoencoderKL.from_pretrained(base_model, subfolder="vae", torch_dtype=dtype)
    unet = UNet2DConditionModel.from_pretrained(base_model, subfolder="unet", torch_dtype=dtype)

    # Freeze everything
    for model in [vae, text_encoder_one, text_encoder_two, unet]:
        model.requires_grad_(False)

    return noise_scheduler, (tokenizer_one, tokenizer_two), (text_encoder_one, text_encoder_two), vae, unet


def encode_prompt_sdxl(tokenizers, text_encoders, prompt: str, device: torch.device):
    """Encode a prompt through both SDXL text encoders and concatenate."""
    embeds = []
    pooled = None
    for tokenizer, encoder in zip(tokenizers, text_encoders):
        inputs = tokenizer(prompt, padding="max_length", max_length=tokenizer.model_max_length,
                           truncation=True, return_tensors="pt")
        input_ids = inputs.input_ids.to(device)
        with torch.no_grad():
            outputs = encoder(input_ids, output_hidden_states=True)
        embeds.append(outputs.hidden_states[-2])
        if pooled is None:
            pooled = outputs[0]  # First encoder's pooled output
    return torch.cat(embeds, dim=-1), pooled


def add_noise(latents, noise, timesteps, scheduler):
    """Add noise to latents at given timesteps (forward diffusion)."""
    return scheduler.add_noise(latents, noise, timesteps)


def compute_sdxl_add_time_ids(resolution: int, device: torch.device, dtype: torch.dtype):
    """Create the add_time_ids tensor for SDXL conditioning."""
    original_size = (resolution, resolution)
    crops_coords_top_left = (0, 0)
    target_size = (resolution, resolution)
    return torch.tensor([original_size + crops_coords_top_left + target_size],
                        device=device, dtype=dtype)


def save_lora_weights(unet, path: Path):
    """Save LoRA weights as safetensors."""
    path.parent.mkdir(parents=True, exist_ok=True)
    state_dict = get_peft_model_state_dict(unet)
    save_file(state_dict, str(path))
    logger.info(f"Saved LoRA weights → {path}")


def generate_validation(unet, vae, text_encoders, tokenizers, noise_scheduler,
                        prompts, resolution, steps, cfg, seed, output_dir, device, dtype):
    """Generate validation images at a training checkpoint."""
    from diffusers import StableDiffusionXLPipeline

    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    # Build a temporary pipeline with the current UNet (including LoRA)
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        unet=unet,
        vae=vae,
        text_encoder=text_encoders[0],
        text_encoder_2=text_encoders[1],
        tokenizer=tokenizers[0],
        tokenizer_2=tokenizers[1],
        torch_dtype=dtype,
    )
    pipe.enable_model_cpu_offload()
    generator = torch.Generator(device=device).manual_seed(seed)

    for i, prompt in enumerate(prompts):
        image = pipe(prompt=prompt, num_inference_steps=steps, guidance_scale=cfg,
                     width=resolution, height=resolution, generator=generator).images[0]
        image.save(output_dir / f"val_{i:02d}.png")
        logger.info(f"  Validation: {prompt[:50]}... → {output_dir / f'val_{i:02d}.png'}")

    del pipe
    torch.cuda.empty_cache()


@click.command()
@click.option("--config", required=True, type=click.Path(exists=True), help="YAML config file")
def main(config: str):
    """Train a LoRA adapter on SDXL for music score style transfer."""
    # Load config
    with open(config) as f:
        cfg = yaml.safe_load(f)

    model_cfg, lora_cfg, train_cfg = cfg["model"], cfg["lora"], cfg["training"]
    data_cfg, output_cfg, val_cfg = cfg["data"], cfg["output"], cfg.get("validation", {})

    resolution = train_cfg["resolution"]

    # Accelerator
    accelerator = Accelerator(
        mixed_precision=train_cfg["mixed_precision"],
        gradient_accumulation_steps=train_cfg["gradient_accumulation_steps"],
    )

    logger.info(f"=== coda-score LoRA Training ===")
    logger.info(f"Base model: {model_cfg['base']}")
    logger.info(f"LoRA rank: {lora_cfg['rank']}, alpha: {lora_cfg['alpha']}")
    logger.info(f"Resolution: {resolution}x{resolution}")
    logger.info(f"Max steps: {train_cfg['max_steps']}")
    logger.info(f"Learning rate: {train_cfg['learning_rate']}")
    logger.info(f"Optimizer: {train_cfg['optimizer']}")

    if train_cfg.get("seed"):
        set_seed(train_cfg["seed"])

    # Load models
    logger.info("Loading SDXL models...")
    noise_scheduler, tokenizers, text_encoders, vae, unet = load_models(
        model_cfg["base"], model_cfg.get("variant", "fp16")
    )

    # Attach LoRA
    lora_config = LoraConfig(
        r=lora_cfg["rank"],
        lora_alpha=lora_cfg["alpha"],
        target_modules=lora_cfg["target_modules"],
        lora_dropout=lora_cfg.get("dropout", 0.05),
    )
    unet.add_adapter(lora_config)
    cast_training_params(unet, dtype=torch.float32)

    if train_cfg.get("gradient_checkpointing"):
        unet.enable_gradient_checkpointing()

    # Count trainable params
    trainable = sum(p.numel() for p in unet.parameters() if p.requires_grad)
    total = sum(p.numel() for p in unet.parameters())
    logger.info(f"Trainable: {trainable / 1e6:.2f}M / {total / 1e6:.0f}M ({100 * trainable / total:.2f}%)")

    # Optimizer
    lora_params = [p for p in unet.parameters() if p.requires_grad]
    if train_cfg["optimizer"] == "adamw_8bit":
        import bitsandbytes as bnb
        optimizer = bnb.optim.AdamW8bit(lora_params, lr=train_cfg["learning_rate"])
    else:
        optimizer = torch.optim.AdamW(lora_params, lr=train_cfg["learning_rate"])

    # LR scheduler
    lr_scheduler = get_scheduler(
        train_cfg.get("lr_scheduler", "constant_with_warmup"),
        optimizer=optimizer,
        num_warmup_steps=train_cfg.get("lr_warmup_steps", 100),
        num_training_steps=train_cfg["max_steps"],
    )

    # Dataset
    logger.info(f"Loading dataset from {data_cfg['train_dir']}...")
    default_meta = ScoreMetadata(
        composer="Pierre Boulez",
        trigger_word=data_cfg.get("trigger_word", "in the style of Boulez"),
    )
    dataset = ScoreDataset(
        image_dir=data_cfg["train_dir"],
        metadata_file=data_cfg.get("metadata_file"),
        resolution=resolution,
        split="train",
        default_metadata=default_meta,
    )
    logger.info(f"Dataset: {len(dataset)} images")

    dataloader = DataLoader(dataset, batch_size=train_cfg["batch_size"], shuffle=True, num_workers=0)

    # Prepare with Accelerator
    unet, optimizer, dataloader, lr_scheduler = accelerator.prepare(unet, optimizer, dataloader, lr_scheduler)

    # Move frozen models to device
    weight_dtype = torch.float16 if train_cfg["mixed_precision"] == "fp16" else torch.float32
    vae.to(accelerator.device, dtype=weight_dtype)
    text_encoders[0].to(accelerator.device, dtype=weight_dtype)
    text_encoders[1].to(accelerator.device, dtype=weight_dtype)

    # Pre-compute add_time_ids (same for all samples at fixed resolution)
    add_time_ids = compute_sdxl_add_time_ids(resolution, accelerator.device, weight_dtype)

    # Training loop
    logger.info("Starting training...")
    global_step = 0
    output_dir = Path(output_cfg["dir"]) / output_cfg["name"]
    data_iter = iter(dataloader)

    progress_bar = tqdm(range(train_cfg["max_steps"]), disable=not accelerator.is_main_process, desc="Training")

    while global_step < train_cfg["max_steps"]:
        # Get batch (cycle through dataset)
        try:
            batch = next(data_iter)
        except StopIteration:
            data_iter = iter(dataloader)
            batch = next(data_iter)

        with accelerator.accumulate(unet):
            # 1. Encode images → latents
            pixel_values = batch["pixel_values"].to(accelerator.device, dtype=weight_dtype)
            with torch.no_grad():
                latents = vae.encode(pixel_values).latent_dist.sample() * vae.config.scaling_factor

            # 2. Sample noise + timesteps
            noise = torch.randn_like(latents)
            timesteps = torch.randint(
                0, noise_scheduler.config.num_train_timesteps,
                (latents.shape[0],), device=accelerator.device,
            ).long()
            noisy_latents = add_noise(latents, noise, timesteps, noise_scheduler)

            # 3. Encode text
            caption = batch["caption"][0] if isinstance(batch["caption"], list) else batch["caption"]
            prompt_embeds, pooled = encode_prompt_sdxl(tokenizers, text_encoders, caption, accelerator.device)

            # 4. UNet prediction
            added_cond_kwargs = {"text_embeds": pooled, "time_ids": add_time_ids}
            model_pred = unet(noisy_latents, timesteps, prompt_embeds,
                              added_cond_kwargs=added_cond_kwargs).sample

            # 5. Loss
            loss = F.mse_loss(model_pred.float(), noise.float(), reduction="mean")

            # 6. Backward + step
            accelerator.backward(loss)
            if accelerator.sync_gradients:
                accelerator.clip_grad_norm_(lora_params, train_cfg.get("max_grad_norm", 1.0))
            optimizer.step()
            lr_scheduler.step()
            optimizer.zero_grad()

        if accelerator.sync_gradients:
            global_step += 1
            progress_bar.update(1)
            progress_bar.set_postfix(loss=f"{loss.item():.4f}", lr=f"{lr_scheduler.get_last_lr()[0]:.2e}")

            # Checkpoint + validation
            if global_step % output_cfg["save_every_n_steps"] == 0:
                ckpt_path = output_dir / f"checkpoint-{global_step}.safetensors"
                save_lora_weights(accelerator.unwrap_model(unet), ckpt_path)

                if val_cfg.get("enabled"):
                    logger.info(f"Generating validation images at step {global_step}...")
                    generate_validation(
                        accelerator.unwrap_model(unet), vae, text_encoders, tokenizers,
                        noise_scheduler, val_cfg["prompts"], val_cfg.get("resolution", resolution),
                        val_cfg.get("steps", 30), val_cfg.get("cfg", 7.0), train_cfg.get("seed", 42),
                        output_dir / f"val-{global_step}", accelerator.device, weight_dtype,
                    )

    # Final save
    final_path = output_dir / f"{output_cfg['name']}.safetensors"
    save_lora_weights(accelerator.unwrap_model(unet), final_path)
    logger.info(f"\nTraining complete! Final model: {final_path}")
    logger.info(f"Load with: pipe.load_lora_weights('{final_path}')")


if __name__ == "__main__":
    main()
```

- [ ] **Step 3: Commit**

```bash
git add models/configs/lora_boulez.yaml models/train_lora.py
git commit -m "feat: add SDXL LoRA training script with validation image generation"
```

---

## Task 8: LoRA Training Run

**Goal:** 训练 Boulez LoRA，记录训练指标。

> ⚠️ **前置条件：** Task 6 数据收集完成（≥50 页已预处理）。

- [ ] **Step 1: 确认数据完整性**

```powershell
uv run python -c "
from models.utils.dataset import ScoreDataset
ds = ScoreDataset('data/processed/boulez/', 'data/metadata/annotations/boulez.json', resolution=512)
print(f'✅ Dataset: {len(ds)} images, shape: {ds[0][\"pixel_values\"].shape}')
assert len(ds) >= 50
"
```

- [ ] **Step 2: 开始训练**

```powershell
uv run python models/train_lora.py --config models/configs/lora_boulez.yaml
```

**监控（另一个终端）：**

```powershell
nvidia-smi -l 2
```

- [ ] **Step 3: 如果 OOM（显存不足），调整参数**

编辑 `models/configs/lora_boulez.yaml`：

```yaml
training:
  resolution: 448    # 降分辨率（必须是 64 的倍数：448 = 64×7）
lora:
  rank: 16           # 降 rank
```

或更激进：

```yaml
training:
  resolution: 384    # 64×6
lora:
  rank: 8
```

重新运行训练。

- [ ] **Step 4: 记录训练结果**

| 指标 | 值 |
|------|-----|
| 训练分辨率 | ___×___ |
| LoRA rank | ___ |
| 总步数 | ___ |
| 训练耗时 | ___ |
| 最终 loss | ___ |
| 峰值显存 | ___GB |

检查 checkpoint 验证图像：

```
weights/lora/coda-score-boulez/
├── checkpoint-500.safetensors
├── val-500/
│   ├── val_00.png   ← prompt 1 (有 trigger word)
│   ├── val_01.png   ← prompt 2 (有 trigger word)
│   └── val_02.png   ← prompt 3 (无 trigger word = 对照组)
├── checkpoint-1000.safetensors
├── val-1000/
├── ...
└── coda-score-boulez.safetensors  ← 最终模型
```

- [ ] **Step 5: Commit 训练配置（如果有调整）**

```bash
git add models/configs/lora_boulez.yaml
git commit -m "chore: update LoRA training config after Phase 0 tuning"
```

---

## Task 9: Procedural Layout Generator (for ControlNet)

**Goal:** 用 OpenCV 程序化生成空白五线谱布局图，作为 ControlNet 的结构引导。

**Files:**
- Create: `inference/layout.py`

- [ ] **Step 1: 创建 `inference/layout.py`**

```python
"""Procedural music score layout generator for ControlNet conditioning.

Generates blank staff line layouts using OpenCV — no music theory involved.
The output is a simple structural sketch: horizontal lines grouped into
staff systems, with margins and basic page layout.

This is used as ControlNet conditioning input: the diffusion model
receives the staff line structure and fills in the musical content
entirely through learned visual patterns.
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
    line_color: int = 0,  # black on white
    bg_color: int = 255,  # white background
) -> np.ndarray:
    """Generate a blank music staff layout image.

    Args:
        width: Image width in pixels.
        height: Image height in pixels.
        num_systems: Number of staff systems (rows of staves).
        lines_per_staff: Lines per staff (5 for standard, can vary for modern).
        line_spacing: Pixel gap between staff lines.
        margin_top/bottom/left/right: Page margins.
        line_thickness: Staff line thickness in pixels.
        line_color: Line color (0=black).
        bg_color: Background color (255=white).

    Returns:
        Grayscale image with staff lines drawn.
    """
    canvas = np.full((height, width), bg_color, dtype=np.uint8)

    # Calculate available space
    usable_height = height - margin_top - margin_bottom
    usable_width = width - margin_left - margin_right

    # Distribute systems evenly
    staff_height = (lines_per_staff - 1) * line_spacing
    system_spacing = usable_height // num_systems

    for sys_idx in range(num_systems):
        y_start = margin_top + sys_idx * system_spacing

        for line_idx in range(lines_per_staff):
            y = y_start + line_idx * line_spacing
            cv2.line(
                canvas,
                (margin_left, y),
                (width - margin_right, y),
                line_color,
                line_thickness,
            )

        # Optional: draw a thin bracket/barline at the left edge
        cv2.line(
            canvas,
            (margin_left, y_start),
            (margin_left, y_start + staff_height),
            line_color,
            line_thickness + 1,
        )

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
    """Generate a piano score layout with grand staff (treble + bass per system).

    Each system has two staves: upper (treble) and lower (bass), connected
    by a brace on the left.
    """
    canvas = np.full((height, width), 255, dtype=np.uint8)

    usable_height = height - margin_top - margin_bottom
    system_height = 2 * 4 * line_spacing + gap_between_staves  # 2 staves × 5 lines
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

        # Brace connecting staves
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
    """Generate a layout for graphic notation — rectangular regions instead of staff lines.

    Modern graphic scores (Cardew, Bussotti, Crumb) often use spatial
    regions rather than staff systems. This generates a simple regional layout.
    """
    canvas = np.full((height, width), 255, dtype=np.uint8)

    usable_h = height - 2 * margin
    usable_w = width - 2 * margin

    # Divide into horizontal bands
    band_height = usable_h // num_regions
    for i in range(num_regions):
        y = margin + i * band_height
        # Draw region boundary (dashed effect using dotted lines)
        cv2.rectangle(canvas, (margin, y), (margin + usable_w, y + band_height - 10), 180, 1)
        # Add a subtle center line as a time axis
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
    print(f"Generated {layout_type} layout → {output_path} ({width}x{height})")


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: 生成测试布局图**

```powershell
# 标准五线谱布局（8 个系统）
uv run python inference/layout.py --type staff --systems 8 --output outputs/layout_staff.png

# 钢琴大谱表布局（6 个系统，每组双行）
uv run python inference/layout.py --type piano --systems 6 --output outputs/layout_piano.png

# 图形记谱布局（4 个区域）
uv run python inference/layout.py --type graphic --systems 4 --output outputs/layout_graphic.png
```

打开生成的图像确认：应该是白色背景上的黑色水平线。

- [ ] **Step 3: Commit**

```bash
git add inference/layout.py
git commit -m "feat: add procedural score layout generator for ControlNet conditioning"
```

---

## Task 10: Validation & Phase 0 Report

**Goal:** 对比三种生成模式在有/无 LoRA 情况下的效果，评估 Phase 0 判定标准。

**Files:**
- Create: `inference/validate.py`
- Create: `docs/phase0-report.md`

- [ ] **Step 1: 创建 `inference/validate.py`**

```python
"""Phase 0 validation: side-by-side comparison of base SDXL vs LoRA."""

import sys
from pathlib import Path

import click
import torch

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from utils.device import get_device, get_dtype, get_vram_gb

VALIDATION_PROMPTS = [
    # Prompt 1: With trigger word — should show LoRA style
    "contemporary classical music score, in the style of Boulez, for piano, serial composition, black ink on white paper, high quality engraving",
    # Prompt 2: With trigger word — different aspect
    "modern avant-garde piano score, in the style of Boulez, dense atonal notation, complex rhythms, black ink on white paper",
    # Prompt 3: WITHOUT trigger word — should NOT show LoRA style (control)
    "contemporary classical music score, black ink on white paper, traditional music engraving",
    # Prompt 4: Completely different — tests LoRA specificity
    "a photograph of a cat sitting on a piano",
]


@click.command()
@click.option("--lora", required=True, type=click.Path(exists=True), help="LoRA .safetensors path")
@click.option("--output-dir", default="outputs/validation", help="Output directory")
@click.option("--resolution", default=512, help="Image resolution")
@click.option("--seed", default=42, help="Random seed")
def main(lora, output_dir, resolution, seed):
    """Generate comparison images: base SDXL vs SDXL + LoRA."""
    from diffusers import StableDiffusionXLPipeline

    device = get_device()
    dtype = get_dtype(device)
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    # --- Base SDXL ---
    print("=== Base SDXL (no LoRA) ===")
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=dtype, variant="fp16",
    )
    if get_vram_gb() < 12:
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    gen = torch.Generator(device=device).manual_seed(seed)
    for i, prompt in enumerate(VALIDATION_PROMPTS):
        img = pipe(prompt=prompt, num_inference_steps=30, guidance_scale=7.0,
                   width=resolution, height=resolution, generator=gen).images[0]
        img.save(output_dir / f"base_{i:02d}.png")
        print(f"  base_{i:02d}.png — {prompt[:60]}...")

    del pipe
    torch.cuda.empty_cache()

    # --- SDXL + LoRA ---
    print(f"\n=== SDXL + LoRA ({Path(lora).name}) ===")
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=dtype, variant="fp16",
    )
    pipe.load_lora_weights(lora)
    if get_vram_gb() < 12:
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    gen = torch.Generator(device=device).manual_seed(seed)
    for i, prompt in enumerate(VALIDATION_PROMPTS):
        img = pipe(prompt=prompt, num_inference_steps=30, guidance_scale=7.0,
                   width=resolution, height=resolution, generator=gen).images[0]
        img.save(output_dir / f"lora_{i:02d}.png")
        print(f"  lora_{i:02d}.png — {prompt[:60]}...")

    print(f"\n✅ All images saved to {output_dir}/")
    print("Compare base_00/01 vs lora_00/01 (with trigger) and base_02 vs lora_02 (control)")


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: 运行三种模式验证**

```powershell
# 模式 1: 纯扩散 txt2img（有/无 LoRA 对比）
uv run python inference/validate.py --lora weights/lora/coda-score-boulez/coda-score-boulez.safetensors

# 模式 2: img2img（需要先放入一张 IMSLP 公共领域参考图）
# 从 IMSLP 下载一张 Scriabin/Schoenberg 钢琴谱页面到 references/ 目录
uv run python inference/generate.py --mode img2img --ref-image references/scriabin_page.png --strength 0.55 --lora weights/lora/coda-score-boulez/coda-score-boulez.safetensors --output outputs/val_img2img.png

# 模式 3: ControlNet（使用程序化布局）
uv run python inference/layout.py --type piano --systems 6 --output outputs/layout_piano.png
uv run python inference/generate.py --mode controlnet --layout-image outputs/layout_piano.png --lora weights/lora/coda-score-boulez/coda-score-boulez.safetensors --output outputs/val_controlnet.png
```

> ⚠️ RTX 5060 8GB 上 ControlNet + SDXL + LoRA 组合推理可能显存不足。如果 OOM，跳过 ControlNet 验证，等 RTX 5080 到货后补做。

- [ ] **Step 3: 人工评估（Phase 0 判定标准）**

打开 `outputs/validation/`，逐项检查：

| # | 判定标准 | 如何检查 | 结果 |
|---|---------|---------|------|
| ① | 五线谱密度差异 | 对比 `base_00` vs `lora_00`：LoRA 版五线谱是否更密、音符更分散？ | ✅/❌ |
| ② | 版面风格差异 | 对比 `base_01` vs `lora_01`：页面布局（系统数量、密度分布）是否不同？ | ✅/❌ |
| ③ | 记谱惯例差异 | LoRA 版是否出现 base 不会出现的特征（无调号、临时记号密集等）？ | ✅/❌ |
| ④ | 专家认证 | 你作为现代音乐专业人士，看两眼能说「这确实有点像 Boulez」？ | ✅/❌ |

**成功条件：至少满足 2/4。**

**对照组检查：** `base_02` vs `lora_02`（无 trigger word）应该**没有**明显差异——说明 LoRA 只在触发词激活时才生效。

**三种模式比较：**

| 模式 | 适用场景 | 五线谱结构 | 风格控制 | 显存需求 |
|------|---------|-----------|---------|---------|
| txt2img | 图形记谱 | 模型自己学 | 纯文本+LoRA | 最低 |
| img2img | 传统记谱 | 从参考继承 | 参考+LoRA | 中等 |
| ControlNet | 混合记谱 | 程序化布局 | 布局+LoRA | 最高 |

- [ ] **Step 4: 撰写 Phase 0 报告**

```bash
# 创建报告模板
```

将以下内容写入 `docs/phase0-report.md`：

```markdown
# Phase 0 Report: Environment & Minimal Validation

## 环境
| 项目 | 值 |
|------|-----|
| GPU | RTX 5060 8GB |
| PyTorch | ___ |
| CUDA | ___ |
| Diffusers | ___ |

## SDXL 推理
- 1024² 推理：✅/❌ (显存: ___GB)
- 512² 推理：✅/❌ (显存: ___GB)

## 数据集
- Boulez 页面收集：___页
- 预处理后：___页
- 训练分辨率：___×___

## LoRA 训练
| 参数 | 值 |
|------|-----|
| Resolution | ___×___ |
| Rank | ___ |
| Steps | ___ |
| Training time | ___ |
| Final loss | ___ |
| Peak VRAM | ___GB |

## 验证结果
### 判定标准
- [ ] ① 五线谱密度差异
- [ ] ② 版面风格差异
- [ ] ③ 记谱惯例差异
- [ ] ④ 专家认证

**结果：___/4 通过**

### 图像对比
(在此插入 base vs lora 对比图)

## 结论
- [ ] ✅ PASS — 进入 Phase 1
- [ ] ⚠️ PARTIAL — 调整参数后重新验证
- [ ] ❌ FAIL — 评估是否需要改变路线

## 下一步
(基于结果填写)
```

- [ ] **Step 5: Final Commit**

```bash
git add inference/validate.py docs/phase0-report.md
git commit -m "docs: add Phase 0 validation script and report template"
```

---

## Task Dependency Graph

```
Task 1 (Migration)
  └→ Task 2 (Python Env)
       ├→ Task 3 (SDXL Inference — 三种模式)  ← RTX 5060 首次推理
       ├→ Task 4 (Data Pipeline)
       └→ Task 5 (Dataset & Caption)
              └→ Task 6 (Boulez Data)  ← 手动收集，最大瓶颈
                     ├→ Task 7 (LoRA Script)
                     └→ Task 8 (LoRA Training)
                            ├→ Task 9 (Layout Generator)  ← 可与 Task 3 并行
                            └→ Task 10 (Validation & Report)
```

**关键路径：** Task 6 (数据收集) → Task 8 (训练) → Task 10 (验证)

**预计时间线：**

| Task | 耗时 | 依赖 |
|------|------|------|
| 1. Migration | ~10 min | — |
| 2. Python Env | 30-60 min | PyTorch + CUDA 验证 |
| 3. SDXL Inference | 30-60 min | 模型下载 + 三种模式测试 |
| 4. Data Pipeline | ~30 min | — |
| 5. Dataset & Caption | ~30 min | — |
| **6. Boulez Data** | **1-2 周** | **购买/扫描/处理** |
| 7. LoRA Script | ~15 min | — |
| 8. LoRA Training | 2-4 小时 | 512² on 5060 |
| 9. Layout Generator | ~15 min | — |
| 10. Validation | ~45 min | 三种模式 × 有/无 LoRA |
| **Total** | **2-4 周** | **瓶颈：数据收集** |

> Tasks 1-5、7、9 可以**立即开始**，不依赖数据或 GPU 训练能力。Task 6 是最大瓶颈。
>
> **三种生成模式的使用场景：**
> - **txt2img**：图形记谱（Cardew, Crumb）— 不需要五线谱结构
> - **img2img**：传统五线谱（Boulez, Xenakis）— 用 IMSLP 公共领域乐谱作参考
> - **ControlNet**：混合记谱 — 程序化布局 + LoRA 风格填充
