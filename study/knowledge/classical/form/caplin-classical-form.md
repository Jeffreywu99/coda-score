---
category: form
source: "William Caplin, Classical Form: A Theory of Formal Functions, Oxford University Press, 1998"
confidence: text_derived
tags: [曲式, 形式功能, 乐句, 乐段, 奏鸣曲式, 回旋曲, 海顿, 莫扎特, 贝多芬, 动机发展]
title: Caplin — 古典曲式：形式功能理论
title_en: Classical Form — Formal Function Theory (Caplin)
difficulty: advanced
contexts: [piano, solo, composition]
---

# Caplin — Classical Form

以海顿、莫扎特、贝多芬的器乐作品为对象，提出**形式功能理论**（Formal Function Theory）。核心论断：音乐中的每一段都有一个时间上的语义功能——它是在**开始**、**中间**，还是在**收尾**。

→ 乐器音域、声部进行规则、终止式定义、通用禁止项等见 shared-rules.md

---

## 一、乐句 (Sentence)

**结构**：呈现句 (4小节) + 延续句 (4小节) = 8小节

```
呈现句 (Presentation, mm. 1-4)
  ├── 基本动机 (Basic Idea, 2 bars) — 主和弦延长
  └── 动机重复 (Repetition, 2 bars) — 精确重复或陈述-应答

延续句 (Continuation, mm. 5-8)
  ├── 碎片化 (Fragmentation, mm. 5-6) — 动机拆分 + 和声节奏加速
  └── 终止动机 (Cadential Idea, mm. 7-8) — 终止式进行
```

**调性布局**：I（主延长）→ 和声加速 → V-I（PAC）

**LilyPond 示例**：

```lilypond
\version "2.24.0"
\header { title = "乐句 (Sentence) — C大调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % 呈现句：基本动机 (1-2小节) — 主和弦延长，上行分解和弦
      c4 e g c'' |
      c'' g e c |
      % 呈现句：动机重复 (3-4小节) — 扩展至属和弦，半终止感
      c'4 e g b |
      d'2 c'2 |
      % 延续句：碎片化 (5-6小节) — 动机拆分，和声节奏加快
      c'2 e4 c |
      a'2 c,4 a |
      % 延续句：终止动机 (7-8小节) — ii-V-I 完满正格终止
      g'4 c, e' c |
      d'2 c'2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % 呈现句：I - I6 - I - V
      c4 g, c g, |
      e, b e b |
      c4 g c g |
      g, d' g d |
      % 延续句：IV - vi - V - I（和声节奏加速）
      f4 c' f c |
      a4 e' a e |
      g,4 d' g d |
      c4 g' c g |
    }
  >>
  \layout { }
}
```

**分析**：右手以 C 大三和弦分解上行开始（基本动机），随即分解下行（重复）。延续句将动机拆分为两音一组，配合和声从 IV→vi→V→I 的加速进行，最终在 V-I 上形成完满正格终止（PAC）。

---

## 二、乐段 (Period)

**结构**：前句 (4小节, HC) + 后句 (4小节, PAC) = 8小节

```
前句 (Antecedent, mm. 1-4)
  ├── 基本动机 (2 bars) — 主和弦区域
  └── 对比动机 (2 bars) → 半终止 (HC, 停在 V)

后句 (Consequent, mm. 5-8)
  ├── 基本动机返回 (2 bars) — 重述开头
  └── 对比动机变化 (2 bars) → 完满正格终止 (PAC, V-I)
```

**调性布局**：I → V (HC) ‖ I → I (PAC)

**LilyPond 示例**：

```lilypond
\version "2.24.0"
\header { title = "乐段 (Period) — C大调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % 前句：基本动机 (1-2小节)
      c4 d e f |
      g f e d |
      % 前句：对比动机 (3-4小节) → 半终止 HC
      c4 e g b |
      c'2 g2 |
      % 后句：基本动机返回 (5-6小节)
      c,4 d e f |
      g a b c' |
      % 后句：对比动机变化 (7-8小节) → 完满正格终止 PAC
      c'4 b a g |
      d'2 c'2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % 前句：I - V - I - V
      c4 g c g |
      g, d' g d |
      c4 g c g |
      g, d' g d |
      % 后句：I - vi - ii - V → I
      c4 g c g |
      a4 e' a e |
      f4 c' f c |
      g,4 d' g d |
    }
    % 注：后句最后一小节左手应为 V (G)，配合右手 PAC
    % 实际演奏中可添加 I 和弦的最后一拍
  >>
  \layout { }
}
```

