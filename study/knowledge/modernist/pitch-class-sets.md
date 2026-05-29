---
category: modernist
source: "Pitch-Class Set Theory — practical composition applications"
confidence: text_derived
tags: [音级集合, 福特号, 音程向量, 集合操作, 无调性, 和声, 现代作曲]
title: 音级集合与作曲
title_en: Pitch-Class Sets in Composition
difficulty: advanced
contexts: [modernist, set-theory, atonal, post-tonal]
---

# 音级集合与作曲

音级集合理论（Pitch-Class Set Theory）是分析和创作无调性音乐的核心工具。本文件将抽象的集合理论（福特号、音程向量）与 **LilyPond 实际写作** 连接起来。应用已内置音级集合计算器；本文件教 LLM 如何在作曲中 **使用** 集合。

乐器音域与通用声部规则 → 见 shared-rules.md。LilyPond 基础语法 → 见 lilypond-core-syntax.md。

---

## 一、音级集合基础

### 1.1 音级 (Pitch Class)

将十二平均律的 12 个半音编号为 0-11，忽略八度差异：

| 音级 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|------|---|---|---|---|---|---|---|---|---|---|----|----|
| 音名 | C | C#/Db | D | D#/Eb | E | F | F#/Gb | G | G#/Ab | A | A#/Bb | B |

### 1.2 集合记法

音级集合用花括号表示，如 `{0,4,7}` = C E G = C 大三和弦。集合中的音无序、无重复。

### 1.3 标准序 (Normal Form)

将集合排列为**最紧凑的升序形式**（首尾音程最小）：

- `{0,4,7}` → 标准序 `[0,4,7]`（跨度 7）
- `{7,0,4}` → 重新排列 → `[0,4,7]`
- `{2,7,11,0}` → 标准序 `[11,2,7]`... 需要比较所有轮转排列取最紧凑者

### 1.4 原型 (Prime Form)

将标准序移至以 0 开头，并选择最紧凑的形式（与倒影比较取较紧凑者）：

- C 大三和弦 `{0,4,7}` → 原型 `(0,3,7)`（小三和弦形式更紧凑）
- C 小三和弦 `{0,3,7}` → 原型 `(0,3,7)` ← 相同！

**核心概念**：大三和弦与小三和弦属于**同一集合类**，因为它们是倒影关系。

### 1.5 福特号 (Forte Number)

Allen Forte 为每个集合类编号，格式为 `基数-序号`：

- `3-11` = 三音集合第 11 号 = 大/小三和弦 `(0,3,7)`
- `3-1` = 三音集合第 1 号 = 半音簇 `(0,1,2)`
- `4-23` = 四音集合第 23 号 = 四度叠置 `(0,2,5,7)`

### 1.6 LilyPond 示例：同一集合的三种呈现

```lilypond
\version "2.24.0"
\header { title = "集合 3-11 (0,3,7) 的三种呈现" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 标准序：C Eb G = {0,3,7}
      <c ees g>1^\markup { \bold "标准序 [0,3,7]" }
    }
    \new Staff \relative c' {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 原型 (0,3,7) 从 C 开始 = C Eb G
      % 倒影形式 (0,4,7) 从 C 开始 = C E G
      <c ees g>2^\markup { "原型 (0,3,7)" }
      <c e g>^\markup { "倒影 (0,4,7)" }
    }
  >>
  \layout { }
}
```

### 1.7 LilyPond 示例：C 大三和弦的集合分析标注

```lilypond
\version "2.24.0"
\header { title = "C 大三和弦 — 集合视角" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 3/4
    % 纵向呈现：和弦形式
    <c e g>2^\markup { "{0,4,7} = C E G" }
    % 横向呈现：旋律形式
    c4 e g
    % 不同排列
    e4 g c
    g4 c e
  }
  \layout { }
}
```

---

## 二、音程向量 (Interval Vector)

### 2.1 定义

