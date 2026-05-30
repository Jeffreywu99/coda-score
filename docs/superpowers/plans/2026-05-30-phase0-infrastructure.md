# Phase 0: 基础设施搭建 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将项目从 Tauri/React/TypeScript 完全转型为 Python/PyTorch 项目，搭建 v2.0 所需的基础项目结构和工具链。

**Architecture:** 采用标准 Python 项目布局，使用 `uv` 管理依赖，`pyproject.toml` 定义项目元数据。保留 `study/` 和 `docs/` 目录，归档 v1.x 代码到 `v1-archive` 分支，新建 Python 项目结构。

**Tech Stack:** Python 3.11+, uv, PyTorch 2.x, HuggingFace Diffusers, Gradio

---

## File Structure

```
coda-score/
├── docs/                           # 保留：文档
│   ├── plan-v2.0.md               # 重构方案
│   ├── research-survey.md         # 调研文档
│   └── superpowers/plans/         # 实现计划
├── study/                          # 保留：知识文档
├── data/                           # 新建：数据处理
│   ├── scripts/
│   │   ├── __init__.py
│   │   ├── scrape_imslp.py        # IMSLP 爬取
│   │   ├── preprocess.py          # 图像预处理
│   │   ├── label.py               # 标注工具
│   │   └── split_dataset.py       # 数据集划分
│   ├── raw/                       # 原始扫描（gitignore）
│   ├── processed/                 # 处理后图像（gitignore）
│   └── metadata/                  # 标注数据
│       └── style_taxonomy.yaml    # 风格分类体系
├── models/                         # 新建：模型定义与训练
│   ├── __init__.py
│   ├── train_base.py              # 基座模型训练
│   ├── train_lora.py              # LoRA 风格微调
│   ├── configs/
│   │   ├── base_training.yaml     # 基座训练配置
│   │   └── lora_training.yaml     # LoRA 训练配置
│   └── utils/
│       ├── __init__.py
│       ├── dataset.py             # Dataset 类
│       └── metrics.py             # 评估指标
├── inference/                      # 新建：推理与生成
│   ├── __init__.py
│   ├── generate.py                # CLI 生成脚本
│   └── webui/
│       ├── __init__.py
│       └── app.py                 # Gradio 应用
├── tests/                          # 测试
│   ├── __init__.py
│   ├── test_preprocess.py
│   └── test_dataset.py
├── weights/                        # 模型权重（gitignore）
│   ├── base/
│   └── lora/
├── outputs/                        # 生成结果（gitignore）
├── pyproject.toml                  # Python 项目配置
├── .python-version                 # Python 版本锁定
├── .gitignore                      # 更新
└── README.md                       # 重写
```

---

## Task 1: 归档 v1.x 代码

**Files:**
- Branch: `v1-archive`（新建分支）

- [ ] **Step 1: 检查当前 Git 状态**

```bash
cd d:/vibe-coding-app/coda-score
git status
git log --oneline -5
```

Expected: 工作目录干净，看到最近的提交历史。

- [ ] **Step 2: 创建 v1-archive 分支**

```bash
git branch v1-archive
```

- [ ] **Step 3: 删除 v1.x 文件（保留 docs/ 和 study/）**

```bash
# 删除前端文件
rm -rf src/
rm -rf src-tauri/
rm -f package.json
rm -f package-lock.json
rm -f index.html
rm -rf node_modules/

# 删除 GitHub workflows（需要重写）
rm -rf .github/
```

- [ ] **Step 4: 提交删除操作**

```bash
git add -A
git commit -m "chore: archive v1.x code, prepare for v2.0 restructure

- Remove React/TypeScript frontend
- Remove Tauri/Rust backend
- Preserve docs/ and study/ directories
- v1.x code available in v1-archive branch"
```

- [ ] **Step 5: 推送分支到远程**

```bash
git push origin v1-archive
git push origin main
```

---

## Task 2: 初始化 Python 项目

**Files:**
- Create: `pyproject.toml`
- Create: `.python-version`

- [ ] **Step 1: 创建 .python-version 文件**

```bash
echo "3.11" > .python-version
```

- [ ] **Step 2: 创建 pyproject.toml**

