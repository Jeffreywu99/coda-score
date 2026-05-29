---
category: modernist
source: "Modernist notation conventions - 20th/21st century practice"
confidence: text_derived
tags: [微分音, 现代记谱, 复杂节拍, 现代力度, 无调性, 多节奏]
title: 现代音乐通用规则
title_en: Modernist Music - Common Rules and Conventions
difficulty: advanced
contexts: [modernist, contemporary, avant-garde, experimental]
---

# 现代音乐通用规则 - Modernist Common Rules

本文件涵盖 20/21 世纪现代音乐特有的记谱规范。基础 LilyPond 语法见 `syntax/lilypond-core-syntax.md`，乐器音域及通用规范见 `classical/common/shared-rules.md`。

**关键区别**：古典分支的许多"禁止项"在现代音乐中是允许的（如平行五度、不协和音不解决），但乐器物理极限和 LilyPond 语法规则仍然适用。

---

## 一、微分音记谱 (Microtonal Notation)

### 1.1 LilyPond 内置微分音变音记号

LilyPond 支持四分之一音（quarter-tone）和四分之三音（three-quarter-tone）的变音记号：

| 后缀 | 含义 | 音高偏移 | 示例 |
|------|------|---------|------|
| `-ih` | 四分之一升号 (1/4 sharp) | +50 音分 | `cisih` = C  quarter-tone sharp |
| `-eh` | 四分之一降号 (1/4 flat) | -50 音分 | `deh` = D quarter-tone flat |
| `-isih` | 四分之三升号 (3/4 sharp) | +150 音分 | `cisih` = C three-quarter sharp |
| `-eseh` | 四分之三降号 (3/4 flat) | -150 音分 | `deeseh` = D three-quarter flat |

> **注意**：微分音后缀直接附加在音名后，不需要空格。`cisih` = C + is(升) + ih(半升) = 四分之三升 C。

### 1.2 微分音旋律示例

```lilypond
\version "2.24.0"

\header {
  title = "微分音旋律示例"
  composer = "现代记谱演示"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 四分之一音变化：C → C+1/4升 → C+1/2升(=Cis) → C+3/4升 → D
    c4 cih cis cisih
    % 四分之一音降号：D → D-1/4降 → D-1/2降(=Des) → D-3/4降 → C
    d4 deh des deseh
    % 回到自然音
    c2 r2
  }
  \layout { }
}
```

### 1.3 常见错误：微分音后缀拼写

```lilypond
% 错误：后缀顺序颠倒
\score {
  \new Staff \relative c'' {
    c4 chis cish    % chis 和 cish 都不是合法后缀！
  }
  \layout { }
}

% 正确：先写变音后缀(is/es)，再写微分音后缀(ih/eh)
\score {
  \new Staff \relative c'' {
    c4 cisih cih deseh    % 正确拼写
  }
  \layout { }
}
```

### 1.4 阿拉伯音阶 (Arabic Maqam)

LilyPond 内置阿拉伯音阶支持，包含 24 音等分律：

```lilypond
\version "2.24.0"

\header {
  title = "阿拉伯 Bayati 音阶"
}

% 引入阿拉伯音阶定义
\include "arabic.ly"

\score {
  \new Staff \relative c'' {
    \time 4/4
    % Bayati 音阶：D E-half-flat F G A Bb C D
    d4 dehh f g
    a bes c d
    d c bes a
    g f dehh d2
  }
  \layout { }
}
```

> `arabic.ly` 提供了 `dehh`(D half-flat)、`aehh`(A half-flat) 等快捷别名。引入后可以直接使用阿拉伯音名。

### 1.5 自定义律制

对于非标准律制，可以通过 Scheme 函数自定义音高偏移：

```lilypond
\version "2.24.0"

\header {
  title = "自定义微分音 - 六分之一音"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 使用 tweak 调整音高偏移（单位：半音）
    % 六分之一音 = 1/3 半音 = 33.3 音分
    c4
    -\tweak staff-position 0.3 d
    -\tweak staff-position 0.6 e
    f
    -\tweak staff-position -0.3 g
    a2
  }
  \layout { }
}
```

