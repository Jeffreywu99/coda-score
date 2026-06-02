# 十二音序列 (12-Tone Serialism) - LilyPond 生成指南

## 1. 核心概念与目标
这是面向系统生成引擎（LLM）的规则字典。当用户请求“十二音体系”、“序列主义”、“无调性音高组织”或“十二音矩阵”时，必须严格遵守以下排版与音高组织规则。

十二音体系的核心在于**八度内的12个半音平等出现，且不产生任何传统调性倾向**。

## 2. 强制性全局排版规则 (Engraving Rules)

### 2.1 临时记号风格 (Accidental Style)
无调性音乐绝对**不能使用任何全局调号**（如 `\key c \major`，尽管它是默认的，但必须警惕）。
为了避免演奏者对升降号产生误解，必须在 `Staff` 级别强制启用 `dodecaphonic` 临时记号风格。该风格会在**每一个音符**前强制显示升号、降号或还原号，这是现代无调性乐谱的工业标准。

**必须包含的代码：**
```lilypond
\layout {
  \context {
    \Staff
    \accidentalStyle dodecaphonic
  }
}
```

### 2.2 小节线与系统 (Bar lines & Systems)
为了强调十二音序列的结构（通常按 12 个音为一组 / Row），可以通过隐式小节线或特殊连音线进行逻辑切分。
如果没有特殊节拍要求，使用无拍号或隐藏拍号：
```lilypond
\omit Staff.TimeSignature
```

## 3. 音高生成逻辑 (Pitch Logic)

构建一个十二音序列（Prime, P0），必须包含 `c, cis, d, dis, e, f, fis, g, gis, a, ais, b` 的每一种绝对音高类，不重复，不遗漏。

- **原形 (Prime - P)**: 基础的 12 音列。
- **逆行 (Retrograde - R)**: 原形音列的倒序。
- **倒影 (Inversion - I)**: 以原形第一个音为轴，所有音程反向。
- **逆行倒影 (Retrograde Inversion - RI)**: 倒影的倒序。

当生成代码时，LLM 应尽量按照这四种逻辑组合音符，以体现纯正的序列主义。

## 4. Snippet 参考: 纯正无调性开局模板

此模板演示了如何初始化一个标准的十二音序列记谱环境，包含临时记号风格和基础排版设置。

**关联 Snippet**: `study/knowledge/snippets/12_tone_basic.ly`
