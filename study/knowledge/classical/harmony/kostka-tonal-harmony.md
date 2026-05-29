---
category: harmony
source: "Kostka, Payne & Almen, Tonal Harmony, 8th Edition"
confidence: text_derived
tags: [数字低音, 功能和声, TSD, 和弦外音, 规则清单]
title: Kostka — 调性和声
title_en: Tonal Harmony (Kostka & Payne)
difficulty: beginner
contexts: [satb, choir, vocal, piano]
---

# Kostka — 调性和声

与 Aldwell 互补的分类式规则参考。Aldwell 解释"为什么"声部进行要这样走（Schenker 线条思维），Kostka 回答"什么规则"该遵守——分类清晰，对 AI 校验友好。声部音域、通用禁止项、倾向音解决 → 见 shared-rules.md。

---

## 一、数字低音

**规则**：数字标记低音上方的音程，指示和弦转位。无标记 = 原位三和弦 (5/3)。

| 标记 | 和弦形式 | 低音是和弦的 | LilyPond 实现 |
|------|---------|-------------|--------------|
| 无 / `5/3` | 原位三和弦 | 根音 | 直接写出四个和弦音 |
| `6` | 第一转位 | 三音 | 低音为三音，上方加三度+六度 |
| `6/4` | 第二转位 | 五音 | 低音为五音，上方加四度+六度，重复低音 |
| `7` | 原位七和弦 | 根音 | 四个音全出现（或省五音重根音） |
| `6/5` | 七和弦第一转位 | 三音 | 低音上方六度+五度 |
| `4/3` | 七和弦第二转位 | 五音 | 低音上方四度+三度 |
| `4/2` | 七和弦第三转位 | 七音 | 低音上方四度+二度，七音在低音 |

**正确示例** — 数字低音实现（C 大调 I-I6-I6/4-V7-I）：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c' {
      \clef treble
      % 右手：上方声部实现
      <e' g c>2 <e g c> <c g' c> <b d f> <c e g>
    }
    \new Staff \relative c {
      \clef bass
      % 左手：低音线（对应数字低音标记）
      % I(5/3)  I6    I6/4   V7     I
      c2 e g g c,
    }
  >>
  \layout { }
}
```

**要点**：I6/4 中低音 g 是五音，必须重复（shared-rules.md §3.5）。V7 的七音 f 必须下行解决到 e。

---

## 二、功能分组 (T / S / D)

**规则**：自然音和弦归入三大功能组，功能进行逻辑为 T→S→D→T。反功能 D→S 禁止。

| 功能 | 和弦 | 替代关系 |
|------|------|---------|
| **T（主功能）** | I | vi 可替代 I（弱主功能） |
| **S（下属功能）** | IV, ii | ii 常替代 IV；vi 具双重身份 |
| **D（属功能）** | V, vii° | vii° 是 V 的弱形式（省根音的 V7） |

**正确示例** — T→S→D→T 功能进行（C 大调 I-ii-V-I）：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      g'2 a g e
    }
    \new Staff \relative c' {
      \clef treble
      e2 f d c
    }
    \new Staff \relative c' {
      \clef "treble_8"
      c2 c b g
    }
    \new Staff \relative c {
      \clef bass
      c2 d g c,
    }
  >>
  \layout { }
}
```

**功能分析**：I(T) → ii(S) → V(D) → I(T)。ii 替代 IV 作为下属功能，低音级进下行 c-d 再到 g。

**常见错误**：反功能进行

```lilypond
% WRONG: V(D) → IV(S) = 反功能，在共同写作时期禁止
% Bass: g2 f  ← V 之后不可接 IV
```

---

## 三、声部进行规则清单

生成后逐项核验。详细定义 → 见 shared-rules.md §3.1-§3.7。

| # | 检查项 | 规则 | 参考 |
|---|--------|------|------|
| 1 | 平行五度 | 任两声部连续两个纯五度同向 | shared-rules §3.1 |
| 2 | 平行八度 | 任两声部连续两个纯八度同向 | shared-rules §3.1 |
| 3 | 声部交错 | S 低于 A，或 A 低于 T | shared-rules §3.1 |
| 4 | 声部超越 | 声部越过相邻声部前一个音的位置 | shared-rules §3.1 |
| 5 | S-A / A-T 间距 | 不超过八度 | shared-rules §3.7 |
| 6 | 导音解决 | 外声部导音必须上行到主音 | shared-rules §3.6 |
| 7 | 七音解决 | 和弦七音必须下行级进 | shared-rules §3.6 |
| 8 | 不当重复 | 不重复导音、不重复七音 | shared-rules §3.5 |
| 9 | 隐伏五/八度 | 外声部同向进入纯五/八度 + 高音跳进 | shared-rules §3.3 |
| 10 | 对斜 | 半音变化不可分散在两个声部间 | 本文件 |

**对斜示例**（半音变化应集中在同一声部）：

```lilypond
\version "2.24.0"
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 正确：c#' 到 d' 的半音变化在同一声部
      cis'2 d
    }
    \new Staff \relative c {
      \clef bass
      a2 d
    }
  >>
  \layout { }
}
```

```lilypond
% WRONG: 对斜——c' 在上方声部，c#' 在下方声部
% Soprano: c'2 d'  Alto: a2 cis'  ← 半音交错产生增一度
```

---

## 四、和弦外音

→ 和弦外音的合法性论证见 schoenberg-theory-of-harmony.md。此处聚焦识别与标记。

### 经过音 (Passing Tone, PT)

填充两个和弦音之间的级进音，出现在弱拍：

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % C 大调：e' 是和弦音，d' 是经过音（非和弦音），c' 是和弦音
    e'4 d c b | c1
  }
  \layout { }
}
```

### 辅助音 (Neighbor Tone, NT)

从和弦音级进到相邻音再返回，分上辅助音和下辅助音：

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % e' 是和弦音，f' 是上辅助音，e' 返回
    e'4 f e2 | e4 d e2
  }
  \layout { }
}
```

### 延留音 (Suspension)

前一和弦音延续到后一和弦（强拍），形成不协和后下行级进解决。三步骤：准备→延留→解决。

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 4-3 延留音：f'(准备) → f'(延留，与低音 g 形成四度) → e'(解决)
    f'2~ f4 e | c1
  }
  \layout { }
}
```

### 倚音 (Appoggiatura)

跳进到非和弦音（强拍），然后级进解决（通常反向）：

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % d' 跳进到 f'（倚音），然后级进解决到 e'
    d'4 f' e2 | c1
  }
  \layout { }
}
```

---

## AI 生成约束总结（Kostka 视角）

1. **功能逻辑**：T→S→D→T，禁止反功能 D→S
2. **数字低音**：根据标记实现上方声部，6/4 始终重复低音
3. **校验清单**：生成后逐项核验上述 10 条规则
4. **和弦外音**：经过音/辅助音在弱拍，延留音/倚音在强拍并级进解决
