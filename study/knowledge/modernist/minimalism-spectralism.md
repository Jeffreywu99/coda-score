---
category: modernist
source: "Minimalism and Spectralism — 20th/21st century compositional approaches"
confidence: text_derived
tags: [极简主义, 频谱主义, 相位, 加法过程, 微分音, 泛音列, Reich, Glass, Pärt, Grisey, Murail]
title: 极简主义与频谱主义
title_en: Minimalism and Spectralism
difficulty: advanced
contexts: [modernist, minimalist, spectral, process-music]
---

# 极简主义与频谱主义

极简主义（Minimalism）和频谱主义（Spectralism）是 20 世纪下半叶两大作曲流派。极简主义以**过程**为核心——音乐材料通过重复、相位偏移、加法/减法等技术自然演化；频谱主义以**声音的物理结构**为出发点——泛音列、微分音、音色合成取代了传统功能和声。

> **乐器音域、谱号、`\relative` 参考** → 见 shared-rules.md §1
> **LilyPond 基础语法** → 见 lilypond-core-syntax.md
> **注意**：极简主义与频谱主义有意打破传统声部进行规则（平行五度/八度在此风格中可接受）

---

# PART A: 极简主义 (Minimalism)

---

## 一、相位音乐 (Phase Music)

**核心概念**：Steve Reich 开创的相位技术——两个演奏者演奏完全相同的短小模式，其中一人略微加速，逐渐"滑出"同步状态，产生不断变化的对位关系。

### 1.1 基本原理

| 阶段 | 声部 1 | 声部 2 | 效果 |
|------|--------|--------|------|
| 同相 | E-F#-B-C#-D-F# | E-F#-B-C#-D-F# | 齐奏 |
| 偏移 1 位 | E-F#-B-C#-D-F# | F#-B-C#-D-F#-E | 二声部卡农 |
| 偏移 2 位 | E-F#-B-C#-D-F# | B-C#-D-F#-E-F# | 新对位组合 |
| 偏移 3 位 | E-F#-B-C#-D-F# | C#-D-F#-E-F#-B | 继续变化 |

### 1.2 乐谱示例 — Reich 风格相位音乐

以下示例展示同一 6 音模式在两个声部之间的 4 个相位阶段。Piano 1 始终保持原始顺序，Piano 2 每阶段向前偏移一位。

```lilypond
\version "2.24.0"

\header {
  title = "相位音乐示例"
  subtitle = "Phase Music — 仿 Steve Reich"
}

% 原始 6 音模式（全音阶片段）
pattern = { e'8 fis' b' cis'' d'' fis'' }

\score {
  \new PianoStaff <<
    % Piano 1（上方谱表）：始终保持原始模式
    \new Staff \relative c'' {
      \clef treble
      \time 3/4
      \tempo "Vivace" 4 = 144

      % === 阶段 1：同相（齐奏） ===
      \mark \markup { \bold "阶段 1 — 同相" }
      \repeat unfold 2 { \pattern }

      % === 阶段 2：Piano 2 偏移 1 位 ===
      \mark \markup { \bold "阶段 2 — 偏移 1" }
      \repeat unfold 2 { \pattern }

      % === 阶段 3：Piano 2 偏移 2 位 ===
      \mark \markup { \bold "阶段 3 — 偏移 2" }
      \repeat unfold 2 { \pattern }

      % === 阶段 4：Piano 2 偏移 3 位 ===
      \mark \markup { \bold "阶段 4 — 偏移 3" }
      \repeat unfold 2 { \pattern }
    }

    % Piano 2（下方谱表）：每阶段轮换起始位置
    \new Staff \relative c'' {
      \clef treble

      % 阶段 1：原始顺序（与 Piano 1 同相）
      \repeat unfold 2 { e'8 fis' b' cis'' d'' fis'' }

      % 阶段 2：从第 2 音开始
      \repeat unfold 2 { fis'8 b' cis'' d'' fis'' e' }

      % 阶段 3：从第 3 音开始
      \repeat unfold 2 { b'8 cis'' d'' fis'' e' fis' }

      % 阶段 4：从第 4 音开始
      \repeat unfold 2 { cis''8 d'' fis'' e' fis' b' }
    }
  >>
  \layout { }
}
```

### 1.3 记谱要点

- **渐变过程**：实际演奏中，相位偏移是连续渐变的（通过微小速度差），记谱中只能展示离散阶段
- **排练标记**：用 `\mark` 标注每个阶段，方便演奏者定位
- **模式选择**：Reich 常用 5-12 音的模式，音程以大二度/小三度为主（全音阶色彩）

### 1.4 常见错误

```lilypond
% WRONG: 两个声部使用不同时值——相位音乐要求两声部节奏完全一致
% Piano 1: e'4 fis' b'    (四分音符)
% Piano 2: e'8 fis' b'    (八分音符)  ← 节奏不同就不是相位了

% WRONG: 模式不一致——两声部必须使用完全相同的音高序列
% Piano 1: e'8 fis' b' cis'' d'' fis''
% Piano 2: fis'8 g' b' cis'' d'' fis''  ← g' 不在原模式中
```

---

## 二、加法过程 (Additive Process)