音程向量是一个 6 维向量 `[ic1, ic2, ic3, ic4, ic5, ic6]`，统计集合中所有音程级（interval class）的出现次数。

| 音程级 | 包含的半音数 | 对应音程 |
|--------|-------------|---------|
| ic1 | 1 或 11 | 小二度 / 大七度 |
| ic2 | 2 或 10 | 大二度 / 小七度 |
| ic3 | 3 或 9 | 小三度 / 大六度 |
| ic4 | 4 或 8 | 大三度 / 小六度 |
| ic5 | 5 或 7 | 纯四度 / 纯五度 |
| ic6 | 6 | 三全音 |

### 2.2 计算方法

以三和弦 `{0,4,7}` 为例：

| 音对 | 音程 | 音程级 |
|------|------|--------|
| 0-4 | 4 | ic4 |
| 0-7 | 7 | ic5 |
| 4-7 | 3 | ic3 |

音程向量 = `[0, 0, 1, 1, 1, 0]`

**解读**：三和弦包含 1 个小三度、1 个大三度、1 个纯四五度，没有二度、三全音。这解释了大三和弦的协和性——没有 ic1 和 ic6 这两个最不协和的音程级。

### 2.3 LilyPond 示例：三和弦的音程解剖

```lilypond
\version "2.24.0"
\header { title = "三和弦 {0,4,7} — 音程向量 [0,0,1,1,1,0]" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 完整和弦
    <c e g>2^\markup { "{0,4,7}" }
    % ic4: C-E (大三度 = 4半音)
    <c e>4^\markup { "ic4" }
    % ic5: C-G (纯五度 = 7半音 → ic5)
    <c g>^\markup { "ic5" }
    % ic3: E-G (小三度 = 3半音)
    <e g>^\markup { "ic3" }
    % 再次完整和弦
    <c e g>1^\markup { "[0,0,1,1,1,0]" }
  }
  \layout { }
}
```

### 2.4 Z-关系 (Z-Relation)

两个**不等价**的集合如果拥有**相同的音程向量**，则称为 Z-关系对。它们听起来有相似的音程"色彩"，但音高内容不同——这是作曲中制造对比与统一的重要手段。

经典 Z-关系对：`5-Z17 (0,1,3,4,8)` 与 `5-Z37 (0,3,4,5,8)`，共同音程向量 `[2,1,2,3,2,0]`。

```lilypond
\version "2.24.0"
\header { title = "Z-关系：5-Z17 与 5-Z37" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 5-Z17 = {0,1,3,4,8} = C C# D# E G#
      % 音程向量 [2,1,2,3,2,0]
      <c cis dis e gis>1^\markup { \bold "5-Z17 (0,1,3,4,8)" }
    }
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 5-Z37 = {0,3,4,5,8} = C D# E F G#
      % 相同的音程向量 [2,1,2,3,2,0]
      <c dis e f gis>1^\markup { \bold "5-Z37 (0,3,4,5,8)" }
    }
  >>
  \layout { }
}
```

**作曲应用**：可用 Z-关系对制造"似是而非"的和声对比——两个和弦包含相同的音程含量，但音高不同，适合用于变奏或展开段落。

---

## 三、集合操作

### 3.1 移调 (Transposition)

$$T_n(S) = \{(s + n) \mod 12 \mid s \in S\}$$

将集合的每个音级加 n（模 12）。这是最基本的派生手法。

示例：$T_5(\{0,4,7\}) = \{5,9,0\} = \{F, A, C\}$ = F 大三和弦

```lilypond
\version "2.24.0"
\header { title = "移调：T0 与 T5 的三和弦" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % T0: {0,4,7} = C E G
    <c e g>2^\markup { "T0: {0,4,7}" }
    % T5: {5,9,0} = F A C（每个音上移 5 个半音 = 纯四度）
    <f a c>^\markup { "T5: {5,9,0}" }
    % T7: {7,11,2} = G B D
    <g b d>2^\markup { "T7: {7,11,2}" }
    % T9: {9,1,4} = A C# E
    <a cis e>^\markup { "T9: {9,1,4}" }
  }
  \layout { }
}
```