**分析**：前句旋律从 C 音级进上行，终止于属和弦（HC，"问号"）。后句重述开头材料，但旋律继续上行至高点后回落，以 V-I 完满终止（PAC，"句号"）形成问答呼应。

---

## 三、二段体 (Binary)

**结构**：A(8小节) ‖ B(8小节) = 16小节

```
A段 (mm. 1-8)
  ├── 呈示 (4 bars) — 主调建立
  └── 转调 (4 bars) — 走向属调，以 PAC 结束在 V

B段 (mm. 9-16)
  ├── 展开/对比 (4 bars) — 属调或新调区域
  └── 回归 (4 bars) — 返回主调，PAC 结束
```

**调性布局**：A: I → V (PAC) ‖ B: V → I (PAC)

**LilyPond 示例**：

```lilypond
\version "2.24.0"
\header { title = "二段体 (Binary) — C大调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % === A段 (1-8小节) ===
      % A段呈示 (1-4小节) — C大调建立
      c4 e g c'' |
      c'' g e c |
      c'4 e g b |
      d'2 c'2 |
      % A段转调 (5-8小节) — 引入 F#，转至 G大调
      c'4 e g c'' |
      d,4 fis a d' |
      a'4 f d a |
      g'4 b d'' b' |
      \bar "||"
      % === B段 (9-16小节) ===
      % B段展开 (9-12小节) — G大调区域
      b4 d' g' d' |
      d4 g a b |
      a4 b c' e' |
      e'4 d c g |
      % B段回归 (13-16小节) — 返回C大调
      c,4 e g c'' |
      a4 c e a' |
      d'2 d' |
      c'2 c' |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % === A段 ===
      % C - C - C - G
      c4 g, c g, |
      e, b e b |
      c4 g c g |
      g, d' g d |
      % C - D - D7 - G (转调至G大调)
      c4 g, c g, |
      d4 a d a |
      fis4 c' fis c |
      g4 b d' b |
      \bar "||"
      % === B段 ===
      % G - D - C - G (G大调区域)
      g4 d' g d |
      d, a' d a |
      c4 g c g |
      g, d' g d |
      % C - Am - G - C (返回C大调)
      c4 g, c g, |
      a4 e' a e |
      g,4 d' g d |
      c4 g' c g |
    }
  >>
  \layout { }
}
```

**分析**：A 段前半巩固 C 大调，后半引入 F#（D 大调和弦 = V/V）转至 G 大调并以 PAC 结束。B 段在 G 大调展开后逐步返回 C 大调，最终以 V-I PAC 完成整体对称结构。

---

## 四、三段体 (Ternary)

**结构**：A(8小节) + B(8小节, 对比中部) + A'(8小节, 再现) = 24小节

```
A段 (mm. 1-8)   — 主调，紧凑型主题
B段 (mm. 9-16)  — 对比调（关系小调或下属方向），新材料
A'段 (mm. 17-24) — 主调再现（可带装饰变化）
```

**调性布局**：A: I → I ‖ B: vi → vi ‖ A': I → I

**LilyPond 示例**：