**核心概念**：Philip Glass 的标志性技术——一个短小动机逐音增长（2→3→4→...→8 音），到达最大长度后再逐音缩减（减法过程）。音乐的形式直接由**数量的变化**驱动。

### 2.1 加法过程原理

```
阶段 1: [E-F#]                        → 2 音
阶段 2: [E-F#-E] + [G#]               → 3 音（基础 + 1）
阶段 3: [E-F#-E] + [G#-A]             → 4 音（基础 + 2）
阶段 4: [E-F#-E] + [G#-A-B]           → 5 音（基础 + 3）
...
阶段 7: [E-F#-E] + [G#-A-B-C#-D-E-F#] → 8 音（完整）
```

### 2.2 乐谱示例 — Glass 风格加法/减法过程

```lilypond
\version "2.24.0"

\header {
  title = "加法过程"
  subtitle = "Additive Process — 仿 Philip Glass"
}

% 右手固定开头（2 音动机）
baseRH = { e''8 fis'' }

% 右手逐阶段增长的尾部
growA = { e''8 }                                    % +1 音 = 3 音总计
growB = { e''8 gis'' }                              % +2 音 = 4 音总计
growC = { e''8 gis'' a'' }                          % +3 音 = 5 音总计
growD = { e''8 gis'' a'' b'' }                      % +4 音 = 6 音总计
growE = { e''8 gis'' a'' b'' cis''' }               % +5 音 = 7 音总计
growF = { e''8 gis'' a'' b'' cis''' d''' e''' fis''' } % +6 音 = 8 音总计

\score {
  \new PianoStaff <<
    % 右手（上方谱表）：加法 → 减法
    \new Staff \relative c'' {
      \clef treble
      \key e \major

      % --- 加法阶段（2→8 音） ---
      \time 2/8
      \baseRH                                                       % 阶段 1：2 音
      \time 3/8
      \baseRH \growA                                                % 阶段 2：3 音
      \time 4/8
      \baseRH \growB                                                % 阶段 3：4 音
      \time 5/8
      \baseRH \growC                                                % 阶段 4：5 音
      \time 6/8
      \baseRH \growD                                                % 阶段 5：6 音
      \time 7/8
      \baseRH \growE                                                % 阶段 6：7 音
      \time 8/8
      \baseRH \growF                                                % 阶段 7：8 音（完整）

      % --- 减法阶段（8→2 音） ---
      \time 7/8
      \baseRH \growE                                                % 阶段 8：7 音
      \time 6/8
      \baseRH \growD                                                % 阶段 9：6 音
      \time 5/8
      \baseRH \growC                                                % 阶段 10：5 音
      \time 4/8
      \baseRH \growB                                                % 阶段 11：4 音
      \time 3/8
      \baseRH \growA                                                % 阶段 12：3 音
      \time 2/8
      \baseRH                                                       % 阶段 13：2 音（回归）
    }

    % 左手（下方谱表）：持续低音脉动
    \new Staff \relative c {
      \clef bass
      \key e \major

      \time 2/8  e,4                                          % 2 音 = 1 拍
      \time 3/8  e,4.                                         % 3 音 = 1.5 拍
      \time 4/8  e,2                                          % 4 音 = 2 拍
      \time 5/8  e,2~ e,8                                     % 5 音 = 2.5 拍
      \time 6/8  e,2.                                         % 6 音 = 3 拍
      \time 7/8  e,2.~ e,8                                    % 7 音 = 3.5 拍
      \time 8/8  e,1                                          % 8 音 = 4 拍
      \time 7/8  e,2.~ e,8                                    % 7 音
      \time 6/8  e,2.                                         % 6 音
      \time 5/8  e,2~ e,8                                     % 5 音
      \time 4/8  e,2                                          % 4 音
      \time 3/8  e,4.                                         % 3 音
      \time 2/8  e,4                                          % 2 音（回归）
    }
  >>
  \layout { }
}
```

### 2.3 分析要点

- **时间签名跟随过程**：拍号从 2/8 增长到 8/8 再缩减回来，视觉上直观呈现加法/减法
- **固定开头 + 增长尾部**：`baseRH` 始终不变（E-F#），尾部每次多一个音，体现"累积"效果
- **左手脉动**：持续低音 E 以不同时值填充每小节，提供和声锚点
- **调性色彩**：使用 E 大调音阶片段，体现 Glass 偏好自然音阶材料的特点

### 2.4 变体技巧

| 变体 | 说明 | 代表作品 |
|------|------|---------|
| 加法 + 重复 | 每个阶段重复 2-4 次再进入下一阶段 | Glass: *Music in Fifths* |
| 分组加法 | 1+2+3 而非 1+1+1 | Glass: *Einstein on the Beach* |
| 多声部加法 | 不同声部在不同时刻开始加法 | Glass: *Music in Similar Motion* |
| 节奏加法 | 音高不变，只增长节奏时值 | Reich: *Drumming* |

---

## 三、模块化重复 (Modular Repetition)

**核心概念**：Terry Riley 的 *In C* 开创了模块化重复——53 个短小音乐模块，演奏者各自独立重复每个模块任意次数，形成不断变化的复调织体。每个模块像一块"马赛克瓷砖"，自由组合产生涌现式结构。

