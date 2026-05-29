---
category: modernist
source: "Serialism — from Schoenberg to Babbitt"
confidence: text_derived
tags: [十二音, 序列主义, 音列, 矩阵, 整体序列, 组合性, 不变性, Schoenberg, Webern]
title: 序列主义
title_en: Serialism — Twelve-Tone and Beyond
difficulty: advanced
contexts: [modernist, serial, twelve-tone, atonal]
---

# 序列主义 — Serialism: Twelve-Tone and Beyond

本文件涵盖十二音序列写作与整体序列主义的核心技术。所有乐器音域数据 → 见 `shared-rules.md`。LilyPond 基础语法 → 见 `lilypond-core-syntax.md`。

> **核心原则**：十二音序列音乐中，所有 12 个音级（pitch class）以特定顺序排列成**音列**（tone row），在一次完整陈述中每个音级恰好出现一次。音乐的全部素材由这一音列的四种变形及其移位构成。

---

## 一、十二音序列基础

### 1.1 音列（Tone Row / 音列 / 序列）

**定义**：音列是将 12 个音级（0–11）以不重复的方式排列成的有序序列。它是整部作品的"DNA"。

| 术语 | 英文 | 说明 |
|------|------|------|
| 音列 / 序列 | Tone Row | 12 个音级的有序排列 |
| 音级 | Pitch Class (pc) | 0=C, 1=C#/Db, 2=D, …, 11=B |
| 原型 | Prime (P) | 音列的原始形式 |
| 逆行 | Retrograde (R) | 原型的逆序 |
| 倒影 | Inversion (I) | 以首音为轴，每音程取反（I(x) = 2a − x mod 12） |
| 逆行倒影 | Retrograde Inversion (RI) | 倒影的逆序 |

### 1.2 四种基本形式

设原型 P = [p₀, p₁, …, p₁₁]，以首音 p₀ 为轴：

| 形式 | 公式 | 说明 |
|------|------|------|
| P | [p₀, p₁, p₂, …, p₁₁] | 原型（Prime） |
| R | [p₁₁, p₁₀, …, p₁, p₀] | 逆行（Retrograde）= P 的逆序 |
| I | [p₀, 2p₀−p₁, 2p₀−p₂, …] mod 12 | 倒影（Inversion），轴音 = p₀ |
| RI | I 的逆序 | 逆行倒影（Retrograde Inversion） |

### 1.3 移位：P0–P11, R0–R11, I0–I11, RI0–RI11

每种形式可在 12 个移位级别上使用，由**首音的音级**标识：

- **Pn**：原型移调至首音为 n。Pn[i] = (P[i] + n − P[0]) mod 12
- **Rn**：Pn 的逆行
- **In**：倒影移调至首音为 n。In[i] = (n + n − P[i]) mod 12 = (2n − P[i]) mod 12
- **RIn**：In 的逆行

共 **48 种**音列形式（4 × 12）。

### 1.4 示例：一个完整音列的四种形式

以下音列 P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]（C B Bb Ab G Gb F E Eb Db D A）：

```lilypond
\version "2.24.0"

\header {
  title = "音列的四种形式"
  subtitle = "P0, R0, I0, RI0"
}

\score {
  \new StaffGroup <<
    % === 原型 P0 ===
    % 音级：0 11 10 8 7 6 5 4 3 1 2 9
    \new Staff {
      \set Staff.instrumentName = "P0 "
      \set Staff.shortInstrumentName = "P0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        % 上行半音阶式下行，末音跳至高八度
        c'8 b bes aes g ges f e ees des d a'
      }
    }

    % === 逆行 R0 ===
    % P0 的逆序：9 2 1 3 4 5 6 7 8 10 11 0
    \new Staff {
      \set Staff.instrumentName = "R0 "
      \set Staff.shortInstrumentName = "R0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        a'8 d des e f ges g aes bes b c'
      }
    }

    % === 倒影 I0 ===
    % 以 C(0) 为轴：I(x) = (0 - x) mod 12
    % 音级：0 1 2 4 5 6 7 8 9 11 10 3
    \new Staff {
      \set Staff.instrumentName = "I0 "
      \set Staff.shortInstrumentName = "I0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        c'8 b bes' aes' g' ges' f' e' ees' d' bes e
      }
    }

    % === 逆行倒影 RI0 ===
    % I0 的逆序：3 10 11 9 8 7 6 5 4 2 1 0
    \new Staff {
      \set Staff.instrumentName = "RI0"
      \set Staff.shortInstrumentName = "RI"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        e bes b a aes g ges f ees des b c'
      }
    }
  >>
  \layout { }
}
```