```toml
[project]
name = "coda-score"
version = "2.0.0"
description = "Graphical modern music score generation system using diffusion models"
readme = "README.md"
requires-python = ">=3.11"
license = { text = "MIT" }
authors = [
    { name = "Jeffrey", email = "jeffrey@example.com" }
]
keywords = ["music", "score", "diffusion", "modern-music", "graphic-notation"]
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3.11",
    "Topic :: Multimedia :: Sound/Audio",
]

dependencies = [
    # Core ML
    "torch>=2.1.0",
    "torchvision>=0.16.0",
    "diffusers>=0.27.0",
    "transformers>=4.38.0",
    "accelerate>=0.27.0",
    "safetensors>=0.4.0",
    # Image processing
    "pillow>=10.0.0",
    "opencv-python>=4.9.0",
    "scikit-image>=0.22.0",
    # Data processing
    "numpy>=1.26.0",
    "pandas>=2.2.0",
    "pyyaml>=6.0",
    # Web scraping
    "requests>=2.31.0",
    "beautifulsoup4>=4.12.0",
    "tqdm>=4.66.0",
    # Web UI
    "gradio>=4.19.0",
    # CLI
    "click>=8.1.0",
    "rich>=13.7.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0.0",
    "pytest-cov>=4.1.0",
    "ruff>=0.2.0",
    "mypy>=1.8.0",
    "pre-commit>=3.6.0",
]
train = [
    "wandb>=0.16.0",
    "tensorboard>=2.16.0",
    "peft>=0.9.0",
    "bitsandbytes>=0.42.0",
]

[project.scripts]
coda-generate = "inference.generate:main"
coda-scrape = "data.scripts.scrape_imslp:main"
coda-preprocess = "data.scripts.preprocess:main"
coda-label = "data.scripts.label:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["data", "models", "inference"]

[tool.ruff]
line-length = 100
target-version = "py311"
select = ["E", "F", "I", "W", "UP"]
ignore = ["E501"]

[tool.ruff.isort]
known-first-party = ["data", "models", "inference"]

[tool.mypy]
python_version = "3.11"
strict = true
ignore_missing_imports = true

[tool.pytest.ini_options]
testpaths = ["tests"]
python_files = ["test_*.py"]
addopts = "-v --tb=short"
```

- [ ] **Step 3: 初始化 uv 环境**

```bash
# 如果未安装 uv，先安装
# pip install uv 或 curl -LsSf https://astral.sh/uv/install.sh | sh

uv venv
uv pip install -e ".[dev]"
```

Expected: 创建 `.venv` 目录，安装所有依赖。

- [ ] **Step 4: 验证 Python 环境**

```bash
source .venv/bin/activate  # Linux/Mac
# 或 .venv\Scripts\activate  # Windows

python -c "import torch; print(f'PyTorch: {torch.__version__}')"
python -c "import diffusers; print(f'Diffusers: {diffusers.__version__}')"
```

Expected: 输出版本号。

- [ ] **Step 5: 提交项目配置**

```bash
git add pyproject.toml .python-version
git commit -m "feat: initialize Python project with pyproject.toml

- Python 3.11+ requirement
- PyTorch 2.x, Diffusers, Gradio dependencies
- Development tools: pytest, ruff, mypy
- CLI entry points for data processing and inference"
```

---

## Task 3: 创建目录结构

**Files:**
- Create: 多个 `__init__.py` 和占位文件

- [ ] **Step 1: 创建数据目录**

```bash
# 数据处理模块
mkdir -p data/scripts
mkdir -p data/raw
mkdir -p data/processed
mkdir -p data/metadata

# 创建 __init__.py
touch data/__init__.py
touch data/scripts/__init__.py
```

- [ ] **Step 2: 创建模型目录**

```bash
# 模型训练模块
mkdir -p models/configs
mkdir -p models/utils

# 创建 __init__.py
touch models/__init__.py
touch models/utils/__init__.py
```

- [ ] **Step 3: 创建推理目录**

```bash
# 推理模块
mkdir -p inference/webui

# 创建 __init__.py
touch inference/__init__.py
touch inference/webui/__init__.py
```

- [ ] **Step 4: 创建权重和输出目录**

```bash
mkdir -p weights/base
mkdir -p weights/lora
mkdir -p outputs

# 添加 .gitkeep 保持空目录
touch weights/base/.gitkeep
touch weights/lora/.gitkeep
touch outputs/.gitkeep
```

- [ ] **Step 5: 创建测试目录**

```bash
mkdir -p tests
touch tests/__init__.py
```

- [ ] **Step 6: 提交目录结构**

```bash
git add -A
git commit -m "feat: create v2.0 directory structure

- data/: data collection, preprocessing, metadata
- models/: training scripts and configs
- inference/: CLI and Gradio web UI
- tests/: pytest test suite
- weights/: model checkpoints (gitignored)
- outputs/: generated scores (gitignored)"
```

---

## Task 4: 更新 .gitignore

**Files:**
- Modify: `.gitignore`

- [ ] **Step 1: 重写 .gitignore**

```gitignore
# Python
__pycache__/
*.py[cod]
*$py.class
*.so
*.egg-info/
dist/
build/
.eggs/

# Virtual environment
.venv/
venv/
ENV/

# uv
uv.lock

# Data (too large for GitHub)
data/raw/
data/processed/
!data/raw/.gitkeep
!data/processed/.gitkeep

# Model weights (too large for GitHub)
weights/**/*.bin
weights/**/*.pt
weights/**/*.safetensors
weights/**/*.ckpt
!weights/**/.gitkeep

# Generated outputs
outputs/*.png
outputs/*.pdf
outputs/*.tiff
!outputs/.gitkeep

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
desktop.ini

# Environment
.env
.env.local

# Logs
*.log
wandb/
runs/

# Jupyter
.ipynb_checkpoints/
*.ipynb

# Testing
.coverage
htmlcov/
.pytest_cache/

# mypy
.mypy_cache/

# Study materials (large files)
study/pdfs/
study/ocr-output/
```

