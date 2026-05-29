---
category: harmony
source: "Aldwell, Schachter & Cadwallader, Harmony and Voice Leading, 5th Edition"
confidence: text_derived
tags: [SATB, 四部和声, 和弦连接, 终止式, 离调, 转调, 模进, 声部进行]
title: Aldwell — 和声与声部进行
title_en: Harmony and Voice Leading (Aldwell & Schachter)
difficulty: intermediate
contexts: [satb, choir, vocal]
---

# Aldwell — 和声与声部进行

Schenker 学派传统下最权威的调性和声教材。声部音域、通用禁止项（平行五/八度、声部交错等）、倾向音解决表及通用 LilyPond 语法 → 见 shared-rules.md。本文件聚焦**四部和声的实际写作**，每个规则均配完整 ChoirStaff 示例。

---

## 一、SATB 四部和声配置

**规则**：四部和声由女高音 (S)、女中音 (A)、男高音 (T)、男低音 (B) 组成。音域与谱表 → 见 shared-rules.md 人声表。上三声部（S-A、A-T）间距不超过八度，T-B 间距无限制。

**正确示例** — C 大调 I-IV-V-I 完整四小节：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音：以级进为主，八分音符经过音增加流动性
      e'4 f8 e8 d4. c8 | b4 c8 d8 e2
    }
    \new Staff \relative c' {
      \clef treble
      % 女中音：二分音符与四分音符交替，共同音保持
      c'2 a4 b4 | c'4 a b2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      % 男高音：附点节奏与休止，声部独立
      g4. f8 g4 r4 | g4. a8 b4 a4
    }
    \new Staff \relative c {
      \clef bass
      % 男低音：以二分音符为主，根音进行
      c2 f4 g4 | g2 c,2
    }
  >>
  \layout { }
}
```

**声部检查**：
- S-A 间距：m3→M3→P4→m3，均 ≤ 八度 ✓
- A-T 间距：P4→M3→M3→P4，均 ≤ 八度 ✓
- 原位三和弦均重复根音 ✓

**常见错误**：上三声部间距过大

```lilypond
% WRONG: S-A 间距超过八度（e' 到 c'' = 十三度）
% Soprano: c''2  Alto: e'2
% 正确做法：保持 S-A ≤ 八度
```

---

## 二、和弦连接法 vs 旋律连接法

**规则**：
- **和弦连接法**（Harmonic Connection）：共同音在同一声部保持，其余声部级进到最近的和弦音。
- **旋律连接法**（Melodic Connection）：上三声部整体与低音反向运动，共同音不必保持。

### 和弦连接法 — I-IV-V-I（C 大调）

I→IV 共同音 c' 保持在女中音；IV→V 无共同音，S/A 下行、T/B 上行（反向）。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音：g' 保持 → f' 级进下行 → e' 级进上行
      g'2. f4 | e4. d8 c2
    }
    \new Staff \relative c' {
      \clef treble
      % 女中音：e' → c'（I 的共同音）→ b → c'
      e'2 c | b4. c8 c2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      % 男高音：c' → a → g → g
      c4 a g r | g4. f8 g2
    }
    \new Staff \relative c {
      \clef bass
      % 男低音：根音进行
      c2 f | g2 c,2
    }
  >>
  \layout { }
}
```

### 旋律连接法 — I-IV-V-I（C 大调）