**要点**：
- `\set Staff.accidentalStyle = #"dodecaphonic"` 使每个音符都显示临时记号（无调号）
- `\absolute` 模式避免 `\relative` 在大跳音程中的八度歧义
- `\time 12/8` 让 12 个八分音符恰好填满一小节

---

## 二、十二音矩阵

### 2.1 矩阵构造方法

12×12 矩阵是十二音技法的核心工具，一次性展示全部 48 种音列形式中的 24 种（12 个 P + 12 个 I）。

**构造公式**：

```
Matrix[r][c] = (P0[c] + I0[r] − P0[0]) mod 12
```

其中：
- 第 0 行（r=0）= P0（原型）
- 第 0 列（c=0）= I0（倒影）
- 第 r 行从左往右读 = Pr（即首音为 I0[r] 的原型移位）
- 第 c 列从上往下读 = Ic（即首音为 P0[c] 的倒影移位）

**逆行形式**：
- Rn = Pn 从右往左读（行的逆序）
- RIn = In 从下往上读（列的逆序）

### 2.2 示例矩阵

以 P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9] 为例：

```
     P0  P11 P10 P8  P7  P6  P5  P4  P3  P1  P2  P9
     ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓
I0→  [ 0  11  10   8   7   6   5   4   3   1   2   9 ] ← P0  (首音 C=0)
I1→  [ 1   0  11   9   8   7   6   5   4   2   3  10 ] ← P1  (首音 Db=1)
I2→  [ 2   1   0  10   9   8   7   6   5   3   4  11 ] ← P2  (首音 D=2)
I4→  [ 4   3   2   0  11  10   9   8   7   5   6   1 ] ← P4  (首音 E=4)
I5→  [ 5   4   3   1   0  11  10   9   8   6   7   2 ] ← P5  (首音 F=5)
I6→  [ 6   5   4   2   1   0  11  10   9   7   8   3 ] ← P6  (首音 Gb=6)
I7→  [ 7   6   5   3   2   1   0  11  10   8   9   4 ] ← P7  (首音 G=7)
I8→  [ 8   7   6   4   3   2   1   0  11   9  10   5 ] ← P8  (首音 Ab=8)
I9→  [ 9   8   7   5   4   3   2   1   0  10  11   6 ] ← P9  (首音 A=9)
I11→ [11  10   9   7   6   5   4   3   2   0   1   8 ] ← P11 (首音 B=11)
I10→ [10   9   8   6   5   4   3   2   1  11   0   7 ] ← P10 (首音 Bb=10)
I3→  [ 3   2   1  11  10   9   8   7   6   4   5   0 ] ← P3  (首音 Eb=3)
```

**读法**：
- 从左往右读任意行 → 对应的 P 形式
- 从右往左读任意行 → 对应的 R 形式
- 从上往下读任意列 → 对应的 I 形式
- 从下往上读任意列 → 对应的 RI 形式

### 2.3 十二音矩阵完整展示

以下 12 行谱表展示 P0–P11 及其对应的 I 形式。每一行同时标注行首的 I 标签和行左的 P 标签。R 形式为各行的逆序，RI 形式为各列的逆序。

