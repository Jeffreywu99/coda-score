# coda-score v2.0 — 图形化现代音乐乐谱生成系统

> 方案日期：2026-05-30 | Mermer × Jeff
>
> **核心原则**：完全图形化生成。扩散模型在乐谱图像层面学习与生成。数据层如有符号化乐谱，统一使用 MusicXML。

---

## 1. 为什么走图形化路线

现代学院派音乐（序列主义、频谱音乐、图形记谱、扩展技法、New Complexity）的记谱法本身就是创作的一部分。符号化路线（ABC/REMI/LilyPond token → LLM）在此面临根本性限制：

- 图形记谱（Cardew, Bussotti, Crumb）
- 比例记谱（Boulez 后期, Lutosławski）
- 空间化布局（Stockhausen, Xenakis）
- 微音程、嵌套连音、非标准谱表（Ferneyhough, Haas）

对这些记谱形式，任何 tokenization 都意味着信息损失。唯一无损的表示是图像本身。

**为什么不走 LLM 符号化路线？**
- NotaGen（1.6M ABC token）和 GnM 走 LLM 路线，聚焦 Baroque/Classical/Romantic
- 它们明确不碰现代学院派——因为 tokenization 做不到
- MusicScore 用了扩散模型但基于 IMSLP 1-bit 黑白传统乐谱，不涉及现代记谱

**为什么图像层面选择 MusicXML 作为辅助格式？**
- MusicXML 是行业交换标准，Finale/Sibelius/Dorico/MuseScore 全部原生支持
- 渲染质量由制谱软件保证（Dorico 的 engraving 质量业界最高）
- 现代音乐符号的扩展支持更完善
- 不用于生成路径，仅作为数据增强桥梁：`MusicXML → 渲染 → PNG → 训练集`

---

## 2. 差异化定位

| | NotaGen / GnM | MusicScore | **coda-score v2.0** |
|---|---|---|---|
| 方法 | LLM 符号化 | 扩散模型图像 | **扩散模型图像** |
| 数据 | 1.6M ABC token | 200K IMSLP 图像 | **精选现代学院派乐谱图像** |
| 领域 | Baroque/Classical/Romantic | 通用 IMSLP | **20/21世纪学院派** |
| 记谱 | 传统五线谱 | 传统五线谱 | **传统 + 图形 + 非常规** |
| 符号辅助 | ABC notation | — | **MusicXML（渲染→图像）** |
| 微调 | CLaMP-DPO RL | — | **LoRA（可叠加组合）** |
| 权重 | 开源 | 开源 | **闭源，学术研究** |

目前没有任何项目做现代音乐的图形化乐谱生成。

---

## 3. 系统架构

```
┌──────────────────────────────────────────────────────┐
│                  用户界面层                            │
│  Gradio Web UI — 风格选择 → 参数调整 → 批量生成        │
│  CLI — 脚本化生成、参数扫描                            │
├──────────────────────────────────────────────────────┤
│                  推理引擎                              │
│  扩散模型 Pipeline：prompt → latent → image           │
│  + Style LoRA 动态加载                                 │
│  + 超分辨率后处理                                      │
│  + LoRA 组合（多风格叠加，权重调节）                     │
├──────────────────────────────────────────────────────┤
│                  模型核心                              │
│  SDXL 基座（现成权重，无需自训）                        │
│  + 风格 LoRA（每个作曲家/技法一个）                     │
│  + 高分辨率策略（生成 1024² → 超分 4096²）             │
├──────────────────────────────────────────────────────┤
│                  数据层                                │
│  原始扫描 → 预处理 → 标注（多维风格标签）               │
│  MusicXML 乐谱 → Dorico/MuseScore 渲染 → 训练图像       │
├──────────────────────────────────────────────────────┤
│                  开发环境                              │
│  Windows/WSL2（主力训练，RTX 5060 12GB）               │
│  MacBook（代码开发、推理测试、数据标注、文档）           │
│  代码同步：Git + GitHub                                │
│  大文件同步：iCloud / SMB 共享                          │
└──────────────────────────────────────────────────────┘
```

---

## 4. 开发环境与硬件

### 4.1 双机分工

