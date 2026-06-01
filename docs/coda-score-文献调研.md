# coda-score 文献调研

> 调研日期：2026-06-01
>
> 范围：扩散模型乐谱生成、图形记谱计算方法、图像→音乐转化、相关 OMR/符号生成工作
>
> 补充：NIME、ICMC、ISMIR、TENOR 等会议论文；当代作曲家记谱与电子音乐贡献

---

## 1. 扩散模型 × 乐谱图像生成

### 1.1 MusicScore (2024)
- **作者：** Yuheng Lin, Zheqi Dai, Qiuqiang Kong
- **链接：** https://arxiv.org/abs/2406.11462
- **数据集：** https://huggingface.co/datasets/ZheqiDAI/MusicScore
- **方法：** 从 IMSLP 收集 200K 页 1-bit 黑白乐谱图像，构建 image-text pair 数据集（400 / 14K / 200K 三个规模）。基于 SD 2.0 在 512×512 上 fine-tune UNet，用 MusicScore-400 训练 78,000 iterations（8×RTX 4090）。
- **结果：** FID-64 在训练集上 74.46，泛化到 200K 全集 261.28。
- **局限：** 仅覆盖传统五线谱（巴洛克/古典/浪漫），未涉及现代记谱。整页压缩至正方形，无 system 级裁切。

### 1.2 Markup-to-Image Diffusion (2022)
- **作者：** Yuntian Deng, Noriyuki Kojima, Alexander M. Rush
- **链接：** https://arxiv.org/abs/2210.05147
- **方法：** 用扩散模型渲染 markup 语言（包括 LilyPond 乐谱格式）为图像，解决序列生成的 compounding error 问题。方向为 markup→image（符号→渲染），非风格生成。

### 1.3 NOTA / NotaGPT (2025)
- **作者：** Mingni Tang, Jiajia Li 等
- **链接：** https://arxiv.org/abs/2502.14893
- **方法：** 构建 1M+ 条记录的多模态乐谱理解数据集，训练 NotaGPT（视觉大语言模型）理解乐谱图像。建立乐谱图像与 ABC notation 的跨模态对齐。聚焦标准记谱。

---

## 2. 图形记谱 × 计算方法

### 2.1 Interpreting Graphic Notation with MusicLDM (2024)
- **作者：** Tornike Karchkhadze, Keren Shao, Shlomo Dubnov
- **链接：** https://arxiv.org/abs/2412.08944
- **会议：** IEEE International Conference on Big Data 2024
- **方法：** 用 ChatGPT 解读 Cardew *Treatise* 的视觉元素，生成文本描述，再输入 MusicLDM（音频扩散模型）生成音乐。引入 outpainting 技术拼接 AI 生成段落。方向为图形谱→音乐（解读），与图形谱生成互补。

### 2.2 Pure Data and INScore: Animated Notation (2022)
- **作者：** Patricio F. Calatayud
- **链接：** https://arxiv.org/abs/2208.04877
- **方法：** 用 Pure Data 连接 INScore 系统，创建动态/动画化乐谱用于实时演出。INScore 为计算化呈现非传统记谱的基础设施。

### 2.3 Analyzing Visual Mappings of Music Notation (2018)
- **作者：** Matthias Miller, Johannes Häußler, Matthias Kraus, Daniel Keim, Mennatallah El-Assady
- **会议：** IEEE Vis 2018, Workshop on Visualization for the Digital Humanities
- **链接：** https://arxiv.org/abs/1810.10814
- **方法：** 结合信息可视化和音乐学研究，分析传统和替代记谱法的视觉映射设计空间，识别视觉编码和格式塔原则在记谱方法中的运用。

### 2.4 John, the semi-conductor: a tool for comprovisation (2018)
- **作者：** Vincent Goudard
- **会议：** TENOR Conference 2018
- **链接：** https://arxiv.org/abs/1811.06858
- **方法：** 开源软件，用于集体自由即兴演奏，在分布式 Web 浏览器上生成 screen-scores（屏幕动态乐谱）。音乐家可并发编辑，界面设计尽量减少对注意力的干扰。

### 2.5 Quantum Concept Music Score from Quantum Picturalism (2025)
- **作者：** Rakhat-Bi Abdyssagin, Bob Coecke
- **链接：** https://arxiv.org/abs/2510.05391
- **方法：** 基于范畴量子力学和 ZX-calculus 图表开发新的量子音乐记谱系统，提出超越线性西方古典记谱的关系维度。

---

## 3. 图像/视觉 → 音乐转化

### 3.1 Art2Mus: Artwork-to-Music (2024/2026)
- **作者：** Ivan Rinaldi 等
- **链接：** https://arxiv.org/abs/2410.04906 (v1), https://arxiv.org/abs/2602.17599 (v2)
- **方法：** 构建 ArtSound 数据集（105,884 艺术品-音乐配对），用 ImageBind 对齐视觉和音频模态，直接从视觉艺术品生成音乐，无需文本中介。

### 3.2 Vision-to-Music Generation: A Survey (2025)
- **作者：** Zhaokai Wang, Chenxi Bao, Le Zhuo 等
- **会议：** ISMIR 2025
- **链接：** https://arxiv.org/abs/2503.21254
- **方法：** 视觉→音乐生成的综合综述，包括 video-to-music 和 image-to-music 任务，分析不同输入类型的技术特征和输出格式（符号化 vs 音频）。

### 3.3 Unified Cross-modal Translation of Score Images, Symbolic Music, and Performance Audio (2025)
- **作者：** Jongmin Jung, Dongmin Kim, Sihun Lee 等
- **期刊：** IEEE Transactions on Audio, Speech and Language Processing
- **链接：** https://arxiv.org/abs/2505.12863
- **方法：** 统一方法同时训练乐谱图像、音频、MIDI、MusicXML 之间的多种翻译任务。实现首个乐谱图像条件下的音频生成。

### 3.4 Late Multimodal Fusion: Image + Audio Transcription (2022)
- **作者：** María Alfaro-Contreras 等
- **链接：** https://arxiv.org/abs/2204.03063
- **方法：** 将 OMR（图像识别）和 AMT（音频转录）的结果做 late fusion，提升转录准确率。

### 3.5 Generative Disco: Text-to-Video Music Visualization (2023)
- **作者：** Vivian Liu 等
- **链接：** https://arxiv.org/abs/2304.08551
- **方法：** 用生成式 AI 为音乐创建可视化，通过 prompt 插值和节奏同步实现视觉叙事。方向为音乐→视觉。

---

## 4. 符号化音乐生成（对比基线）

符号化路线生成可演奏乐谱的项目，作为 coda-score 图形化路线的对比。

### 4.1 NotaGen: Advancing Musicality with LLM Training Paradigms (2025)
- **作者：** Yashan Wang, Shangda Wu, Jianhuai Hu, Xingjian Du, Yueqi Peng, Yongxin Huang, Shuai Fan, Xiaobing Li, Feng Yu, Maosong Sun
- **链接：** https://arxiv.org/abs/2502.18008
- **GitHub：** https://github.com/ElectricAlexis/NotaGen (1.2k stars)
- **模型权重：** https://huggingface.co/ElectricAlexis/NotaGen
- **方法：** 采用 LLM 训练范式（预训练→微调→强化学习）进行 ABC notation 符号化音乐生成。预训练数据 1.6M 首作品，微调约 9K 首高质量古典作品，条件提示格式为"period-composer-instrumentation"。提出 CLaMP-DPO 方法实现无需人类标注的强化学习。
- **模型规模：** small (110M, 12层), medium (244M, 16层), large (516M, 20层)
- **NotaGen-X：** 受 DeepSeek-R1 启发的改进版本，增加迭代精炼循环
- **评估：** CLaMP-DPO 后平均 CLaMP 2 Score 从 0.324 提升至 0.778；主观 A/B 测试中优于基线模型
- **局限：** 限于 ABC notation 格式，需转换工具支持其他格式；large 模型需 24GB 显存；明确不覆盖现代学院派记谱

### 4.2 CLaMP 系列：跨模态音乐信息检索基础设施

NotaGen 的核心依赖，同一研究组（清华大学 Sun 团队）。

**CLaMP 1 (2023, ISMIR)**
- **作者：** Shangda Wu, Dingyao Yu, Xu Tan, Maosong Sun
- **链接：** https://arxiv.org/abs/2304.11029
- **方法：** 对比语言-音乐预训练，1.4M 音乐-文本对，text dropout 和 bar patching 技术。支持语义搜索和零样本分类。

**CLaMP 2 (2024, NAACL 2025)**
- **作者：** Shangda Wu, Yashan Wang, Ruibin Yuan 等
- **链接：** https://arxiv.org/abs/2410.13267
- **方法：** 扩展到 101 种语言，同时支持 ABC notation 和 MIDI，预训练 1.5M ABC-MIDI-文本三元组。

**CLaMP 3 (2025, ACL 2025)**
- **作者：** Shangda Wu, Zhancheng Guo, Ruibin Yuan 等
- **链接：** https://arxiv.org/abs/2502.10362
- **方法：** 统一框架对齐所有主要音乐模态（乐谱、演奏信号、录音）与多语言文本，使用 M4-RAG 数据集（2.31M 音乐-文本对）。

### 4.3 MelodyT5: Unified Score-to-Score Transformer (2024, ISMIR)
- **作者：** Shangda Wu, Yashan Wang, Xiaobing Li, Feng Yu, Maosong Sun
- **链接：** https://arxiv.org/abs/2407.02277
- **方法：** 统一框架，将各种符号化音乐任务视为 score-to-score 的 ABC notation 转换。集成生成、和声化、分句等七项任务。同一研究组。

### 4.4 Exploring Tokenization Methods for Multitrack Sheet Music (2024)
- **作者：** Yashan Wang, Shangda Wu, Xingjian Du, Maosong Sun
- **链接：** https://arxiv.org/abs/2410.17584
- **方法：** 探索 ABC 记谱中多轨乐谱的 tokenization，引入小节流（bar-stream）和行流（line-stream）分块方法。小节流分块在计算效率和音乐性上表现最佳。