```lilypond
\version "2.24.0"
\header { title = "三段体 (Ternary) — C大调/A小调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % === A段 (1-8小节) — C大调 ===
      % 抒情主题 (1-4小节)
      c4 d e g |
      g f e c |
      c'4 d e a |
      a2 g2 |
      % 主题重复 (5-8小节)
      c,4 d e g |
      g a b c' |
      c'4 b a g |
      d'2 c'2 |
      % === B段 (9-16小节) — A小调，对比中部 ===
      \key a \minor
      % 新材料 (9-12小节) — 关系小调，下行旋律
      a'4 b c' e' |
      e'4 d c a |
      a4 b c' e' |
      e'4 d b g |
      % 属准备 (13-16小节) — E大调（a小调的V）引回A段
      c'4 e a c'' |
      b4 d gis b |
      a4 c' e a |
      b4 d gis e |
      \bar "||"
      % === A'段 (17-24小节) — C大调再现 ===
      \key c \major
      % 主题再现 (17-20小节)
      c,4 d e g |
      g f e c |
      c'4 d e a |
      a2 g2 |
      % 终止加固 (21-24小节)
      c,4 d e g |
      g a b c' |
      c'4 b a g |
      d'2 c'2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % === A段 ===
      % C - F - C - G
      c4 g c g |
      f, c' f c |
      c4 g c g |
      g, d' g d |
      % C - Am - G - C
      c4 g c g |
      a4 e' a e |
      g,4 d' g d |
      c4 g' c g |
      % === B段 — A小调 ===
      % Am - Dm - Am - E (a小调区域)
      a,4 e' a e |
      d4 a' d a |
      a,4 e' a e |
      e4 b' e b |
      % Am - E - Am - E (属准备，E大调)
      a,4 e' a e |
      e4 b' e b |
      a4 e' a e |
      e4 b' e b |
      \bar "||"
      % === A'段 — C大调再现 ===
      c4 g c g |
      f, c' f c |
      c4 g c g |
      g, d' g d |
      % C - Am - G - C
      c4 g c g |
      a4 e' a e |
      g,4 d' g d |
      c4 g' c g |
    }
  >>
  \layout { }
}
```

**分析**：A 段为 C 大调抒情主题，以级进旋律为特征。B 段转入 A 小调（关系小调），使用分解和弦式旋律形成对比，并在 13-16 小节通过 E 大调属和弦（a 小调的 V）准备再现。A' 段完整再现 A 段材料。

---

## 五、小步舞曲与三声中部 (Minuet & Trio)

**结构**：小步舞曲 (16小节二段体) + 三声中部 (16小节二段体) + Da Capo

```
Minuet (3/4拍, 主调)
  ├── A (8 bars): I → V (HC)
  └── B (8 bars): V → I (PAC)

Trio (3/4拍, 对比调)
  ├── A (8 bars): 对比调建立
  └── B (8 bars): 对比调 PAC

Da Capo: 重复 Minuet（不反复）
```

**调性布局**：Minuet: C大调 ‖ Trio: A小调 ‖ Da Capo: C大调

**LilyPond 示例**（Minuet 完整 + Trio 缩略）：

```lilypond
\version "2.24.0"
\header { title = "小步舞曲与三声中部 (Minuet & Trio)" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 3/4
      % === 小步舞曲 Minuet (C大调, 3/4拍) ===
      % A段 (1-8小节)
      c4 e g |          % 基本动机：上行分解
      c'' g e |         % 动机倒影：下行
      c'4 f a |         % 对比动机：IV级色彩
      d'2 c'4 |         % 收束至 HC
      c'4 e g |         % 动机重复
      c'' g e |         % 下行
      c'4 f a |         % IV级
      d'2 c'4 |         % PAC
      % B段 (9-16小节)
      g'4 d' g |        % 从属音区开始
      a4 d' a' |        % 上行至高点
      a4 g e |          % 回落
      c'2 a4 |          % 经过音
      a4 g e |          % 继续下行
      c'4 a c |         % 反弹
      f4 a c'' |        % 上行准备终止
      d'2 c'4 |         % PAC
      \bar "||"
      % === 三声中部 Trio (A小调, 缩略展示8小节) ===
      \key a \minor
      % A段开头 (1-4小节)
      a'4 b c' |        % 级进上行
      e' d c' |         % 级进下行
      a4 b d' |         % 上行跳进
      e'2 d'4 |         % HC
      % ... (B段省略，结构同上)
      % B段结尾 (回到 A小调 PAC)
      a4 c' e' |        % 上行
      a'4 e' c' |       % 高点回落
      e'4 d' b |        % 下行
      a'2 a4 |          % PAC
      \bar "||"
      % Da Capo al Fine → 重复 Minuet
    }
    \new Staff \relative c {
      \clef bass
      \time 3/4
      % === 小步舞曲 ===
      % A段
      c4 g, c |
      c4 g c |
      f,4 c' f |
      g,4 d' g |
      c,4 g c |
      c4 g c |
      f,4 c' f |
      g,4 d' g |
      % B段
      g4 d' g |
      d,4 a' d |
      c4 g c |
      a,4 e' a |
      f4 c' f |
      c4 g c |
      f,4 c' f |
      g,4 d' g |
      \bar "||"
      % === 三声中部 (缩略) ===
      \key a \minor
      a,4 e a |
      a,4 e a |
      d,4 a' d |
      e,4 b e |
      % ... (省略中间段落)
      a,4 e a |
      a,4 e a |
      e,4 b e |
      a,4 e a |
      \bar "||"
    }
  >>
  \layout { }
}
```