- [ ] **Step 2: 提交**

```bash
git add .gitignore
git commit -m "chore: update .gitignore for Python ML project

- Ignore Python cache and virtual env
- Ignore raw/processed data (use Git LFS or local storage)
- Ignore model weights (use HuggingFace Hub or local storage)
- Ignore generated outputs
- Ignore wandb/tensorboard logs"
```

---

## Task 5: 创建风格分类体系

**Files:**
- Create: `data/metadata/style_taxonomy.yaml`

- [ ] **Step 1: 创建风格分类 YAML**

```yaml
# data/metadata/style_taxonomy.yaml
# Modern Music Score Style Taxonomy
# 现代音乐乐谱风格分类体系

periods:
  early_modern:
    name: "Early Modern"
    name_zh: "早期现代"
    years: "1900-1945"
    description: "Late Romantic transition to atonality"
    composers:
      - "Scriabin, Alexander"
      - "Ives, Charles"
      - "Schoenberg, Arnold (early)"
      - "Berg, Alban"
      - "Webern, Anton"
      - "Satie, Erik"
      - "Debussy, Claude"
      - "Ravel, Maurice"
      - "Stravinsky, Igor"
      - "Bartók, Béla"

  post_war_avant_garde:
    name: "Post-War Avant-Garde"
    name_zh: "战后先锋派"
    years: "1945-1970"
    description: "Serialism, electronic music, graphic notation emergence"
    composers:
      - "Boulez, Pierre"
      - "Stockhausen, Karlheinz"
      - "Ligeti, György"
      - "Xenakis, Iannis"
      - "Carter, Elliott"
      - "Babbitt, Milton"
      - "Berio, Luciano"
      - "Maderna, Bruno"
      - "Nono, Luigi"
      - "Feldman, Morton"
      - "Cage, John"
      - "Brown, Earle"

  contemporary:
    name: "Contemporary"
    name_zh: "当代"
    years: "1970-present"
    description: "New complexity, spectralism, postmodernism"
    composers:
      - "Ferneyhough, Brian"
      - "Grisey, Gérard"
      - "Murail, Tristan"
      - "Saariaho, Kaija"
      - "Reich, Steve"
      - "Glass, Philip"
      - "Adams, John"
      - "Birtwistle, Harrison"
      - "Dillon, James"
      - "Barrett, Richard"

styles:
  serialism:
    name: "Serialism"
    name_zh: "序列主义"
    description: "12-tone and total serialism"
    visual_features:
      - "Dense note clusters"
      - "Complex rhythm patterns"
      - "No key signature"
      - "Frequent time signature changes"
    tags: ["twelve-tone", "dodecaphony", "total-serialism"]

  spectralism:
    name: "Spectralism"
    name_zh: "频谱音乐"
    description: "Sound spectrum as compositional material"
    visual_features:
      - "Microtonal notation"
      - "Overtone series symbols"
      - "Extended technique markings"
      - "Complex dynamics"
    tags: ["spectral", "microtonal", "acoustic"]

  graphic_notation:
    name: "Graphic Notation"
    name_zh: "图形记谱"
    description: "Visual/graphic elements replace traditional notation"
    visual_features:
      - "Free-form lines and shapes"
      - "Non-traditional symbols"
      - "Spatial layout as time"
      - "Color usage (rare)"
    tags: ["graphic", "visual", "experimental"]

  minimalism:
    name: "Minimalism"
    name_zh: "简约主义"
    description: "Repetition, phase shifting, gradual process"
    visual_features:
      - "Repetitive patterns"
      - "Phase offset markings"
      - "Simple notation"
      - "Extended durations"
    tags: ["repetitive", "phase", "process"]

  new_complexity:
    name: "New Complexity"
    name_zh: "新复杂主义"
    description: "Extreme notational complexity"
    visual_features:
      - "Hyper-dense notation"
      - "Nested tuplets"
      - "Multiple staves per instrument"
      - "Microtonal and micro-rhythmic"
    tags: ["complex", "dense", "ferneyhough"]

  stochastic:
    name: "Stochastic"
    name_zh: "随机/概率"
    description: "Probability distributions, game theory"
    visual_features:
      - "Mathematical notation"
      - "Decision diagrams"
      - "Non-linear layouts"
    tags: ["probability", "xenakis", "game-theory"]

  aleatoric:
    name: "Aleatoric"
    name_zh: "偶然音乐"
    description: "Indeterminacy, performer choice"
    visual_features:
      - "Mobile form"
      - "Optional passages"
      - "Graphic elements"
      - "Text instructions"
    tags: ["indeterminate", "chance", "cage"]

  extended_techniques:
    name: "Extended Techniques"
    name_zh: "扩展技法"
    description: "Non-standard playing methods"
    visual_features:
      - "Custom symbols"
      - "Legend/key pages"
      - "Annotated diagrams"
    tags: ["extended", "non-standard", "experimental"]

  electronic:
    name: "Electronic"
    name_zh: "电子音乐"
    description: "Electronic/tape music notation"
    visual_features:
      - "Waveform representations"
      - "Block diagrams"
      - "Timeline notation"
    tags: ["electronic", "tape", "synthesizer"]

  microtonal:
    name: "Microtonal"
    name_zh: "微音程"
    description: "Intervals smaller than semitone"
    visual_features:
      - "Quarter-tone accidentals"
      - "Custom accidentals"
      - "Frequency annotations"
    tags: ["microtonal", "quarter-tone", "just-intonation"]

instrumentation:
  solo:
    name: "Solo"
    description: "Single instrument"
    examples: ["piano", "violin", "flute", "clarinet", "voice"]

  chamber:
    name: "Chamber"
    description: "Small ensemble (2-10 players)"
    examples: ["string quartet", "piano trio", "wind quintet"]

  orchestral:
    name: "Orchestral"
    description: "Full orchestra"
    examples: ["symphony orchestra", "chamber orchestra"]

  vocal:
    name: "Vocal"
    description: "Voice(s) with or without instruments"
    examples: ["solo voice", "choir", "voice + piano"]

  electronic:
    name: "Electronic"
    description: "Electronic media"
    examples: ["tape", "live electronics", "computer"]

  mixed:
    name: "Mixed"
    description: "Mixed media"
    examples: ["instrument + tape", "voice + electronics"]

notation_types:
  traditional:
    name: "Traditional"
    description: "Standard Western notation"

  proportional:
    name: "Proportional"
    description: "Space proportional to time"

  graphic:
    name: "Graphic"
    description: "Graphic/visual notation"

  mixed:
    name: "Mixed"
    description: "Combination of traditional and graphic"

  text:
    name: "Text Score"
    description: "Text-based instructions"

page_types:
  full_page:
    name: "Full Page"
    description: "Complete page of score"

  system:
    name: "System"
    description: "Single system (staff group)"

  fragment:
    name: "Fragment"
    description: "Partial/excerpt"
```