### 3.1 In C 的核心规则

1. 所有模块基于 C 大调音阶（无调号）
2. 每个模块 1-4 拍，演奏者自由重复 3-∞ 次
3. 演奏者独立决定何时进入下一个模块
4. 一个持续的高音 C 八度脉动贯穿全曲（"脉冲"声部）

### 3.2 乐谱示例 — 三个独立模块

以下展示三个不同长度的模块，各自在独立谱表上重复。由于长度差异（6:8:7 个十六分音符），它们之间的相位关系持续变化。

```lilypond
\version "2.24.0"

\header {
  title = "模块化重复"
  subtitle = "Modular Repetition — 仿 Terry Riley In C"
}

% 模块 A：6 个十六分音符（分解三和弦上行）
moduleA = { e''16 g'' c''' g'' e'' g'' }

% 模块 B：8 个十六分音符（音阶式上下行）
moduleB = { c''8 d''16 e'' f'' e'' d''8 c''16 }

% 模块 C：7 个十六分音符 + 1 个休止（附点节奏）
moduleC = { g'8. e''16 g''8 r16 e''16 }

\score {
  \new StaffGroup <<
    % 模块 A（长笛）：6 音模块 × 8 次 = 48 个十六分音符
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      \tempo "Fast, with energy" 4 = 132
      \mark \markup { \bold "模块 A" }
      \repeat unfold 8 { \moduleA }
    }

    % 模块 B（双簧管）：8 音模块 × 6 次 = 48 个十六分音符
    \new Staff \relative c'' {
      \clef treble
      \mark \markup { \bold "模块 B" }
      \repeat unfold 6 { \moduleB }
    }

    % 模块 C（单簧管）：7+1 音模块 × 6 次 = 48 个十六分音符
    \new Staff \relative c' {
      \clef treble
      \mark \markup { \bold "模块 C" }
      \repeat unfold 6 { \moduleC }
    }

    % 脉冲声部（钢琴高音 C 八度）
    \new Staff \relative c''' {
      \clef treble
      \mark \markup { \bold "脉冲" }
      \repeat unfold 24 { c8 }
    }
  >>
  \layout { }
}
```

### 3.3 过程分析

| 时间点 | 模块 A 位置 | 模块 B 位置 | 模块 C 位置 | 织体效果 |
|--------|-----------|-----------|-----------|---------|
| 第 1 拍 | 起始 (E-G-C) | 起始 (C-D-E) | 起始 (G) | 稀疏 |
| 第 4 拍 | 第 3 轮 | 第 2 轮 | 第 2 轮 | 渐密 |
| 第 8 拍 | 第 5 轮 | 第 3 轮 | 第 3 轮 | 稠密 |
| 第 12 拍 | 第 8 轮（结束） | 第 5 轮 | 第 5 轮 | A 退出 |

**关键**：模块 A（6 音）循环 8 次完成，但 B（8 音）才循环 6 次、C（8 音含休止）也是 6 次。不同长度的模块在时间中"漂移"，产生不断变化的对位。

### 3.4 写作指南

- **模块长度**：1-4 拍最佳。太短会失去个性，太长难以记忆
- **音域控制**：所有模块应在合理音域内，确保任何组合都好听
- **节奏多样**：混合八分、十六分、附点，避免所有模块节奏雷同
- **休止符**：在模块中加入休止，制造呼吸感和织体空隙

---

## 四、极简和声 (Minimalist Harmony)

**核心概念**：极简主义的和声语言通常是**自然音阶但非功能性**的——不使用传统 T-S-D-T 进行，而是创造静态的和声场（harmonic field），或使用极其缓慢的和声变换。

### 4.1 极简和声与传统和声对比

| 特征 | 传统功能和声 | 极简和声 |
|------|-----------|---------|
| 进行逻辑 | T→S→D→T | 静态或渐变 |
| 和声节奏 | 每 1-2 拍换和弦 | 每 4-16 小节换一次 |
| 声部进行 | 严格（禁平行五/八度） | 自由（平行五度可接受） |
| 调性 | 明确的大小调 | 调式或泛调性 |
| 典型和弦 | 三和弦/七和弦 | 附加音和弦、空五度 |

### 4.2 Arvo Pärt 的钟鸣作曲法 (Tintinnabuli)

**核心规则**：
- **旋律声部 (M-voice)**：自然音阶级进（上行或下行，无跳进）
- **钟鸣声部 (T-voice)**：始终演奏主三和弦的音（根音/三音/五音），选择与 M-voice 最近的三和弦音
- **和声**：始终锚定在一个三和弦上（如 A 小调 = A-C-E）

**T-voice 选音规则（第一位置——取 M 音上方最近三和弦音）**：

| M-voice 音 | A 小调三和弦音 (A C E) | T-voice 选择（上方最近） |
|------------|----------------------|----------------------|
| A | A, C, E | A（同度） |
| B | A, C, E | C（上方二度） |
| C | A, C, E | C（同度） |
| D | A, C, E | E（上方二度） |
| E | A, C, E | E（同度） |
| F | A, C, E | A（上方四度） |
| G | A, C, E | A（上方二度） |