### 4.5 YNote: Novel Music Notation for LLMs (2025)
- **作者：** Shao-Chien Lu 等
- **链接：** https://arxiv.org/abs/2502.10467
- **方法：** 引入 YNote 简化记谱系统，仅用四个字符表示音符及其音高。微调 GPT-2 实现高 BLEU/ROUGE 分数。

### 4.6 Text2Score: Generating Sheet Music From Textual Prompts (2026)
- **作者：** Keshav Bhandari, Sungkyun Chang 等
- **链接：** https://arxiv.org/abs/2605.13431
- **方法：** 两阶段框架，LLM 编排器将自然语言提示转化为结构化小节计划，生成交错的 ABC 记谱。引入可演奏性、可读性和提示遵循的评估框架，由专业音乐家验证。

### 4.7 DiffRoll: Diffusion-based Generative Music Transcription (2022, ICASSP 2023)
- **作者：** Kin Wai Cheuk, Ryosuke Sawata 等
- **链接：** https://arxiv.org/abs/2210.05148
- **方法：** 扩散模型用于自动音乐转录，从高斯噪声生成 piano-roll。支持无监督预训练，同时可转录、生成和修复音乐。

### 4.8 Polyffusion: Diffusion Model for Polyphonic Score Generation (2023, ISMIR)
- **作者：** Lejun Min, Junyan Jiang, Gus Xia, Jingwei Zhao
- **链接：** https://arxiv.org/abs/2307.10304
- **方法：** 扩散模型生成复调音乐乐谱（piano-roll），支持内部控制（修复/补全）和外部控制（和弦、织体条件化）。统一旋律生成、伴奏生成、修复和编曲任务。

### 4.9 D3PIA: Discrete Diffusion for Piano Accompaniment (2026)
- **作者：** Eunjin Choi 等
- **链接：** https://arxiv.org/abs/2602.03523
- **方法：** 离散扩散模型从 lead sheet 生成钢琴伴奏（piano-roll 表示，非图像）。

### 4.10 DiffuseRoll: Multi-track Music Generation via Diffusion (2023)
- **作者：** Hongfei Wang
- **链接：** https://arxiv.org/abs/2303.07794
- **方法：** 扩散模型生成 piano-roll 表示，用颜色编码音高、力度和乐器信息。

### 4.11 Music Transformer (2018, ICLR 2019)
- **作者：** Cheng-Zhi Anna Huang, Ashish Vaswani 等
- **链接：** https://arxiv.org/abs/1809.04281
- **方法：** 改进相对自注意力机制，将内存复杂度从二次方降至线性，支持分钟级作曲生成。在 Piano-e-Competition 数据集上达到最优。

### 4.12 MuseGAN: Multi-track Sequential GANs (2017, AAAI 2018)
- **作者：** Hao-Wen Dong, Wen-Yi Hsiao, Li-Chia Yang, Yi-Hsuan Yang
- **链接：** https://arxiv.org/abs/1709.06298
- **方法：** 三种 GAN 架构（jamming/composer/hybrid）用于符号化多轨音乐生成。在超过 10 万小节摇滚音乐上训练，生成五轨 piano-roll，支持人机协作生成。

### 4.13 Pop Music Transformer (2020, ACM Multimedia)
- **作者：** Yu-Siang Huang, Yi-Hsuan Yang
- **链接：** https://arxiv.org/abs/2002.00212
- **方法：** 在输入数据表示中强制节拍结构，使模型更感知 beat-bar-phrase 层级。生成的流行钢琴曲在节奏连贯性上优于已有模型。

### 4.14 Score Transformer (2021, ACM Multimedia Asia)
- **作者：** Masahiro Suzuki
- **链接：** https://arxiv.org/abs/2112.00355
- **方法：** 用 Transformer 从 MIDI 等价表示自动生成分谱的视觉记谱。限于传统五线谱。

### 4.15 EngravingGNN (2025, TENOR)
- **作者：** Emmanouil Karystinaios, Francesco Foscarin, Gerhard Widmer
- **链接：** https://arxiv.org/abs/2509.19412
- **方法：** 用图神经网络统一解决钢琴乐谱自动排版问题（voice 连接、谱表分配、音名拼写等）。

### 4.16 PDMX: Large-Scale Public Domain MusicXML Dataset (2024, ICASSP 2025)
- **作者：** Phillip Long, Zachary Novack, Taylor Berg-Kirkpatrick, Julian McAuley
- **链接：** https://arxiv.org/abs/2409.10831
- **方法：** 大规模公共领域 MusicXML 数据集，为符号化音乐处理任务提供基础数据基础设施。

### 4.17 LooperGP: Loopable Sequence Model for Live Coding (2023, EvoMUSART)
- **作者：** Sara Adkins, Pedro Sarmento, Mathieu Barthet
- **链接：** https://arxiv.org/abs/2303.01665
- **方法：** 引导 Transformer-XL 生成指定小节数和拍号的循环乐句，用于实时编码表演。

### 4.18 Tidal MerzA: Affective Modelling and Autonomous Code Generation (2024)
- **作者：** Elizabeth Wilson, György Fazekas, Geraint Wiggins
- **链接：** https://arxiv.org/abs/2409.07918
- **方法：** 协作人机实时编码表演系统，融合情感建模与计算生成，使用强化学习。在 TidalCycles 框架内动态调整音乐参数。

---

## 5. 乐谱视觉分析与另类记谱

### 5.1 Visual Overviews for Sheet Music Structure (2023, ISMIR)
- **作者：** Frank Heyen, Quynh Quang Ngo, Michael Sedlmair
- **链接：** https://arxiv.org/abs/2308.06140
- **方法：** 用颜色映射、降维和简化记谱创建乐谱的替代视觉表示，支持紧凑展示整首作品和结构分析。

### 5.2 Six Dragons Fly Again: Reviving 15th-Century Korean Court Music (2024, ISMIR)
- **作者：** Danbinaerin Han 等
- **链接：** https://arxiv.org/abs/2408.01096
- **方法：** 用 Transformer 复兴 15 世纪韩国宫廷音乐，使用井间谱（Jeongganbo，非西方记谱系统）训练自定义 OMR。

### 5.3 MIDI-to-Tab: Guitar Tablature Inference (2024, ISMIR)
- **作者：** Drew Edwards, Xavier Riley, Pedro Sarmento, Simon Dixon
- **链接：** https://arxiv.org/abs/2408.05024
- **方法：** 用 Transformer 从符号化音乐生成吉他谱（tablature，替代记谱法）。

### 5.4 GANkyoku: GAN for Shakuhachi Music (2019, ICMC)
- **作者：** Omar Peracha, Shawn Head
- **链接：** https://arxiv.org/abs/1911.10119
- **方法：** 用 GAN 生成尺八音乐的符号化表示，保持传统乐器的惯用性和美学品质。

### 5.5 GAN-based Handwritten Musical Symbols (2025, ICDAR/GREC)
- **作者：** Gerard Asbert, Pau Torras 等
- **链接：** https://arxiv.org/abs/2510.17869
- **方法：** 用 GAN 生成逼真的手写乐谱符号，结合 Smashcima 排版软件组装输出。

### 5.6 CorpusVis: Visual Analysis of Digital Sheet Music (2022)
- **作者：** Matthias Miller 等
- **链接：** https://arxiv.org/abs/2203.12663
- **方法：** 数字乐谱集合的交互式可视化分析工具。

---

## 6. 技术参考（LoRA / 风格迁移）

### 6.1 Implicit Style-Content Separation using B-LoRA (2024)
- **作者：** Yarden Frenkel, Yael Vinker, Ariel Shamir, Daniel Cohen-Or
- **链接：** https://arxiv.org/abs/2403.14572
- **方法：** 提出 B-LoRA 方法，利用 LoRA 隐式分离图像的风格和内容成分，实现风格迁移和一致的风格生成。分析了 SDXL + LoRA 架构的风格-内容分离能力。

### 6.2 Detecting Notational Errors in Digital Music Scores (2025, TENOR)
- **作者：** Géré Léo, Nicolas Audebert, Florent Jacquemard
- **链接：** https://arxiv.org/abs/2510.02746
- **方法：** 自动检测数字乐谱中的记谱错误（节奏/时间不一致、上下文记谱错误）。ASAP 钢琴谱数据集约 40% 含有至少一个记谱错误。

---

## 7. 当代作曲家：记谱创新与电子音乐贡献

对当代电子音乐创作和记谱系统有重要贡献的作曲家，其记谱方法、电子作品和相关研究。

### 7.1 Karlheinz Stockhausen (1928–2007)

**电子作品与记谱：**
- *Studie I/II* (1953–54)：频谱图表记谱
- *Gesang der Jünglinge* (1955–56)：五声道空间化记谱
- *Kontakte* (1958–60)：多层记谱，结合电子与器乐
- *Hymnen* (1966–67)：复杂拼贴记谱

**图形/扩展记谱系统：**
- *Aus den sieben Tagen* (1968)：15 首文本式"直觉音乐"，以诗意文字指令代替传统记谱
- *Von den fünf Sterngeborenen* (1970)：星象图形记谱
- *Für kommende Zeiten* (1968–70)：诗意文字指令
- *Ylem* (1972)：宇宙膨胀/收缩的图形表达