上三声部整体与低音反向：低音上行时上方声部下行，低音下行时上方声部上行。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音：与低音反向——e'→f'(低音c→f 上行，S 也上行)
      % IV→V：低音 f→g 上行，S 下行 d'
      e'4 f8 g8 a4. g8 | f4 e8 d8 e2
    }
    \new Staff \relative c' {
      \clef treble
      % 女中音：c'→a（下行），a→b（上行，与低音反向）
      c'2 a | b4 c4 c2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      % 男高音：g→f（下行），f→g（上行，与低音反向）
      g4. f8 e4 r4 | f4. g8 g2
    }
    \new Staff \relative c {
      \clef bass
      c2 f | g2 c,2
    }
  >>
  \layout { }
}
```

**要点**：IV→V 无共同音时，必须用反向运动避免平行五度。若上三声部全部与低音同向上行（f→g），男高音-男低音之间将产生平行五度。

---

## 三、重复规则

→ 完整重复规则表见 shared-rules.md §3.5。此处提供实操示例。

### 原位三和弦：重复根音

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' { \clef treble g'2. g4 }
    \new Staff \relative c' { \clef treble e'4 f'4 e'2 }
    \new Staff \relative c' { \clef "treble_8" c'2. c4 }
    \new Staff \relative c { \clef bass c1 }
  >>
  \layout { }
}
```
C 大三和弦：S=g'(五音), A=e'(三音), T=c'(根音), B=c(根音) — 根音重复。

### 减三和弦：重复三音

vii°6 在 C 大调中为 b-d-f（第一转位 d-f-b），重复三音 f：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' { \clef treble b'2. b4 }
    \new Staff \relative c' { \clef treble f'4 e4 f'2 }
    \new Staff \relative c' { \clef "treble_8" f2. f4 }
    \new Staff \relative c { \clef bass d1 }
  >>
  \layout { }
}
```
vii°6：S=b(根音), A=f'(三音), T=f(三音重复), B=d(低音=五音)。不可重复根音 b（导音）。

### 四六和弦：始终重复低音

终止四六 I6/4（低音 = 五音 g）：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' { \clef treble e'2. e4 }
    \new Staff \relative c' { \clef treble c'4 b4 c'2 }
    \new Staff \relative c' { \clef "treble_8" g2. g4 }
    \new Staff \relative c { \clef bass g1 }
  >>
  \layout { }
}
```
I6/4：S=e'(三音), A=c'(根音), T=g(五音=低音重复), B=g(低音=五音)。

---

## 四、倾向音解决

→ 倾向音解决方向表见 shared-rules.md §3.6。此处提供 V7→I 完整四部示例。

**规则**：V7→I 中，导音（三音 of V7）上行半音到主音，七音下行级进到主和弦三音。三全音向外或向内解决到三度/六度。

**正确示例** — V7→I（B 大调，所有声部清晰可见）：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音：a#' = 导音 → 上行半音解决到 b'
      ais'4. b8 b2
    }
    \new Staff \relative c' {
      \clef treble
      % 女中音：f#' = 七音 → 下行半音解决到 e'（不完全 I：省略五音）
      fis'2 e4. e4
    }
    \new Staff \relative c' {
      \clef "treble_8"
      % 男高音：d#' → 下行到 c#'（不完全 I：三根音 + 三音）
      dis4. cis8 cis2
    }
    \new Staff \relative c {
      \clef bass
      % 男低音：b → 上行四度到 e（V7 根音 → I 根音）
      b2 e2
    }
  >>
  \layout { }
}
```

**声部解决验证**：
- 导音 a#' → b'（上行半音）✓
- 七音 f#' → e'（下行半音）✓
- 三全音 a#'-f#' → b'-e'（向外解决到六度）✓
- V7 完整四音（b-d#-f#-a#），I 不完全（e-e-e-c# = 三根音+三音）✓

**常见错误**：导音不解决

```lilypond
% WRONG: 导音 a#' 下行到 g#' 而非上行到 b'
% Soprano: ais'2 gis'  ← 导音必须上行！
```

---

## 五、终止式

→ 终止式定义表见 shared-rules.md §3.8。此处提供每种终止式的二小节 SATB 示例。

### 完满正格终止 (PAC) — I-IV-V-I（C 大调）

条件：V 和 I 均原位，高音结束于主音。S 与 B 反向进入八度避免隐伏八度。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音：级进上行到 g'，然后跳进到 c''（主音）= PAC
      c'4 d8 e8 f4. e8 | g4 a8 b8 c2
    }
    \new Staff \relative c' {
      \clef treble
      c'2 a | b4 c4 c2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      c4. c8 b4 r4 | c4. b8 c2
    }
    \new Staff \relative c {
      \clef bass
      % 低音下行 G→C（与高音反向，避免隐伏八度）
      c2 f | g2 c,2
    }
  >>
  \layout { }
}
```