- [ ] **Step 2: 提交**

```bash
git add data/metadata/style_taxonomy.yaml
git commit -m "feat: add modern music style taxonomy

- 3 periods: Early Modern, Post-War Avant-Garde, Contemporary
- 10 styles: serialism, spectralism, graphic notation, etc.
- Instrumentation categories
- Notation types
- Page types for dataset labeling"
```

---

## Task 6: 创建数据预处理模块骨架

**Files:**
- Create: `data/scripts/preprocess.py`
- Create: `tests/test_preprocess.py`

- [ ] **Step 1: 编写测试**

```python
# tests/test_preprocess.py
import pytest
from pathlib import Path
from PIL import Image
import numpy as np

from data.scripts.preprocess import (
    is_score_image,
    crop_to_content,
    normalize_image,
)


class TestIsScoreImage:
    def test_valid_score_image(self, tmp_path: Path) -> None:
        """Test detection of valid score-like image."""
        # Create a simple black and white image (like sheet music)
        img = Image.new("L", (100, 100), 255)
        # Add some black lines (like staff lines)
        for y in range(20, 80, 15):
            for x in range(10, 90):
                img.putpixel((x, y), 0)
        img_path = tmp_path / "score.png"
        img.save(img_path)

        assert is_score_image(img_path) is True

    def test_blank_image(self, tmp_path: Path) -> None:
        """Test rejection of blank image."""
        img = Image.new("L", (100, 100), 255)
        img_path = tmp_path / "blank.png"
        img.save(img_path)

        assert is_score_image(img_path) is False

    def test_non_image_file(self, tmp_path: Path) -> None:
        """Test rejection of non-image file."""
        text_file = tmp_path / "not_image.txt"
        text_file.write_text("not an image")

        assert is_score_image(text_file) is False


class TestCropToContent:
    def test_crop_removes_margins(self) -> None:
        """Test that cropping removes white margins."""
        # Create image with white margins and black content
        img = Image.new("L", (200, 200), 255)
        # Draw content in center
        for y in range(50, 150):
            for x in range(50, 150):
                img.putpixel((x, y), 0)

        cropped = crop_to_content(img, margin=10)

        # Cropped should be smaller and centered on content
        assert cropped.width < 200
        assert cropped.height < 200

    def test_crop_with_margin(self) -> None:
        """Test that margin parameter adds padding."""
        img = Image.new("L", (100, 100), 255)
        # Small content area
        for y in range(40, 60):
            for x in range(40, 60):
                img.putpixel((x, y), 0)

        cropped = crop_to_content(img, margin=5)

        # Should have some margin around content
        assert cropped.width > 20
        assert cropped.height > 20


class TestNormalizeImage:
    def test_normalize_grayscale(self) -> None:
        """Test conversion to grayscale."""
        img = Image.new("RGB", (100, 100), (255, 0, 0))
        result = normalize_image(img)

        assert result.mode == "L"

    def test_normalize_contrast(self) -> None:
        """Test contrast enhancement."""
        img = Image.new("L", (100, 100), 128)
        # Add some variation
        for y in range(40, 60):
            for x in range(40, 60):
                img.putpixel((x, y), 100)

        result = normalize_image(img, enhance_contrast=True)

        # Should have enhanced contrast (wider pixel range)
        pixels = list(result.getdata())
        assert max(pixels) - min(pixels) >= max(img.getdata()) - min(img.getdata())

    def test_normalize_resize(self) -> None:
        """Test resizing to target size."""
        img = Image.new("L", (100, 100), 128)
        result = normalize_image(img, target_size=(256, 256))

        assert result.size == (256, 256)
```