**分析**：小步舞曲为典雅的 3/4 拍舞曲，A 段以主-属交替建立 C 大调，B 段从属音区开始逐步回落。三声中部转入 A 小调，提供更暗的色彩对比。演奏时 Minuet → Trio → Minuet da capo（反复时不加反复记号）。

---

## 六、回旋曲式 (Rondo)

**结构**：A(4小节) → B(4小节) → A(4小节) → C(4小节) → A(4小节) = 20小节

```
叠部 A (Refrain) — 主调，性格鲜明的主题
插部 B (Episode 1) — 属调，对比材料
叠部 A — 主调原样再现
插部 C (Episode 2) — 关系小调/下属调，更深对比
叠部 A — 主调最终再现
```

**调性布局**：A(I) → B(V) → A(I) → C(vi) → A(I)

**LilyPond 示例**：

```lilypond
\version "2.24.0"
\header { title = "回旋曲式 (Rondo) — C大调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % === 叠部 A (1-4小节) — C大调主题 ===
      c4 e' c g' |      % 跳跃式主题
      e' c' e g |       % 回落
      c4 e' c g' |      % 重复
      d'2 c'2 |         % 终止
      % === 插部 B (5-8小节) — G大调 ===
      \key g \major
      d'4 e fis a' |    % 级进上行（新旋律）
      a' g fis e |      % 下行
      g4 e c' e |       % 混合进行
      d'2 a2 |          % HC in G
      % === 叠部 A (9-12小节) — C大调再现 ===
      \key c \major
      c4 e' c g' |
      e' c' e g |
      c4 e' c g' |
      d'2 c'2 |
      % === 插部 C (13-16小节) — A小调 ===
      \key a \minor
      e'4 c e a' |      % 小调色彩
      e'4 d b g, |      % 下行
      c'4 e' g' c'' |   % 上行高潮
      a'4 e' c' a' |    % 回落
      \bar "||"
      % === 叠部 A (17-20小节) — C大调最终再现 ===
      \key c \major
      c,4 e' c g' |
      e' c' e g |
      c4 e' c g' |
      d'2 c'2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % === 叠部 A ===
      c4 g, c g, |
      c4 g c g |
      c4 g, c g, |
      g4 d' g d |
      % === 插部 B (G大调) ===
      \key g \major
      g,4 d' g d |
      d, a' d a |
      c4 g c g |
      g,4 d' g d |
      % === 叠部 A ===
      \key c \major
      c4 g, c g, |
      c4 g c g |
      c4 g, c g, |
      g4 d' g d |
      % === 插部 C (A小调) ===
      \key a \minor
      a,4 e' a e |
      d4 a' d a |
      e4 b' e b |
      e,4 b' e b |
      \bar "||"
      % === 叠部 A ===
      \key c \major
      c4 g, c g, |
      c4 g c g |
      c4 g, c g, |
      c4 g' c g |
    }
  >>
  \layout { }
}
```

**分析**：叠部 A 以跳跃式分解和弦为特征（C-E-C-G），每次回归都在 C 大调上确认。插部 B 在 G 大调上以级进旋律形成对比。插部 C 转入 A 小调，用更暗的色彩和上行高潮提供最大对比，随后叠部最终再现完成对称结构。

---

## 七、奏鸣曲式骨架 (Sonata Form)

**结构**：呈示部(8小节) → 展开部(4小节) → 再现部(8小节) → 尾声(4小节) = 24小节