| | Windows/WSL2（主力机） | MacBook（辅助机） |
|---|---|---|
| **角色** | 训练机 | 开发机 |
| **GPU** | RTX 5060 12GB | Apple Silicon（MPS） |
| **职责** | LoRA 训练、数据处理、批量推理 | 代码编写、MusicXML 渲染、单张推理、文档 |
| **关键能力** | CUDA + SDXL LoRA 训练 | Dorico/MuseScore 原生客户端 |

### 4.2 GPU 策略

**当前设备：RTX 5060 12GB**

| 任务 | 可行性 | 说明 |
|------|--------|------|
| SDXL LoRA 训练 | ✅ 完全可行 | 12GB + fp16 + gradient checkpointing + 8-bit AdamW |
| 推理生成 | ✅ 轻松 | 单张 1024² 几秒 |
| SDXL 全量 fine-tune | ❌ 不够 | 需 24GB+，跳过——用现成 SDXL 直接做 LoRA |

**分辨率策略（两阶段）**：

| 阶段 | 训练分辨率 | 目的 |
|------|-----------|------|
| 验证阶段 | 768×768 | 快速跑通全流程，调试 pipeline，确认 LoRA 能抓到风格 |
| 正式训练 | 1024×1024 | SDXL 原生分辨率，乐谱细节更清晰，batch_size=1 + fp16 + gradient checkpointing + 8-bit AdamW 可卡进 12GB |

- 推理生成：1024×1024（原生）
- 最终输出：超分至 4096×4096
- 备用：如果 1024² 真的爆显存，降至 960×960 或 896×896（保持 64 的倍数）

**升级路径**（仅在实际遇到瓶颈后考虑）：
- 如果 12GB 确实不够 → RTX 5080（预期 16-24GB）
- 如果要做全量 fine-tune → 租云 GPU（AutoDL 等，A100 约 ¥10-20/h）

### 4.3 代码与数据同步

| 内容 | 同步方式 | 说明 |
|------|----------|------|
| 代码 | Git + GitHub | 主力，两台机器 push/pull |
| 数据集（raw/processed） | iCloud 或 SMB 共享 | 大文件，不进入 Git |
| 模型权重（LoRA .safetensors） | 手动传或 iCloud | 单个 < 200MB，传一次即可 |
| 生成结果（outputs） | 本地存储 | 不需要同步 |

---

## 5. 模型设计

### 5.1 基础模型

**SDXL 1.0（现成权重，不做全量 fine-tune）**

为什么跳过基座训练（Phase A）：
- SDXL 已见过海量视觉数据，对线条、文字、版式有基础理解
- 乐谱是黑白为主、结构性强、纹理简单的图像类型
- 直接用 SDXL + LoRA 做风格微调，先跑通最小闭环
- 全量 fine-tune（需 A100/40GB）是锦上添花，非必须

### 5.2 LoRA 是什么

一种轻量微调技术。不修改原模型（SDXL 几亿参数不动），只训练一个很小的附加模块（几百万参数，< 200MB 文件）。

类比：SDXL 像个画了十年素描的美院毕业生，什么都能画。LoRA 相当于给他看 200 页 Boulez 手稿，说「往这个方向偏一点」——只调整了一点点手势和习惯。

**LoRA 的核心优势：**
- 显存需求极低（12GB vs 全量 24GB+）
- 训练快（几小时 vs 几天）
- 可叠加：`LoRA_Boulez × 70% + LoRA_GraphicNotation × 30%`
- 文件小，方便管理和分发

### 5.3 LoRA 策略

```
单个 LoRA = 一个风格 / 一个作曲家

训练数据：该风格的精选乐谱（目标 50-500 页）
训练参数：
  - LoRA rank: 16-64（乐谱纹理简单，低 rank 也有效）
  - Learning rate: 1e-4 ~ 5e-4
  - Resolution: 1024×1024（batch_size=1, fp16, gradient checkpointing, 8-bit AdamW 可卡进 12GB）
  - Steps: 1000-5000
  - Batch size: 1（梯度累积补足）
  - Mixed precision: fp16
  - Optimizer: 8-bit AdamW

输出：coda-score-{style}.safetensors（< 200MB）
触发词：如 "in the style of Boulez", "graphic notation score"
```

**LoRA 组合（核心创新点）：**