- [ ] **Step 2: 运行测试验证失败**

```bash
pytest tests/test_preprocess.py -v
```

Expected: FAIL - `ModuleNotFoundError: No module named 'data.scripts.preprocess'`

- [ ] **Step 3: 实现预处理模块**

```python
# data/scripts/preprocess.py
"""Image preprocessing pipeline for music score images."""

from pathlib import Path
from typing import Optional

import click
import numpy as np
from PIL import Image, ImageEnhance, ImageFilter
from tqdm import tqdm


def is_score_image(path: Path, threshold: float = 0.05) -> bool:
    """Check if an image looks like a music score.

    Detects score-like images by checking for:
    - Sufficient dark pixels (notes, staff lines)
    - Horizontal line patterns (staff lines)

    Args:
        path: Path to image file.
        threshold: Minimum ratio of dark pixels to total pixels.

    Returns:
        True if image appears to be a music score.
    """
    try:
        img = Image.open(path).convert("L")
    except Exception:
        return False

    arr = np.array(img)
    dark_pixels = np.sum(arr < 128)
    total_pixels = arr.size
    dark_ratio = dark_pixels / total_pixels

    if dark_ratio < threshold:
        return False

    # Check for horizontal lines (staff lines indicator)
    row_darkness = np.mean(arr < 128, axis=1)
    # Staff lines should create rows with high darkness ratio
    line_rows = np.sum(row_darkness > 0.3)

    return line_rows >= 3  # At least 3 potential staff lines


def crop_to_content(img: Image.Image, margin: int = 20, threshold: int = 240) -> Image.Image:
    """Crop image to content area, removing white margins.

    Args:
        img: Input image.
        margin: Pixels to add around content as padding.
        threshold: Pixel value threshold for "white" detection.

    Returns:
        Cropped image.
    """
    img_gray = img.convert("L")
    arr = np.array(img_gray)

    # Find non-white pixels
    mask = arr < threshold
    if not mask.any():
        return img  # All white, return original

    # Find bounding box of content
    rows = np.any(mask, axis=1)
    cols = np.any(mask, axis=0)
    rmin, rmax = np.where(rows)[0][[0, -1]]
    cmin, cmax = np.where(cols)[0][[0, -1]]

    # Add margin
    rmin = max(0, rmin - margin)
    rmax = min(arr.shape[0] - 1, rmax + margin)
    cmin = max(0, cmin - margin)
    cmax = min(arr.shape[1] - 1, cmax + margin)

    return img.crop((cmin, rmin, cmax + 1, rmax + 1))


def normalize_image(
    img: Image.Image,
    target_size: Optional[tuple[int, int]] = None,
    enhance_contrast: bool = True,
    denoise: bool = True,
) -> Image.Image:
    """Normalize image for consistent processing.

    Args:
        img: Input image.
        target_size: Optional (width, height) to resize to.
        enhance_contrast: Whether to enhance contrast.
        denoise: Whether to apply mild denoising.

    Returns:
        Normalized grayscale image.
    """
    # Convert to grayscale
    img = img.convert("L")

    # Denoise
    if denoise:
        img = img.filter(ImageFilter.MedianFilter(size=3))

    # Enhance contrast
    if enhance_contrast:
        enhancer = ImageEnhance.Contrast(img)
        img = enhancer.enhance(1.5)

    # Resize if target specified
    if target_size:
        img = img.resize(target_size, Image.Resampling.LANCZOS)

    return img


def preprocess_image(
    input_path: Path,
    output_path: Path,
    target_size: tuple[int, int] = (1024, 1024),
) -> Optional[Path]:
    """Full preprocessing pipeline for a single image.

    Args:
        input_path: Path to input image.
        output_path: Path to save processed image.
        target_size: Target size for output.

    Returns:
        Output path if successful, None if image rejected.
    """
    if not is_score_image(input_path):
        return None

    img = Image.open(input_path)
    img = crop_to_content(img)
    img = normalize_image(img, target_size=target_size)

    output_path.parent.mkdir(parents=True, exist_ok=True)
    img.save(output_path, quality=95)

    return output_path


@click.command()
@click.argument("input_dir", type=click.Path(exists=True, path_type=Path))
@click.argument("output_dir", type=click.Path(path_type=Path))
@click.option("--size", default=1024, help="Target image size (square)")
@click.option("--skip-invalid", is_flag=True, help="Skip images that don't look like scores")
def main(input_dir: Path, output_dir: Path, size: int, skip_invalid: bool) -> None:
    """Preprocess music score images.

    INPUT_DIR: Directory containing raw images.
    OUTPUT_DIR: Directory to save processed images.
    """
    target_size = (size, size)
    output_dir.mkdir(parents=True, exist_ok=True)

    image_extensions = {".png", ".jpg", ".jpeg", ".tiff", ".tif"}
    input_files = [
        f for f in input_dir.rglob("*") if f.suffix.lower() in image_extensions
    ]

    processed = 0
    skipped = 0

    for input_file in tqdm(input_files, desc="Preprocessing"):
        output_file = output_dir / f"{input_file.stem}.png"
        result = preprocess_image(input_file, output_file, target_size)

        if result:
            processed += 1
        else:
            skipped += 1
            if not skip_invalid:
                click.echo(f"Skipped: {input_file}")

    click.echo(f"\nProcessed: {processed}, Skipped: {skipped}")


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: 运行测试验证通过**

```bash
pytest tests/test_preprocess.py -v
```

Expected: All tests PASS.

- [ ] **Step 5: 提交**

```bash
git add data/scripts/preprocess.py tests/test_preprocess.py
git commit -m "feat: add image preprocessing pipeline