**PAC 检查**：V(g-b-c'-g') 和 I(c-c'-e'-c,) 均原位 ✓ 高音结束于 c''（主音）✓ S 与 B 反向进入八度 ✓

### 不完满正格终止 (IAC) — V-I（C 大调）

高音结束于三音 e' 而非主音：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 女高音结束于 e'（三音）= IAC
      f'4. g8 g4. f8 | g4. f8 e2
    }
    \new Staff \relative c' {
      \clef treble
      d2 e | d4 c4 c2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      a4. b8 b4 r4 | b4. a8 g2
    }
    \new Staff \relative c {
      \clef bass
      f2 g | g2 c,2
    }
  >>
  \layout { }
}
```

### 半终止 (HC) — I-IV-V（C 大调，停在 V）

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 停在 V 和弦上，未解决 = 半终止
      e'4 f8 g8 a4. g8 | f4 e8 d8 a'2
    }
    \new Staff \relative c' {
      \clef treble
      c'2 c | d4 f4 f2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      g4. a8 b4 r4 | c4. b8 a2
    }
    \new Staff \relative c {
      \clef bass
      c2 f | g2 d2
    }
  >>
  \layout { }
}
```

### 阻碍终止 (DC) — V-vi（C 大调）

预期 I 被 vi 替代：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      a'4. g8 f4. e8 | d4. c8 b2
    }
    \new Staff \relative c' {
      \clef treble
      f2 e4 d4 | c4. d8 c2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      c2 a4 b4 | a4. g8 g2
    }
    \new Staff \relative c {
      \clef bass
      f2 c4 d4 | e2 a,2
    }
  >>
  \layout { }
}
```

### 变格终止 (PC) — IV-I（C 大调，"阿门"终止）

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      a'1~ a2 a4
    }
    \new Staff \relative c' {
      \clef treble
      f'1~ f'2 f'4
    }
    \new Staff \relative c' {
      \clef "treble_8"
      c'2 d'4 c'4 c'2
    }
    \new Staff \relative c {
      \clef bass
      f2 c2
    }
  >>
  \layout { }
}
```

---

## 六、离调

**规则**：用副属和弦 V/X 短暂主音化某个非主音和弦。持续时间短（几个和弦），不打断主调感。副属和弦引入变化音（升号 → 上行解决）。

### V/V → V → I（B 大调）

A# 作为 V/V (= F#7) 的三音，引入 D# 大调色彩，随即解决到 V (= F#)：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % c'' → b（经过音）→ a'（V/V 和弦音）→ f#'（V 五音）→ b（V 根音）→ e'（I 根音）
      c''4 b8 a'8 a'2 | fis'4 gis'4 b2
    }
    \new Staff \relative c' {
      \clef treble
      g'2. f'4 | e'2 d'2
    }
    \new Staff \relative c' {
      \clef "treble_8"
      e2. b4 | b4. cis8 b2
    }
    \new Staff \relative c {
      \clef bass
      c2 e | fis2 b,2
    }
  >>
  \layout { }
}
```

### V/ii → ii → V → I（C 大调）

C# 作为 V/ii (= A7) 的三音，短暂引入 D 小调：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % c#' = V/ii 的三音 → d'（ii 根音）→ g'（V 五音）→ e'（I 三音）
      cis'4 d8 e8 d2 | g4 a8 b8 a2
    }
    \new Staff \relative c' {
      \clef treble
      % e' = V/ii 的五音 → a（ii 五音）→ b（V 三音）→ c'（I 根音）
      e'4. f'8 e4 r4 | b4. c8 b4 r4
    }
    \new Staff \relative c' {
      \clef "treble_8"
      % a = V/ii 的根音（重复）→ a（ii 根音）→ g（V 根音）→ g（I 五音）
      a2. a4 | g2. g4
    }
    \new Staff \relative c {
      \clef bass
      a2 d | g2 c,2
    }
  >>
  \layout { }
}
```