同时激活多个 LoRA，加权混合：
- 「Boulez 70% + 图形记谱 30%」= 带图形元素的序列主义谱面
- 「Xenakis 80% + 简约主义 20%」= 随机过程的重复模式化
- 权重可调，通过 Web UI 滑块实时控制

### 5.4 高分辨率

乐谱可读性需要高分辨率：
1. 训练：768×768（LoRA）
2. 推理：1024×1024（SDXL 原生）
3. 后处理：Real-ESRGAN 或 SD Upscale → 4096×4096
4. 长乐谱：分页生成 + 拼接

---

## 6. 数据策略

### 6.1 数据来源

| 来源 | 内容 | 版权 | 用途 |
|------|------|------|------|
| IMSLP 公共领域 | 1955年前现代作品（Scriabin, Ives, early Schoenberg, Berg, Webern, Satie, Varèse 等） | ✅ 安全 | 基座数据，可公开发布 |
| 自扫描/收集 | 1950年后学院派（Boulez, Stockhausen, Ligeti, Xenakis, Carter, Babbitt, Ferneyhough, Grisey, Murail, Haas 等） | 🔒 版权期 | 研究训练，权重不开源 |
| MusicXML 渲染 | 现有 MusicXML 现代乐谱 → Dorico/MuseScore 渲染 | 取决于源文件 | 数据增强 |
| LilyPond/ABC | 公共领域作品重新渲染 | ✅ | 补充数据 |

### 6.2 MusicXML 的边界

MusicXML 只作为**数据增强桥梁**，不在生成路径里：

```
适用：传统五线谱（钢琴、室内乐、管弦乐）→ Dorico 渲染 → 训练图像
不适用：图形记谱、比例记谱、文字谱、完全非传统记谱
```

### 6.3 风格标注体系

每个乐谱图像附带多维标签（JSON/YAML）：

**时期**
- Early Modern (1900–1945)
- Post-War Avant-Garde (1945–1970)
- Late 20th Century (1970–2000)
- Contemporary (2000–)

**作曲家**：标准化全名

**风格/技法**（多选）
- serialism / integral serialism
- spectralism
- stochastic / algorithmic
- minimalism / post-minimalism
- graphic notation
- text score
- proportional notation
- extended techniques
- microtonality
- new complexity
- aleatoric / indeterminate
- electronic / live electronics
- spatial music
- pointillism
- collage / polystylism

**编制**
- solo（细分乐器：piano, violin, cello, flute ……）
- chamber（duo, trio, quartet, ensemble）
- orchestral
- vocal / choral
- electronic
- mixed

**记谱类型**
- traditional 5-line staff
- proportional notation
- graphic notation
- text score
- mixed
- alternate notation / tablature

**页面属性**
- 页类型：full page / system / fragment / cover
- 来源：scan / render / photo

### 6.4 数据规模目标

| 阶段 | 用途 | 最低 | 目标 |
|------|------|------|------|
| 验证实验 | 单个 LoRA 验证 | 50 页 | 100 页 |
| 风格 LoRA | 独立风格训练 | 100 页/风格 | 500 页/风格 |
| 全风格覆盖 | 6-8 核心风格 | 600 页 | 3,000 页 |
| 论文数据集 | 可公开发布部分 | — | 1,000+ 页 |

### 6.5 预处理 Pipeline

```
原始扫描/PDF
  → 转换为 PNG（300-600 DPI）
  → 去噪（自适应阈值/CLAHE，重点去除泛黄底色）
  → 页面裁剪（检测乐谱区域，去除空白边框）
  → 标准化到 1024×1024（保持纵横比，黑边填充）
  → 可选：灰度化（保留 engraving 黑白特性）
  → 与元数据配对（同名的 .json/.yaml 标注文件）
```

### 6.6 数据增强

训练时在线增强：
- 轻微旋转（±2°，模拟扫描倾斜）
- 亮度/对比度微调
- 轻微 JPEG 压缩噪声（模拟不同扫描质量）
- 裁剪（random crop）

---

## 7. 训练 Pipeline

### 7.1 训练工作流