### 3.2 倒影 (Inversion)

$$I(S) = \{(12 - s) \mod 12 \mid s \in S\}$$

以 C(0) 为轴的倒影。倒影后通常再做移调 $T_nI$。

示例：$I(\{0,4,7\}) = \{0, 8, 5\} = \{C, G\sharp, F\}$

```lilypond
\version "2.24.0"
\header { title = "倒影操作" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 原型 {0,4,7} = C E G
    <c e g>2^\markup { "P: {0,4,7}" }
    % I: {0,8,5} = C G# F → 标准序 {0,5,8} = C F G#
    <c f gis>^\markup { "I: {0,5,8}" }
    % T2I: {2,10,7} = D Bb G → 标准序 {2,7,10} = D G Bb
    <d g bes>2^\markup { "T2I: {2,7,10}" }
    % T7I: {7,3,0} = G D# C → 标准序 {0,3,7} = C D# G
    <c ees g>^\markup { "T7I: {0,3,7}" }
  }
  \layout { }
}
```

**注意**：$T_7I(\{0,4,7\}) = \{0,3,7\}$ = 小三和弦！这证实了大三和弦与小三和弦的倒影关系。

### 3.3 补集 (Complementation)

补集 = 12 个音级中不在 S 内的所有音级。

$$\overline{S} = \{0,1,...,11\} \setminus S$$

三和弦 `{0,4,7}` 的补集是 `{1,2,3,5,6,8,9,10,11}`（9 音集合）。

一个集合与其补集的音程向量有固定的数学关系（补集定理）：补集向量的每个分量 = 原向量对应分量 + 某个常数。

```lilypond
\version "2.24.0"
\header { title = "补集：三和弦与其 9 音补集" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 三和弦 {0,4,7} = C E G
      <c e g>1^\markup { "S = {0,4,7}" }
    }
    \new Staff \relative c' {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 补集 {1,2,3,5,6,8,9,10,11} = C# D D# F F# G# A A# B
      <cis d dis f fis gis a bes b>1^\markup { "补集 = 其余 9 个音级" }
    }
  >>
  \layout { }
}
```

### 3.4 子集与超集 (Subset / Superset)

- **子集**：S 的一部分音级构成的集合。如 `{0,4}` 是 `{0,4,7}` 的子集。
- **超集**：包含 S 的更大集合。如 `{0,2,4,7,9}` (五声音阶) 是 `{0,4,7}` 的超集。

**作曲应用**：从超集中提取子集作为动机，或将子集扩展为超集来发展材料。

```lilypond
\version "2.24.0"
\header { title = "子集-超集关系" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 子集：{0,4} = C E（大三度）
    <c e>4^\markup { "子集 {0,4}" }
    % 三和弦：{0,4,7} = C E G
    <c e g>^\markup { "三和弦 {0,4,7}" }
    % 超集：{0,2,4,7,9} = C D E G A（五声音阶）
    <c d e g a>2^\markup { "超集 5-35 (0,2,4,7,9)" }
    % 完整展示五声音阶旋律
    c8 d e g a g e d
  }
  \layout { }
}
```

### 3.5 综合操作示例

```lilypond
\version "2.24.0"
\header { title = "集合操作综合：4-23 (0,2,5,7)" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 原型 T0: {0,2,5,7} = C D F G
    <c d f g>4^\markup { "T0" }
    % T3: {3,5,8,10} = D# F G# Bb
    <dis f gis bes>^\markup { "T3" }
    % T6: {6,8,11,1} = F# G# B C#
    <fis gis b cis>^\markup { "T6" }
    % T9: {9,11,2,4} = A B D E
    <a b d e>^\markup { "T9" }
    % I(T0): {0,10,7,5} = C Bb G F → 标准序 {0,5,7,10}
    <c f g bes>2^\markup { "I" }
    % T5I: {5,3,0,10} → {0,3,5,10} = F D# F Bb
    <f bes c' ees'>^\markup { "T5I" }
  }
  \layout { }
}
```