> 对于复杂微分音系统（如 Harry Partch 43 音律），通常需要自定义字体和音名映射，超出基础 LilyPond 范围。

---

## 二、现代拍号 (Modern Time Signatures)

### 2.1 附加拍号 (Additive Meters)

现代音乐常用不规则分组拍号，如 2+2+2+3/8。LilyPond 用 Scheme 列表指定分组：

```lilypond
\version "2.24.0"

\header {
  title = "附加拍号 2+2+2+3/8"
}

\score {
  \new Staff \relative c'' {
    % 2+2+2+3/8：每组分别加小节线
    \time #'((2 2 2 3) . 8)
    c8 d | e f | g a | bes a g |
    c8 d | e f | g a | bes a g |
  }
  \layout { }
}
```

> `\time #'((2 2 2 3) . 8)` 中，`#'(...)` 是 Scheme 列表，表示将 9/8 拍分为 2+2+2+3 四个子组。LilyPond 会自动在子组间显示虚线小节线。

### 2.2 常见错误：附加拍号语法

```lilypond
% 错误：缺少 Scheme 引号
\score {
  \new Staff \relative c'' {
    \time ((2 2 3) . 8)    % 编译错误！缺少 #'
    c8 d e f g
  }
  \layout { }
}

% 正确：必须加 #' 前缀
\score {
  \new Staff \relative c'' {
    \time #'((2 2 3) . 8)   % 正确
    c8 d e f g
  }
  \layout { }
}
```

### 2.3 复合节拍（多节拍记谱, Polymetric Notation）

不同声部使用不同拍号时，需要启用独立拍号模式：

```lilypond
\version "2.24.0"

\header {
  title = "多节拍记谱 - 3/4 对 2/4"
}

\score {
  \new PianoStaff <<
    % 启用每个谱表独立计拍
    \set PianoStaff.followVoice = ##t

    \new Staff \relative c'' {
      \time 3/4
      c4 d e
      f g a
    }
    \new Staff \relative c' {
      \time 2/4
      g4 a
      b c
      d e
    }
  >>
  \layout {
    \context {
      \Score
      % 关闭全系统小节线对齐，允许各声部独立
      \remove "Default_bar_line_engraver"
    }
    \context {
      \Staff
      % 在每个谱表上独立绘制小节线
      \consists "Default_bar_line_engraver"
    }
  }
}
```

> **关键**：多节拍记谱需要在 `\layout` 中将小节线绘制从 `\Score` 移到 `\Staff` 层级，否则两个声部的小节线会被强制对齐。

### 2.4 无小节线（散拍/Cadenza）

用于无固定节拍的段落（如华彩段、自由即兴）：

```lilypond
\version "2.24.0"

\header {
  title = "散拍记谱 - Cadenza"
}

\score {
  \new Staff \relative c'' {
    % 有拍号段落
    \time 4/4
    c4 d e f

    % 进入散拍：取消拍号和小节线
    \cadenzaOn
    c4 d8 e f4. g8 a4 b c2
    d8 e f g a4 b c2
    % 散拍中不自动插入小节线，需要手动 \bar "|" 如需分隔

    % 退出散拍：恢复正常节拍
    \cadenzaOff
    \time 4/4
    c4 d e f
  }
  \layout { }
}
```

> `\cadenzaOn` 关闭自动小节线，`\cadenzaOff` 恢复。散拍段落中 LilyPond 不会自动检查每小节时值是否对齐。

### 2.5 比例记谱 (Proportional Notation)

现代音乐中，音符的水平间距可以反映实际时间比例：

```lilypond
\version "2.24.0"

\header {
  title = "比例记谱"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 比例记谱：音符水平间距与时值成正比
    \set Score.proportionalNotationDuration = #(ly:make-moment 1/8)
    c1
    c2 c4 c8 c16 c32 c64
  }
  \layout { }
}
```