```
1. 准备数据
   data/processed/{composer_name}/
   ├── page_001.png
   ├── page_001.json          ← 标注元数据
   ├── page_002.png
   └── ...

2. 配置训练
   models/configs/lora_boulez.yaml
   → LoRA rank, lr, steps, resolution, trigger words

3. 执行训练（在 Windows/WSL2 上）
   python models/train_lora.py --config configs/lora_boulez.yaml

4. 输出
   weights/lora/coda-score-boulez.safetensors
```

### 7.2 LoRA 训练配置模板

```yaml
# models/configs/lora_template.yaml
model:
  base: "stabilityai/stable-diffusion-xl-base-1.0"
  revision: null

lora:
  rank: 32          # 16-64，乐谱纹理简单
  alpha: 16         # 通常 rank 的一半
  target_modules:
    - "to_k"
    - "to_q"
    - "to_v"
    - "to_out.0"

training:
  resolution: 768       # 验证阶段用低分辨率快速跑通，正式训练切到 1024
  batch_size: 1
  gradient_accumulation_steps: 4
  learning_rate: 2e-4
  max_steps: 3000
  mixed_precision: "fp16"
  optimizer: "adamw_8bit"

data:
  train_dir: "data/processed/{style_name}/"
  metadata_file: "data/metadata/annotations/{style_name}.json"
  trigger_word: "in the style of {composer_name}"
  caption_template: "contemporary classical music score, {composer} style, {instrumentation}, {notation_type} notation, high quality engraving, black ink on white paper"

output:
  dir: "weights/lora/"
  name: "coda-score-{style_name}.safetensors"
  save_every_n_steps: 500
```

### 7.3 训练监控

```
- TensorBoard 或 W&B 记录 loss 曲线
- 每 500 步保存 checkpoint 并生成验证样本
- 验证样本用固定 seed 对比不同 step 的风格收敛情况
```

---

## 8. 生成 Pipeline

### 8.1 推理流程

```
用户输入
  ├── 风格：Boulez / Xenakis / 图形记谱 / 自定义组合
  ├── 编制：solo piano / string quartet / chamber ensemble
  ├── 记谱：traditional / graphic / mixed
  ├── 页数：1-4 页
  └── 可选：参考图像（img2img）、ControlNet 布局

       ↓

Prompt 构建
  "contemporary classical music score, in the style of {composer},
   for {instrumentation}, {notation_type} notation,
   high quality music engraving, black ink on white paper,
   {additional_tags}"

       ↓

模型加载
  ├── Base: SDXL
  └── LoRA: 按选择加载（加权组合）

       ↓

扩散生成（在 Windows/WSL2 或 Mac 均可推理）
  ├── 参数：CFG 4-8, Steps 30-50
  ├── 分辨率：1024×1024
  └── 输出：PNG

       ↓

后处理
  ├── 对比度/阈值优化（确保打印清晰）
  ├── 超分辨率（4x → 4096×4096）
  └── 可选：PDF / TIFF 导出
```

### 8.2 生成模式

| 模式 | 方法 | 用途 |
|------|------|------|
| 文本→图像 | Prompt 驱动 | 常规生成，风格化乐谱 |
| 图像→图像 | img2img，参考已有乐谱 | 风格迁移、变奏、续写 |
| ControlNet | 草图/结构图引导 | 控制段落布局、系统划分 |
| Inpainting | 局部修改 | 修改特定区域（如替换记号） |
| LoRA 组合 | 多个 LoRA 加权叠加 | 混合风格（论文核心卖点） |

### 8.3 CLI 工具

```bash
# 单张生成
python inference/generate.py \
  --style boulez \
  --instruments "piano" \
  --notation traditional \
  --output outputs/boulez_piano_001.png

# LoRA 组合生成
python inference/generate.py \
  --lora boulez:0.7,graphic:0.3 \
  --instruments "string quartet" \
  --notation mixed \
  --output outputs/hybrid_001.png

# 批量参数扫描
python inference/generate.py \
  --lora boulez \
  --scan cfgs 4,6,8 \
  --scan steps 30,40,50 \
  --output-dir outputs/boulez_scan/
```

### 8.4 Web UI

基于 Gradio，功能：
- 风格选择（下拉 + 搜索）
- LoRA 组合滑块（多风格叠加，实时权重调节）
- 编制选择
- 记谱类型
- 参数调节（CFG, Steps, Seed）
- 批量生成
- 历史记录浏览
- 生成结果导出