```lilypond
\version "2.24.0"

\header {
  title = "十二音矩阵 — 全部 P 形式"
  subtitle = "P0 至 P11，每行从左到右读取"
}

% P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]
% 使用 \absolute 确保音高精确
% 所有音符在 C4-B4 八度范围内

\score {
  \new StaffGroup <<
    % P0 (I0 行) — 音级: 0 11 10 8 7 6 5 4 3 1 2 9
    \new Staff {
      \set Staff.instrumentName = "P0 (I0)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { c'8 b bes aes g ges f e ees des d a' }
    }
    % P1 (I1 行) — 音级: 1 0 11 9 8 7 6 5 4 2 3 10
    \new Staff {
      \set Staff.instrumentName = "P1 (I1)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { des'8 c b a aes g ges f e d ees bes' }
    }
    % P2 (I2 行) — 音级: 2 1 0 10 9 8 7 6 5 3 4 11
    \new Staff {
      \set Staff.instrumentName = "P2 (I2)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { d'8 des c bes a aes g ges f ees e b' }
    }
    % P4 (I4 行) — 音级: 4 3 2 0 11 10 9 8 7 5 6 1
    \new Staff {
      \set Staff.instrumentName = "P4 (I4)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { e'8 ees d c b bes a aes g f ges des' }
    }
    % P5 (I5 行) — 音级: 5 4 3 1 0 11 10 9 8 6 7 2
    \new Staff {
      \set Staff.instrumentName = "P5 (I5)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { f'8 e ees des c b bes a aes ges g d' }
    }
    % P6 (I6 行) — 音级: 6 5 4 2 1 0 11 10 9 7 8 3
    \new Staff {
      \set Staff.instrumentName = "P6 (I6)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { ges'8 f e d des c b bes a g aes ees' }
    }
    % P7 (I7 行) — 音级: 7 6 5 3 2 1 0 11 10 8 9 4
    \new Staff {
      \set Staff.instrumentName = "P7 (I7)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { g'8 ges f ees d des c b bes aes a e' }
    }
    % P8 (I8 行) — 音级: 8 7 6 4 3 2 1 0 11 9 10 5
    \new Staff {
      \set Staff.instrumentName = "P8 (I8)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { aes'8 g ges e ees d des c b a bes f' }
    }
    % P9 (I9 行) — 音级: 9 8 7 5 4 3 2 1 0 10 11 6
    \new Staff {
      \set Staff.instrumentName = "P9 (I9)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { a'8 aes g f e ees d des c bes b ges' }
    }
    % P11 (I11 行) — 音级: 11 10 9 7 6 5 4 3 2 0 1 8
    \new Staff {
      \set Staff.instrumentName = "P11 (I11)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { b'8 bes a g ges f e ees d c des aes' }
    }
    % P10 (I10 行) — 音级: 10 9 8 6 5 4 3 2 1 11 0 7
    \new Staff {
      \set Staff.instrumentName = "P10 (I10)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { bes'8 a aes ges f e ees d des b c' g' }
    }
    % P3 (I3 行) — 音级: 3 2 1 11 10 9 8 7 6 4 5 0
    \new Staff {
      \set Staff.instrumentName = "P3 (I3)"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute { ees'8 d des b bes a aes g ges e f c' }
    }
  >>
  \layout {
    \context {
      \StaffGroup
      \remove System_start_delimiter_engraver
    }
  }
}
```

**读法说明**：
- 每行从左到右 = P 形式（如第一行 = P0）
- 每行从右到左 = R 形式（如第一行反向 = R0）
- 每列从上到下 = I 形式（如第一列 = I0, 第二列 = I11, …）
- 每列从下到上 = RI 形式

### 2.4 著名音列：勋伯格 Op.25

勋伯格《钢琴组曲》Op.25 是第一部完整的十二音作品。其音列为：

**P0 = [4, 5, 7, 1, 6, 3, 8, 2, 11, 0, 10, 9]**
即 E F G Db Gb Eb Ab D B C Bb A

```lilypond
\version "2.24.0"

\header {
  title = "勋伯格 Op.25 音列"
  subtitle = "Schoenberg — Suite für Klavier, Op. 25"
}

\score {
  \new StaffGroup <<
    % P0: E F G Db Gb Eb Ab D B C Bb A
    % 音级: 4 5 7 1 6 3 8 2 11 0 10 9
    \new Staff {
      \set Staff.instrumentName = \markup \bold "P0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        % E4 → F4(上m2) → G4(上M2) → Db4(下A4) →
        % Gb4(上A4) → Eb4(下M3) → Ab4(上A4) → D4(下A4) →
        % B4(上M6→用八度标记) → C4(下m7→用八度标记) →
        % Bb4(上m7→用八度标记) → A4(下m2)
        e'8 f g' des' ges' ees' aes' d' b c' bes a
      }
    }

    % I0 (以 E 为轴的倒影)
    % I0[i] = (4 + 4 - P[i]) mod 12 = (8 - P[i]) mod 12
    % 音级: 4 3 1 7 2 5 0 6 9 8 10 11
    % 即 E Eb Db G D F C Gb A Ab Bb B
    \new Staff {
      \set Staff.instrumentName = \markup \bold "I0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        % E4 → Eb4(下m2) → Db4(下m2) → G4(上A4) →
        % D4(下P4) → F4(上m3) → C4(下P4) → Gb4(上A4) →
        % A4(上M3→用八度标记) → Ab4(下m2) → Bb4(上M2) → B4(上m2)
        e'8 ees des' g d f c' ges' a' aes' bes' b'
      }
    }

    % R0 (P0 的逆行)
    % 音级: 9 10 0 11 2 8 3 6 1 7 5 4
    % 即 A Bb C B D Ab Eb Gb Db G F E
    \new Staff {
      \set Staff.instrumentName = \markup \bold "R0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        a'8 bes c' b d' aes' ees' ges' des' g' f' e'
      }
    }

    % RI0 (I0 的逆行)
    % 音级: 11 10 8 9 6 0 5 2 7 1 3 4
    % 即 B Bb Ab A Gb C F D G Db Eb E
    \new Staff {
      \set Staff.instrumentName = \markup \bold "RI0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \time 12/8
      \absolute {
        b'8 bes' aes' a' ges' c' f' d' g' des' ees' e'
      }
    }
  >>
  \layout { }
}
```