- is_score_image(): detect score-like images
- crop_to_content(): remove white margins
- normalize_image(): grayscale, contrast, denoise
- CLI: coda-preprocess <input> <output>"
```

---

## Task 7: 创建 Dataset 类骨架

**Files:**
- Create: `models/utils/dataset.py`
- Create: `tests/test_dataset.py`

- [ ] **Step 1: 编写测试**

```python
# tests/test_dataset.py
import pytest
from pathlib import Path
from PIL import Image
import yaml

from models.utils.dataset import ScoreDataset, ScoreMetadata


class TestScoreMetadata:
    def test_from_dict(self) -> None:
        """Test creating metadata from dictionary."""
        data = {
            "file": "score_001.png",
            "composer": "Boulez, Pierre",
            "period": "post_war_avant_garde",
            "styles": ["serialism"],
            "instrumentation": "solo",
            "notation_type": "traditional",
            "page_type": "full_page",
        }
        meta = ScoreMetadata.from_dict(data)

        assert meta.file == "score_001.png"
        assert meta.composer == "Boulez, Pierre"
        assert meta.period == "post_war_avant_garde"
        assert "serialism" in meta.styles

    def test_to_prompt(self) -> None:
        """Test converting metadata to text prompt."""
        meta = ScoreMetadata(
            file="test.png",
            composer="Stockhausen, Karlheinz",
            period="post_war_avant_garde",
            styles=["serialism", "electronic"],
            instrumentation="electronic",
            notation_type="graphic",
            page_type="full_page",
        )
        prompt = meta.to_prompt()

        assert "Stockhausen" in prompt
        assert "serialism" in prompt
        assert "graphic notation" in prompt


class TestScoreDataset:
    @pytest.fixture
    def sample_dataset(self, tmp_path: Path) -> tuple[ScoreDataset, Path]:
        """Create a sample dataset for testing."""
        # Create images
        img_dir = tmp_path / "images"
        img_dir.mkdir()

        for i in range(3):
            img = Image.new("L", (256, 256), 128)
            img.save(img_dir / f"score_{i:03d}.png")

        # Create metadata
        metadata = [
            {
                "file": f"score_{i:03d}.png",
                "composer": "Test Composer",
                "period": "contemporary",
                "styles": ["minimalism"],
                "instrumentation": "solo",
                "notation_type": "traditional",
                "page_type": "full_page",
            }
            for i in range(3)
        ]

        metadata_file = tmp_path / "metadata.yaml"
        with open(metadata_file, "w") as f:
            yaml.dump(metadata, f)

        dataset = ScoreDataset(
            image_dir=img_dir,
            metadata_file=metadata_file,
        )

        return dataset, img_dir

    def test_len(self, sample_dataset: tuple[ScoreDataset, Path]) -> None:
        """Test dataset length."""
        dataset, _ = sample_dataset
        assert len(dataset) == 3

    def test_getitem(self, sample_dataset: tuple[ScoreDataset, Path]) -> None:
        """Test getting an item."""
        dataset, _ = sample_dataset
        item = dataset[0]

        assert "image" in item
        assert "prompt" in item
        assert isinstance(item["image"], Image.Image)

    def test_filter_by_style(self, sample_dataset: tuple[ScoreDataset, Path]) -> None:
        """Test filtering by style."""
        dataset, _ = sample_dataset
        filtered = dataset.filter_by_style("minimalism")

        assert len(filtered) == 3

    def test_filter_by_style_no_match(
        self, sample_dataset: tuple[ScoreDataset, Path]
    ) -> None:
        """Test filtering by style with no matches."""
        dataset, _ = sample_dataset
        filtered = dataset.filter_by_style("serialism")

        assert len(filtered) == 0