### 8.5 生成后的图像编辑

图形化生成输出的是 PNG，不可逐音修改。但可以通过图像编辑器进行视觉层面的调整——这对现代记谱恰好是自然的工作流：

**传统记谱部分**

需要精确修改时：
- 用 OMR（oemer）反向提取音高，验证生成结果与预期的偏差
- 在 Photoshop/GIMP/Affinity 中用印章工具、选区移动等修整个别符号
- 不追求完美演奏级精度——这不是这条路线的设计目标

**图形记谱部分**（这是图像编辑真正发光的地方）

现代图形记谱本身就是视觉创作。Cardew 的线条、Crumb 的螺旋谱表、Bussotti 的图形——这些在传统制谱软件里根本画不出来，但在图像编辑器里是原生操作：
- 拉曲线、调不透明度、叠加图层
- 变形、缩放、旋转图形符号
- 添加纹理、渐变、非标准视觉元素
- 多页拼接、空间化布局调整

**与符号化路线的编辑对比**

| | 符号化（MuseScore/Dorico） | 图形化生成 + 图像编辑 |
|---|---|---|
| 改单个音符音高 | ✅ 原生支持 | ⚠️ 能用但繁琐 |
| 改图形符号形态 | ❌ 画不了 | ✅ 原生支持 |
| 版面微调 | ✅ | ✅ |
| 非标准视觉元素 | ❌ | ✅ |
| 批量规则修改 | ✅（如全部升高半音） | ❌ |

两害相权：coda-score 选择了对现代记谱更友好的编辑方式。

---

## 9. 评估体系

传统图像生成指标（FID, CLIP Score）对现代乐谱不完全适用。分层评估：

### 9.1 视觉质量
- FID：整体风格一致性
- CLIP Score：prompt-图像对齐
- LPIPS：生成结果多样性

### 9.2 乐谱特定
- 五线谱线是否连续、平行
- 音符、谱号、休止符形态是否正确
- 版面密度是否合理
- 系统分布是否均匀

### 9.3 风格一致性
- 训练风格分类器，评估生成结果为指定风格的概率
- Jeff 盲测：生成乐谱 vs 真实现代乐谱，能否区分
- 同行评估：邀请其他作曲/音乐学研究者评分

### 9.4 可用性
- 打印为 A4 后阅读体验
- 人类音乐家能否理解（不要求完美演奏）
- 对传统记谱部分：OMR 反向提取音高的准确率

---

## 10. 项目结构

```
coda-score/
├── docs/                              # 文档
│   ├── plan-v2.0.md                  # 本方案（主文档）
│   ├── plan-v1.x.md                  # 旧方案（归档参考）
│   ├── research-survey.md            # 学术调研
│   └── data-taxonomy.md              # 风格分类体系详解
├── data/                              # 数据（gitignore 主体）
│   ├── scripts/
│   │   ├── scrape_imslp.py           # IMSLP 爬取
│   │   ├── preprocess.py             # 图像预处理
│   │   ├── label.py                  # 标注工具（GUI 或 CLI）
│   │   ├── render_musicxml.py        # MusicXML → Dorico/MuseScore 渲染
│   │   └── split_dataset.py          # 训练/验证集划分
│   ├── raw/                          # 原始扫描（gitignore）
│   ├── processed/                    # 处理后图像（gitignore）
│   └── metadata/                     # 标注数据
│       ├── style_taxonomy.yaml       # 风格分类体系
│       └── annotations/              # 逐文件标注 JSON
│           ├── boulez/
│           ├── xenakis/
│           └── ...
├── models/                            # 模型训练
│   ├── train_lora.py                 # LoRA 风格训练（核心）
│   ├── train_base.py                 # 基座 fine-tune（可选，需云 GPU）
│   ├── configs/                      # 训练配置
│   │   ├── lora_template.yaml
│   │   ├── lora_boulez.yaml
│   │   └── ...
│   └── utils/
│       ├── dataset.py                # PyTorch Dataset
│       ├── metrics.py                # 评估指标
│       └── augment.py                # 数据增强
├── inference/                         # 推理与生成
│   ├── generate.py                   # CLI 生成工具
│   └── webui/
│       ├── app.py                    # Gradio Web UI
│       └── components/
├── weights/                           # 模型权重（gitignore）
│   └── lora/                         # LoRA .safetensors 文件
│       ├── coda-score-boulez.safetensors
│       ├── coda-score-graphic.safetensors
│       └── ...
├── outputs/                           # 生成结果（gitignore）
├── notebooks/                         # Jupyter 实验笔记
├── study/                             # 保留：现有研究文档
├── v1-archive/                        # 保留：v1.x 代码
├── pyproject.toml
├── requirements.txt
├── .gitignore
└── README.md
```