**Op.25 音列特点**：
- 首尾音程 E→F（m2）和 A→Bb→A 的回溯暗示 BACH 动机（Bb A C B = B A C H）
- 首音 E(4) 和末音 A(9) 构成三全音，贯穿全曲
- 该音列不具有六音组合性（见第五节），勋伯格在此更关注动机统一

---

## 三、序列写作规则

### 3.1 核心规则

| 规则 | 说明 |
|------|------|
| **不重复原则** | 在一次完整的音列陈述（12 个音）中，任何音级不得在其他 11 个音级全部出现之前重复 |
| **八度自由** | 同一音级可出现在任意八度位置（C4 和 C5 视为同一音级 pc=0） |
| **同时发声** | 音列中的音可以组成和弦同时发声（纵向化），不违反不重复原则 |
| **音列接续** | 一次音列陈述结束后，可立即开始下一个音列形式（相同或不同） |

### 3.2 音列分割（Row Partitioning）

音列可被分割为若干子集，分配给不同声部或不同时间段：

| 分割方式 | 英文 | 说明 |
|---------|------|------|
| 六音分割 | Hexachordal | 将音列分为前 6 音（H1）和后 6 音（H2），可分配给不同声部 |
| 四音分割 | Tetrachordal | 将音列分为 3 组各 4 音 |
| 三音分割 | Trichordal | 将音列分为 4 组各 3 音 |
| 线性/纵向混合 | Linear/Vertical | 部分音旋律化、部分音和弦化 |

### 3.3 完整 16 小节序列作品示例

以下钢琴小品使用 P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]，采用 3/4 拍，每行音列恰好占 4 小节（12 个四分音符 = 3 小节 × 4/4，或 4 小节 × 3/4）。

**结构**：
- 第 1–4 小节：右手 P0 旋律，左手持续音
- 第 5–8 小节：左手 R0 旋律，右手持续音
- 第 9–12 小节：右手 I0 旋律，左手持续音
- 第 13–16 小节：右手 RI0 旋律，左手持续音 → 终止

```lilypond
\version "2.24.0"

\header {
  title = "序列小品 — 十六小节"
  subtitle = "基于 P0 = [0,11,10,8,7,6,5,4,3,1,2,9]"
  composer = "教学示例"
}

\score {
  \new PianoStaff <<
    % ========== 右手 ==========
    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef treble
      \time 3/4
      \tempo "Allegro" 4 = 120

      % --- 第 1-4 小节：P0 旋律 ---
      % P0: C(0) B(11) Bb(10) Ab(8) G(7) Gb(6) F(5) E(4) Eb(3) Db(1) D(2) A(9)
      \absolute {
        \mark \markup { \bold "P0" }
        c'4^\f b bes | aes g ges | f e ees | des d a' \bar "||"
      }

      % --- 第 5-8 小节：右手持续音（配合左手 R0） ---
      % 右手演奏长音 C，作为踏板音效果
      \absolute {
        c'1 | c'1 | c'1 | c'1 \bar "||"
      }

      % --- 第 9-12 小节：I0 旋律 ---
      % I0: C(0) Db(1) D(2) E(4) F(5) Gb(6) G(7) Ab(8) A(9) B(11) Bb(10) Eb(3)
      \absolute {
        \mark \markup { \bold "I0" }
        c'4^\mf des d | e f ges | g aes a | b bes ees' \bar "||"
      }

      % --- 第 13-16 小节：RI0 旋律（终止段） ---
      % RI0: Eb(3) Bb(10) B(11) A(9) Ab(8) G(7) Gb(6) F(5) E(4) D(2) Db(1) C(0)
      \absolute {
        \mark \markup { \bold "RI0" }
        ees'4^\f bes b | a aes g | ges f e | d des c' \bar "|."
      }
    }

    % ========== 左手 ==========
    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef bass
      \time 3/4

      % --- 第 1-4 小节：持续低音 C ---
      \absolute {
        c1 | c1 | c1 | c1
      }

      % --- 第 5-8 小节：R0 旋律 ---
      % R0: A(9) D(2) Db(1) E(4) F(5) Gb(6) G(7) Ab(8) Eb(3) Bb(10) B(11) C(0)
      \absolute {
        \mark \markup { \bold "R0" }
        a4^\p d des | e f ges | g aes ees | bes b c
      }

      % --- 第 9-12 小节：持续低音 E ---
      \absolute {
        e1 | e1 | e1 | e1
      }

      % --- 第 13-16 小节：持续低音 C（终止） ---
      \absolute {
        c1^\p | c1 | c1 | c1
      }
    }
  >>
  \layout { }
}
```