---

## 四、集合和声写作

### 4.1 核心思维

在传统和声中，和弦来自调式音阶的三度叠置。在集合和声中，**一个集合就是全部的和声材料**：

- **纵向化**：集合 → 和弦（音同时发声）
- **横向化**：集合 → 旋律（音先后发声）
- **混合化**：部分纵向 + 部分横向

### 4.2 集合和声进行

选取一个集合，通过移调产生和声进行。关键是选择**移调距离**来控制共同音数量：

| 移调距离 | 共同音数 (4-23) | 声部进行特点 |
|---------|----------------|-------------|
| T0/T12 | 4（相同） | 无变化 |
| T5/T7 | 2 | 平滑，两个共同音保持 |
| T2/T10 | 2 | 平滑，半音运动 |
| T3/T9 | 1 | 中等对比 |
| T4/T8 | 0 | 最大对比（无共同音） |
| T6 | 0 | 三全音关系，最大对比 |

### 4.3 不变量声部引导

**不变量 (Invariant)** = 在操作下保持不变的音级。利用不变量实现平滑声部连接：

- T0 → T7：共同音 {2,7} = D, G 保持，其余音半音移动
- T7 → T2：共同音 {2,9} = D, A 保持
- T2 → T9：共同音 {4,9} = E, A 保持

### 4.4 完整 8 小节示例：以 4-23 (0,2,5,7) 为和声基础

```lilypond
\version "2.24.0"
\header {
  title = "集合和声习作"
  subtitle = "基于 4-23 (0,2,5,7) 的 8 小节钢琴段落"
}

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4

      % === 第 1 小节：T0 = {0,2,5,7} = C D F G ===
      % 左手和弦 + 右手琶音织体
      <d' f g>2 <c d f>
      % === 第 2 小节：T7 = {7,9,0,2} = G A C D ===
      % 共同音 D, G 保持；C→A, F→C 移动
      <c' d g>2 <a c d>
      % === 第 3 小节：T2 = {2,4,7,9} = D E G A ===
      % 共同音 D, A 保持
      <d e a>2 <e g a>
      % === 第 4 小节：T9 = {9,11,2,4} = A B D E ===
      % 共同音 D, A(→E方向) 平滑连接
      <b d e>2 <a b d>
      % === 第 5-6 小节：右手八分音符旋律化 ===
      % T7: G A C D 旋律形态
      g8 a c' d' c' a g4
      % T2: D E G A 旋律形态
      d,8 e g a g e d4
      % === 第 7 小节：T9 再现 ===
      <b' d e>2 <a b d>
      % === 第 8 小节：回归 T0 ===
      <d f g>1
    }
    \new Staff \relative c {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4

      % 第 1 小节：T0 低音 C D F G
      <c d f g>1
      % 第 2 小节：T7 低音 G A C D
      <g a c d>1
      % 第 3 小节：T2 低音 D E G A
      <d e g a>1
      % 第 4 小节：T9 低音 F# B D E
      <fis b d e>1
      % 第 5 小节：T7 低音持续
      <g a c d>1
      % 第 6 小节：T2 低音持续
      <d e g a>1
      % 第 7 小节：T9
      <fis b d e>2 <g a c d>
      % 第 8 小节：回归 T0
      <c d f g>1
    }
  >>
  \layout { }
}
```

**和声分析**：
- 移调路径：T0 → T7 → T2 → T9 → T7 → T2 → T9 → T0
- 每步移调保持 2 个共同音，声部进行平滑
- 第 1-4 小节：和弦织体；第 5-6 小节：旋律织体（同一集合横向展开）
- 第 8 小节回归 T0，形成拱形结构

