---
category: harmony
source: "Schoenberg, Harmonielehre (Theory of Harmony), 1911"
confidence: text_derived
tags: [不协和解放, 线条思维, 半音化, 悬浮调性, 无调性, 动机]
title: Schoenberg — 和声学
title_en: Theory of Harmony (Schoenberg)
difficulty: advanced
contexts: [piano, solo-melody, chamber]
---

# Schoenberg — 和声学

1911 年出版的 *Harmonielehre* 既是和声教材也是哲学宣言。核心论断：**不协和的解放是西方音乐演进的内在必然**。通用禁止项和声部进行基础规则 → 见 shared-rules.md。本文件聚焦从调性到半音化到无调性的**连续演进**，示例从传统到现代逐步过渡。

---

## 一、半音化和声演进

**规则**：调性音乐的半音化是一个连续光谱——从纯自然音到副属和弦到增六和弦到悬浮调性，每一步都是前一步的自然延伸。

### 阶段 1：纯自然音（C 大调 I-IV-V-I）

所有音符在调内，和声功能清晰：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 右手：自然音和弦
      <e' g c>1 <f a c> <g b d> <e g c>
    }
    \new Staff \relative c {
      \clef bass
      % 左手：根音低音
      c1 f g c,
    }
  >>
  \layout { }
}
```

### 阶段 2：半音化扩张（C 大调，含增六和弦）

引入变化音——德国增六和弦 (Gr+6) 在属和弦前制造张力。Ab-F# 的增六度向外解决到 G 的八度：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % I → Gr+6（Ab-C-D-F#）→ V → I
      <e' g c>1 <d f bes,> <d g b> <e g c>
    }
    \new Staff \relative c {
      \clef bass
      % 低音：C → Ab(增六低音) → G → C
      c1 aes g c,
    }
  >>
  \layout { }
}
```

**增六和弦解决验证**：
- Ab（降六级）→ G（下行半音）✓
- F#（升四级）→ G（上行半音）✓
- 增六度 Ab-F# 向外解决到八度 G-G ✓

### 阶段 3：晚期浪漫——悬浮调性

调性被暗示但从不通过终止式确认。半音化声部进行取代功能和声，每个和弦由线条运动自然产生：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 3/4
      % 半音化上行线条：f'→f#'→g'→g#'→a'
      <f' b d>2 <fis a c> <g bes des> | <gis b d>2.
    }
    \new Staff \relative c {
      \clef bass
      % 低音半音下行，调性悬浮
      bes2 a aes | g2.
    }
  >>
  \layout { }
}
```

**分析**：无明确调中心。每个和弦由上下声部的半音反向运动自然产生，不需要功能标签。

---

## 二、线条思维

**规则**：Schoenberg 颠覆传统教学路线——**先有独立线条的运动，和声作为结果自然形成**。不是"先选和弦再填声部"，而是"写声部，从声部的交汇中识别和声"。

### 二声部对位——和声从线条中涌现

两条独立的半音化线条，纵向和声完全由横向运动决定：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 高声部：半音上行线条
      e'4 f fis g gis a | bes b c cis d dis
    }
    \new Staff \relative c' {
      \clef bass
      % 低声部：半音下行线条（反向）
      c4 b bes a aes g | fis f e ees d des
    }
  >>
  \layout { }
}
```

**线条分析**：
- 高声部：e→f→f#→g→g#→a→bb→b→c→c#→d→d#（半音上行）
- 低声部：c→b→bb→a→ab→g→f#→f→e→eb→d→db（半音下行）
- 纵向音程不断变化，和声色彩由线条运动自然产生
- 无功能和声关系，无终止式——**一致性靠线条的连续性维持**

### 对 AI 生成的启示

生成多声部音乐时：
1. 先构思各声部的**横向旋律线**（级进为主，跳进后反向）
2. 和声结构作为**结果**而非**原因**——从线条交汇中识别和声
3. 这与项目的**序列矩阵**天然契合：矩阵提供线条材料，声部规则保证线条互动