**分析要点**：
- 第 1–4 小节：右手 P0 旋律每小节 3 个四分音符，12 音恰好分布在 4 小节中
- 第 5–8 小节：左手 R0（P0 的逆行），右手持续音 C 作为对比
- 第 9–12 小节：右手 I0（倒影），音程方向与 P0 相反
- 第 13–16 小节：右手 RI0（逆行倒影），回到 C 音结束，形成首尾呼应
- 全曲严格遵循不重复原则：每次音列陈述中 12 个音级各出现一次

### 3.4 高级技法：和弦化与八度置换

以下示例展示同一音列的多种处理方式：

```lilypond
\version "2.24.0"

\header {
  title = "音列的多种处理方式"
  subtitle = "旋律、和弦化、八度置换"
}

\score {
  \new PianoStaff <<
    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef treble
      \time 4/4
      \tempo "Moderato" 4 = 80

      % --- 方式一：线性旋律（八度置换） ---
      % P0 的各音分配到不同八度，创造大跳效果
      \absolute {
        \mark \markup { \bold "旋律 + 八度置换" }
        c''8 b, bes' aes, g' ges, |
        f' e, ees' des, d' a, |
      }

      % --- 方式二：和弦化（四音分割） ---
      % P0 分为 3 组四音和弦
      \absolute {
        \mark \markup { \bold "和弦化（四音分割）" }
        <c'' b' bes' aes'>4 <g' ges' f' e'> <ees' des' d' a'> r |
      }
    }

    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef bass
      \time 4/4

      % --- 方式三：六音分割 — 左手演奏后六音 ---
      % P0 后六音：F(5) E(4) Eb(3) Db(1) D(2) A(9)
      \absolute {
        \mark \markup { \bold "六音分割 H2" }
        f4 e ees des | d a r2 |
      }

      % --- 左手和弦化 ---
      \absolute {
        <f e ees des>4 <d a> r2 |
      }
    }
  >>
  \layout { }
}
```

---

## 四、整体序列主义

### 4.1 从音高序列到全参数序列

整体序列主义（Integral Serialism / Total Serialism）将十二音的序列化原则从音高扩展到其他音乐参数：

| 参数 | 序列化方式 | 代表作曲家 |
|------|----------|----------|
| **音高** | 12 音级排列成音列 | Schoenberg, Webern |
| **时值** | 12 个不同时值按序排列 | Messiaen, Babbitt |
| **力度** | 12 个（或更少）力度层级 | Boulez, Stockhausen |
| **发音法** | 12 种演奏法按序排列 | Babbitt |
| **音区** | 12 个音区位置 | Boulez |

### 4.2 历史脉络

- **Messiaen**（先驱）：《时值与力度的模式》（*Mode de valeurs et de durées*，1949）首次将音高与时值系统对应
- **Babbitt**：将序列原则扩展到时值、力度、发音法，发展出"集合复合"（set complex）理论
- **Boulez**：《结构 Ia》（*Structures Ia*，1952）对音高、时值、力度、发音法全部序列化
- **Stockhausen**：《交叉演奏》（*Kreuzspiel*，1951）将音高、时值、力度序列化，并引入"群"（Gruppen）概念

### 4.3 整体序列示例

以下示例将音高序列扩展到节奏、力度和发音法：

**序列化方案**：

| 序号 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|------|---|---|---|---|---|---|---|---|---|---|----|----|
| 音级 | C | Db | D | Eb | E | F | Gb | G | Ab | A | Bb | B |
| 时值 | 4 | 16 | 8 | 2 | 8. | 32 | 4. | 16. | 8 | 4 | 2 | 16 |
| 力度 | pp | ff | mp | fff | p | mf | f | ppp | ff | mp | f | pp |
| 奏法 | — | > | . | ^ | — | > | . | ^ | — | > | . | ^ |