### 4.3 乐谱示例 — Pärt 风格钟鸣作曲法（8 小节）

```lilypond
\version "2.24.0"

\header {
  title = "钟鸣作曲法"
  subtitle = "Tintinnabuli — 仿 Arvo Pärt"
}

\score {
  \new PianoStaff <<
    % 右手：旋律声部 (M-voice) + 钟鸣声部 (T-voice)
    \new Staff \relative c' {
      \clef treble
      \key a \minor
      \time 4/4
      \tempo "Lento, tranquillo" 4 = 60

      % M-voice：A 自然小调音阶上行再下行（级进，无跳进）
      % T-voice：始终取 A 小调三和弦 (A-C-E) 中 M 音上方最近的音
      <<
        % 旋律声部 (M-voice)
        { a'2 b | c d | e f | g a |
          g f | e d | c b | a1 }
        \\
        % 钟鸣声部 (T-voice)：三和弦音 A-C-E
        % 规则：取 M 音上方最近的三和弦音
        { e2 a, | a a | c c | e e |
          e c | c a | a e | a1 }
      >>
    }

    % 左手：持续低音 A（锚定调性）
    \new Staff \relative c {
      \clef bass
      \key a \minor
      a1~ | a~ | a~ | a~ |
      a~ | a~ | a~ | a |
    }
  >>
  \layout { }
}
```

### 4.4 逐小节验证

| 小节 | M-voice | T-voice | M→T 音程 | T 是否为三和弦音 |
|------|---------|---------|---------|---------------|
| 1 | A4, B4 | E4, A3 | P5↓, M2↓ | E✓, A✓ |
| 2 | C5, D5 | A3, A3 | m3↓, P4↓ | A✓, A✓ |
| 3 | E5, F5 | C4, C4 | M3↓, P4↓ | C✓, C✓ |
| 4 | G5, A5 | E4, E4 | M3↓, P5↓ | E✓, E✓ |
| 5 | G5, F5 | E4, C4 | M3↓, P4↓ | E✓, C✓ |
| 6 | E5, D5 | C4, A3 | M3↓, P4↓ | C✓, A✓ |
| 7 | C5, B4 | A3, E3 | m3↓, P5↓ | A✓, E✓ |
| 8 | A4 | A3 | P8↓ | A✓ |

**要点**：
- M-voice 始终级进（无跳进），形成平滑的拱形旋律
- T-voice 始终锚定在 A-C-E 三和弦音上，形成"钟声"般的静态背景
- 低音持续 A（pedal point），锚定调性
- 整体效果：极度宁静、冥想式的声音世界

### 4.5 极简和声渐变技术

除了钟鸣法，极简主义还使用以下和声技术：

| 技术 | 说明 | 代表作曲家 |
|------|------|----------|
| 和声渐变 | 每次只改变一个音（如 C-E-G → C-E-A → D-E-A） | Glass, Adams |
| 静态和声场 | 一个和弦持续数十小节 | Young, La Monte |
| 附加音和弦 | 三和弦 + 附加二度/四度/六度 | Pärt, Górecki |
| 调式交替 | 在同主音大/小调之间缓慢切换 | Pärt |

---

## 五、节奏过程 (Rhythmic Processes)

### 5.1 节拍变换 (Metric Modulation)

**核心概念**：通过一个"桥梁时值"（pivot duration）实现速度变换。旧速度中的某个细分时值成为新速度的基本拍。

**经典公式**：旧速度的四分音符三连音 = 新速度的四分音符
- 若旧速度 ♩=120，则三连音中每个四分 = 200ms
- 新速度的四分音符 = 200ms → ♩=300... 不对

正确公式：旧速度的**附点四分音符** = 新速度的四分音符
- ♩=120 → 附点四分 = 750ms → 新速度 ♩=80

### 5.2 乐谱示例 — 节拍变换

```lilypond
\version "2.24.0"

\header {
  title = "节拍变换"
  subtitle = "Metric Modulation"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4

    % 第一阶段：原始速度 ♩=120
    \tempo "Allegro" 4 = 120
    \mark \markup { \bold "♩=120" }
    c'8 d e f g a b c
    c,8 d e f g a b c

    % 过渡：四分音符三连音暗示新速度
    % 三连音的每个四分 = 旧速度的 2/3 个四分 = 新速度的 1 个四分
    \mark \markup { \bold "过渡：♩. = ♩" }
    \tuplet 3/2 { c4 d e } \tuplet 3/2 { f g a }
    \tuplet 3/2 { b c d } \tuplet 3/2 { e f g }

    % 第二阶段：新速度（三连音四分 = 新四分）
    % ♩=120 的三连音四分 → 新速度 ♩=80
    \time 3/4
    \tempo "Moderato" 4 = 80
    \mark \markup { \bold "♩=80（新速度）" }
    a,4 b c
    d e f
    g a b
    c2.
  }
  \layout { }
}
```

### 5.3 等节奏 (Isorhythm)

**中世纪技法在极简主义中的复兴**：将音高序列（color）与节奏序列（talea）设为不同长度，产生长周期的循环错位。