```
呈示部 (Exposition)
  ├── 主部 P (4 bars) — 主调，紧凑型主题
  └── 副部 S + 结束部 C (4 bars) — 属调，PAC 确认

展开部 (Development)
  └── 核心 (4 bars) — 模进/碎片化，属持续音准备再现

再现部 (Recapitulation)
  ├── 主部 P (4 bars) — 主调
  └── 副部 S (4 bars) — 主调（不再转调）

尾声 (Coda)
  └── (4 bars) — 主调加固，最终 PAC
```

**调性布局**：呈示部 I→V ‖ 展开部 G持续音 ‖ 再现部 全I ‖ 尾声 I

**LilyPond 示例**（教学简化版）：

```lilypond
\version "2.24.0"
\header { title = "奏鸣曲式骨架 (Sonata Form) — C大调" }
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % ======= 呈示部 EXPOSITION =======
      % 主部 P (1-4小节) — C大调，紧凑型主题
      c4 e g c'' |      % 主部动机：上行分解
      c'' g e c |       % 下行回应
      c'4 e g b |       % 扩展
      d'2 c'2 |         % 半终止感
      % 副部 S + 结束部 C (5-8小节) — G大调
      \key g \major
      d'4 e' b' d' |    % 副部主题：D大调上开始（新调区）
      b'4 d'' g'' d'' | % 上行至高音区
      g'4 b d'' b' |    % G大调确认
      d'4 b g' b, |     % 结束部：回落
      \bar "||"
      % ======= 展开部 DEVELOPMENT =======
      \key c \major
      % 核心 (9-12小节) — 模进下行，属持续音(G pedal)
      f'4 e c a |       % 碎片化：动机拆分
      b4 d g d |        % 模进
      g4 a c' d |       % 上行准备
      b4 g d' b |       % 回落至属和弦
      \bar "||"
      % ======= 再现部 RECAPITULATION =======
      % 主部 P (13-16小节) — C大调
      c,4 e' g c' |     % 主题再现
      c'4 g e c |       % 下行
      c'4 e g b |       % 扩展
      d'2 c'2 |         % 过渡
      % 副部 S (17-20小节) — 移至C大调（不再转调！）
      g4 a b d' |       % 副部在主调上
      d'4 a g b |       % 回应
      c'4 e' g' c'' |   % 上行
      a'4 e' c' a' |    % 回落
      \bar "||"
      % ======= 尾声 CODA =======
      % (21-24小节) — 加固主调
      c'4 d' c' d' |    % 持续音型
      c'4 d' c' d' |    % 重复
      d'2 c'2 |         % 终止式
      c'1 |             % 最终主音
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % ======= 呈示部 =======
      % 主部：C大调
      c4 g, c g, |
      e, b e b |
      c4 g c g |
      g, d' g d |
      % 副部：转至G大调
      \key g \major
      d4 a d a |
      g,4 d' g d |
      g4 b d' b |
      g,4 b d' b |
      \bar "||"
      % ======= 展开部 =======
      \key c \major
      % G pedal（属持续音）
      g,1 |
      g1 |
      g1 |
      g1 |
      \bar "||"
      % ======= 再现部 =======
      % 主部：C大调
      c4 g, c g, |
      c4 g c g |
      c4 g c g |
      g,4 d' g d |
      % 副部：C大调（不再转调）
      g,4 d' g d |
      g,4 d' g d |
      c4 g' c g |
      c,4 g' c g |
      \bar "||"
      % ======= 尾声 =======
      c4 g, c g, |
      c4 g, c g, |
      g4 d' g d |
      c,1 |
    }
  >>
  \layout { }
}
```

**分析**：呈示部主部在 C 大调建立紧凑主题，副部转至 G 大调以新旋律材料对比。展开部使用 G 持续音（dominant pedal）营造张力，通过动机碎片化推进。再现部的关键变化：副部**移至 C 大调**，不再转调——这是奏鸣曲式的核心原则。尾声以主持续音加固终止。

---

## 八、动机发展技法

以一个简单动机（C-E-G，上行大三度+小三度）演示六种核心发展技法。