---

## 11. 开发阶段

### Phase 0：环境搭建 + 最小验证（1-2周） 🎯 当前

**目标：确认 5060 能跑通 LoRA 训练全流程**

- [ ] Windows/WSL2 上搭建 PyTorch + CUDA + Diffusers 环境
- [ ] 验证 SDXL 推理（生成第一张图）
- [ ] 收集 50 页单一风格乐谱（建议从 Boulez 钢琴作品开始，数据相对好找）
- [ ] 标注这 50 页的元数据
- [ ] 用 768×768 低分辨率训练第一个 LoRA（目标：快速跑通全流程，先不跟显存较劲）
- [ ] 生成对比图（无 LoRA vs 有 LoRA）
- [ ] **决策点：效果是否值得继续？** → 如果 50 页 LoRA 已经能看出风格变化，继续 Phase 1
- [ ] 确认低分辨率效果后，切到 1024×1024 正式训练参数

**Phase 0 判定标准（什么算「看出风格变化」）**

对比纯 SDXL 和 SDXL + Boulez LoRA 的生成结果，以下至少满足两项：

1. **五线谱密度差异**：LoRA 版本的五线谱明显更密，音符分布更分散（Boulez 序列主义的典型视觉特征——无调号、密集音符、频繁变换节奏型）
2. **版面风格差异**：LoRA 版本的页面布局（系统数量、音符密度分布）明显区别于普通 SDXL 随意生成的古典风乐谱
3. **记谱惯例差异**：LoRA 版本出现了普通 SDXL 不会出现的现代记谱特征——如无调号、无拍号、频繁临时记号、非传统符干方向
4. **Jeff 认证**：你作为现代音乐专业人士，看两眼能说「这确实有点像 Boulez 的风格」

如果四条全不满足——LoRA 没抓到任何东西——则需要评估是否数据量太少、训练参数不对、还是这条路从根本上走不通。

**Boulez 钢琴作品数据获取清单**

Boulez 的作品中，以下最容易获取（IMSLP 有版权但可通过图书馆/自扫描）：