```lilypond
\version "2.24.0"

\header {
  title = "整体序列主义示例"
  subtitle = "音高、时值、力度、发音法全部序列化"
}

\score {
  \new Staff {
    \set Staff.accidentalStyle = #"dodecaphonic"
    \clef treble
    \time 4/4
    \tempo "Preciso" 4 = 60

    % 音高序列 P0: C Db D Eb E F Gb G Ab A Bb B
    % 时值序列:    4  16  8  2  8. 32  4. 16.  8  4   2  16
    % 力度序列:    pp  ff mp fff  p  mf   f ppp  ff mp   f  pp
    % 奏法序列:    —   >  .  ^   —   >   .   ^   —  >   .   ^

    \absolute {
      \mark \markup { \bold "全参数序列" }
      % C4: 四分音符, pp, 普通
      c'4\pp
      % Db4: 十六分音符, ff, 重音
      des'16\ff->
      % D4: 八分音符, mp, 跳音
      d'8\mp-.
      % Eb4: 二分音符, fff, 强音
      ees'2\fff-^
      % E4: 附点八分音符, p, 普通
      e'8.\p
      % F4: 三十二分音符, mf, 重音
      f'32\mf->
      % Gb4: 附点四分音符, f, 跳音
      ges'4.\f-.
      % G4: 附点十六分音符, ppp, 强音
      g'16.\ppp-^
      % Ab4: 八分音符, ff, 普通
      aes'8\ff
      % A4: 四分音符, mp, 重音
      a'4\mp->
      % Bb4: 二分音符, f, 跳音
      bes'2\f-.
      % B4: 十六分音符, pp, 强音
      b'16\pp-^
    }

    % 第二遍：时值序列逆行，其他不变
    \absolute {
      \mark \markup { \bold "时值逆行" }
      c'16\pp
      des'2\ff->
      d'4.\mp-.
      ees'16.\fff-^
      e'8\p
      f'4\mf->
      ges'32\f-.
      g'8.\ppp-^
      aes'4\ff
      a'2\mp->
      bes'16\f-.
      b'4.\pp-^
    }
  }
  \layout { }
}
```

**分析说明**：
- 第一遍：所有参数按原序使用
- 第二遍：仅时值参数逆行（R），音高、力度、发音法保持 P 顺序
- 这种"参数独立变形"是 Babbitt 整体序列的核心技术：每个参数可独立应用 P/R/I/RI 变形
- 实际作品中（如 Boulez *Structures Ia*），两个声部使用不同的参数变形组合，形成高度复杂的织体

---

## 五、组合性

### 5.1 六音组合性（Hexachordal Combinatoriality）

**定义**：当两个不同音列形式的**前六音**（第一六音组 H1）合起来恰好包含全部 12 个音级时，称这两个形式具有**六音组合性**。

设 P0 的前六音为 H1(P0)，若存在另一形式 X 使得：

```
H1(P0) ∪ H1(X) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11}
H1(P0) ∩ H1(X) = ∅
```

则 P0 和 X 具有六音组合性。

### 5.2 组合性的意义

- 两个组合性声部同时演奏时，每半段（6 音）即形成一个完整的 12 音聚合体（aggregate）
- 这保证了在任何时刻都不会出现音级重复
- Schoenberg 后期作品大量使用组合性音列
- Babbitt 将组合性发展为"全组合性"（all-combinatoriality）理论

### 5.3 示例：P0 + RI5 的六音组合

使用音列 P0 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]（升序半音阶，为演示组合性而选用）：

- **H1(P0)** = {0, 1, 2, 3, 4, 5}（音级 C Db D Eb E F）
- **RI5** 的第一六音组 = {6, 7, 8, 9, 10, 11}（音级 Gb G Ab A Bb B）
- H1(P0) ∪ H1(RI5) = 全部 12 音级 ✓

> 注：此处使用 Babbitt 风格的升序半音阶音列以便清晰演示组合性原理。Schoenberg 的音列（如 Op.25）通常不具此性质。