---

## 三、不协和解放

**规则**：Schoenberg 认为"协和与不协和之间没有本质区别——它们只是泛音列中距离基音远近不同的音"。不协和不需要准备和解决，它可以作为**独立实体**存在。

### 传统处理：延留音需要准备和解决

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 4-3 延留音：f'(准备) → f'(延留) → e'(解决)
      f'2~ f4 e | c1
    }
    \new Staff \relative c {
      \clef bass
      c2 g c,1
    }
  >>
  \layout { }
}
```

### 解放处理：不协和作为独立和弦

同样的音程结构，不再解决——不协和和弦独立存在，具有自身的表达价值：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 不协和和弦直接进行，无准备无解决
      <c' e fis bes>1 <d f ais e'> | <ees g b d'> <c e g c'>
    }
    \new Staff \relative c {
      \clef bass
      c1 d | ees c
    }
  >>
  \layout { }
}
```

**分析**：
- 第一个和弦 C-E-F#-Bb 包含增四度 (C-F#) 和大七度 (C-B)，传统上需要解决
- 但在 Schoenberg 语境中，它直接进行到下一个不协和和弦
- **不协和获得了与协和同等的结构地位**——它不是"需要解释的例外"，而是"新的和声材料"

### 演进逻辑

```
传统（Aldwell/Kostka）：不协和 → 需要准备 → 需要解决 → 依附于协和
Schoenberg：不协和 = 更远的协和 → 独立实体 → 不需要解决
```

---

## 四、声部进行在半音化中的应用

**规则**：即使在无调性语境中，声部进行原则仍然适用——级进优先、跳进后反向、声部独立性。区别在于：半音进行取代自然音级进，音程关系不再受调性约束。

### 半音化声部进行——四声部钢琴织体

四个声部各自做半音级进，纵向形成不断变化的不协和和弦：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 高声部：半音上行
      c''4 cis d dis | e f fis g
    }
    \new Staff \relative c' {
      \clef treble
      % 次高声部：半音下行（反向）
      a'4 aes g ges | f e ees d
    }
    \new Staff \relative c' {
      \clef bass
      % 次低声部：半音上行
      e4 f fis g | gis a bes b
    }
    \new Staff \relative c {
      \clef bass
      % 低声部：半音下行
      c4 b bes a | aes g ges f
    }
  >>
  \layout { }
}
```

**声部检查**：
- 每个声部均做半音级进（最小的可能运动）✓
- 高声部与次高声部反向运动 ✓
- 低声部与次低声部反向运动 ✓
- 声部间无交错（S > A > T > B 始终成立）✓
- 纵向和声完全由半音线条的交汇产生，无功能标签

### 与调性声部进行的对比

| 维度 | 调性（Aldwell/Kostka） | 半音化（Schoenberg） |
|------|----------------------|-------------------|
| 音阶 | 自然音阶（7 音） | 半音阶（12 音） |
| 级进 | 大二度/小二度 | 小二度为主 |
| 和声目标 | 功能和声 (T-S-D-T) | 无功能目标，线条驱动 |
| 不协和 | 需要准备和解决 | 独立存在 |
| 终止式 | 必须有 | 不需要 |
| 调中心 | 明确 | 悬浮或无 |
| 声部规则 | 仍然有效 | **仍然有效**——级进、反向、独立性 |

---

## AI 生成约束总结（Schoenberg 视角）

1. **线条优先**：先构思各声部横向线条，和声作为结果
2. **不协和不需要解决**：在半音化/无调性语境中，不协和可独立存在
3. **动机一致性**：调性悬浮时，用动机关系（倒影、逆行、扩大、缩小）维持统一
4. **半音级进是合法的声部运动**：12 个半音均为可用材料
5. **调性是可选的**：无调性段落不需要终止式，一致性靠线条和动机维持
6. **声部进行规则仍然有效**：即使在无调性中，级进优先、跳进后反向、声部独立性仍需遵守