| 作品 | 页数估算 | 来源 | 记谱特征 |
|------|---------|------|----------|
| *12 Notations* (1945) | ~12 页 | IMSLP / 自扫描 | 早期，12 音但较规整，入门友好 |
| *Sonatine* (1946) | ~30 页 | 可购 Henle 版扫描 | 典型序列主义钢琴写作 |
| *Première Sonate* (1946) | ~20 页 | 可购 | 爆发力强，视觉密度高 |
| *Deuxième Sonate* (1948) | ~48 页 | 可购 | 战后经典，50 页量最足 |
| *Structures I* (1952，双钢琴） | ~88 页 | Universal Edition 可购 | 整体序列主义里程碑 |

**推荐策略**：
- 入手 *Deuxième Sonate*（48 页，一本搞定大部分数据量）
- 补 *12 Notations*（12 页）作为简单样本
- 凑齐 50 页即可开始 Phase 0 训练
- 如果扫描条件受限，先用 IMSLP 上 Boulez 标签下能找到的所有公共领域乐谱（含其他作曲家），凑量优先

### Phase 1：风格 LoRA 核心集合（4-8周）

**目标：覆盖论文所需的核心风格，验证 LoRA 组合**

按优先级逐个训练：

1. **序列主义**（Boulez piano works）— 第一个，数据最易得
2. **图形记谱**（Cardew "Treatise", Crumb, Bussotti）— 论文核心差异化
3. **随机/算法**（Xenakis）— 数学化，视觉特征独特
4. **简约主义**（Reich, Glass）— 重复模式，视觉清晰
5. **频谱音乐**（Grisey, Murail）— 微音程 + 特殊记谱
6. **New Complexity**（Ferneyhough）— 视觉密度极高，最难

每个 LoRA：
- 收集 100+ 页
- 训练 2-3 天（5060）
- 生成验证样本
- 评估风格一致性

**LoRA 组合实验**（Phase 1 后期）：
- 两个 LoRA 叠加（如 Boulez + 图形）
- 权重扫描（0.1-1.0 步长 0.1）
- 分析混合效果（论文关键实验）

### Phase 2：生成系统（2-3周）

- [ ] CLI 工具完整实现
- [ ] Gradio Web UI
  - 风格选择 + LoRA 组合滑块
  - 实时生成预览
  - 批量模式
- [ ] 超分辨率后处理自动化
- [ ] 导出功能（PNG / PDF / TIFF）

### Phase 3：论文实验（4-6周）

- [ ] 定量评估（FID, CLIP Score, 风格分类准确率）
- [ ] 人类评估：
  - Jeff 盲测（生成 vs 真实现代乐谱）
  - 同行评估（3-5 位音乐研究者）
- [ ] LoRA 组合消融实验
- [ ] 风格转移实验（用 ControlNet 保持结构 + 换 LoRA 风格）
- [ ] 与基线对比（纯 SDXL、纯 SD + LoRA、MusicScore 等）

### Phase 4（可选）：全量基座 fine-tune

**仅在前几阶段效果不足时才考虑**

- 租云 GPU（AutoDL A100 40GB）
- 收集 10K+ 页公共领域乐谱
- 全量 fine-tune SDXL
- 在基座上重新训练所有 LoRA
- 对比新旧基座的效果差异

---

## 12. 论文方向

### 12.1 三个贡献角度

1. **首个现代音乐乐谱图像生成系统**
   - 技术挑战：现代记谱的图像复杂性、风格多样性
   - 方法设计：LoRA 组合实现风格可控生成

2. **现代音乐乐谱图像数据集**
   - 风格分类体系（见第 6 节）
   - 标注数据集（包含多维度标签）
   - 可公开发布公共领域部分

3. **基于 LoRA 组合的风格化生成框架**
   - 可叠加、权重可调的多风格混合
   - 这是与其他乐谱生成工作的明确区分点

### 12.2 建议论文结构

```
数据集贡献 → 方法（LoRA 训练 + 组合） → 实验 → 人类评估
```

对标 MusicScore 但聚焦现代音乐。MusicScore 的贡献是「大型乐谱图像数据集 + 扩散模型基座」，你的贡献是「现代音乐风格 LoRA 体系 + 组合生成框架 + 现代乐谱数据集」。

---

## 13. 项目意义

### 13.1 核心论证：现代记谱是一种独立的视觉语法

这个项目在技术上是「扩散模型生成乐谱图像」，但它回答的实际上是一个音乐学问题：

> 20 世纪作曲家创造的视觉记谱语言（图形记谱、比例记谱、非标准谱表等），是传统音符的「附加装饰」，还是一个独立于符号化音乐理论的视觉系统？

实验逻辑很简单：如果一个纯粹在图像层面学习的模型（扩散模型），不接触任何音乐理论、音高关系、和声规则——仅靠「看」乐谱图像——就能学会生成特定风格的谱面，那就证明了这种视觉语法的存在及其独立性。

Boulez 去掉调号和拍号、Cardew 只用线条不用音符、Ferneyhough 把五线谱塞到像集成电路——这些决定执行层面是视觉的。扩散模型抓到了它们，就意味着它们确实在那里，是可学习的视觉规律，不是主观解读。

### 13.2 对作曲教育的启示（人能否这样学）

推论：既然机器可以不依赖音乐理论、仅凭视觉模式学习来生成特定风格的记谱，人类的作曲学习过程中，视觉模仿的认知权重可能被显著低估了。

传统作曲教学路径：调性和声（2-3年）→ 对位（1-2年）→ 曲式（1年）→ 20世纪技法（往往蜻蜓点水）。但如果现代记谱真的是一种可通过视觉习得的独立语法，前面的传统训练对于「写出看起来像现代音乐的东西」可能不是逻辑前提。

这与作曲实践的历史事实一致：
- Boulez、Stockhausen、Ligeti 都受过完整的传统训练——这往往是「传统训练必要论」的证据
- 但从认知机制的角度，他们的现代作品里，传统和声学的直接迁移几乎为零——他们学到的是结构思维和批判意识，而非「写对和声进行」的技能
- 一个学生直接从 Step 4 开始，大量看现代谱面，通过视觉模仿生成——这种路径的可行性，目前的教育学几乎没有讨论过

**论文中如何呈现**：不需要专门做教育实验。在讨论部分用一段话提出即可，审稿人喜欢这种「这个发现对其他领域意味着什么」的延展。

### 13.3 三层论证结构

```
技术层：扩散模型可以用纯视觉方式学习现代记谱
    ↓
音乐学层：这证明了现代记谱的视觉语法独立于符号化音乐理论
    ↓
教育层：人类的作曲学习或许可以少绕一些理论弯路
```

论文的标准结构是「实验 → 讨论」。「技术层」是实验部分，「音乐学层」和「教育层」放在讨论部分作为两个延展论点。不需要额外数据，从方法反推即可成立。

---

## 14. 风险与缓解

| 风险 | 严重度 | 缓解 |
|------|--------|------|
| **音高精度不足**：扩散模型生成的音符位置偏离实际音高 | 🔴 高 | 接受图形记谱的"近似性"；传统记谱部分可用 OMR 验证；不承诺可演奏 |
| **数据极度稀缺**：现代乐谱数字图像少 | 🔴 高 | 自扫描为主；MusicXML 渲染补充；接受小数据集（few-shot） |
| **风格边界模糊**：现代风格交叉多，难标注 | 🟡 中 | 多标签标注；允许风格混合；本身就是研究课题 |
| **图形记谱难评估**：无法用传统指标 | 🟡 中 | 以 Jeff 专家评估为核心；盲测 + 同行评估 |
| **5060 显存不够**：LoRA 训练爆显存 | 🟢 低 | 降分辨率（960²/896²）；降 LoRA rank 16；8-bit AdamW；gradient checkpointing |
| **版权**：训练数据含版权作品 | 🟢 低 | 权重不开源；论文仅展示公共领域产品的生成结果；学术 TDM 免责 |

---

## 15. 工具链

| 组件 | 选择 | 说明 |
|------|------|------|
| 深度学习框架 | PyTorch 2.x + CUDA | — |
| 扩散模型 | HuggingFace Diffusers | SDXL pipeline |
| LoRA 训练 | diffusers（PEFT） | 自带 LoRA 支持，无需 Kohya |
| 图像处理 | OpenCV, Pillow, scikit-image | — |
| 图像编辑（后处理） | Photoshop / GIMP / Affinity | 图形记谱编辑原生于图像编辑器 |
| Web UI | Gradio | 快速搭建，HuggingFace 生态 |
| OMR 验证 | oemer (BreezeWhite) | 可选，前处理验证 |
| 超分辨率 | Real-ESRGAN | 4x 超分 |
| 实验跟踪 | W&B 或 TensorBoard | — |
| 依赖管理 | uv / pip | — |
| 版本管理 | Git + GitHub | 代码 |
| 大文件传输 | iCloud / SMB | 数据集和权重 |
| 训练硬件 | RTX 5060 12GB（本地） | LoRA 训练主力 |
| 备选训练 | AutoDL / 云 GPU | 如需全量 fine-tune |
| 推理硬件 | 5060 或 MacBook (MPS) | 单张推理两台都能跑 |

---

## 16. 下一步行动

**现在要做的事（Phase 0）：**

1. 在 Windows/WSL2 上装 PyTorch CUDA + Diffusers
2. 跑通 SDXL 基础推理（生成任意一张图，验证 CUDA 可用）
3. 收集 50 页 Boulez 钢琴作品乐谱（IMSLP + 自扫描）
4. 标注这 50 页
5. 训练第一个 LoRA
6. 对比有/无 LoRA 的生成结果

5 和 6 是整个项目能否走下去的关键验证——确认 5060 + SDXL + LoRA 确实能抓到现代乐谱的风格特征。如果这一步效果明显，后续的 Phase 1-3 就是时间和工作量的问题。