> `proportionalNotationDuration` 设定基准时值。设为 `1/8` 表示每个八分音符占据相同的水平空间。适合需要精确时间对应的现代作品。

---

## 三、现代调号替代方案 (Alternative Key Signatures)

### 3.1 无调性：隐藏调号

无调性音乐（atonal）不显示任何调号，但临时记号在每小节后自动取消：

```lilypond
\version "2.24.0"

\header {
  title = "无调性记谱 - 隐藏调号"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 隐藏调号（不显示任何调号标记）
    \omit Staff.KeySignature

    % 每个变化音都需要写临时记号
    c4 cis d ees
    e f fis gis
    a bes b c
    cis d ees e
  }
  \layout { }
}
```

> `\omit Staff.KeySignature` 隐藏调号显示，但不影响音高。所有变化音必须在每个音符前显式标注临时记号。

### 3.2 泛调性：显示所有升号或所有降号

某些现代作品使用"全升号"或"全降号"作为调号，暗示泛调性 (pan-tonal)：

```lilypond
\version "2.24.0"

\header {
  title = "自定义调号 - 全升号"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 自定义调号：使用 Scheme 指定每个音级的变音
    % 格式：#'(音级列表 八度列表)
    % 显示所有升号的调号（F# C# G# D# A# E# B#）
    \set Staff.keyAlterations =
      #'(((0 . 1) (1 . 1) (2 . 1) (3 . 1) (4 . 1) (5 . 1) (6 . 1)) . ())
    c4 d e f
    g a b c
  }
  \layout { }
}
```

> 这是高级用法。`keyAlterations` 接受 Scheme 数据结构，格式为 `((半音变化列表) . (八度偏移列表))`。每个变化项为 `(音级 . 变音值)`，其中变音值 `1` = 升号，`-1` = 降号，`1/2` = 四分之一升。

### 3.3 自定义调号排列

现代作品可能使用非传统调号排列（如 Bartok 式调号）：

```lilypond
\version "2.24.0"

\header {
  title = "自定义调号 - Bartok 式"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 自定义调号：F# 和 Bb 同时出现（非传统调号）
    \set Staff.keyAlterations =
      #'(((5 . 1) (1 . -1)) . ())
    c4 d e f
    g a b c
  }
  \layout { }
}
```

### 3.4 常见错误：`\key` 与 `\omit` 的冲突

```lilypond
% 错误：同时设置调号又隐藏调号，逻辑混乱
\score {
  \new Staff \relative c'' {
    \key d \major            % 设置了 D 大调
    \omit Staff.KeySignature % 又隐藏了调号
    c4 d e f                 % LilyPond 仍按 D 大调解释音高！
  }
  \layout { }
}

% 正确做法 A：无调性 —— 不设置 \key，直接隐藏
\score {
  \new Staff \relative c'' {
    \omit Staff.KeySignature
    c4 cis d ees             % 每个音明确标注
  }
  \layout { }
}

% 正确做法 B：有调号 —— 设置 \key，正常显示
\score {
  \new Staff \relative c'' {
    \key d \major
    c4 d e fis               % F 自动变为 F#（调号规定）
  }
  \layout { }
}
```

---

## 四、现代谱号使用 (Modern Clef Usage)

### 4.1 打击乐谱号

无固定音高的打击乐器使用打击乐谱号，音符位置代表不同乐器而非音高：

```lilypond
\version "2.24.0"

\header {
  title = "打击乐谱号"
}

\score {
  \new DrumStaff {
    \drummode {
      \time 4/4
      % bd = 低音鼓, sn = 小军鼓, hh = 踩镲, cymc = 碎音镲
      bd4 sn bd sn
      hh8 hh hh hh cymc4 sn
      bd8 sn bd sn bd4 sn
    }
  }
  \layout { }
}
```

> `\drummode` 中使用打击乐专用音名（`bd`, `sn`, `hh` 等），不是普通音名。完整列表见 LilyPond 手册 "Percussion notes" 章节。

### 4.2 吉他六线谱 (TabStaff)