```lilypond
\version "2.24.0"

\header {
  title = "等节奏"
  subtitle = "Isorhythm — Color(3音) × Talea(4节奏)"
}

% Color（音高循环）：3 个音 C-D-E，重复 4 次 = 12 音
% Talea（节奏循环）：4 个时值 四分-八分-八分-四分，重复 3 次 = 12 拍
% 3 × 4 = 12，完整的等节奏周期

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    \tempo "Moderato" 4 = 100

    % 完整周期 = 12 拍 = 3 小节（4/4 拍）
    % Talea 循环 1：C(四分) D(八分) E(八分) C(四分)
    % Talea 循环 2：D(四分) E(八分) C(八分) D(四分)
    % Talea 循环 3：E(四分) C(八分) D(八分) E(四分)
    c'4 d8 e c4 d8 e8
    d4 e8 c d4 e8 c8
    e4 c8 d e4 c8 d8
  }
  \layout { }
}
```

**过程分析**：

| 拍号 | Color 位置 | Talea 位置 | 实际音 | 时值 |
|------|-----------|-----------|--------|------|
| 1 | 1 (C) | 1 (四分) | C5 | ♩ |
| 2 | 2 (D) | 2 (八分) | D5 | ♪ |
| 2.5 | 3 (E) | 3 (八分) | E5 | ♪ |
| 3 | 1 (C) | 4 (四分) | C5 | ♩ |
| 4 | 2 (D) | 1 (四分) | D5 | ♩ |
| 5 | 3 (E) | 2 (八分) | E5 | ♪ |
| ... | ... | ... | ... | ... |

Color（3 音循环）与 Talea（4 节奏循环）的最小公倍数 = 12，因此 12 拍后两序列重新对齐。

### 5.4 多节拍 (Polytempo / Polymetry)

**核心概念**：不同声部使用不同的节拍划分，在同一时间框架内创造独立的律动层。

```lilypond
\version "2.24.0"

\header {
  title = "多节拍"
  subtitle = "Polymetry — 3/4 vs 4/4 同时进行"
}

\score {
  \new PianoStaff <<
    % 右手：3/4 拍（三拍一组）
    \new Staff \relative c'' {
      \clef treble
      \set Staff.timeSignaturePlacement = #'()
      \time 3/4
      \tempo "Presto" 4 = 144
      \mark \markup { \bold "右手：3/4" }

      \repeat unfold 4 {
        <c' e' g'>4 <d' f' a'> <e' g' b'>
      }
    }

    % 左手：4/4 拍（四拍一组），使用 measureLength 对齐小节线
    \new Staff \relative c {
      \clef bass
      \set Staff.timeSignaturePlacement = #'()
      \time 4/4
      \set Score.measureLength = #(ly:make-moment 3/4)
      \mark \markup { \bold "左手：4/4" }

      % 每 3 拍（右手一个小节）左手演奏 3 个四分音符
      % 但左手的音乐逻辑是 4 拍分组
      c,4 e g c
      e g c e
      g c e g
    }
  >>
  \layout { }
}
```

**效果**：右手每 3 拍一个循环，左手每 4 拍一个循环。两者的重音点在每 12 拍（最小公倍数 3×4）重新对齐一次，中间产生持续的律动冲突。

---

# PART B: 频谱主义 (Spectralism)

---

## 六、频谱和声基础 (Spectral Harmony Basics)

**核心概念**：频谱主义（Gérard Grisey, Tristan Murail 等）将**声音的物理结构**作为作曲出发点。传统和声基于功能和弦（I-IV-V-I），频谱和声基于**泛音列**——一个基音的物理泛音直接构成和弦。

### 6.1 泛音列速查表

以 C3（基频 ≈ 130.8 Hz）为例：

| 泛音 | 频率比 | 音名 | 与十二平均律偏差 | LilyPond 标记 | 音程（相对基音） |
|------|--------|------|-----------------|--------------|---------------|
| 1 | 1:1 | C3 | 0 | `c` | 纯一度 |
| 2 | 2:1 | C4 | 0 | `c'` | 纯八度 |
| 3 | 3:1 | G4 | +2 cents | `g'` | 纯十二度 |
| 4 | 4:1 | C5 | 0 | `c''` | 两个八度 |
| 5 | 5:1 | E5 | -14 cents | `e''` | 大三度+两八度 |
| 6 | 6:1 | G5 | +2 cents | `g''` | 纯五度+两八度 |
| **7** | **7:1** | **B♭5** | **-31 cents** | **`bes''`+eh** | **小七度+两八度（偏低）** |
| 8 | 8:1 | C6 | 0 | `c'''` | 三个八度 |
| 9 | 9:1 | D6 | +4 cents | `d'''` | 大二度+三八度 |
| 10 | 10:1 | E6 | -14 cents | `e'''` | 大三度+三八度 |
| **11** | **11:1** | **F#6** | **-49 cents** | **`fis'''`+eh** | **增四度+三八度（偏低）** |
| 12 | 12:1 | G6 | +2 cents | `g'''` | 纯五度+三八度 |
| **13** | **13:1** | **A6** | **+41 cents** | **`a'''`+ih** | **大六度+三八度（偏高）** |
| 14 | 14:1 | B♭6 | -31 cents | `bes'''`+eh | 小七度+三八度 |
| 15 | 15:1 | B6 | -12 cents | `b'''` | 大七度+三八度 |
| 16 | 16:1 | C7 | 0 | `c''''` | 四个八度 |