---

## 五、常用集合及其音乐特征

### 5.1 速查表

| 福特号 | 原型 | 音名示例 | 音程向量 | 音乐特征 |
|--------|------|---------|---------|---------|
| 3-1 | (0,1,2) | C-C#-D | [2,1,0,0,0,0] | 半音簇，极度紧张 |
| 3-11 | (0,3,7) | C-Eb-G | [0,0,1,1,1,0] | 三和弦，协和稳定 |
| 4-23 | (0,2,5,7) | C-D-F-G | [0,1,1,1,1,0] | 四度/五声音阶片段，空旷开阔 |
| 4-25 | (0,2,6,8) | C-D-F#-G# | [0,2,0,2,0,2] | 全音阶片段，朦胧模糊 |
| 5-35 | (0,2,4,7,9) | C-D-E-G-A | [0,2,2,2,2,1] | 五声音阶，东方色彩 |
| 6-35 | (0,2,4,6,8,10) | C-D-E-F#-G#-A# | [0,6,0,6,0,3] | 全音阶，无方向感 |
| 6-Z32 | (0,2,4,5,7,9) | C-D-E-F-G-A | [4,2,4,2,2,1] | 自然音六音列，调性暗示 |

### 5.2 各集合的旋律与和声示例

#### 3-1 (0,1,2) — 半音簇