```lilypond
\version "2.24.0"

\header {
  title = "吉他六线谱"
}

\score {
  <<
    % 五线谱
    \new Staff \relative c' {
      \clef "treble_8"
      \time 4/4
      e4 a d' g
      b e' r2
    }
    % 六线谱（自动转换指法位置）
    \new TabStaff \relative c' {
      e4 a d' g
      b e' r2
    }
  >>
  \layout { }
}
```

> `TabStaff` 自动将音符映射到吉他品格位置。可以并用五线谱和六线谱。

### 4.3 谱号中途切换

现代音乐中频繁切换谱号是常见的（如大提琴在高音区使用高音谱号）：

```lilypond
\version "2.24.0"

\header {
  title = "谱号中途切换 - 大提琴"
}

\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    % 低音区使用低音谱号
    c4 d e f
    g a b c'

    % 进入高音区，切换到次中音谱号
    \clef tenor
    d'4 e f g
    a b c d

    % 更高音区，切换到高音谱号
    \clef treble
    e'4 f g a

    % 回到低音区
    \clef bass
    c,4 d e f
  }
  \layout { }
}
```

### 4.4 常见错误：谱号切换后忘记调整 `\relative` 参照

```lilypond
% 错误：切换谱号后，\relative 的八度推断可能偏移
\score {
  \new Staff \relative c {
    \clef bass
    c4 d e f
    \clef treble
    g a b c        % 音符仍在低音区八度！谱号变了但音高没变
  }
  \layout { }
}

% 正确：用 \absolute 或显式八度标记确保音高正确
\score {
  \new Staff {
    \clef bass
    \relative c { c4 d e f }
    \clef treble
    \relative c'' { g4 a b c }    % 新的 \relative 块，独立设定八度
  }
  \layout { }
}
```

> 谱号改变**不影响音高**，只影响显示位置。但 `\relative` 的八度推断可能让实际音高偏离预期。建议在谱号切换处开启新的 `\relative` 块。

---

## 五、现代力度与表情 (Modern Dynamics)

### 5.1 极端力度

LilyPond 内置支持 `\ppppp`（五个 p）到 `\fffff`（五个 f）：

```lilypond
\version "2.24.0"

\header {
  title = "极端力度记号"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 从极弱到极强
    c4\ppppp c\pppp c\ppp c\pp
    c4\p c\mp c\mf c\f
    c4\ff c\fff c\ffff c\fffff
  }
  \layout { }
}
```

### 5.2 文字力度说明

现代作品常在力度记号旁附加文字说明：

```lilypond
\version "2.24.0"

\header {
  title = "文字力度"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 力度 + 文字说明
    c4^\markup { \italic "ppp possibile" }
    c^\markup { \italic "fff con forza" }
    c^\markup { \italic "quasi niente" }
    c^\markup { \italic "morendo" }

    % 自定义力度文字（代替传统 pp/ff）
    c4-\tweak self-alignment-X #LEFT
       ^\markup { \bold "sempre ppp" }
    c c c
  }
  \layout { }
}
```

### 5.3 力度曲线与突变

现代音乐中力度变化更加极端和突然：

```lilypond
\version "2.24.0"

\header {
  title = "现代力度变化"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 极端的力度对比（不经过渐强渐弱）
    c4\ppp c\fff c\ppp c\fff

    % 超短渐强（sforzando 后立刻弱）
    c4-\markup { \dynamic "sfpp" }
    c-\markup { \dynamic "sffp" }
    c-\markup { \dynamic "sfp" }
    c-\markup { \dynamic "fp" }

    % 极长的渐强跨越多个小节
    c4\ppp\< c c c | c c c c | c c c c\ffff\!
  }
  \layout { }
}
```

### 5.4 自定义力度符号

```lilypond
\version "2.24.0"

\header {
  title = "自定义力度符号"
}

% 定义自定义力度
niente = #(make-dynamic-script
           (markup #:normal-text #:italic "n"))

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 使用自定义力度 "n" (niente = 无声)
    c4\niente c\< c c\p
    c4\> c c c\niente\!
  }
  \layout { }
}
```