> **加粗行** = 与十二平均律偏差 ≥ 30 cents，需要微分音修正

### 6.2 乐谱示例 — 泛音列和弦（C3 基音）

以下展示 C3 基音的泛音 1-8 构成的和弦。第 7 泛音使用微分音修正。

```lilypond
\version "2.24.0"

\header {
  title = "泛音列和弦"
  subtitle = "Spectral Chords from Harmonic Series of C3"
}

\score {
  \new PianoStaff <<
    % 右手（高音区泛音 4-8）
    \new Staff \relative c' {
      \clef treble
      \time 4/4
      \tempo "Largo, sonoro" 4 = 50

      % 和弦 1：泛音 1-4（纯音程，完全协和）
      % C4 + E4 + G4 + C5
      <c' e' g' c''>1^\markup { \small "泛音 1-4: C E G C" }

      % 和弦 2：泛音 1-6（加入大三度和纯五度）
      % C4 + E4 + G4 + C5 + E5 + G5
      <c' e' g' c'' e'' g''>1^\markup { \small "泛音 1-6: +E5 G5" }

      % 和弦 3：泛音 1-7（加入第 7 泛音：偏低 31 cents 的 B♭）
      % 第 7 泛音用 quarter-flat（四分之一降）修正
      <c' e' g' c'' e'' g'' beses''>1^\markup {
        \small "泛音 1-7: +B♭(7th, ¼♭)"
      }

      % 和弦 4：泛音 1-8（完整八度泛音列）
      <c' e' g' c'' e'' g'' beses'' c'''>1^\markup {
        \small "泛音 1-8: +C6 (完整)"
      }
    }

    % 左手（低音区基音 + 八度）
    \new Staff \relative c {
      \clef bass
      % 持续低音 C3（基音）
      c,1~ | c~ | c~ | c
    }
  >>
  \layout { }
}
```

### 6.3 非谐频谱 (Inharmonic Spectra)

并非所有声音都遵循整数倍泛音列。钟声、锣声、钢琴低音弦等产生**非谐频谱**（inharmonic spectra），泛音频率不是基频的整数倍。

| 声源 | 频谱特征 | 作曲应用 |
|------|---------|---------|
| 钟声 | 非整数倍泛音（如 1, 2.76, 5.4, 8.93...） | Grisey: *Les Espaces Acoustiques* |
| 锣 | 密集非谐泛音 | Murail: *Gondwana* |
| 钢琴低音 | 拉伸泛音（高音泛音偏高） | Saariaho: *Lichtbogen* |

### 6.4 常见错误

```lilypond
% WRONG: 用十二平均律音高直接替代泛音列
% 泛音 7 写成普通 B♭（偏高 31 cents，听起来不"纯净"）
% 应使用微分音修正：<c' e' g' c'' e'' g'' beses''>

% WRONG: 忽略泛音的空间分布
% 将所有泛音塞在一个八度内会造成浑浊
% 应保持泛音的自然八度分布（低音稀疏，高音密集）
```

---

## 七、微分音和声 (Microtonal Harmony)

### 7.1 LilyPond 微分音记号速查表

| 后缀 | 含义 | 变化量 | 等效 | 示例 |
|------|------|--------|------|------|
| `ih` | quarter-sharp（四分之一升） | +50 cents | ¼♯ | `cih` = C+¼♯ |
| `eh` | quarter-flat（四分之一降） | -50 cents | ¼♭ | `ceh` = C+¼♭ |
| `isih` | three-quarter-sharp（四分之三升） | +150 cents | ¾♯ | `cisih` = C+¾♯ |
| `eseh` | three-quarter-flat（四分之三降） | -150 cents | ¾♭ | `ceseh` = C+¾♭ |

**构建规则**：
- 升号系 `is` + 四分之一升 `ih` = `isih`（四分之三升）
- 降号系 `es` + 四分之一降 `eh` = `eseh`（四分之三降）
- E 和 A 的降号系使用 `ees`/`aes`（双 e/a 规则同普通变化音）

### 7.2 纯律 vs 十二平均律

| 音程 | 纯律比例 | 纯律 cents | 平均律 cents | 差值 | 微分音修正 |
|------|---------|-----------|------------|------|----------|
| 大三度 | 5:4 | 386 | 400 | -14 | 可忽略 |
| 纯五度 | 3:2 | 702 | 700 | +2 | 可忽略 |
| 小七度 | 7:4 | 969 | 1000 | **-31** | ¼♭ |
| 增四度 | 11:8 | 551 | 600 | **-49** | ½♭ (≈eh) |
| 大六度 | 13:8 | 841 | 900 | **+41** | ¼♯ (≈ih) |
| 大七度 | 15:8 | 1088 | 1100 | -12 | 可忽略 |

> **关键**：第 7、11、13 泛音的偏差最大，是频谱音乐中微分音使用的主要来源

### 7.3 乐谱示例 — 泛音阶旋律（C4 基音）