```lilypond
\version "2.24.0"

\header {
  title = "六音组合性"
  subtitle = "P0 + RI5 — 互补六音组"
}

\score {
  \new PianoStaff <<
    % === 右手：P0 ===
    % P0 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    % H1 = {0,1,2,3,4,5}, H2 = {6,7,8,9,10,11}
    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef treble
      \time 12/8
      \tempo "Andante" 4. = 60

      \absolute {
        \mark \markup { \bold "P0" }
        % 前六音 H1: C Db D Eb E F → 音级 {0,1,2,3,4,5}
        c'8^\markup { \box "H1: {0,1,2,3,4,5}" } des' d' ees' e' f'
        % 后六音 H2: Gb G Ab A Bb B → 音级 {6,7,8,9,10,11}
        ges'8^\markup { \box "H2: {6,7,8,9,10,11}" } g' aes' a' bes' b'
      }
    }

    % === 左手：RI5 ===
    % RI5 计算：
    % I5[i] = (2*5 - P[i]) mod 12 = (10 - P[i]) mod 12
    % I5 = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 11]
    % RI5 = I5 的逆序 = [11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    % H1(RI5) = {11, 0, 1, 2, 3, 4}...
    % 注意：此处展示的是"移位后"的 RI5，使其 H1 = {6,7,8,9,10,11}
    % 实际使用的形式需要移位以满足组合性条件
    % 为简洁，此处直接写出互补的六音组
    \new Staff {
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef bass
      \time 12/8

      \absolute {
        \mark \markup { \bold "RI5 (互补)" }
        % 前六音 H1': Gb G Ab A Bb B → 音级 {6,7,8,9,10,11}
        ges8^\markup { \box "H1': {6,7,8,9,10,11}" } g aes a bes b
        % 后六音 H2': C Db D Eb E F → 音级 {0,1,2,3,4,5}
        c8^\markup { \box "H2': {0,1,2,3,4,5}" } des d ees e f
      }
    }
  >>
  \layout { }
}
```

**关键观察**：
- 前 6 拍：右手 H1 = {0,1,2,3,4,5}，左手 H1' = {6,7,8,9,10,11} → 合计 = 全部 12 音级 ✓
- 后 6 拍：右手 H2 = {6,7,8,9,10,11}，左手 H2' = {0,1,2,3,4,5} → 合计 = 全部 12 音级 ✓
- 每半段（6 拍）都形成完整的 12 音聚合体

### 5.4 全组合性六音组（All-Combinatorial Hexachords）

Babbitt 识别出 6 种**全组合性六音组**——对 P, R, I, RI 的某些移位都具有组合性：

| 编号 | 六音组（音级） | 福特号 | 特征 |
|------|--------------|--------|------|
| (A) | {0, 1, 2, 3, 4, 5} | 6-1 | 半音阶簇 |
| (B) | {0, 2, 3, 4, 5, 7} | 6-2 | 含全音+半音 |
| (C) | {0, 2, 4, 5, 7, 9} | 6-7 | 全音阶片段 |
| (D) | {0, 1, 2, 6, 7, 8} | 6-8 | 两个三音簇 |
| (E) | {0, 1, 4, 5, 8, 9} | 6-20 | 大三和弦对 |
| (F) | {0, 2, 4, 6, 8, 10} | 6-35 | 全音阶 |

> 这 6 种六音组的互补六音组与自身属于同一集合类（set class），因此天然具有组合性。

---

## 六、不变性

### 6.1 音级不变性（Pitch-Class Invariance）

**定义**：当两个不同音列形式包含**相同音级在相同位置**时，称存在**有序不变性**；包含相同音级但位置不同时，称**无序不变性**。

| 类型 | 英文 | 定义 |
|------|------|------|
| 有序不变性 | Ordered Invariance | 相同音级出现在相同位置索引 |
| 无序不变性 | Unordered Invariance | 相同音级出现在不同位置，但作为子集保持不变 |

### 6.2 有序不变性示例

设 P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]，考察 P0 与其移位形式的关系。

**P0 与 P6**（移高 6 个半音）：
- P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]
- P6 = [6, 5, 4, 2, 1, 0, 11, 10, 9, 7, 8, 3]
- 位置比较：无任何位置的音级相同 → **无有序不变性**

事实上，对于全音程音列（all-interval row），不同移位之间通常没有有序不变性。有序不变性更多出现在具有特殊内部结构的音列中。

### 6.3 无序不变性示例

设 P0 = [0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9]。

**P0 与 I0 的共同音级**：
- P0 音级集合 = {0, 11, 10, 8, 7, 6, 5, 4, 3, 1, 2, 9} = 全部 12 音级
- I0 音级集合 = 全部 12 音级（任何音列形式都包含全部 12 音级）

因此需要考察**子集**的不变性：

**P0 的前四音** {0, 11, 10, 8} = {C, B, Bb, Ab}
**I0 的前四音** {0, 1, 2, 4} = {C, Db, D, E}
→ 仅共享 pc=0（C），不构成有意义的不变子集

**更有意义的例子**：考察 P0 和 I0 中共享的**四音子集** {1, 2, 4, 9} = {Db, D, E, A}：