```lilypond
\version "2.24.0"
\header { title = "3-1 (0,1,2) — 半音簇" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：密集簇
    <c cis d>2^\markup { "和声" }
    % 旋律：半音蠕动
    c8 cis d cis c d cis d^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：半音簇适合制造紧张氛围。可以移调到不同音区，或在不同声部叠加形成"音簇带"。

#### 3-11 (0,3,7) — 三和弦

```lilypond
\version "2.24.0"
\header { title = "3-11 (0,3,7) — 三和弦" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：小三和弦
    <c ees g>2^\markup { "和声" }
    % 旋律：琶音展开
    c8 ees g c' g ees c4^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：虽然三和弦有调性联想，但在无调性语境中通过非功能性移调（如 T1, T3, T6）可消除调性感。

#### 4-23 (0,2,5,7) — 四度/五声音阶四音列

```lilypond
\version "2.24.0"
\header { title = "4-23 (0,2,5,7) — 四度叠置" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：纯四五度叠置，空旷音响
    <c d f g>2^\markup { "和声" }
    % 旋律：五声音阶片段
    c8 d f g f d c4^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：4-23 是最常用的无调性集合之一。它缺少半音和三全音（ic1=0, ic6=0），音响空旷透明，适合营造空间感。Bartók、Messiaen 频繁使用。

#### 4-25 (0,2,6,8) — 全音阶四音列

```lilypond
\version "2.24.0"
\header { title = "4-25 (0,2,6,8) — 全音阶片段" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：全音阶和弦，Debussy 式
    <c d fis gis>2^\markup { "和声" }
    % 旋律：全音阶音型
    c8 d fis gis fis d c4^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：全音阶音响无方向感（只有大二度和三全音），适合制造朦胧、漂浮的意境。Debussy 的标志性语汇。

#### 5-35 (0,2,4,7,9) — 五声音阶

```lilypond
\version "2.24.0"
\header { title = "5-35 (0,2,4,7,9) — 五声音阶" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：五声音阶纵叠
    <c d e g a>2^\markup { "和声" }
    % 旋律：五声音阶上下行
    c8 d e g a g e d^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：五声音阶是跨文化的通用语汇。在无调性语境中，可通过非传统移调（如 T1, T6）打破调性联想，保留其特有的音程色彩。

#### 6-35 (0,2,4,6,8,10) — 全音阶

```lilypond
\version "2.24.0"
\header { title = "6-35 (0,2,4,6,8,10) — 全音阶" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：全音阶纵叠
    <c d e fis gis ais>2^\markup { "和声" }
    % 旋律：全音阶音型
    c8 d e fis gis ais gis fis^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：全音阶只有两个移调形式（T0 和 T1），因为 T2 = T0。利用这一对称性可以制造"静止"或"悬浮"的音响效果。

#### 6-Z32 (0,2,4,5,7,9) — 自然音六音列

```lilypond
\version "2.24.0"
\header { title = "6-Z32 (0,2,4,5,7,9) — 自然音六音列" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 和声：自然音纵叠（缺 B 音的 C 大调片段）
    <c d e f g a>2^\markup { "和声" }
    % 旋律：自然音音型
    c8 d e f g a g f^\markup { "旋律" }
  }
  \layout { }
}
```

**作曲提示**：自然音六音列暗示调性但不确立调性（缺少第 7 个音）。适合在调性与无调性之间游走的音乐，如 Stravinsky 新古典时期作品。

---

## 六、集合组合技法

### 6.1 四种基本组合方式

| 技法 | 定义 | 音响效果 |
|------|------|---------|
| **线性** (Linear) | 集合作为旋律动机，音依次出现 | 清晰的横向线条 |
| **纵向** (Vertical) | 集合作为和弦，音同时发声 | 明确的和声色彩 |
| **交织** (Interleaved) | 多个集合形式在不同声部交替/重叠 | 复杂的织体层次 |
| **聚合补全** (Aggregate) | 互补集合组合成完整十二音 | 音高完满感 |

### 6.2 线性技法

集合的音按时间顺序展开为旋律。可通过节奏变化、音区变化、装饰变奏来发展动机：

```lilypond
\version "2.24.0"
\header { title = "线性技法：4-23 旋律变奏" }

\score {
  \new Staff \relative c' {
    \clef treble
    \set Staff.extraNatural = ##f
    \accidentalStyle dodecaphonic
    \time 4/4
    % 原型：均等节奏
    c4 d f g
    % 节奏变奏：附点
    c8. d16 f8. g16 c4
    % 音区变奏：宽音域跳进
    c,8 g'' d, f'
    % 逆行：g f d c
    g4 f d c
  }
  \layout { }
}
```

### 6.3 纵向技法

集合的音同时发声构成和弦。可通过不同排列、重复、分布来改变音响密度：

```lilypond
\version "2.24.0"
\header { title = "纵向技法：4-23 和弦变体" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 密集排列
      <c d f g>2^\markup { "密集" }
      % 开放排列（间隔八度展开）
      <c d' f' g'>^\markup { "开放" }
    }
    \new Staff \relative c {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      % 分散到双手
      <c d>2 <f g>
      % 低音加厚
      <c, d f g>1^\markup { "加厚" }
    }
  >>
  \layout { }
}
```

### 6.4 交织技法

两个或多个集合形式在不同声部同时进行，形成重叠与交错。类似复调但材料来自同一集合：

```lilypond
\version "2.24.0"
\header { title = "交织技法：T0 与 T7 交错" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4
      % 右手：T0 = {0,2,5,7} 八分音符
      c8 d f g c' d' f' g'
      % 右手：继续 T0 逆行
      g'8 f' d' c' g f d c
    }
    \new Staff \relative c' {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4
      % 左手：T7 = {7,9,0,2} 延迟一拍进入
      r8 g a c' d' g a c'
      % 左手：T7 逆行
      r8 c' a g d' c a g
    }
  >>
  \layout { }
}
```

**效果**：两个集合形式在时间上错开、在音高上互补（T0 和 T7 共享 D 和 G），形成丰富的对位织体。

### 6.5 聚合补全技法

**聚合补全 (Aggregate Completion)**：选取两个互补的集合形式（无共同音级），组合后覆盖全部 12 个音级。

4-23 `{0,2,5,7}` 的三个互补移调：
- T4 `{4,6,9,11}` = E F# A B — 与 T0 无共同音
- T6 `{6,8,11,1}` = F# G# B C# — 与 T0 无共同音
- T8 `{8,10,1,3}` = G# Bb C# D# — 与 T0 无共同音

任意一对互补形式组合 = 完整十二音聚合体。

```lilypond
\version "2.24.0"
\header { title = "聚合补全：T0 + T4 = 十二音" }

\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4
      % T0 旋律：C D F G
      c8 d f g c' d' f' g'
    }
    \new Staff \relative c' {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4
      % T4 旋律：E F# A B — 补全剩余 4 个音级
      e8 fis a b e' fis' a' b'
    }
  >>
  \layout { }
}
```

**注意**：T0 贡献 {C, D, F, G}，T4 贡献 {E, F#, A, B}，合计 {C, C#/Db缺失...}。实际上 T4 = {4,6,9,11} = {E, F#, A, B}，补全了 {1,3,4,6,8,9,10,11} 中的 4 个音。要完成完整十二音聚合，需要**三个** 4-23 形式：T0 + T4 + T8 = 全部 12 音。

### 6.6 完整 16 小节作品示例

以下钢琴小品综合运用四种组合技法，以 4-23 (0,2,5,7) 为唯一音高材料。

```lilypond
\version "2.24.0"
\header {
  title = "集合小品"
  subtitle = "4-23 (0,2,5,7) 四种组合技法 — 16 小节"
}

\score {
  \new PianoStaff <<
    % ====== 右手 ======
    \new Staff \relative c' {
      \clef treble
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4

      % ===== A: 线性 (第 1-4 小节) =====
      % 第 1 小节：T0 旋律动机
      c8 d f g f d c4
      % 第 2 小节：T7 移调
      g8 a c' d' c' a g4
      % 第 3 小节：T2 展开
      d8 e g a e d a'4
      % 第 4 小节：T0 回归
      g8 f d c d f g4

      % ===== B: 纵向 (第 5-8 小节) =====
      % 第 5 小节：T0 块状和弦
      <c d f>4 <d f g> <c d f> <d f g>
      % 第 6 小节：T7 块状和弦
      <g a c>4 <a c d> <g a c> <a c d>
      % 第 7 小节：T2 块状和弦
      <d e g>4 <e g a> <d e g> <e g a>
      % 第 8 小节：T0 回归和弦
      <c d f>2 <d f g>

      % ===== C: 交织 (第 9-12 小节) =====
      % 第 9 小节：T0 八分音符
      c8 d f g c' d' f' g'
      % 第 10 小节：T0 逆行
      g'8 f' d' c' g f d c
      % 第 11 小节：T2
      d,8 e g a d' e' g' a'
      % 第 12 小节：T2 逆行
      a'8 g' e' d' a g e d

      % ===== D: 聚合补全 (第 13-16 小节) =====
      % 第 13 小节：T0 旋律 {0,2,5,7}
      c4 d f g
      % 第 14 小节：T4 旋律 {4,6,9,11} — 互补
      e4 fis a b
      % 第 15 小节：T8 和弦 {8,10,1,3} + T4 和弦 {4,6,9,11} = 完整聚合
      <gis bes cis dis>2 <e fis a b>
      % 第 16 小节：T0 收束回归
      <c d f g>1
    }

    % ====== 左手 ======
    \new Staff \relative c {
      \clef bass
      \set Staff.extraNatural = ##f
      \accidentalStyle dodecaphonic
      \time 4/4

      % ===== A: 线性 (第 1-4 小节) =====
      % 长音支撑
      c1
      g1
      d1
      c1

      % ===== B: 纵向 (第 5-8 小节) =====
      % T0 低音和弦
      <c d f g>1
      % T7 低音和弦
      <g a c d>1
      % T2 低音和弦
      <d e g a>1
      % T0 低音和弦
      <c d f g>1

      % ===== C: 交织 (第 9-12 小节) =====
      % T7 延迟一拍进入（与右手 T0 交织）
      r8 g a c' d' g a c'
      r8 c' a g d' c a g
      % T9 延迟进入（与右手 T2 交织）
      r8 fis a b d' fis a b
      r8 b a fis d' b a fis

      % ===== D: 聚合补全 (第 13-16 小节) =====
      % T4 和弦 {4,6,9,11} — 与右手 T0 互补
      <e fis a b>1
      % T0 和弦 — 与右手 T4 互补
      <c d f g>1
      % T0 + T4 双和弦 — 右手 T8+T4 互补，左手补全
      <c d f g>2 <e fis a b>
      % 最终回归 T0
      <c d f g>1
    }
  >>
  \layout { }
}
```

**结构分析**：

| 段落 | 小节 | 技法 | 右手 | 左手 | 集合形式 |
|------|------|------|------|------|---------|
| A | 1-4 | 线性 | 八分音符旋律 | 长音支撑 | T0→T7→T2→T0 |
| B | 5-8 | 纵向 | 四分音符和弦 | 全音符和弦 | T0→T7→T2→T0 |
| C | 9-12 | 交织 | 八分音符跑动 | 延迟一拍模仿 | T0→T0R→T2→T2R |
| D | 13-16 | 聚合补全 | 旋律→和弦 | 互补和弦 | T0+T4→T8+T4→T0 |

**聚合验证（第 15-16 小节）**：
- 右手：{G#, Bb, C#, D#} + {E, F#, A, B} = {1,3,4,6,8,9,10,11}（8 音）
- 左手：{C, D, F, G} + {E, F#, A, B} = {0,2,4,5,6,7,9,11}（8 音）
- 合计：{0,1,2,3,4,5,6,7,8,9,10,11} = 全部 12 个音级 ✓

---

## 七、作曲实践指南

### 7.1 从集合到作品的步骤

1. **选择核心集合**：根据想要的音响色彩选取（参见第五节速查表）
2. **计算可用移调**：确定哪些移调有共同音（平滑连接），哪些无共同音（强对比）
3. **设计结构**：分配线性、纵向、交织、聚合补全段落的比例
4. **声部引导**：利用不变量音实现平滑声部连接
5. **织体变化**：同一集合通过不同织体（琶音/震音/跑动/持续音）产生多样性

### 7.2 避免单调

| 问题 | 解决方案 |
|------|---------|
| 所有和弦听起来一样 | 使用不同移调、不同排列、不同音区 |
| 旋律缺乏方向感 | 加入节奏动机、力度变化、音区对比 |
| 织体过于单一 | 在四种组合技法间切换 |
| 缺乏高潮 | 在聚合补全段落使用最密集织体 |
| 缺乏统一性 | 始终回到核心集合的 T0 形式 |

### 7.3 与其他技法结合

- **集合 + 十二音**：用十二音序列的子集作为动机材料
- **集合 + 节奏集合**：将音级集合的音程比例映射到节奏（如 ic5 = 5:7 时值比）
- **集合 + 音色**：不同集合形式分配给不同乐器音色（→ 见 adler 管弦乐配器法）
- **集合 + 调性暗示**：选择有调性暗示的集合（如 5-35 五声音阶、6-Z32 自然音列），在调性与无调性之间游走

---

## 附录：LilyPond 现代音乐常用设置

```lilypond
% 十二音临时记号风格（每个音都显示临时记号）
\accidentalStyle dodecaphonic

% 取消多余的还原记号（配合 dodecaphonic 使用）
\set Staff.extraNatural = ##f

% 隐藏调号（无调性音乐）
\omit Staff.KeySignature

% 隐藏拍号（自由节奏）
\omit Staff.TimeSignature

% 在小节线上方标注集合名称
c4^\markup { "{0,2,5,7}" }
```