以下旋律使用 C4 基音的泛音 8-16 构成音阶，包含第 11 泛音（¼♯F）和第 13 泛音（¼♯A）以及第 7 泛音区（¾♭B）。

```lilypond
\version "2.24.0"

\header {
  title = "泛音阶旋律"
  subtitle = "Harmonic Scale Melody — C4 基音泛音 8-16"
}

\score {
  \new Staff \relative c' {
    \clef treble
    \time 4/4
    \tempo "Andante espressivo" 4 = 72

    % 第 1 小节：泛音 8-12 上行（C D E F+¼♯ G）
    % 第 11 泛音 F# 偏低 49 cents → 用 fih（quarter-sharp）近似
    c'8 d' e' fih' g'
    g' fih' e' d' c'

    % 第 2 小节：泛音 12-16 继续上行（G A+¼♯ B♭ B C）
    % 第 13 泛音 A 偏高 41 cents → 用 aisih（three-quarter-sharp）近似
    g'8 aisih' bes' b' c''
    c'' b' bes' aisih' g'

    % 第 3 小节：下行到第 7 泛音区
    % 第 7 泛音 B♭ 偏低 31 cents → 用 beseh（three-quarter-flat）近似
    g'8 f' e' d'
    c' bes, a, g,

    % 第 4 小节：回归基音
    % 第 11 泛音 F（下方八度）使用 quarter-sharp
    f,8 g, a, c
    e, g, c,1
  }
  \layout { }
}
```

### 7.4 逐音分析

| 音 | 泛音号 | 实际频率 | 与平均律偏差 | LilyPond 写法 | 微分音标记 |
|----|--------|---------|------------|-------------|----------|
| C5 | 8 | 523 Hz | 0 | `c'` | 无 |
| D5 | 9 | 587 Hz | +4 | `d'` | 无（可忽略） |
| E5 | 10 | 654 Hz | -14 | `e'` | 无（可忽略） |
| F+¼♯ | 11 | 726 Hz | -49 | `fih'` | **quarter-sharp** |
| G5 | 12 | 784 Hz | +2 | `g'` | 无 |
| A+¼♯ | 13 | 880 Hz | +41 | `aisih'` | **three-quarter-sharp** |
| B♭-¼♭ | 14 | 930 Hz | -31 | `bes'`+eh | quarter-flat |
| B5 | 15 | 980 Hz | -12 | `b'` | 无 |
| C6 | 16 | 1047 Hz | 0 | `c''` | 无 |

### 7.5 微分音和弦构建原则

| 原则 | 说明 |
|------|------|
| 低音稀疏，高音密集 | 遵循泛音列的自然分布 |
| 微分音在高音区 | 7th/11th/13th 泛音的偏差主要影响高音区 |
| 避免微分音冲突 | 相邻音之间不超过 ¼ 音差异 |
| 基音锚定 | 低音持续基音，提供听觉参照点 |

---

## 八、频谱技法 (Spectral Techniques)

### 8.1 器乐合成 (Instrumental Synthesis)

**核心概念**：用原声乐器模拟电子音乐中的频谱效果。例如，让多个乐器各演奏泛音列中的一个泛音，合在一起"合成"出完整的频谱。

| 技术 | 实现方式 | 效果 |
|------|---------|------|
| 泛音分配 | 每件乐器演奏一个泛音 | 整体呈现泛音列结构 |
| 微分音修正 | 用 ¼ 音修正偏差泛音 | 接近纯律的"纯净"音色 |
| 动态分层 | 低音泛音强奏，高音泛音弱奏 | 模拟自然泛音的能量分布 |
| 音色融合 | 使用相似音色的乐器组 | 产生"单一声音"的错觉 |

### 8.2 频率调制 (Frequency Modulation)

在声学乐器上模拟 FM 合成：

- **颤音扩展**：从窄颤音（小二度）逐渐扩展到宽颤音（大七度）
- **差音暗示**：两件乐器演奏接近的频率，产生可闻的差拍（beat frequency）
- **环形调制模拟**：两件乐器的音程关系模拟载波×调制波的频谱

### 8.3 乐谱示例 — 8 小节频谱作品（弦乐四重奏）

以下作品展示"泛音列展开"过程：从大提琴的基音 C2 开始，各声部逐渐引入更高的泛音，最终在第 6 小节达到完整的频谱，然后收缩回归基音。