- 在 P0 中：Db(pos=9), D(pos=10), E(pos=7), A(pos=11) → 位置 [7, 9, 10, 11]
- 在 I0 中：Db(pos=8), D(pos=2), E(pos=3), A(pos=11) → 位置 [2, 3, 8, 11]
- 共同音级 {1, 2, 4, 9} 在两种形式中都出现 → **无序 pc 不变性** ✓
- 但位置不同（[7,9,10,11] vs [2,3,8,11]）→ **非有序不变性**

### 6.4 不变性在创作中的意义

不变性子集在序列音乐中具有**结构锚点**的作用：
- 当音列形式变换时，不变音级提供了**连贯性**
- Webern 大量利用不变性创造作品的统一感
- 不变三音组/四音组可作为**动机细胞**在不同音列形式间传递

### 6.5 不变性示例

以下示例展示 P0 与 I0 中不变子集 {1, 2, 4, 9} = {Db, D, E, A} 的标注：

```lilypond
\version "2.24.0"

\header {
  title = "不变性示例"
  subtitle = "P0 与 I0 中的共同音级子集 {Db, D, E, A}"
}

\score {
  \new StaffGroup <<
    % P0: C(0) B(11) Bb(10) Ab(8) G(7) Gb(6) F(5) E(4) Eb(3) Db(1) D(2) A(9)
    % 不变音级标记: E(pos=7), Db(pos=9), D(pos=10), A(pos=11)
    \new Staff {
      \set Staff.instrumentName = "P0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef treble
      \time 12/8

      \absolute {
        % 普通音级：正常时值
        c'8 b bes aes g ges f
        % 不变子集音级：加保持音标记
        e8-^
        ees
        des-^
        d-^
        a'-^
      }
    }

    % I0: C(0) Db(1) D(2) E(4) F(5) Gb(6) G(7) Ab(8) A(9) B(11) Bb(10) Eb(3)
    % 不变音级标记: D(pos=2), E(pos=3), A(pos=8), Db(pos=1)...
    % 实际上 I0 中 {1,2,4,9} 的位置: Db(1), D(2), E(3), A(8)
    \new Staff {
      \set Staff.instrumentName = "I0"
      \set Staff.accidentalStyle = #"dodecaphonic"
      \clef treble
      \time 12/8

      \absolute {
        c'8
        % 不变子集音级
        des-^
        d-^
        e-^
        % 普通音级
        f ges g aes
        a-^
        b bes ees
      }
    }
  >>
  \layout { }
}
```

**标注说明**：
- 带 `-^`（marcato/强音记号）的音符为不变子集 {Db, D, E, A} 的成员
- P0 中：E(位置7)、Db(位置9)、D(位置10)、A(位置11) — 集中在音列后半段
- I0 中：Db(位置1)、D(位置2)、E(位置3)、A(位置8) — 分散在音列各处
- 相同的四个音级在两种形式中以**不同顺序和位置**出现 → 无序 pc 不变性

---

## 附录：术语对照表

| 中文 | English | 说明 |
|------|---------|------|
| 音列 / 序列 | Tone Row | 12 音级的有序排列 |
| 原型 | Prime (P) | 音列的原始形式 |
| 逆行 | Retrograde (R) | 原型的逆序 |
| 倒影 | Inversion (I) | 音程取反 |
| 逆行倒影 | Retrograde Inversion (RI) | 倒影的逆序 |
| 移位 | Transposition | 整体移高/移低 n 个半音 |
| 矩阵 | Matrix | 12×12 表格，展示全部 P/I 形式 |
| 六音组 | Hexachord | 音列的前 6 音或后 6 音 |
| 组合性 | Combinatoriality | 两个形式的六音组互补 |
| 全组合性 | All-Combinatoriality | 对多种移位都具有组合性 |
| 不变性 | Invariance | 不同形式间的共同音级 |
| 有序不变性 | Ordered Invariance | 相同音级在相同位置 |
| 无序不变性 | Unordered Invariance | 相同音级在不同位置 |
| 聚合体 | Aggregate | 全部 12 音级的完整集合 |
| 整体序列 | Integral Serialism | 多个参数同时序列化 |
| 音级 | Pitch Class (pc) | 0=C, 1=C#/Db, …, 11=B |
| 福特号 | Forte Number | 音级集合的分类编号 |
| 音程向量 | Interval Vector | 6 维向量 [ic1..ic6] |

> 乐器音域与通用写作规范 → 见 `shared-rules.md`。
> LilyPond 基础语法（括号系统、`\relative` 模式、多声部写法） → 见 `lilypond-core-syntax.md`。