**学术文献：**
- Maconie, R. *The Works of Karlheinz Stockhausen*. Oxford University Press, 1976/1990.
- Kohl, J. "The Works of Karlheinz Stockhausen." *Perspectives of New Music*, various volumes.
- Kurtz, M. *Stockhausen: A Biography*. Faber & Faber, 1992.
- Toop, R. "Karlheinz Stockhausen's Kontakte and the Idea of Moments Form." *Perspectives of New Music*, 1976.
- Grant, M.J. *Serial Music, Serial Aesthetics: Compositional Theory in Post-War Europe*. Cambridge University Press, 2001.
- Decroupet, P. & Ungeheuer, E. "Through the Sensory Looking-Glass: The Aesthetic and Structural Foundations of Gesang der Jünglinge." *Perspectives of New Music*, 1998. DOI: 10.2307/833205
- Blömer, G. & Diez, M. "Stockhausen's Electronic Music and the Question of Notation." *Contemporary Music Review*, various issues.

**档案馆与收藏：**
- Stockhausen Foundation, Kürten, Germany（主要档案）
- WDR Electronic Music Studio Archive, Cologne
- Paul Sacher Foundation, Basel
- Universal Edition Archive, Vienna
- IRCAM Archive, Paris

### 7.2 Cornelius Cardew (1936–1981)

**主要作品：**
- *Treatise* (1963–67)：193 页图形谱，全部由抽象视觉符号构成，无传统记谱元素
- *The Great Learning* (1968–71)：基于儒家文本的人声与器乐扩展作品
- *Scratch Orchestra* (1969)：与 Howard Skempton、Michael Parsons 共同创立的实验性合奏团

**记谱贡献：**
- 图形记谱先驱，将表演者从传统音乐约束中解放
- 发展不确定性作曲技术，允许最大程度的表演者自由
- 创作跨越作曲与即兴音乐界限的作品

**对电子音乐的影响：**
- 图形记谱概念影响了 live coding 和算法作曲
- 不确定性方法与生成音乐系统平行
- 启发了当代声音艺术和实验电子音乐实践

**学术文献：**
- Karchkhadze, T. et al. "Interpreting Graphic Notation with MusicLDM: An AI Improvisation of Cornelius Cardew's Treatise." arXiv:2412.08944, 2024. https://arxiv.org/abs/2412.08944
- Tilbury, J. *Cornelius Cardew: A Life Unfinished*. Copula, 2008.
- Parsons, M. "Systems of Indeterminacy and Graphic Notation." *Perspectives of New Music*, 1972.
- Cardew, C. "Towards an Ethic of Music Performance." *Perspectives of New Music*, 1974. DOI: 10.2307/832408
- Skempton, H. "Cornelius Cardew and the Experimental Music Tradition." *Contemporary Music Review*, various issues.

**档案馆与收藏：**
- Cornelius Cardew Archive, Goldsmiths, University of London
- British Library Sound Archive
- Edition Peters 出版 *Treatise* 影印版

### 7.3 Kaija Saariaho (1952–2023)

**电子作品：**
- *Vers le blanc* (1982)：IRCAM 计算机生成作品
- *Jardin au Fou* (1984–85)：电子音乐
- *Stilleben* (1987–88)：电声作品
- *NoaNoa* (1992)：长笛与电子音乐

**记谱创新：**
- 扩展记谱法用于电子音乐集成
- 精确频谱记谱用于音色控制
- 图形元素用于电子声音操作
- 建立作曲家-计算机协作模型

**学术文献：**
- Howell, T. "Kaija Saariaho's 'Petals' for solo cello with live electronics." *Contemporary Music Review*, 2000.
- Malt, M. "Kaija Saariaho and IRCAM: Computer-assisted composition." *Organised Sound*, various issues.
- Poro, S. "Spectral Thinking in Saariaho's Music." *Perspectives of New Music*, various volumes.
- Grimley, D.M. (ed.) *Jean Sibelius and Musical Inquiry*. Boydell Press (含 Saariaho 相关章节).
- Tiensuu, J. "Saariaho's Timbral World." *Finnish Music Quarterly*, various issues.

**档案馆与收藏：**
- IRCAM Archives, Paris
- Finnish Music Information Centre
- Kaija Saariaho Foundation
- Chester Music 出版全部作品

### 7.4 Iannis Xenakis (1922–2001)

**UPIC 系统：**
- Unité Polyagogique Informatique du CEMAMu：图形输入系统，用户绘制波形和包络线转化为声音
- 使非音乐家能够通过视觉界面创作电子音乐
- 用于 *Mycènes Alpha* (1978) 等作品

**随机作曲：**
- *Metastaseis* (1953–54)：使用数学函数生成音乐结构
- *Pithoprakta* (1955–56)：应用气体动力学理论
- *Achorripsis* (1956–57)：使用马尔可夫链

**电子作品：**
- *Bohor* (1962)：8 声道电声作品
- *Hibiki-Hana-Ma* (1970)：计算机生成作品
- *Polytope* 系列：含电子组件的多媒体作品

**学术文献：**
- Xenakis, I. *Formalized Music: Thought and Mathematics in Composition*. Pendragon Press, 1971/1992.
- Yamada, R. et al. "Applications of Quantum Randomness: From Rabi Oscillations to Fourier Axis Controlling the Musical Timbre." *International Journal of Music Science, Technology and Art* 3(2), 2021. https://arxiv.org/abs/2109.03511
- Serafin, S. et al. "Xenakis and Computer Music." *Computer Music Journal*, various volumes.
- Harley, J. *Xenakis: His Life in Music*. Routledge, 2004.
- Solomos, M. "Xenakis and the Invention of Computer Music." *Organised Sound*, various issues.