```

- [ ] **Step 2: 运行测试验证失败**

```bash
pytest tests/test_dataset.py -v
```

Expected: FAIL - `ModuleNotFoundError`

- [ ] **Step 3: 实现 Dataset 类**

```python
# models/utils/dataset.py
"""Dataset classes for music score images."""

from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

import yaml
from PIL import Image
from torch.utils.data import Dataset


@dataclass
class ScoreMetadata:
    """Metadata for a single score image."""

    file: str
    composer: str
    period: str
    styles: list[str]
    instrumentation: str
    notation_type: str
    page_type: str
    title: Optional[str] = None
    year: Optional[int] = None
    notes: Optional[str] = None

    @classmethod
    def from_dict(cls, data: dict) -> "ScoreMetadata":
        """Create metadata from dictionary."""
        return cls(
            file=data["file"],
            composer=data["composer"],
            period=data["period"],
            styles=data.get("styles", []),
            instrumentation=data.get("instrumentation", "unknown"),
            notation_type=data.get("notation_type", "traditional"),
            page_type=data.get("page_type", "full_page"),
            title=data.get("title"),
            year=data.get("year"),
            notes=data.get("notes"),
        )

    def to_prompt(self) -> str:
        """Convert metadata to text prompt for diffusion model."""
        styles_str = ", ".join(self.styles) if self.styles else "traditional"
        notation_map = {
            "traditional": "traditional notation",
            "graphic": "graphic notation",
            "proportional": "proportional notation",
            "mixed": "mixed notation",
            "text": "text score",
        }
        notation_str = notation_map.get(self.notation_type, self.notation_type)

        parts = [
            "contemporary classical music score",
            f"in the style of {self.composer}",
            f"{styles_str} style",
            f"{notation_str}",
            f"{self.instrumentation} instrumentation",
            "high quality engraving",
            "black and white",
        ]

        return ", ".join(parts)


class ScoreDataset(Dataset):
    """PyTorch Dataset for music score images."""

    def __init__(
        self,
        image_dir: Path,
        metadata_file: Path,
        transform: Optional[callable] = None,
        target_size: tuple[int, int] = (1024, 1024),
    ) -> None:
        """Initialize dataset.

        Args:
            image_dir: Directory containing score images.
            metadata_file: Path to YAML metadata file.
            transform: Optional torchvision transform.
            target_size: Target image size.
        """
        self.image_dir = Path(image_dir)
        self.transform = transform
        self.target_size = target_size

        # Load metadata
        with open(metadata_file) as f:
            raw_metadata = yaml.safe_load(f)

        self.metadata: list[ScoreMetadata] = [
            ScoreMetadata.from_dict(m) for m in raw_metadata
        ]

        # Filter to existing images
        self.metadata = [
            m for m in self.metadata if (self.image_dir / m.file).exists()
        ]

    def __len__(self) -> int:
        return len(self.metadata)

    def __getitem__(self, idx: int) -> dict:
        meta = self.metadata[idx]
        img_path = self.image_dir / meta.file

        img = Image.open(img_path).convert("RGB")
        img = img.resize(self.target_size, Image.Resampling.LANCZOS)

        if self.transform:
            img = self.transform(img)

        return {
            "image": img,
            "prompt": meta.to_prompt(),
            "metadata": meta,
        }

    def filter_by_style(self, style: str) -> "ScoreDataset":
        """Return new dataset filtered by style."""
        filtered = ScoreDataset.__new__(ScoreDataset)
        filtered.image_dir = self.image_dir
        filtered.transform = self.transform
        filtered.target_size = self.target_size
        filtered.metadata = [m for m in self.metadata if style in m.styles]
        return filtered

    def filter_by_period(self, period: str) -> "ScoreDataset":
        """Return new dataset filtered by period."""
        filtered = ScoreDataset.__new__(ScoreDataset)
        filtered.image_dir = self.image_dir
        filtered.transform = self.transform
        filtered.target_size = self.target_size
        filtered.metadata = [m for m in self.metadata if m.period == period]
        return filtered

    def get_style_distribution(self) -> dict[str, int]:
        """Get distribution of styles in dataset."""
        dist: dict[str, int] = {}
        for meta in self.metadata:
            for style in meta.styles:
                dist[style] = dist.get(style, 0) + 1
        return dist
```

- [ ] **Step 4: 运行测试验证通过**

```bash
pytest tests/test_dataset.py -v
```

Expected: All tests PASS.

- [ ] **Step 5: 提交**

```bash
git add models/utils/dataset.py tests/test_dataset.py
git commit -m "feat: add ScoreDataset and ScoreMetadata classes