```
原型动机：C5 E5 G5（上行分解三和弦）

技法一览：
├── 重复 (Repetition) — 原样或移调重复
├── 模进 (Sequence) — 移至不同音级
├── 倒影 (Inversion) — 音程方向反转
├── 逆行 (Retrograde) — 时间反序
├── 扩大 (Augmentation) — 时值加倍
└── 缩小 (Diminution) — 时值减半
```

**LilyPond 示例**（各技法依次展示，每段独立的相对八度上下文）：

```lilypond
\version "2.24.0"
\header { title = "动机发展技法 (Motivic Development)" }

% ① 原型 (Original) — C-E-G 上行分解三和弦
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 c'4 e g r | c'4 e g r | }
    \new Staff \relative c { \clef bass \time 4/4 c4 g' c g, | c4 g' c g, | }
  >>
  \layout { }
}

% ② 重复 (Repetition) — 移至G音级
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 g4 b d' r | g4 b d' r | }
    \new Staff \relative c { \clef bass \time 4/4 g,4 d' g d, | g,4 d' g d, | }
  >>
  \layout { }
}

% ③ 模进 (Sequence) — 依次上移
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 d4 fis a r | e4 g b r | }
    \new Staff \relative c { \clef bass \time 4/4 d,4 a' d a, | e,4 b' e b, | }
  >>
  \layout { }
}

% ④ 倒影 (Inversion) — 音程方向反转 (C-Ab-F)
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 c'4 aes f r | c'4 aes f r | }
    \new Staff \relative c { \clef bass \time 4/4 f,4 c' f c, | f,4 c' f c, | }
  >>
  \layout { }
}

% ⑤ 逆行 (Retrograde) — 时间反序 (G-E-C)
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 g'4 e c r | g'4 e c r | }
    \new Staff \relative c { \clef bass \time 4/4 c4 g c g, | c4 g c g, | }
  >>
  \layout { }
}

% ⑥ 扩大 (Augmentation) — 时值加倍
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4 c'2 e2 | g'2 c''2 | }
    \new Staff \relative c { \clef bass \time 4/4 c,2 g'2 | c2 g'2 | }
  >>
  \layout { }
}

% ⑦ 缩小 (Diminution) — 时值减半
\score {
  \new PianoStaff <<
    \new Staff \relative c'' { \clef treble \time 4/4
      c'8 e g c' e' g' e' c' | c'4 r r2 | }
    \new Staff \relative c { \clef bass \time 4/4
      c,8 g' c g, c8 g' c g, | c,4 r r2 | }
  >>
  \layout { }
}
```

**各技法说明**：

| 技法 | 中文 | 操作 | 效果 |
|------|------|------|------|
| Repetition | 重复 | 原样重复或移至其他音级 | 巩固动机印象 |
| Sequence | 模进 | 保持音程关系，逐级移位 | 产生方向感与动力 |
| Inversion | 倒影 | 上行变下行，下行变上行 | 创造"镜像"对比 |
| Retrograde | 逆行 | 从尾到头反序演奏 | 隐藏动机来源，增加统一性 |
| Augmentation | 扩大 | 时值成倍增加 | 拉宽线条，增加庄严感 |
| Diminution | 缩小 | 时值成倍缩减 | 加速推进，增加紧张度 |

---

## 九、AI 生成约束总结

1. **语义-语法分离**：先决定"这里在做什么"（开始的陈述？中间的过渡？结束的收束？），再配和声
2. **乐句结构**：呈现（2+2）→ 延续（碎片化+终止动机），标准 8 小节
3. **乐段结构**：前句 HC → 后句 PAC，问答呼应
4. **奏鸣曲呈示部**：P(主调) → TR(转调) → S(从属调+PAC) → C(加固)
5. **再现原则**：所有材料回归主调——奏鸣曲式的核心
6. **松散 vs 紧凑**：主部应是紧凑型主题，连接部和副部可以松散化
7. **动机一致性**：所有段落材料应源于主部主题或与之相关
8. **调性对称**：回旋曲式每次叠部回归主调，插部在对比调
9. **三段体对比**：B 段必须在调性、材料、性格上与 A 段形成明确对比
10. **终止式层级**：HC 用于"未完成"的位置，PAC 用于段落/全曲结束