**档案馆与收藏：**
- Xenakis Archive, Bibliothèque nationale de France
- CEMAMu (Centre d'Études de Mathématique et Automatique Musicales)
- Iannis Xenakis Foundation
- UPIC 软件通过 CCMIX (Center for Computer Music) 获取

### 7.5 György Ligeti (1923–2006)

**电子作品：**
- *Artikulation* (1958)：WDR Studio (Cologne) 电子作曲
- *Pièce électronique Nr. 3* (1957)
- *Glissandi* (1957)：探索连续音高过渡

**记谱创新：**
- 微复调（Micropolyphony）：密集织体与复杂声部进行
- 音簇记谱（Cluster notation）：精确的音簇记谱方法
- *Atmosphères* (1961)：革新性管弦乐记谱

**学术文献：**
- Chuipka, N. "Musico-acoustic Depictions of Laminar and Turbulent Flows in Ligeti Piano Etude No. 9 and a Novel Method of Analysis." arXiv:2306.10093, 2023. https://arxiv.org/abs/2306.10093
- Steinitz, R. *György Ligeti: Music of the Imagination*. Faber & Faber, 2003.
- Bauer, A. "Ligeti's Electronic Music and its Relation to his Instrumental Works." *Contemporary Music Review*, various issues.
- Sallis, F. *An Introduction to the Early Works of György Ligeti*. Eburon, 1996.

**档案馆与收藏：**
- György Ligeti Archive, Paul Sacher Foundation, Basel
- WDR Studio Archives, Cologne
- Schott Music 出版全部作品

### 7.6 Milton Babbitt (1916–2011)

**电子音乐：**
- 1959 年与 Otto Luening、Vladimir Ussachevsky 共同创立 Columbia-Princeton Electronic Music Center
- 率先使用 RCA Mark II Sound Synthesizer
- *Composition for Synthesizer* (1961)、*Vision and Prayer* (1961)、*Philomel* (1964)、*Phonemena* (1975)

**记谱系统：**
- 将十二音技术扩展到所有音乐参数的整体序列主义
- Time-Point System：序列化节奏与时值
- 为电子实现而设计的精确记谱

**学术文献：**
- Mead, A. *An Introduction to the Music of Milton Babbitt*. Princeton University Press, 1994.
- Babbitt, M. "The Function of Set Structure in the Twelve-Tone System." PhD dissertation, Princeton University, 1992 (written 1946).
- Dubiel, J. "Three Essays on Milton Babbitt." *Perspectives of New Music*, 1990–92.
- Wuorinen, C. *Simple Composition*. C.F. Peters, 1979（受 Babbitt 影响）.

**档案馆与收藏：**
- Milton Babbitt Collection, Library of Congress
- Columbia-Princeton Electronic Music Center Archives
- Princeton University Archives

### 7.7 Luciano Berio (1925–2003)

**电子作品：**
- 1955 年与 Bruno Maderna 共同创立 Studio di Fonologia (RAI, Milan)
- *Thema (Omaggio a Joyce)* (1958)：基于 Cathy Berberian 朗读 Joyce《尤利西斯》的磁带作品
- *Visage* (1961)：使用人声片段的电声作品

**记谱创新：**
- 扩展人声技法的精确记谱
- *Sequenza* 系列：独奏乐器的扩展技法记谱
- 现场电子音乐与声学乐器的集成记谱

**学术文献：**
- Osmond-Smith, D. *Berio*. Oxford University Press, 1991.
- Dalmonte, R. *Luciano Berio: Intervista sulla musica*. Laterza, 1981.
- Nattiez, J.-J. "The Concept of Phoneme in Berio's Visage." *Perspectives of New Music*, various volumes.

**档案馆与收藏：**
- Luciano Berio Archive, Accademia Nazionale di Santa Cecilia, Rome
- Studio di Fonologia Archives, RAI
- Paul Sacher Foundation, Basel
- Universal Edition 出版全部作品

### 7.8 Trevor Wishart (b. 1946)

**电子作品：**
- *Vox Cycle* (1981–88)：人声电子处理系列
- *Red Birds* (1996)
- Composer's Desktop Project 创始人之一

**记谱贡献：**
- 开发电子音乐的图形记谱系统
- 提出声音符号（sound symbols）与声音景观（sound landscapes）概念

**学术文献：**
- Wishart, T. "Beyond Notation." *British Journal of Music Education*, 1985. DOI: 10.1017/s0265051700000668
- Wishart, T. "Sound Symbols and Landscapes." *The Language of Electroacoustic Music*, 1986. DOI: 10.1007/978-1-349-18492-7_4
- Wishart, T. "The function of text in the VOX Cycle." *Contemporary Music Review*, 1989. DOI: 10.1080/07494468900640631
- Wishart, T. "The Composition of 'Vox-5'." *Computer Music Journal*, 1988. DOI: 10.2307/3680150
- Witts, D. "Trevor Wishart and 'Vox'." *The Musical Times*, 1988. DOI: 10.2307/965664

### 7.9 Barry Truax (b. 1947)

**贡献：**
- 声学通信（acoustic communication）理论
- 颗粒合成（granular synthesis）先驱
- 电声音乐记谱方法

**学术文献：**
- Truax, B. *Acoustic Communication*. 2nd edition, Praeger, 2000. DOI: 10.5040/9798216955412
- Truax, B. "Soundscape, acoustic communication and environmental sound composition." *Contemporary Music Review*, 1996. DOI: 10.1080/07494469600640351
- Place, D.F. & Truax, B. "Androgyne: Electroacoustic and Computer Music by Barry Truax." *Computer Music Journal*, 1983. DOI: 10.2307/3679597
- Truax, B. "Electroacoustic Music and the Digital Future." *Circuit*, 2010. DOI: 10.7202/902261ar

### 7.10 Jonathan Harvey (1939–2012)

**电子作品：**
- *Mortuos Plango, Vivos Voco* (1980)：IRCAM 创作，频谱分析与计算机合成
- 将频谱主义与电子音乐结合

**学术文献：**
- Harvey, J. "'Mortuos Plango, Vivos Voco': A Realization at IRCAM." *Computer Music Journal*, 1981. DOI: 10.2307/3679502
- Harvey, J. "Spectralism." *Contemporary Music Review*, 2000. DOI: 10.1080/07494460000640331
- Whittall, A. "Jonathan Harvey and Spectralism as 'Spiritual Breakthrough'." *The Oxford Handbook of Spectral Music*, 2022. DOI: 10.1093/oxfordhb/9780190633547.013.25
- Various. "Jonathan Harvey's Mortuos Plango, Vivos Voco." *Analytical Methods of Electroacoustic Music*, 2005. DOI: 10.4324/9780203959718-11

### 7.11 Gérard Grisey (1946–1998) & Tristan Murail (b. 1947)

**频谱音乐与电子音乐关联：**
- 频谱分析作为声学乐器与电子音乐的桥梁
- 声音频谱的记谱方法

**学术文献：**
- Cagney, L. *Gérard Grisey and Spectral Music*. Cambridge University Press, 2023. DOI: 10.1017/9781009399494
- Anderson, J. "Grisey, Gérard." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.45479
- Smith, R.B. "An Interview with Tristan Murail." *Computer Music Journal*, 2000. DOI: 10.1162/014892600559146
- Anderson, J. "Murail, Tristan." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.44940
- Anderson, J. & Murail, T. "In Harmony." *The Musical Times*, 1993. DOI: 10.2307/1003053

### 7.12 Alvin Lucier (1931–2021)

**作品与记谱：**
- *I Am Sitting in a Room* (1969)：利用房间共振的技术性记谱
- 图形谱与技术性指令相结合的记谱方法

**学术文献：**
- Horta, A. "I am sitting in a room de Alvin Lucier." *Artnodes*, 2023. DOI: 10.7238/artnodes.v0i32.409943
- Lucier, A. & Margolin, A. "Conversation with Alvin Lucier." *Perspectives of New Music*, 1981. DOI: 10.2307/942399
- Sanders, L. "Lucier, Alvin." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.47065

### 7.13 Christian Wolff (b. 1934) & Earle Brown (1926–2002)

**图形记谱与不确定性：**
- Brown 的 *December 1952* 等图形谱作品
- Wolff 的早期不确定性作品
- 对当代电子音乐实践（生成音乐、live coding）的持续影响

**学术文献：**
- Bröndum, L. "Graphic Notation, Indeterminacy and Improvisation: Implementing Choice Within a Compositional Framework." *Open Cultural Studies*, 2018. DOI: 10.1515/culture-2018-0058
- Alden, J. "From Neume to Folio: Mediaeval Influences on Earle Brown's Graphic Notation." *Contemporary Music Review*, 2007. DOI: 10.1080/07494460701414140
- Brown, E. "The Notation and Performance of New Music." *The Musical Quarterly*, 1986. DOI: 10.1093/mq/lxxii.2.180
- Kim, R. *Beyond Notation*. University of Michigan Press, 2017. DOI: 10.3998/mpub.9688443

### 7.14 Brian Eno (b. 1948)

**生成音乐：**
- *Music for Airports* (1978)：基于规则的生成音乐
- 提出"生成音乐"（Generative Music）概念
- 使用记谱系统控制概率性音乐结构

**学术文献：**
- Lysaker, J.T. *Brian Eno's Ambient 1: Music for Airports*. Oxford University Press, 2018. DOI: 10.1093/oso/9780190497293.001.0001
- Eno, B. "Ambient Music." *Audio Culture*, 2017. DOI: 10.5040/9781501318399.ch-013
- Various. "Generative Musik." *Brian Eno*, 2024. DOI: 10.5771/9783741002564-191

### 7.15 Pierre Schaeffer (1910–1995) & Pierre Henry (1927–2017)

**Musique Concrète 与记谱：**
- Schaeffer：声音客体（objets sonores）理论与还原聆听（écoute réduite）
- Henry：具象音乐实践先驱
- 具象音乐记谱挑战：如何记录不可符号化的声音材料

**学术文献：**
- Schaeffer, P. *Sound Objects* (English translation by John Dack). Duke University Press, 2020. DOI: 10.1515/9781478002536-003
- Dack, J. "Pierre Schaeffer and the (Recorded) Sound Source." *Sound Objects*, 2019. DOI: 10.1215/9781478002536-002
- Fulcher, J.F. "From 'the Voice of the Maréchal' to Musique Concrète." *The Oxford Handbook of the New Cultural History of Music*, 2011. DOI: 10.1093/oxfordhb/9780195341867.013.0016
- Manning, P. "Paris and Musique Concrète." *Electronic and Computer Music*, 2013. DOI: 10.1093/acprof:oso/9780199746392.003.0002
- Dhomont, F. "Henry, Pierre." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.12813
- Dhomont, F. "Parmegiani, Bernard." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.45625

### 7.16 电声音乐记谱方法专题文献

- Patton, K. "Morphological notation for interactive electroacoustic music." *Organised Sound*, 2007. DOI: 10.1017/s1355771807001781
- Mikako. "The aesthetics of notation in Japanese Electroacoustic Music." *Musica/Tecnologia*, 2019. DOI: 10.13128/music_tec-11165
- Hope, C. et al. "Editorial: New Strategies for Music Notation and Representation in Electroacoustic Music." *Organised Sound*, 2025. DOI: 10.1017/s135577182510109x
- Hildebrand, N. "Phenomenological open graphic notation with chaotic systems in interactive electroacoustic music." *Organised Sound*, 2025. DOI: 10.1017/s1355771825100666

---

## 8. 现代主义音乐流派记谱特征与演奏法研究

各流派的记谱视觉特征、创新方法，及其与电子音乐/数字工具的关联。

### 8.1 频谱音乐（Spectral Music）

**记谱特征：**
- 微分音音高由泛音列分析推导，使用1/4音、1/6音、1/8音等微分音记号
- 音色过渡用图形化渐变符号表示（如从噪音到谐波的光滑过渡）
- 时间结构以声音的物理衰减过程为模型，非传统节拍组织
- 频谱和声记谱：和弦基于特定泛音列的频率比，使用特殊指法标注

**与电子音乐共享概念框架：**
- 加法合成（additive synthesis）：从简单正弦波组件构建复杂音色
- 减法合成（subtractive synthesis）：频谱滤波与塑形
- 频域思维（frequency-domain thinking）：声音作为频谱而非音程序列
- DAW 中的频谱分析显示与频谱记谱共享概念基础

**学术文献：**
- Grisey, G. & Fineberg, J. "Did you say spectral?" *Contemporary Music Review*, 2000. DOI: 10.1080/07494460000640311
- Fineberg, J. "Guide to the basic concepts and techniques of spectral music." *Contemporary Music Review*, 2000. DOI: 10.1080/07494460000640271
- Cagney, L. *Gérard Grisey and Spectral Music*. Cambridge University Press, 2023. DOI: 10.1017/9781009399494
- *The Oxford Handbook of Spectral Music* (edited volume), 2022. DOI: 10.1093/oxfordhb/9780190633547
- Carrick, R. "Performing Spectral Ensemble Works by Grisey and Dumitrescu." *The Oxford Handbook of Spectral Music*, 2022. DOI: 10.1093/oxfordhb/9780190633547.013.56
- Lazzarini, V. *Spectral Music Design*. Oxford University Press, 2021. DOI: 10.1093/oso/9780197524015.001.0001
- Lazzarini, V. "Non-Linear Synthesis of Spectra." *Spectral Music Design*, 2021. DOI: 10.1093/oso/9780197524015.003.0008
- Lazzarini, V. "Composing the Spectrum." *Spectral Music Design*, 2021. DOI: 10.1093/oso/9780197524015.003.0012
- Lazzarini, V. "Computer Sound Design." *Spectral Music Design*, 2021. DOI: 10.1093/oso/9780197524015.003.0011
- Lazzarini, V. "Time-Frequency Processing." *Spectral Music Design*, 2021. DOI: 10.1093/oso/9780197524015.003.0006
- Saariaho, K. "Timbre and harmony: Interpolations of timbral structures." *Contemporary Music Review*, 1987. DOI: 10.1080/07494468708567055
- Saariaho, K. "Colour, Timbre and Harmony." *The Voice of Music*, 2019. DOI: 10.4324/9781315189116-22
- Harmeyer, J. "Liminal aesthetics: perspectives on harmony and timbre in the music of Messiaen, Murail, and Saariaho." DOI: 10.18297/etd/3177
- Harvey, J. & Machover, T. "Spectres." *Computer Music Journal*, 1988. DOI: 10.2307/3680346
- Kleczkowski, P. "Group Additive Synthesis." *Computer Music Journal*, 1989. DOI: 10.2307/3679851
- Morrison, L. "Encoding Post-Spectral Sound." *Music Theory Online*, 2021. DOI: 10.30535/mto.27.3.10
- Ungeheuer, E. "From the Elements to the Continuum: Timbre Composition in Early Electronic Music." *Timbre Composition in Electroacoustic Music*, 2019. DOI: 10.4324/9781315077376-3
- Zattra, L. "Hugues Dufourt, La musique spectrale. Une révolution épistémologique." *Twentieth-Century Music*, 2018. DOI: 10.1017/s1478572218000099
- Bell, J. & Carey, B. "Animation notation, score distribution and AR-VR environments for spectral mimetic transfer in music composition." 2019. DOI: 10.29007/8w1w

### 8.2 序列主义/后序列主义（Serialism / Post-Serialism）

**记谱特征：**
- 比例记谱（proportional notation）：时间轴以物理时间而非节拍划分
- 极端力度记号（ppppp–fffff）与精确动态分级
- 音高组织以音列变换矩阵为基础，记谱上无调号、大量临时记号
- 参数化控制：音高、时值、力度、音色独立序列化，视觉上高度密集
- 与电子音乐参数控制的对应关系：MIDI 中 pitch/velocity/duration/timbre 的独立控制

**学术文献：**
- Bernstein, Z. "Serialism." *Music (Oxford Bibliographies)*, 2019. DOI: 10.1093/obo/9780199757824-0265
- Zagorski, M. "The Aesthetics of Serialism." *The Cambridge Companion to Serialism*, 2023. DOI: 10.1017/9781108592116.003
- Delaere, M. "Serialism in Western Europe." *The Cambridge Companion to Serialism*, 2023. DOI: 10.1017/9781108592116.013
- Nolan, C. "Theorising Serialism." *The Cambridge Companion to Serialism*, 2023. DOI: 10.1017/9781108592116.002
- Carr, M. "Stravinsky's Path to Serialism." *The Cambridge Companion to Serialism*, 2023. DOI: 10.1017/9781108592116.012
- Rešidbegović, D. "Composition and Notation of Parameters in Electronic Music: Approximate Reductionist Graphical Notation." *INSAM Journal of Contemporary Music, Art and Technology*, 2020. DOI: 10.51191/issn.2637-1898.2020.3.4.89
- Boulez, P. "Timbre and Composition – Timbre and Language." *Contemporary Music Review*, 1987. DOI: 10.1080/07494468708567057
- Zazulia, E. "Notation." *Music (Oxford Bibliographies)*, 2024. DOI: 10.1093/obo/9780199757824-0324

### 8.3 新复杂主义（New Complexity）

**记谱特征：**
- 极高视觉密度：嵌套连音（五连音内嵌七连音内嵌十一连音）、频繁变换拍号
- 微观节奏记谱：1/128 音符、复杂连音组合
- 多层记谱：多个独立时间层同时呈现
- 视觉上接近集成电路板的复杂度
- 记谱本身作为作曲元素，而非仅为演奏指令

**学术文献：**
- Duncan, S.P. "Re-Complexifying the Function(s) of Notation in the Music of Brian Ferneyhough and the 'New Complexity'." *Perspectives of New Music*, 2010. DOI: 10.1353/pnm.2010.0015
- Ferneyhough, B. "Il Tempo della Figura." *Perspectives of New Music*, 1993. DOI: 10.2307/833031
- Ferneyhough, B. "The Tactility of Time (Darmstadt Lecture 1988)." *Perspectives of New Music*, 1993. DOI: 10.2307/833032
- "Notation." *Brian Ferneyhough* (book chapter), 2013. DOI: 10.2307/j.ctv36xvgkg.9
- Rosser, P. "Brian Ferneyhough and the 'Avant-Garde Experience': Benjaminian Tropes in 'Funérailles'." *Perspectives of New Music*, 2010. DOI: 10.1353/pnm.2010.0002
- Fitch, L. "Brian Ferneyhough, 'Postmodern Modernist'." *The Modernist Legacy*, 2017. DOI: 10.4324/9781315085876-12
- Toop, R. "Ferneyhough, Brian." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.09503

### 8.4 简约主义/后简约主义（Minimalism / Post-Minimalism）

**记谱特征：**
- 过程记谱（process notation）：记谱描述变化过程而非固定结果
- 相位标记（phase notation）：Reich 的相位移动记谱
- 加法过程：音符逐步增减的模式化记谱
- 视觉上高度重复，模式渐变
- 与电子音乐循环（loop）、模式（pattern）和音序器的对应关系

**学术文献：**
- Potter, K. "Minimalism." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.40603
- Potter, K. "Minimalism (USA)." *Oxford Music Online*, 2014. DOI: 10.1093/gmo/9781561592630.article.a2257002
- Singsen, D. "Post-minimalism." *Oxford Art Online*, 2011. DOI: 10.1093/gao/9781884446054.article.t2090791
- Kostka, S. & Santa, M. "Minimalism and Beyond." *Materials and Techniques of Post-Tonal Music*, 2018. DOI: 10.4324/9781315229485-15

### 8.5 随机/算法作曲（Stochastic / Algorithmic）

**记谱特征：**
- 概率分布记谱：事件发生概率的可视化表示
- 图形元素：密度云、粒子分布图
- 数学函数图像直接嵌入记谱
- UPIC 系统：图形输入直接转化为声音
- 与计算机音乐算法生成的直接对应

**学术文献：**
- Serra, M.-H. "Stochastic Composition and Stochastic Timbre: GENDY3 by Iannis Xenakis." *Perspectives of New Music*, 1993. DOI: 10.2307/833052
- Xenakis, I. "The Origins of Stochastic Music." *Tempo*, 1966. DOI: 10.1017/s004029820003429x
- Luque, S. "The Stochastic Synthesis of Iannis Xenakis." *Leonardo Music Journal*, 2009. DOI: 10.1162/lmj.2009.19.77
- Lohner, H. & Xenakis, I. "Interview with Iannis Xenakis." *Computer Music Journal*, 1986. DOI: 10.2307/3680096
- Hoffmann, P. "Xenakis, Iannis." *Oxford Music Online*, 2001. DOI: 10.1093/gmo/9781561592630.article.30654
- Babbitt, M. "Twelve-Tone Rhythmic Structure and the Electronic Medium." *Perspectives of New Music*, 1962. DOI: 10.2307/832179
- Babbitt, M. "Twelve-Tone Invariants as Compositional Determinants." *The Musical Quarterly*, 1960. DOI: 10.1093/mq/xlvi.2.246
- Babbitt, M. "The Twelve-Tone Tradition." *Music Theory Spectrum*, 2012. DOI: 10.1525/mts.2012.34.1.1

### 8.6 音色记谱与电子音乐交叉研究

**核心问题：** 现代记谱从音高中心转向音色中心，与电子音乐的音色驱动思维趋同。

**学术文献：**
- Vaggione, H. "Timbre as Syntax: A Spectral Modeling Approach." *Timbre Composition in Electroacoustic Music*, 2019. DOI: 10.4324/9781315077376-7
- Smalley, D. "Defining Timbre – Refining Timbre." *Timbre Composition in Electroacoustic Music*, 2019. DOI: 10.4324/9781315077376-4
- Campbell, E. "Timbre, Technology, and Hybridization in the Music of Michaël Levinas." *The Oxford Handbook of Spectral Music*, 2022. DOI: 10.1093/oxfordhb/9780190633547.013.59

### 8.7 空间音乐记谱

**学术文献：**
- Carré, P. & Delécluse, F. "Spatial Treatment of Sound in the Polytope de Cluny." *Xenakis - Back to the Roots*, 2024. DOI: 10.14361/9783839474297-007
- Hope, C. "Drawing Sound in Space: Digital Spatial Notation as Constitutive Audiovisuality." *Organised Sound*, 2025. DOI: 10.1017/s1355771825101027
- Marandola, F. et al. "Polytope XXI." *Meta-Xenakis*, 2024. DOI: 10.11647/obp.0390.50
- Pisano, G. "Stereo and Ambisonics: A Reflection over Parallel Spatialization Techniques." *Array*, 2023. DOI: 10.25350/array.v20223481
- Politis, A. "Gaunt Coefficients for Complex and Real Spherical Harmonics with Applications to Spherical Array Processing and Ambisonics." arXiv:2407.06847, 2024. https://arxiv.org/abs/2407.06847

### 8.8 音簇/音团记谱（Sound Mass）

**学术文献：**
- Vickery, L. "The Limitations of Representing Sound and Notation on Screen." *Organised Sound*, 2014. DOI: 10.1017/s135577181400020x
- Sinding-Larsen, H. "Musical Notation as the Externalization of Imagined, Complex Sound." *The Oxford Handbook of Sound and Imagination*, Vol. 2, 2019. DOI: 10.1093/oxfordhb/9780190460242.013.41

### 8.9 不确定性/偶然记谱与生成音乐

**学术文献：**
- Behrman, D. "What Indeterminate Notation Determines." *Perspectives of New Music*, 1965. DOI: 10.2307/832504
- Cage, J. "Composition as Process: Indeterminacy." *Audio Culture*, 2017. DOI: 10.5040/9781501318399.ch-029
- Skurvida, S. "Technologies of Indeterminacy." *John Cage Composing, Computing, and Curating*, 2025. DOI: 10.4324/9781032721385-5
- Grella-Możejko, P. "Earle Brown—Form, Notation, Text." *Contemporary Music Review*, 2007. DOI: 10.1080/07494460701424636
- Morse, B.R. "Cage's Attitudes Regarding Indeterminacy." *Indeterminacy, the I Ching, and John Cage*, 2025. DOI: 10.1201/9781003710370-7

### 8.10 扩展技法记谱

**学术文献：**
- Ellard, L. et al. "Bridging the Gap: Introducing Extended Techniques and Contemporary Notation through Newly Composed Etudes for Clarinet." DOI: 10.12794/metadc1703365
- Casey, R. "Beyond Symbols: Indexical Notation for Sound Morphology." *Organised Sound*, 2025. DOI: 10.1017/s1355771825100708
- Casey, R. "Developing a Phenomenological Approach to Music Notation." *Organised Sound*, 2015. DOI: 10.1017/s1355771815000047

### 8.11 记谱与感知

**学术文献：**
- McAdams, S. "Perception and Cognition in Music Listening." *Perception and Cognition of Music*, 2024. DOI: 10.1093/oso/9780198939177.003.0001
- Valiquet, P. "Scoring the Listener." *Material Cultures of Music Notation*, 2022. DOI: 10.4324/9780429342837-5
- Leinbach, C. et al. "A Multi-Dimensional Approach towards Understanding Music Notation through Cognition." DOI: 10.12794/metadc1703356

### 8.12 图形记谱跨流派研究

**学术文献：**
- Strachan, J. "Canavangard, Udo Kasemets's Trigon, and Marshall McLuhan: Graphic Notation in the Electronic Age." *Twentieth-Century Music*, 2017. DOI: 10.1017/s1478572217000214
- Reutz Drobnič, V. & Schumacher, M. "Graphic Notation and the Mind: Cognitive, Psychological and Physiological Mechanisms of Graphic Score Performance." *SSRN Electronic Journal*, 2026. DOI: 10.2139/ssrn.6168650
- Stewart, Z.W. "Midpoint Notation: An Experimental Music Notation System at the Intersection of Graphic Notation and Conventional Western Notation." DOI: 10.31274/td-20240329-507
- "From Xenakis's UPIC to Graphic Notation Today." 2024. DOI: 10.61608/9783775747424-004

### 8.13 数字记谱系统

**学术文献：**
- Kuuskankare, M. "ENP: A System for Contemporary Music Notation." *Contemporary Music Review*, 2009. DOI: 10.1080/07494460903322505
- Dannenberg, R.B. "Extending Music Notation through Programming." *Contemporary Music Review*, 1996. DOI: 10.1080/07494469600640061
- Freeman, J. & Colella, A. "Tools for Real-Time Music Notation." *Contemporary Music Review*, 2010. DOI: 10.1080/07494467.2010.509599
- Hajdu, G. & Didkovsky, N. "On the Evolution of Music Notation in Network Music Environments." *Contemporary Music Review*, 2009. DOI: 10.1080/07494460903422313
- Sköld, M. "Relating Electronic Musical Instruments to Composition and Notation." *Organised Sound*, 2025. DOI: 10.1017/s1355771825100903

---

## 9. 现代音乐记谱法系统化研究

对现代记谱法进行分类、比较、历史梳理的系统性研究，以及记谱与演奏解读之间的关系。

### 9.1 基础性与历史综述

**Karkoschka, E. *Das Schriftbild der Neuen Musik* (1966) / *Notation in New Music* (1972)**
- 英译：Ruth Koenig, 1972/1973
- DOI: 10.2307/843123（英文版书评，*Journal of Music Theory*）
- Stone, K. 书评：*Perspectives of New Music*, 1967. DOI: 10.2307/832165
- 20世纪记谱法的奠基性系统调查，对当代乐谱的解读与实现提供批评性指南。至今仍为最全面的现代记谱分类学著作，尚无同等规模的后续研究取代。

**Karkoschka, E. "Analysis of new music" (1987)**
- DOI: 10.1080/09298218708570484
- 新音乐分析方法的系统性讨论。

**Stone, K. "Problems and Methods of Notation" (1963)**
- *Perspectives of New Music*. DOI: 10.2307/832100

**Stone, K. "New Notation for New Music, Part 1 & 2" (1976)**
- *Music Educators Journal*. DOI: 10.2307/3395118, DOI: 10.2307/3395098

**Stone, K. "A note on notation" (1977)**
- DOI: 10.2307/3395130

**Cage, J. (ed.) *Notations* (1969, Something Else Press)**
- 书评：*Journal of Aesthetics and Art Criticism*, 1970. DOI: 10.2307/428512
- 收录大量实验性记谱实例的历史性汇编。
- Peng, C.-L. "Indeterminate-oriented to rational-oriented: John Cage, paper imperfections, and graphic notations." 2022. DOI: 10.5937/newso2260062p

### 9.2 跨流派比较与分类

**Accornero, G. "Was 1974 the End of Music History? Universalism, cybernetics, and the International Conference of New Musical Notation" (2022)**
- *Material Cultures of Music Notation*. DOI: 10.4324/9780429342837-3
- 记录1974年试图标准化当代记谱法的国际会议。

**Bowers, R. "Proportional notation" (2001)**
- *Oxford Music Online*. DOI: 10.1093/gmo/9781561592630.article.22424

**Polansky, L. & Read, G. "20th Century Microtonal Notation" (1991)**
- *Leonardo Music Journal*. DOI: 10.2307/1513133

### 9.3 演奏者解读与记谱-演奏关系

**Pace, I. "Notation, Time and the Performer's Relationship to the Score in Contemporary Music" (2009)**
- *Unfolding Time*. DOI: 10.2307/j.ctt9qdxmr.8
- 考察演奏者如何解读复杂当代记谱，以及乐谱指令与实际演奏之间的时间关系。

**Lee, H.J. & Ahn, S.H. "Performer Autonomy and Creative Interpretation in Early and Contemporary Music: Focusing on Graphic Notation and Indeterminate Music" (2025)**
- DOI: 10.35174/jkci.2025.06.25.2.121

**Brooks, M.S. "Notation and performance practice in avant-garde music of the 1960s: The changing composer/performer relationship"**
- Wesleyan University. DOI: 10.14418/wes01.2.428

**Brüstle, C. "Timekeepers – Sound Artists – Drum Machines: Studies of Notation and Performance in Contemporary Music for Solo Percussionist" (2009)**
- *Twentieth-Century Music*. DOI: 10.1017/s1478572213000145

**Knyt, E. "Between Composition and Transcription: Ferruccio Busoni and Music Notation" (2014)**
- *Twentieth-Century Music*. DOI: 10.1017/s1478572213000145

**"Work and Notation" (2022)**
- *Twentieth-Century Music in the West*. DOI: 10.1017/9781108680899.006

**Goolsby, T.W. "Eye Movement in Music Reading: Effects of Reading Ability, Notational Complexity, and Encounters" (1994)**
- *Music Perception*. DOI: 10.2307/40285756
- 通过眼动追踪研究演奏者对复杂记谱的认知加工，揭示专家与新手阅读复杂乐谱的差异。

### 9.4 记谱认知与心理机制

**Reutz Drobnič, V. & Schumacher, M. "Graphic Notation and the Mind: Cognitive, Psychological and Physiological Mechanisms of Graphic Score Performance" (2026)**
- *SSRN Electronic Journal*. DOI: 10.2139/ssrn.6168650

**Kurkela, K. "Score, vision, action" (1989)**
- *Contemporary Music Review*, 4(1). DOI: 10.1080/07494468900640461
- 从认知与符号学角度考察乐谱记谱与演奏者行动之间的关系。

### 9.5 记谱技术与计算机排版

**Maz, A. "Computer Music Notation" (2023)**
- DOI: 10.4324/9781003345138-11

**Byrd, D. "Music Notation Software and Intelligence" (1994)**
- DOI: 10.2307/3680518

**Belkin, A. "Macintosh Notation Software: Present and Future" (1994)**
- DOI: 10.2307/3680522

**Grande, C. & Belkin, A. "The Development of the Notation Interchange File Format" (1996)**
- DOI: 10.2307/3680416

**Haken, L. & Blostein, D. "The Tilia Music Representation: Extensibility, Abstraction, and Notation Contexts for the Lime Music Editor" (1993)**
- DOI: 10.2307/3680942

### 9.6 UPIC 系统与图形记谱历史

**"From Xenakis's UPIC to Graphic Notation Today" (2024, 多章节)**
- "The UPIC: History, Institutions, and Implications." DOI: 10.61608/9783775747424-002
- "Xenakis and the UPIC." DOI: 10.61608/9783775747424-004
- "The Road to the UPIC." DOI: 10.61608/9783775747424-005

### 9.7 TENOR 会议系列

**TENOR Boston 2023: The Eighth International Conference**
- DOI: 10.17760/d20511476
- 专注于记谱技术与表示的国际会议系列。

---

## 10. 谱面到听觉：现代音乐的记谱与感知

记谱意图与听觉感知之间的关系，包括认知负荷、音色感知、复杂性感知等。

### 10.1 无调性/现代音乐的听觉感知

**Dibben, N. "The Cognitive Reality of Hierarchic Structure in Tonal and Atonal Music" (1994)**
- *Music Perception*. DOI: 10.2307/40285753
- 测试听者是否真正感知到无调性音乐中被理论化的层级结构，发现无调性层级的认知现实性证据有限。

**Dibben, N. "The Perception of Structural Stability in Atonal Music" (1999)**
- *Music Perception*. DOI: 10.2307/40285794
- 调查影响听者对无调性结构稳定性感知的因素，识别显著性和不协和度为关键感知线索。

**Lalitte, P., Bigand, E. et al. "On Listening to Atonal Variants of Two Piano Sonatas by Beethoven" (2009)**
- *Music Perception*. DOI: 10.1525/mp.2009.26.3.223
- 考察听者如何感知熟悉调性作品的无调性变换，揭示无调性结构处理的认知挑战。

**Mikumo, M. "Encoding Strategies for Tonal and Atonal Melodies" (1992)**
- *Music Perception*. DOI: 10.2307/40285539
- 调性音乐受益于层级编码，无调性旋律依赖表层特征记忆。

**Kim, S.-G. et al. "Temporal Structure Perception in Tonal and Atonal Music" (2026)**
- Center for Open Science. DOI: 10.31234/osf.io/c7vnu_v1

### 10.2 音色感知

**McAdams, S. *Perception and Cognition of Music* (2024)**
- DOI: 10.1093/oso/9780198939177.003.0001（音乐聆听中的感知与认知）
- DOI: 10.1093/oso/9780198939177.003.0003（音乐音色感知）
- DOI: 10.1093/oso/9780198939177.003.0005（音乐聆听的时间性）
- DOI: 10.1093/oso/9780198939177.003.0002（音乐聆听中的听觉组织过程）

**McAdams, S. "Timbre as a Structuring Force in Music" (2019)**
- *Springer Handbook of Auditory Research*. DOI: 10.1007/978-3-030-14832-4_8

**McAdams, S. "Musical Timbre Perception" (2013)**
- *The Psychology of Music*. DOI: 10.1016/b978-0-12-381460-9.00002-x

**Wallmark, Z. "Semantic Crosstalk in Timbre Perception" (2019)**
- *Music & Science*. DOI: 10.1177/2059204319846617

**Gregory, A.H. "Timbre and Auditory Streaming" (1994)**
- *Music Perception*. DOI: 10.2307/40285649

**Ten Hoopen, C. "Issues in timbre and perception" (1994)**
- *Contemporary Music Review*. DOI: 10.1080/07494469400640301

### 10.3 微分音感知

**Leung, Y. & Dean, R.T. "The difficulty of learning microtonal tunings rapidly" (2018)**
- *Psychomusicology: Music, Mind, and Brain*. DOI: 10.1037/pmu0000207
- 听者在缺乏熟悉结构线索的情况下难以快速学习微分音音程。

**Leung, Y. & Dean, R.T. "Learning a well-formed microtonal scale" (2018)**
- *Journal of New Music Research*. DOI: 10.1080/09298215.2018.1432060

**Jordan, D.S. "Influence of the diatonic tonal hierarchy at microtonal intervals" (1987)**
- *Perception & Psychophysics*. DOI: 10.3758/bf03210483

### 10.4 复杂性感知与认知负荷

**Truax, B. "The Inner and Outer Complexity of Music" (1994)**
- *Perspectives of New Music*. DOI: 10.2307/833161
- 区分记谱复杂性（外在）与感知复杂性（内在），直接讨论乐谱与聆听体验之间的差距。

**Toop, R. "On Complexity" (1993)**
- *Perspectives of New Music*. DOI: 10.2307/833036

**DeLio, T. "The Complexity of Experience" (1993)**
- *Perspectives of New Music*. DOI: 10.2307/833039

**Ulman, E. "Some Thoughts on the New Complexity" (1994)**
- *Perspectives of New Music*. DOI: 10.2307/833163

**Boros, J. "Why Complexity? (Part Two)" (1994)**
- *Perspectives of New Music*. DOI: 10.2307/833155

**Fox, C. "New Complexity" (2001)**
- *Oxford Music Online*. DOI: 10.1093/gmo/9781561592630.article.51676

**Finnissy, M. "New Perspectives on Old Complexity" (1993)**
- *Perspectives of New Music*. DOI: 10.2307/833042

**Beauvois, M.W. "Quantifying Aesthetic Preference and Perceived Complexity for Fractal Melodies" (2007)**
- *Music Perception*. DOI: 10.1525/mp.2007.24.3.247

### 10.5 记谱意图与感知结果的差距

**"Composing and listening: A reply to Nattiez" (2004)**
- *Perception and Cognition of Music*. DOI: 10.4324/9780203344262-30

**Smalley, D. "The listening imagination: Listening in the electroacoustic era" (1996)**
- *Contemporary Music Review*. DOI: 10.1080/07494469600640071
- 讨论听者如何参与电声与当代音乐，提出理解复杂声音材料感知参与的框架。

**English, L. "Relational Listening: A Politics of Perception" (2017)**
- *Contemporary Music Review*. DOI: 10.1080/07494467.2017.1395141

**Milne, A.J. "The Music Perception Toolbox" (2026)**
- Center for Open Science. DOI: 10.31234/osf.io/83697_v1
- 提供分析听者在音高、节奏和结构方面实际感知内容的工具。

**Maia, A. "Clapping Music: Complexity and Information in Reich's Rhythm Space" (2020)**
- *Perspectives of New Music*. DOI: 10.1353/pnm.2020.0003

**Wong, M. & Danesi, M. "Color, shape, and sound: A proposed system of music notation" (2015)**
- *Semiotica*. DOI: 10.1515/sem-2014-0086
- 提出替代记谱系统以更好地传达感知意图，解决传统记谱的不足。

### 10.6 计算化谱面→声音系统

**Cella, C.-E. "Music Information Retrieval and Contemporary Classical Music: A Successful Failure" (2020)**
- *Transactions of the International Society for Music Information Retrieval*. DOI: 10.5334/tismir.55
- 批判性考察 MIR 系统在当代古典音乐中的困境：MIR 研究主要聚焦流行音乐，对当代古典音乐的需求服务不足。

**Jung, J. et al. "Unified Cross-modal Translation of Score Images, Symbolic Music, and Performance Audio" (2025)**
- arXiv:2505.12863. https://arxiv.org/abs/2505.12863

**Zhao, Q. et al. "MuseAgent-1: Interactive Grounded Multimodal Understanding of Music Scores and Performance Audio" (2025)**
- arXiv:2601.11968. https://arxiv.org/abs/2601.11968

**Dratschuk, D. & Swoboda, P. "Transcoda: End-to-End Zero-Shot Optical Music Recognition via Data-Centric Synthetic Training" (2025)**
- arXiv:2605.10835. https://arxiv.org/abs/2605.10835

**Yang, G. et al. "LEGATO: Large-scale End-to-end Generalizable Approach to Typeset OMR" (2025)**
- arXiv:2506.19065. https://arxiv.org/abs/2506.19065

**Martinez-Sevilla, J.C. et al. "Sheet Music Benchmark: Standardized Optical Music Recognition Evaluation" (2025)**
- arXiv:2506.10488. https://arxiv.org/abs/2506.10848

**Xu, N. et al. "From Image to Music Language: A Two-Stage Structure Decoding Approach for Complex Polyphonic OMR" (2025)**
- arXiv:2604.20522. https://arxiv.org/abs/2604.20522

### 10.7 乐谱跟随与谱面-音频对齐

**Henkel, M. & Widmer, G. "Real-Time Music Following in Score Sheet Images via Multi-Resolution Prediction" (2021)**
- *Frontiers in Computer Science*. DOI: 10.3389/fcomp.2021.718340
- 深度学习实时乐谱跟随，直接处理乐谱图像而非符号表示，可应用于非标准记谱系统。

**Giavitto, J.-L. & Echeveste, R. "Real-Time Matching of Antescofo Temporal Patterns" (2014)**
- DOI: 10.1145/2643135.2643158
- Antescofo 系统的时间模式匹配引擎，广泛用于当代音乐演出的乐谱跟随。

**Chang, M. et al. "RUMAA: Repeat-Aware Unified Music Audio Analysis" (2025)**
- DOI: 10.1109/waspaa66052.2025.11230990

**Cont, A. "Realtime Audio to Score Alignment for Polyphonic Music" (2006)**
- DOI: 10.1109/icassp.2006.1661258

**Keshet, J. et al. "A Large Margin Algorithm for Speech-to-Phoneme and Music-to-Score Alignment" (2007)**
- *IEEE Transactions on Audio, Speech, and Language Processing*. DOI: 10.1109/tasl.2007.903928

### 10.8 自动转录与当代音乐

**Carvalho, L. & Smaragdis, P. "Towards end-to-end polyphonic music transcription" (2017)**
- DOI: 10.1109/waspaa.2017.8170013

**Li, X. & Song, Y. "Deep learning-driven automatic music score recognition and digital transcription" (2026)**
- DOI: 10.1007/s43926-026-00332-8

**Tamer, N.C. et al. "Rubato: Transcribing Piano Music with Timestamps" (2026)**
- arXiv:2605.24291. https://arxiv.org/abs/2605.24291

### 10.9 计算分析特定作曲家与技法

**Harley, J. "Computational Approaches to Composition of Notated Instrumental Music: Xenakis and the Other Pioneers" (2011)**
- *Oxford Handbooks Online*. DOI: 10.1093/oxfordhb/9780199792030.013.0005

**Mandolini, R. "Boulez – Xenakis: Two Converging Musical Heuristics" (2021)**
- DOI: 10.47191/ijsshr/v4-i5-27

**Bauer, A. "György Ligeti" (2020)**
- *Oxford Bibliographies in Music*. DOI: 10.1093/obo/9780199757824-0271

---

## 11. 记谱符号的图形词汇与符号学

扩展技法记谱中具体视觉符号的分类、语义分析，以及同一符号在不同作曲家作品中的歧义问题。

### 11.1 记谱符号学与视觉语法理论

**Lidov, D. "Notation in Music" (2018)**
- *Semiotics*. DOI: 10.5840/cpsem201810
- 将符号学理论（Saussure/Peirce）直接应用于音乐记谱，分析记谱符号如何在音乐交流中作为符号系统运作。

**Cazden, N. "Staff Notation as a Non-Musical Communications Code" (1961)**
- *Journal of Music Theory*. DOI: 10.2307/842874
- 将传统五线谱视为具有代码性质的交流系统，分析记谱代码与音乐声音之间的差距。

**Gligo, N. (1997)**
- 记谱符号的理论框架研究。

**Leleu, J.-L. (2015)**
- 对 Boulez 记谱实践的符号学分析。

**Posner, M. & Ness, D.**
- 将 Peirce 符号学框架应用于音乐记谱分析。

**Kim, D.H.-s., Toyoda, E. & Cypess, R. "Historical Performance and the Ethos of Graphic Notation" (2023)**
- *Historical Performance and New Music*. DOI: 10.4324/9781003300229-6
- 将图形记谱与历史演奏实践传统关联，考察图形谱的哲学基础。

### 11.2 记谱作为演奏者必须解码的"代码"

**Kanno, M. "Prescriptive notation: Limits and challenges" (2007)**
- *Contemporary Music Review*. DOI: 10.1080/07494460701250890
- 考察记谱的规定性功能及其局限，识别记谱意图与演奏实现之间的差距。

**Black, R. "Contemporary Notation and Performance Practice: Three Difficulties" (1983)**
- *Perspectives of New Music*. DOI: 10.2307/832938
- 识别演奏者阅读当代记谱时面临的具体挑战，考察新音乐记谱代码中的歧义。

**Judd, R. "Composers, Performers and Notation"**
- *Music Theory Online*. DOI: 10.30535/mto.0.8.1
- 分析作曲家记谱选择与演奏者解读挑战之间的关系。

### 11.3 扩展技法记谱的基础性乐器专著

**Turetzky, B. *The Contemporary Contrabass* (1974)**
- University of California Press.
- 低音提琴扩展技法先驱性目录，包含系统化记谱方案。涵盖弓压、泛音、击弓、预备技法等，每种技法配有标准记谱符号。

**Strange, P. & Strange, A. *The Contemporary Violin* (2003)**
- University of California Press.
- 小提琴扩展技法综合目录。系统整理 sul ponticello、sul tasto、col legno、各类泛音、弓压变体等技法的记谱符号。

**Arkoudis, E.V. "Contemporary Music Notation for the Flute: A Unified Guide to Notational Symbols for Composers and Performers"**
- West Virginia University. DOI: 10.33915/etd.3860
- 长笛扩展技法符号的系统化统一目录，为作曲家和演奏者提供标准化参考。

**Lependorf, J. "Contemporary Notation for the Shakuhachi: A Primer for Composers" (1989)**
- *Perspectives of New Music*. DOI: 10.2307/833414
- 教作曲家如何用当代符号为日本传统乐器记谱。

**Mabry, S. "Vocal Hybrids: Sprechstimme and Recitation" (2002)**
- *Exploring Twentieth-Century Vocal Music*, Oxford University Press. DOI: 10.1093/oso/9780195141986.003.0006
- 考察模糊言语与歌唱边界的混合人声技法及其记谱表示。

**Bryn-Julson, P. & Mathews, P. "The Sprechstimme in Pierrot lunaire" (2008)**
- *Inside Pierrot Lunaire*, Rowman & Littlefield. DOI: 10.5771/9780810862258-43
- 分析 Schoenberg 1912 年作品中 Sprechstimme 记谱法的先驱性运用。

**Ziegler, J.B. et al. "A pedagogical guide for extended and extreme vocal techniques used in contemporary classical vocal music"**
- University of Iowa. DOI: 10.17077/etd.z3k3-cj0s
- 当代人声扩展技法教学指南，涵盖气声、舌响、低语等极端人声技法。

**Bleier, K. "extended piano techniques. Perspektiven 1981–2018"**
- musiconn.publish. DOI: 10.25366/2025.9
- 1981–2018 年钢琴扩展技法综合调查，涵盖预备钢琴、琴内操作、音簇演奏的记谱方式。

**van Dijk, N. "Brass Multiphonics in Jazz"**
- Victoria University of Wellington. DOI: 10.26686/wgtn.16999489
- 爵士铜管多音（multiphonics）技法的运用与记谱。

**Ellard, L. "Bridging the Gap: Introducing Extended Techniques and Contemporary Notation through Newly Composed Etudes for Clarinet"**
- University of North Texas. DOI: 10.12794/metadc1703365
- 通过新创作的练习曲教单簧管演奏者阅读和演奏扩展技法（多音、微分音等）。

### 11.4 打击乐记谱系统

**Vickery, L., Devenish, L., James, S. & Hope, C. "Expanded Percussion Notation in Recent Works" (2017)**
- *Contemporary Music Review*. DOI: 10.1080/07494467.2017.1371879
- 考察当代澳大利亚作曲家如何为打击乐器开发超越传统西方记谱的扩展记谱系统。

**Hope, C. "Wording New Paths: Text-Based Notation in New Solo Percussion Works" (2017)**
- *Contemporary Music Review*. DOI: 10.1080/07494467.2017.1370793
- 分析当代独奏打击乐作品中的文本记谱与混合记谱方法。

### 11.5 同一符号的作曲家歧义

**Boutwell, B. "'The Breathing of Sound Itself': Notation and Temporality in Feldman's Music to 1970" (2013)**
- *Contemporary Music Review*. DOI: 10.1080/07494467.2014.882548
- 考察 Feldman 独特的时间记谱方法，展示其记谱选择如何反映特定音乐哲学。

**Brooks, M.S. "Notation and performance practice in avant-garde music of the 1960s: The changing composer/performer relationship"**
- Wesleyan University. DOI: 10.14418/wes01.2.428
- 记录 1960 年代先锋派作曲家如何引入特异性记谱法，打破标准化实践，导致作曲家-演奏者角色的重新协商。

### 11.6 记谱标准化与数字化

**Kuuskankare, M. "ENP: A System for Contemporary Music Notation" (2009)**
- *Contemporary Music Review*. DOI: 10.1080/07494460903322505
- 提出当代记谱的系统化框架（ENP），尝试为扩展技法创建统一符号系统。

**Kretz, J. "Extending the KLANGPILOT Score Language for Real-Time Notation" (2010)**
- *Contemporary Music Review*. DOI: 10.1080/07494467.2010.509591
- 开发当代乐谱记谱的形式化语法，解决扩展技法与非标准符号的系统化表示问题。

**Good, M.D. "MusicXML" (2012)**
- *Structuring Music through Markup Language*, IGI Global. DOI: 10.4018/978-1-4666-2497-9.ch009
- 描述 MusicXML 架构如何表示扩展技法符号。

**Good, M.D. "Using MusicXML 2.0 for Music Editorial Applications" (2009)**
- DOI: 10.1515/9783110231144.157

**Byrd, D. "Music Notation Software and Intelligence" (1994)**
- *Computer Music Journal*. DOI: 10.2307/3680518
- 早期批判性考察记谱软件的局限性，指出需要更智能的符号处理。

**Rešidbegović, D. "Composition and Notation of Parameters in Electronic Music: Approximate Reductionist Graphical Notation" (2020)**
- *INSAM Journal of Contemporary Music, Art and Technology*. DOI: 10.51191/issn.2637-1898.2020.3.4.89
- 提出电子音乐参数的图形记谱系统，解决合成器设置与实时电子控制的记谱问题。

**SMuFL (Standard Music Font Layout)**
- 音乐字体标准化项目，定义了当代音乐符号的字体规范，但主要存在于技术规范文档而非同行评审学术文献中。

### 11.7 记谱符号的图形词汇具体分类

以下为本调研识别出的主要记谱图形符号类别及其在不同作曲家间的差异：

**弓压（Bow Pressure）**
- 常见符号：实心三角形、楔形
- 不同作曲家对三角形的大小、朝向、填充方式有各自约定
- 相关研究：Matsutani (2001) 关于力度测量；Strange & Strange (2003) 综合目录

**波浪线（Wave Lines）**
- 可表示：颤音类型、震音、不规则震音、气声、呼吸声
- 波浪线的密度、振幅、频率变化承载不同语义
- 同一波浪线形态在不同作曲家作品中可能意味着完全不同的演奏法

**音簇（Clusters）**
- 实心矩形块、虚线框、阴影区域
- Ligeti、Penderecki、Xenakis 各有不同的音簇记谱方式

**多音（Multiphonics）**
- 指法图、音高列、特殊符号标注
- 管乐、弦乐的多音记谱方式差异显著
- 相关研究：Braasch & Skovira (2025) DOI: 10.1121/10.0041102

**电子乐器/实时电子**
- 合成器参数图形化、信号处理流程图
- 相关研究：Rešidbegović (2020) DOI: 10.51191/issn.2637-1898.2020.3.4.89

**核心问题：** 没有统一的符号标准。同一视觉符号（如黑色方块、波浪线）在不同作曲家作品中可能意味着完全不同的演奏指令。这是现代记谱法研究中最根本的挑战之一。

---

## 12. 调研结论

### 关键空白

| 方向 | 状态 |
|------|------|
| 扩散模型生成传统五线谱图像 | MusicScore（效果有限） |
| 扩散模型生成现代记谱/图形记谱图像 | 无人做过 |
| AI 解读图形记谱→音乐 | 1 篇（Cardew Treatise + MusicLDM） |
| 视觉艺术品→音乐 | Art2Mus 等 |
| 符号化路线生成现代音乐 | 明确不覆盖 |
| 动态/交互式图形乐谱生成 | 少量（INScore, John），无 AI |
| 非标准记谱的自动排版 | 少量（EngravingGNN, Score Transformer），限于传统谱 |
| 非西方记谱的计算化处理 | 少量（井间谱、尺八、吉他谱），非图形谱 |

### 检索覆盖情况

| 来源 | 状态 | 说明 |
|------|------|------|
| arXiv | 已覆盖 | 主要搜索源 |
| ISMIR 2023–2025 | 已覆盖 | 通过 arXiv API |
| ICMC 2019 | 部分覆盖 | |
| TENOR 2018/2025 | 部分覆盖 | |
| NIME | 未直接访问 | 网站 403/timeout |
| IEEE Vis 2018 | 已覆盖 | |
| Organised Sound | 部分覆盖 | 通过 DOI 检索 |
| Contemporary Music Review | 部分覆盖 | 通过 DOI 检索 |
| Computer Music Journal | 部分覆盖 | 通过 DOI 检索 |
| Perspectives of New Music | 部分覆盖 | 通过 DOI 检索 |
| Google Scholar | 被阻断 | bot detection |
| NIME/ICMC 会议网站 | 被阻断 | 403/timeout |