- ScoreMetadata: dataclass for score image metadata
- ScoreMetadata.to_prompt(): generate diffusion prompts
- ScoreDataset: PyTorch Dataset with filtering
- Support for style, period, instrumentation filters"
```

---

## Task 8: 创建 README

**Files:**
- Create: `README.md`

- [ ] **Step 1: 编写 README**

```markdown
# coda-score v2.0

> Graphical modern music score generation using diffusion models

## Overview

coda-score is a research project that generates modern classical music scores using latent diffusion models. Unlike symbolic approaches (ABC/MusicXML/LilyPond), coda-score works directly at the image level — any notation style becomes pixels.

**Focus:** 20th/21st century academic modern music — serialism, spectralism, graphic notation, new complexity, and beyond.

## Architecture

```
User Input → Prompt Builder → SDXL + LoRA → Score Image → Post-process → Output
```

- **Base Model:** Fine-tuned Stable Diffusion XL
- **Style Control:** LoRA modules per composer/style
- **Resolution:** 1024×1024 native, up to 4096×4096 with upscaling

## Installation

```bash
# Clone repository
git clone https://github.com/yourusername/coda-score.git
cd coda-score

# Create virtual environment (requires Python 3.11+)
uv venv
source .venv/bin/activate  # Linux/Mac
# or .venv\Scripts\activate  # Windows

# Install dependencies
uv pip install -e ".[dev]"
```

## Usage

### Generate Scores (CLI)

```bash
coda-generate \
  --style serialism \
  --composer "Boulez, Pierre" \
  --instrumentation solo \
  --output outputs/boulez_solo.png
```

### Web UI

```bash
python -m inference.webui.app
```

### Preprocess Data

```bash
coda-preprocess data/raw data/processed --size 1024
```

## Project Structure

```
coda-score/
├── data/           # Data collection and preprocessing
│   ├── scripts/    # Scraping, preprocessing, labeling
│   ├── raw/        # Raw scanned images
│   ├── processed/  # Preprocessed images
│   └── metadata/   # Style taxonomy and labels
├── models/         # Model training
│   ├── configs/    # Training configurations
│   └── utils/      # Dataset classes, metrics
├── inference/      # Generation
│   ├── generate.py # CLI tool
│   └── webui/      # Gradio interface
├── study/          # Music theory reference
└── docs/           # Documentation
```

## Style Taxonomy

See `data/metadata/style_taxonomy.yaml` for the complete classification system:

- **Periods:** Early Modern (1900-1945), Post-War Avant-Garde (1945-1970), Contemporary (1970-)
- **Styles:** Serialism, Spectralism, Graphic Notation, Minimalism, New Complexity, etc.

## Research Status

This is an active research project. Model weights are not publicly available.

## License

MIT License — code only. Training data subject to separate copyright considerations.

## Citation

```bibtex
@misc{coda-score2026,
  title={coda-score: Graphical Modern Music Score Generation},
  author={Jeffrey},
  year={2026},
  howpublished={\url{https://github.com/yourusername/coda-score}}
}
```
```

- [ ] **Step 2: 提交**

```bash
git add README.md
git commit -m "docs: add README for v2.0

- Project overview and architecture
- Installation instructions
- Usage examples (CLI, Web UI)
- Project structure
- Style taxonomy reference"
```

---

## Task 9: 最终验证

- [ ] **Step 1: 运行所有测试**

```bash
pytest tests/ -v --tb=short
```

Expected: All tests PASS.

- [ ] **Step 2: 验证 CLI 入口点**

```bash
coda-preprocess --help
```

Expected: 显示帮助信息。

- [ ] **Step 3: 验证项目结构**

```bash
tree -L 2 -I '__pycache__|*.pyc|.venv|node_modules'
```

Expected: 看到完整的目录结构。

- [ ] **Step 4: 推送到远程**

```bash
git push origin main
```

- [ ] **Step 5: 创建 v2.0 初始 release tag**

```bash
git tag -a v2.0.0-alpha -m "v2.0.0-alpha: Python project initialization

- Migrated from Tauri/React to Python/PyTorch
- Added data preprocessing pipeline
- Added ScoreDataset and metadata classes
- Added style taxonomy for modern music
- CLI tools for data processing"

git push origin v2.0.0-alpha
```

---

## Summary

完成 Phase 0 后，你将拥有：

| 组件 | 状态 |
|------|------|
| Python 项目结构 | ✅ |
| 依赖管理 (uv + pyproject.toml) | ✅ |
| 数据预处理脚本 | ✅ |
| Dataset 类 | ✅ |
| 风格分类体系 | ✅ |
| 测试框架 | ✅ |
| CLI 入口点 | ✅ |

**下一步:** Phase 1 - 数据 Pipeline（IMSLP 爬取、批量预处理、标注工具）