```lilypond
\version "2.24.0"

\header {
  title = "频谱展开"
  subtitle = "Spectral Unfolding — 弦乐四重奏"
}

\score {
  \new StaffGroup <<

    % 第一小提琴（高音泛音：5th, 12th, 16th partial）
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      \tempo "Très lent, lumineux" 4 = 48

      % 小节 1-3：静默（等待低音泛音建立）
      R1 R1 R1

      % 小节 4：引入第 5 泛音 E5（ppp，极弱进入）
      e''1\ppp^\markup { \small "5th partial" }

      % 小节 5：引入第 7 泛音 B♭5（偏低 31 cents → quarter-flat）
      <e'' beses''>1^\markup { \small "+7th partial (¼♭)" }

      % 小节 6：引入第 12 泛音 G5（完整频谱高点）
      <e'' g'' beses''>1^\markup { \small "+12th partial" }

      % 小节 7-8：收缩回归
      <e''>2\> <e''>4 <e''>8 r8 \!
      r1
    }

    % 第二小提琴（中高音泛音：4th, 6th partial）
    \new Staff \relative c'' {
      \clef treble

      % 小节 1-2：静默
      R1 R1

      % 小节 3：引入第 4 泛音 C5
      c''1\pp^\markup { \small "4th partial" }

      % 小节 4：保持
      c''1

      % 小节 5：引入第 6 泛音 G5
      <c'' g''>1^\markup { \small "+6th partial" }

      % 小节 6：保持
      <c'' g''>1

      % 小节 7-8：收缩
      c''2\> c''4 r4 \!
      R1
    }

    % 中提琴（中音泛音：2nd, 3rd partial + 微分音 11th）
    \new Staff \relative c' {
      \clef treble

      % 小节 1：静默
      R1

      % 小节 2：引入第 2 泛音 C4
      c'1\p^\markup { \small "2nd partial" }

      % 小节 3：引入第 3 泛音 G4
      <c' g'>1^\markup { \small "+3rd partial" }

      % 小节 4-5：保持
      <c' g'>1
      <c' g'>1

      % 小节 6：引入第 11 泛音（F+quarter-sharp，偏高音区）
      <c' g' fih''>1^\markup { \small "+11th partial (¼♯)" }

      % 小节 7-8：收缩
      <c' g'>2\> <c' g'>4 r4 \!
      R1
    }

    % 大提琴（基音 C2，持续全曲）
    \new Staff \relative c {
      \clef bass

      % 小节 1：基音 C2 单独进入
      c,1\mp^\markup { \small "Fundamental (1st partial)" }

      % 小节 2-8：持续基音（力度随整体变化）
      c,1
      c,1
      c,1
      c,1
      c,1\mf^\markup { \small "全频谱" }
      c,1\>
      c,1\pp \bar "|."
    }
  >>
  \layout { }
}
```

### 8.4 结构分析

| 小节 | 大提琴 | 中提琴 | 第二小提琴 | 第一小提琴 | 泛音覆盖 | 力度层 |
|------|--------|--------|-----------|-----------|---------|--------|
| 1 | C2 | — | — | — | 1 | mp |
| 2 | C2 | C4 | — | — | 1, 2 | p |
| 3 | C2 | C4+G4 | C5 | — | 1, 2, 3, 4 | p |
| 4 | C2 | C4+G4 | C5 | E5 | 1, 2, 3, 4, 5 | p |
| 5 | C2 | C4+G4 | C5+G5 | E5+B♭5(¼♭) | 1-7 | p |
| 6 | C2 | C4+G4+F♯(¼♯) | C5+G5 | E5+G5+B♭5(¼♭) | 1-12 | mf |
| 7 | C2 | 渐弱→静默 | 渐弱→静默 | 渐弱→静默 | 收缩 | > |
| 8 | C2 | — | — | — | 1（回归） | pp |

### 8.5 频谱作曲要点

| 要点 | 说明 |
|------|------|
| **基音锚定** | 低音声部持续基音，提供听觉参照 |
| **渐进展开** | 从低泛音到高泛音逐个引入，模拟声音的自然展开 |
| **微分音修正** | 第 7 泛音用 ¼♭，第 11 泛音用 ¼♯，第 13 泛音用 ¾♯ |
| **力度分层** | 低泛音较强（mf），高泛音较弱（pp），模拟自然泛音能量递减 |
| **收缩回归** | 高泛音先退出，最终只剩基音——如同声音的自然衰减 |
| **音簇效果** | 在高潮处密集排列所有泛音，产生"声音质量"（sound mass） |

### 8.6 频谱主义代表作品参考

| 作品 | 作曲家 | 核心技术 | 编制 |
|------|--------|---------|------|
| *Dérive* | Grisey | 泛音列和声 | 六重奏 |
| *Partiels* | Grisey | 器乐合成 | 管弦乐 |
| *Les Espaces Acoustiques* | Grisey | 频谱全过程 | 大型管弦乐 |
| *Gondwana* | Murail | 非谐频谱 | 管弦乐 |
| *Lichtbogen* | Saariaho | 音色频谱 | 室内乐 |
| *L'Esprit des dunes* | Saariaho | 电子+声学频谱 | 室内乐+电子 |

---

## 附录：极简主义与频谱主义的比较

| 维度 | 极简主义 | 频谱主义 |
|------|---------|---------|
| **核心材料** | 短小动机/模式 | 泛音列/频谱 |
| **时间观念** | 过程性（渐进变化） | 静态性（声音的"显微镜"） |
| **和声基础** | 自然音阶/调式 | 泛音列/微分音 |
| **节奏特征** | 稳定脉动/重复 | 自由节奏/无拍感 |
| **音高组织** | 十二平均律 | 纯律/微分音 |
| **代表人物** | Reich, Glass, Riley, Pärt | Grisey, Murail, Saariaho |
| **技术难度** | 演奏简单，合奏难 | 演奏极难（微分音） |
| **LilyPond 难点** | 反复结构/多声部 | 微分音标记/复杂节奏 |