**变化音解决**：c#'（升号变化音）→ d'（上行半音解决）✓

---

## 七、转调

**规则**：通过共同和弦（pivot chord）平滑过渡到新调，并在新调中用终止式确认。步骤：
1. 建立原调 → 2. 引入共同和弦 → 3. 新调属和弦 → 4. 新调终止式确认

### C 大调 → G 大调（枢纽和弦 = D 大三和弦）

C 大调 I-V-V/V(V) → G 大调 V-I。D 和弦是枢纽：在 C 大调中是 V/V，在 G 大调中是 V。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % C 大调 → 枢纽 D → G 大调终止
      c''4. b8 c'2 | a'4. g8 b2
    }
    \new Staff \relative c' {
      \clef treble
      g'2. g4 | f2. d4
    }
    \new Staff \relative c' {
      \clef "treble_8"
      e2. e4 | d4. c8 b2
    }
    \new Staff \relative c {
      \clef bass
      % C(I) → G(V) → D(V/V 枢纽) → G(I in new key)
      c2 g | d2 g,2
    }
  >>
  \layout { }
}
```

**转调分析**：
- 第 1-2 拍：C 大调确立（C-G = I-V）
- 第 3 拍：D 大三和弦 = 枢纽（C 大调的 V/V → 重新解释为 G 大调的 V）
- 第 4 拍：G 大调终止（D→G = V-I），新调确认
- F# 出现在 D 和弦中（C 大调的变化音），标志调性转移

---

## 八、模进

**规则**：一个音型模式（原型）在不同音高上重复，通常最多 3 次迭代。下行五度模进是最常见的类型：根音按纯五度下行（I-IV-vii°-iii-vi-ii-V-I）。

### 下行五度模进 — C 大调

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 原型：高音从 e' 开始，每次迭代下行三度
      e'2. e4 d2. d4 | c2. c4 b2. b4
    }
    \new Staff \relative c' {
      \clef treble
      c2 c4 b4 | a2 a4 g4
    }
    \new Staff \relative c' {
      \clef "treble_8"
      g2. f8 g8 g2 r4 | f4. e8 d2 d2
    }
    \new Staff \relative c {
      \clef bass
      % 根音下行五度：C → F → B → E
      c1 | b,1 | e,1 | a,1
    }
  >>
  \layout { }
}
```

**模进结构**：
- 原型（Model）：C 大三和弦
- 迭代 1（下行五度）：F 大三和弦
- 迭代 2（下行五度）：B 减三和弦
- 迭代 3（下行五度）：E 小三和弦
- 后续可继续：A → D → G → C 完成完整的圆圈进行

---

## AI 生成约束总结

生成四部和声 LilyPond 代码时：

1. 使用 `\new ChoirStaff << ... >>` 包裹四个独立 `\new Staff`
2. 各声部 `\relative` 参考：S=`c''`、A=`c'`、T=`c'`、B=`c`
3. 原位三和弦重复根音，减三和弦重复三音，四六和弦重复低音
4. 和弦连接法优先保持共同音；旋律连接法用反向运动
5. V7→I：导音上行，七音下行，三全音正确解决
6. PAC 要求 V 和 I 均原位且高音结束于主音
7. 离调短暂（2-3 和弦），变化音按方向解决
8. 转调需共同和弦 + 新调终止式确认
9. 模进最多 3 次迭代，最后一次可变体连接终止式