---

## 六、现代节奏技法 (Modern Rhythmic Techniques)

### 6.1 复杂连音符与嵌套连音符

现代音乐中常见多层嵌套的连音符：

```lilypond
\version "2.24.0"

\header {
  title = "嵌套连音符"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 五连音内嵌三连音：5 个音的时值内，前 3 个音再分成三连音
    \tuplet 5/4 {
      \tuplet 3/2 { c8 d e }
      f8 g
    }
    % 七连音
    \tuplet 7/4 { c8 d e f g a b }
    % 十一连音
    \tuplet 11/8 { c16 d e f g a b c d e f }
  }
  \layout { }
}
```

### 6.2 非有理时值 (Irrational Durations)

LilyPond 支持时值缩放，用于表示非标准时值比例：

```lilypond
\version "2.24.0"

\header {
  title = "非有理时值"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 时值缩放：c4*2/3 = 四分音符的 2/3 时值
    c4*2/3 d*2/3 e*2/3    % 三个音共占 2 拍（= 2 个四分音符）
    f4 g2                  % 回到正常时值

    % 更复杂的比例
    c4*3/4 d*3/4 e*3/4 f*3/4   % 四个音共占 3 拍
    g1
  }
  \layout { }
}
```

> 时值缩放 `*N/M` 不显示连音标记，仅改变实际时长。适合 Ferneyhough 式复杂节奏记谱。如果需要显示连音数字，应使用 `\tuplet`。

### 6.3 羽化连音 (Feathered Beams)

羽化连音表示渐快或渐慢的连音组，梁的间距逐渐变化：

```lilypond
\version "2.24.0"

\header {
  title = "羽化连音 - 渐快与渐慢"
}

\score {
  \new Staff \relative c'' {
    \time 4/4
    % 渐快羽化连音（accelerando）
    \override Beam.grow-direction = #RIGHT
    \featherDurations #(ly:make-moment 2/1)
    { c16[ d e f g a b c] }

    % 渐慢羽化连音（ritardando）
    \override Beam.grow-direction = #LEFT
    \featherDurations #(ly:make-moment 2/1)
    { c16[ d e f g a b c] }

    % 恢复正常梁
    \revert Beam.grow-direction
    c8 d e f
  }
  \layout { }
}
```

> `\featherDurations #(ly:make-moment 2/1)` 表示首尾音的时值比为 2:1。`grow-direction = #RIGHT` 表示越来越快（音符越来越短），`#LEFT` 表示越来越慢。

### 6.4 多节奏记谱 (Polyrhythm)

两个声部使用不同的节奏细分同时演奏：

```lilypond
\version "2.24.0"

\header {
  title = "多节奏 - 三对四 (3:4)"
}

\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \time 4/4
      % 上方声部：4 连音（将 3 拍分成 4 个音）
      \tuplet 4/3 { c4 d e f }
      g1
    }
    \new Staff \relative c' {
      \time 4/4
      % 下方声部：正常三连音
      \tuplet 3/2 { g4 a b }
      c2
      g2
    }
  >>
  \layout { }
}
```

### 6.5 常见错误：连音比率的含义

```lilypond
% 错误：混淆连音比率的分子分母
\score {
  \new Staff \relative c'' {
    % \tuplet 2/3 表示"3个音的时值内演奏2个音"——这不是三连音！
    \tuplet 2/3 { c8 d e }    % 错误：3个八分音符占1.5拍的时值
  }
  \layout { }
}

% 正确：\tuplet 分子/分母 = "分子个音 占 分母个音 的时值"
\score {
  \new Staff \relative c'' {
    % 三连音：3个音占2个音的时值
    \tuplet 3/2 { c8 d e }    % 正确：3个八分音符 = 1拍
    % 五连音：5个音占4个音的时值
    \tuplet 5/4 { c16 d e f g } % 正确：5个十六分 = 1拍
  }
  \layout { }
}
```

> **记忆口诀**：`\tuplet A/B` = 在 B 个标准音的时值内演奏 A 个音。三连音 = `\tuplet 3/2`（3个音占2个的位置）。

---

## 七、AI 生成约束总结 (AI Generation Constraints for Modern Music)

### 7.1 现代音乐 vs 古典规则

| 规则 | 古典分支 | 现代分支 | 说明 |
|------|---------|---------|------|
| 平行五度/八度 | **禁止** | **允许** | 现代音乐不受此限制 |
| 不协和音解决 | **必须解决** | **不需要** | 不协和音可以自由保持 |
| 声部交错/超越 | **禁止** | **允许**（谨慎使用） | 现代织体可能需要 |
| 倾向音解决 | **必须遵守** | **不适用** | 无调性音乐无倾向音概念 |
| 跳进后反向 | **推荐** | **不强制** | 现代旋律可连续大跳 |
| 声部间距 ≤ 八度 | **上三声部必须** | **不限制** | 现代和声可任意分布 |
| 终止式 | **必须使用** | **可选** | 现代作品可能无传统终止 |

### 7.2 仍然必须遵守的规则

即使现代音乐风格更自由，以下规则**始终有效**：

| 规则 | 来源 | 说明 |
|------|------|------|
| 乐器音域范围 | `shared-rules.md` | 物理极限不可突破：长号吹不出 C7，短笛写不了 C2 |
| LilyPond 文件结构 | `lilypond-core-syntax.md` | `\version` + `\score` + `\layout` 不可省略 |
| 括号配对 | `lilypond-core-syntax.md` | `{}`/`<<>>`/`<>` 必须正确配对 |
| 谱号选择 | `shared-rules.md` | 每个乐器使用正确谱号 |
| 移调乐器记谱 | `shared-rules.md` | Bb 乐器高大二度，F 乐器高纯五度 |
| 每小节时值对齐 | `lilypond-core-syntax.md` | 除 `\cadenzaOn` 外，各声部小节时值必须等于拍号规定 |

### 7.3 现代音乐的额外考量

| 考量 | 说明 |
|------|------|
| 极端音区是合法的 | 小提琴可以到 C7，长笛可以到 C7，这在现代作品中常见 |
| 微分音需要正确后缀 | `ih`/`eh`/`isih`/`eseh`，不可随意编造 |
| 复杂节拍需要 Scheme 语法 | `\time #'((...) . N)` 格式不可省略 `#'` |
| 打击乐使用专用音名 | `\drummode` 中用 `bd`/`sn`/`hh`，不是 `c`/`d`/`e` |
| 无调性需显式标注每个临时记号 | `\omit Staff.KeySignature` 后，每个变化音必须写明 |
| 散拍段落不检查时值 | `\cadenzaOn` 后 LilyPond 不强制对齐，但 `\cadenzaOff` 后恢复 |

### 7.4 AI 生成现代音乐检查清单

生成现代音乐 LilyPond 代码前，对照此清单：

- [ ] 有 `\version "2.24.0"`
- [ ] 有 `\score { ... \layout { } }`
- [ ] 所有括号正确配对（`{}` / `<<>>` / `<>`）
- [ ] 乐器音域在 `shared-rules.md` 规定范围内
- [ ] 移调乐器已正确处理
- [ ] 微分音后缀拼写正确（`ih`/`eh`/`isih`/`eseh`）
- [ ] 复杂拍号使用 Scheme 语法 `#'(...)`
- [ ] 无调性段落已 `\omit Staff.KeySignature` 且每个变化音显式标注
- [ ] 打击乐使用 `\drummode` 和专用音名
- [ ] 散拍段落正确使用 `\cadenzaOn` / `\cadenzaOff`
- [ ] 多节拍已正确配置 `\layout` 中的小节线 engraver
- [ ] 嵌套连音比率正确（`\tuplet A/B` = A个音占B个的时值）
- [ ] 每个 `\new Staff` 有自己的 `\clef` 和 `\relative`（或使用 `\absolute`）
