# LilyPond 完整技法参考（AI可消费版）

> 基于 LilyPond v2.26.0 官方文档整合：学习手册 + 记谱法参考 + 扩展编程/使用手册/代码片段
> 🎵 = 现代作曲特别相关 | 🔧 = Scheme编程/程序化生成相关

---

## 第一部分：基础语法速览

### 1.1 文件结构与基本规则

每个 LilyPond 文件应包含版本声明：

```lilypond
\version "2.26.0"
```

基本文件结构：

```lilypond
\version "2.26.0"

\header {
  title = "Symphony"
  composer = "Composer Name"
  opus = "Op. 9"
}

\score {
  … 音乐表达式 …
  \layout { }
  \midi { }
}
```

**基本规则**：
- 音符和歌词必须被 `{ 花括号 }` 包围
- 花括号前后需有空格（行首/行尾除外）
- 输入**区分大小写**：`c d e` 有效，`C D E` 错误
- 编辑器使用 **UTF-8** 编码，必须使用**直引号**（`'`），不能用弯引号
- 注释：`%` 行注释，`%{ … %}` 块注释（不可嵌套）
- `{ }` 顺序排列音乐，`<< >>` 同时进行的音乐

LilyPond 自动添加谱号、拍号等。简单输入的完整等价形式：

```lilypond
% 简单输入
\relative { c''4 a b c }

% 等价完整形式
\book {
  \score {
    \new Staff {
      \new Voice {
        \relative { c''4 a b c }
      }
    }
    \layout { }
  }
}
```

> **建议**：超过几行音乐时，始终显式创建谱表和声部。

### 1.2 音高输入

#### 绝对模式

小写字母 `a`-`g` 表示音高（荷兰命名系统）。`'` 升高八度，`,` 降低八度。`c'` = 中央C。

```lilypond
{
  \clef treble
  c'4 e' g' c''
  c'4 g b c'
  \clef bass
  c,4 e, g, c
}
```

#### 相对模式（\relative）

最常用的输入方式。八度自动选择，假定下一音符在离前一音符最近位置（三个谱表间距内）：

```lilypond
\relative {
  c' d e f
  g a b c
}
```

起始音符决定起始八度。超过三个谱表间距时，用 `'` 提高、`,` 降低：

```lilypond
\relative {
  a' a, c' f,
  g g'' a,, f'
}
```

变音记号在相对位置计算时**完全被忽略**。

#### \fixed 模式

指定参考音高，只需标记偏离参考点的音符：

```lilypond
\fixed c'' {
  \key a \major
  \time 6/8
  cis8. d16 cis8 e4 e8 |
  b,8. cis16 b,8 d4 d8 |
}
```

| 模式 | 特点 | 适用场景 |
|------|------|----------|
| 绝对模式 | 每个音符需明确八度标记 | 大音程音乐、计算机生成文件 |
| `\fixed` 模式 | 设定参考音高，相对参考点标记 | 常见八度集中区域的乐谱 |
| `\relative` 模式 | 根据前一音符自动计算 | 大多数音乐输入（最快方式） |

#### 变音记号

升号 `is`，降号 `es`，重升 `isis`，重降 `eses`（荷兰语）：

```lilypond
\relative c'' { ais1 aes aisis aeses }
```

提醒变音记号 `!`，谨慎变音记号 `?`：

```lilypond
\relative c'' { cis cis cis! cis? c c c! c? }
```

#### 其他语言音符名称

```lilypond
\language "italiano"
\relative { do' re mi sib }
```

| 语言 | 升号 | 降号 | 重升 | 重降 |
|---|---|---|---|---|
| `nederlands` | `is` | `es` | `isis` | `eses` |
| `english` | `s`/`-sharp` | `f`/`-flat` | `ss`/`x` | `ff` |
| `deutsch` | `is` | `es` | `isis` | `eses` |
| `italiano` | `d` | `b` | `dd` | `bb` |
| `français` | `d` | `b` | `dd`/`x` | `bb` |

八度检查：

```lilypond
\relative {
  c''2 d
  \octaveCheck c'
  e2 f
}
```

### 1.3 节奏输入

时值由音符名称后的数字指定：`1` 全音符、`2` 二分音符、`4` 四分音符等。未指定时值时使用前一时值，第一个音符默认四分音符。

```lilypond
\relative {
  a'1
  a2 a4 a8 a
  a16 a a a a32 a a a a64 a a a a a a a a2
}
```

附点音符在时值数字后加点：

```lilypond
\relative { a'4 a a4. a8 a8. a16 a a8. a8 a4. }
```

休止符用 `r` 输入：

```lilypond
\relative { a'4 r r2 r8 a r4 r4. r8 }
```

缩放时值：

```lilypond
\relative {
  \time 2/4
  a'4*2/3 gis a    % 三连音效果，不打印方括号
  a4 a
  <a d>4*2         % 时值翻倍
  b16*4 c4         % 外观如16分，时值如四分
}
```

使用 `\scaleDurations` 缩放段落：

```lilypond
\relative {
  \time 2/4
  <c'' a>4 c8 a
  \scaleDurations 2/3 {
    <c a f>4. c8 a f
  }
  \scaleDurations 2 {
    <c' a>4 c8 b
  }
}
```

### 1.4 和弦

使用单尖括号 `< >` 将音高括起来，时值放在闭合括号**之后**。`q` 重复前一和弦：

```lilypond
\relative {
  r4 <c'' e g>~ <c f a>2 |
  <c e g>8[ <c f a> <c e g> <c f a>]
    <c e g>8\>[ <c f a> q q]\! |
  r4 <c e g>8.\p q16( q4-. <c f a>) |
}
```

和弦加奏法记号：

```lilypond
\relative {
  <a' c\prall e>1 <a\-> c\-^ e>2 <f\-. a c\-. e\-.>4
  <a\-+ c\-->8. <g\fermata c e\turn>16
}
```

### 1.5 连音（Tuplets）

```lilypond
\relative {
  \tuplet 3/2 { f''8 g a }
  \tuplet 3/2 { c8 r c }
}
```

长段落连音自动分组：

```lilypond
\relative { g'2 r8 \tuplet 3/2 8 { cis16 d e e f g g f e } }
```

嵌套连音：

```lilypond
\relative { c''4 \tuplet 5/4 { f8 e f \tuplet 3/2 { e[ f g] } } f4 }
```

使用 `\tupletSpan` 设置连音分组：

```lilypond
\relative c' {
  \time 2/4
  \tupletSpan 4
  \tuplet 3/2 { c8 c c c c c }
  \tupletSpan \default
  \tuplet 3/2 { c8 c c c c c }
}
```

### 1.6 装饰音

```lilypond
\relative {
  c''2 \grace { a32 b } c2 |
  c2 \appoggiatura b16 c2 |
  c2 \acciaccatura b16 c2 |
}
```

### 1.7 拍号与速度

```lilypond
\relative {
  \time 3/4 a'4 a a
  \time 6/8 a4. a
  \time 4/4 a4 a a a
}
```

```lilypond
\relative {
  \tempo "Andante" a'4 a a
  \tempo 4. = 96 a4. a
  \tempo "Presto" 4 = 120 a4 a a a
}
```

### 1.8 谱号

```lilypond
\relative {
  \clef treble c'1
  \clef alto c1
  \clef tenor c1
  \clef bass c1
}
```

### 1.9 连线与连奏线

延音线用波浪号 `~`，音高不变时可省略后续音高：

```lilypond
\relative { g'4~ 4 c2~ | 4~ 8 a~ 2 | }
```

连奏线用 `(` 和 `)` 标记，乐句连奏线用 `\(` 和 `\)`：

```lilypond
\relative { d''4( c16) cis( d e c cis d) e( d4) }
\relative { g'4\( g8( a) b( c) b4\) }
```

### 1.10 奏法记号与力度

| 语法 | 说明 |
|------|------|
| `-^` | 强调记号（marcato） |
| `--` | 保持音（tenuto） |
| `->` | 重音（accent） |
| `-.` | 跳音点（staccato） |
| `-_` | 断连音（portato） |

[补充] 完整奏法记号示例：

```lilypond
\relative {
  c''4-^ c-- c4-> c-. c2-_
}
```

力度记号通过反斜杠标记：

```lilypond
\relative { c''2\ff c\mf c\p c\pp }
\relative { c''2\< c | c4\ff\> c c c\! }
```

指法用 `-` 加数字，`^`（上方）或 `_`（下方）指定方向：

```lilypond
\relative { c''4-3 e-5 b-2 a-1 }
```

[补充] 指法方向控制完整示例：

```lilypond
\relative {
  c''4-5 a-3 f-1 c'-5 |
  \override Fingering.direction = #DOWN
  c4-5 a-3 f-1 c'-5 |
  \override Fingering.direction = #UP
  c4-5 a-3 f-1 c'-5 |
}
```

`_`、`^` 方向指示符：

```lilypond
\relative {
  c''4-5 a-3 f-1 c'-5 |
  c4_5 a_3 f_1 c'_5 |
  c4^5 a^3 f^1 c'^5 |
}
```

和弦指法：

```lilypond
\relative {
  <c''-5 g-3>4
  <c-5 g-3 e-2>4
  <c-5 g-3 e-2 c-1>4
}
```

和弦中独立方向控制：

```lilypond
\relative {
  <c''-5 g-3 e-2 c-1>4
  <c^5 g_3 e_2 c_1>4
  <c^5 g^3 e^2 c_1>4
}
```

`fingeringOrientations` 属性：

```lilypond
\relative {
  \set fingeringOrientations = #'(left)
  <c'-5>4
  \set fingeringOrientations = #'(right)
  <c'-5>4
  \set fingeringOrientations = #'(up)
  <c'-5>4
  \set fingeringOrientations = #'(down)
  <c'-5>4
}
```

`fingeringOrientations` + 多指组合：

```lilypond
\relative {
  \set fingeringOrientations = #'(up left)
  <c'-5-3>4
  \set fingeringOrientations = #'(up left down)
  <c'-5-3-1>4
}
```

`fingeringOrientations` + `font-size`：

```lilypond
\relative {
  \set fingeringOrientations = #'(left)
  \override Fingering.font-size = -5
  <c'-5>4
  \override Fingering.font-size = 0
  <c'-5>4
  \override Fingering.font-size = 5
  <c'-5>4
}
```

### 1.11 添加文字

```lilypond
\relative { c''2^"espr" a\_"legato" }
\relative {
  c''2^\markup { \bold espr }
  a2_\markup {
    \dynamic f \italic \small { 2nd } \hspace #0.1 \dynamic p }
}
```

### 1.12 连梁控制

[校正] 标准手动连梁语法为 `[ ]`，而非旧语法 `\[ \]`。

```lilypond
\relative { a'8[ ais] d[ ees r d] c16 b a8 }
\relative {
  \autoBeamOff
  a'8 c b4 d8. c16 b4 |
  \autoBeamOn
  a8 c b4 d8. c16 b4 |
}
```

### 1.13 弱起小节

```lilypond
\relative {
  \partial 8 f''8 |
  c2 d |
}
```

### 1.14 小节检查

使用 `|` 标记预期小节线位置，帮助验证时值：

```lilypond
\relative { g'1 | e1 | c2. c' | g4 c g e | c4 r r2 | }
```

### 1.15 变量与函数

变量存储音乐片段和排版调整：

```lilypond
hornNotes = \relative { c''4 b dis c }

\score {
  { \hornNotes }
}
```

后事件也可存储在变量中：

```lilypond
articulationVar = -^-.
artEsprVar = \articulationVar ^>

\relative c' {
  c\articulationVar d e2^\articulationVar
  d2\artEsprVar c_\artEsprVar
}
```

音乐函数使用 `define-music-function` 创建：

```lilypond
padText =
#(define-music-function (padding) (number?)
   #{
     \once \override TextScript.padding = #padding
   #})

\relative {
  c''4^"piu mosso" b a b
  \padText 1.8
  c4^"piu mosso" b a b
}
```

### 1.16 歌词

使用 `\addlyrics` 将歌词与旋律结合：

```lilypond
<<
  \relative {
    \key g \major
    \time 6/8
    d''4 b8 c4 a8 | d4 b8 g4
  }
  \addlyrics {
    Girls and boys come out to play,
  }
>>
```

更复杂的乐谱应使用变量将结构与音符/歌词分离：

```lilypond
melody = \relative { r4 d''8\noBeam g, c4 r }
words  = \lyricmode { And God said, }
upper  = \relative { <g' d g,>2~ <g c, g> }
lower  = \relative { b,2 e }

\score {
  <<
    \new Staff = "singer" <<
      \new Voice = "vocal" { \melody }
      \addlyrics { \words }
    >>
    \new PianoStaff = "piano" <<
      \new Staff = "upper" { \upper }
      \new Staff = "lower" { \clef "bass" \lower }
    >>
  >>
  \layout { }
}
```

---

## 第二部分：现代作曲记谱法 🎵

### 2.1 微音程记谱 🎵

#### 四分音变音记号

| 后缀 | 含义 | 音程变化 |
|------|------|----------|
| `ih` | 半升号（semi-sharp） | +25音分 |
| `eh` | 半降号（semi-flat） | -25音分 |
| `isih` | 一又半升号 | +75音分 |
| `eseh` | 一又半降号 | -75音分 |

递增的四分音音高序列：

```lilypond
\relative c'' { ceseh1 ces ceh c cih cis cisih }
```

其他语言的四分音后缀：

| 语言 | 半升号 | 半降号 | 一又半升号 | 一又半降号 |
|---|---|---|---|---|
| `nederlands` | `ih` | `eh` | `isih` | `eseh` |
| `english` | `qs` | `qf` | `tqs` | `tqf` |
| `deutsch` | `ih` | `eh` | `isih` | `eseh` |
| `italiano` | `sd` | `sb` | `dsd` | `bsb` |
| `français` | `sd` | `sb` | `dsd` | `bsb` |
| `español` | `cs` | `cb` | `tcs` | `tcb` |
| `català` | `qd`/`qs` | `qb` | `tqd`/`tqs` | `tqb` |
| `português` | `sqt` | `bqt` | `stqt` | `btqt` |
| `vlaams` | `hk` | `hb` | `khk` | `bhb` |

德语微音程缩写：

```lilypond
\language "deutsch"
\relative c'' { asah2 eh aih eisih }
```

#### 近似音高 🎵

当音高不完全已知时，使用 `\approximatePitch`，以三角形符头表示：

```lilypond
\relative c'' {
  e4 f g \approximatePitch c
}
```

#### 替代变音记号字形 🎵

通过 `alterationGlyphs` 属性自定义变音记号字形。使用带箭头的微音程变音记号：

```lilypond
\layout {
  \context {
    \Staff
    alterationGlyphs = #alteration-vaticana-glyph-name-alist
  }
}

{ ces' c' cis' }
```

[补充] 上例使用预定义的 Vaticana 字形列表。也可手动指定自定义映射：

```lilypond
\layout {
  \context {
    \Staff
    alterationGlyphs =
      #'((-1/2 . "accidentals.flat.arrowdown")
         (0 . "accidentals.natural.arrowup")
         (1/2 . "accidentals.sharp.arrowup"))
  }
}

{ ces' c' cis' }
```

调号中微音程字形的间距调整：

```lilypond
\layout {
  \context {
    \Staff
    alterationGlyphs =
      #'((-1/2 . "accidentals.flat.arrowdown")
         (0 . "accidentals.natural.arrowup")
         (1/2 . "accidentals.sharp.arrowup"))
    \override KeySignature.padding-pairs =
      #'((("accidentals.sharp.arrowup" . "accidentals.sharp.arrowup")
            . 0.25)
         (("accidentals.flat.arrowdown" . "accidentals.flat.arrowdown")
            . 0.3))
    \override KeyCancellation.padding-pairs =
      #'((("accidentals.natural.arrowup" . "accidentals.natural.arrowup")
            . 0.7))
  }
}

{
  \key cis \major
  ces' c'
  \key ces \major
  cis'
}
```

### 2.2 集群记谱（Clusters）🎵

集群表示连续音高范围的演奏，通过 `\makeClusters` 输入：

```lilypond
\relative \makeClusters { <g' b>2 <c g'> }
```

支持的集群样式：

| 样式 | 说明 |
|------|------|
| `ramp` | 斜坡（默认） |
| `leftsided-stairs` | 左侧楼梯 |
| `rightsided-stairs` | 右侧楼梯 |
| `centered-stairs` | 居中楼梯 |

所有集群样式完整示例：

```lilypond
fragment = { <e' d''>4 <g' a'> <e' a'> r }

{
  \omit Staff.Clef
  \omit Staff.TimeSignature

  <>^\markup \typewriter "ramp"
  \override ClusterSpanner.style = #'ramp
  \makeClusters \fragment

  <>_\markup \typewriter "leftsided-stairs"
  \override ClusterSpanner.style = #'leftsided-stairs
  \makeClusters \fragment

  <>^\markup \typewriter "rightsided-stairs"
  \override ClusterSpanner.style = #'rightsided-stairs
  \makeClusters \fragment

  <>_\markup \typewriter "centered-stairs"
  \override ClusterSpanner.style = #'centered-stairs
  \makeClusters \fragment
}
```

> **注意**：集群至少需要跨越两个和弦才美观；集群不产生 MIDI 输出。

### 2.3 图形记谱——DurationLine 🎵

`DurationLine` 是当代音乐图形记谱的核心工具，用线条延续替代传统符干/符梁。

#### 启用 DurationLine

```lilypond
\layout {
  \context {
    \Voice
    \consists Duration_line_engraver
  }
}
```

#### 支持的线条样式

| 样式 | 说明 |
|------|------|
| `'beam` | 横梁样式 |
| `'line` | 实线 |
| `'dashed-line` | 虚线 |
| `'dotted-line` | 点线 |
| `'zigzag` | 锯齿线 |
| `'trill` | 颤音线 |
| `'none` | 无线条 |

线条终止方式：`#'hook`（仅限横梁样式）、`#'arrow`。

完整样式演示：

```lilypond
\layout {
  \context {
    \Voice
    \consists Duration_line_engraver
    \omit Stem
    \omit Flag
    \omit Beam
    \override NoteHead.duration-log = 2
  }
}

{
  a'1\- s2 r
  \once \override DurationLine.style = #'line
  a'1\- s2 r
  \once \override DurationLine.style = #'dashed-line
  \once \override DurationLine.dash-period = 2
  a'1\- s2 r
  \once \override DurationLine.style = #'dotted-line
  \once \override DurationLine.dash-period = 1
  \once \override DurationLine.bound-details.right.padding = 1
  a'1\- s2 r
  \once \override DurationLine.thickness = 2
  \once \override DurationLine.style = #'zigzag
  a'1\- s2 r
  \once \override DurationLine.style = #'trill
  a'1\- s2 r
  \once \override DurationLine.style = #'none
  a'1\- s2 r
  \once \override DurationLine.bound-details.right.end-style = #'arrow
  a'1\- s2 r
  \override DurationLine.bound-details.right.end-style = #'hook
  a'1\- s2 r
  \override DurationLine.details.hook-direction = #DOWN
  a'1\- s2 r
  \bar "|."
}
```

DurationLine 避开谱号变更：

```lilypond
\layout {
  \context {
    \Voice
    \consists "Duration_line_engraver"
  }
}

<<
  \new Staff {
    g'1\- s \clef "alto" g'
  }
  \new Staff {
    \override DurationLine.bound-details
                          .right.end-on-break-align-group = ##t
    g'1\- s \clef "alto" g' \bar "|."
  }
>>
```

DurationLine 关键属性：

| 属性 | 作用 |
|---|---|
| `DurationLine.style` | 线条样式 |
| `DurationLine.dash-period` | 虚线/点线间距周期 |
| `DurationLine.thickness` | 线条粗细 |
| `DurationLine.bound-details.right.padding` | 右端点内边距 |
| `DurationLine.bound-details.right.end-style` | 右端终止样式 |
| `DurationLine.details.hook-direction` | 钩号方向 |

### 2.4 无节拍音乐与华彩段 🎵

#### 基本华彩段

`\cadenzaOn` 和 `\cadenzaOff` 之间的音乐不计入小节长度：

```lilypond
\relative c'' {
  \override Score.BarNumber.break-visibility = #all-visible
  c4 d e d
  \cadenzaOn
  c4 cis d8[ d d] f4 g4.
  \cadenzaOff
  d4 e d c
}
```

#### 将无节拍段落划分为不规则小节 🎵

```lilypond
cadenzaMeasure = {
  \cadenzaOff
  \partial 1024 s1024
  \cadenzaOn
}

\relative c'' {
  \override Score.BarNumber.break-visibility = #all-visible
  c4 d e d
  \cadenzaOn
  c4 cis \bar "!" d8[ d d] \cadenzaMeasure f4 g4.
  \cadenzaMeasure
  \cadenzaOff
  d4 e d c
}
```

#### 华彩段中必须手动连梁

`\cadenzaOn` 禁用自动连梁：

```lilypond
\relative {
  \repeat unfold 8 { c''8 }
  \cadenzaOn
  cis8 c c c c
  \bar"|"
  c8 c c
  \cadenzaOff
  \repeat unfold 8 { c8 }
}
```

#### 长华彩段中的换行

使用 `\allowBreak`：

```lilypond
\relative {
  c'4 f g c, d f g c
  \cadenzaOn
  c4 cis8
  \allowBreak
  d[ cis c cis]
  \allowBreak
  d[ f g a]
  \allowBreak
  ais[ g f g]
  \allowBreak
  d4 f8
  \allowBreak
  d[ cis] c4
  \allowBreak
  a8[ c] g4
}
```

### 2.5 多节拍记谱 🎵

#### 等长小节的不同拍号

使用 `\polymetric \time` 设置局部拍号，`\scaleDurations` 使小节长度匹配：

```lilypond
\relative <<
  \new Staff {
    \time 3/4
    c'4 c c |
    c4 c c |
  }
  \new Staff {
    \scaleDurations 2/3 {
      \context Staff \polymetric \time 9/8
      \repeat unfold 3 { c8 c c }
      \repeat unfold 3 { c4 c8 }
    }
  }
  \new Staff {
    \scaleDurations 3/5 {
      \context Staff \polymetric \time 3,3,2,2 10/8
      \repeat unfold 2 { c8 c c }
      \repeat unfold 2 { c8 c } |
      c4. c \tuplet 3/2 { c8 c c } c4
    }
  }
>>
```

#### 不等长小节的不同拍号 🎵

使用 `\enablePerStaffTiming` 为每个谱表设置独立拍号：

```lilypond
\layout {
  \enablePerStaffTiming
}

\relative <<
  \new Staff {
    \time 3/4
    c'4 c c |
    c4 c c |
  }
  \new Staff {
    \time 2/4
    c4 c |
    c4 c |
    c4 c |
  }
  \new Staff {
    \time 3/8
    c4. |
    c8 c c |
    c4. |
    c8 c c |
  }
>>
```

仅对一个 score 启用多节拍：

```lilypond
\score {
  <<
    \new Staff { c''1 1 }
    \new Staff { c'2 d' g'2~ 2 }
  >>
}

\score {
  \layout {
    \enablePerStaffTiming
  }
  <<
    \new Staff { \time 4/4 c''1 1 }
    \new Staff { \time 2/4 c'2 d' g'2~ 2 }
  >>
}
```

MIDI 输出中使用多节拍：

```lilypond
\layout {
  \enablePerStaffTiming
}

\midi {
  \enablePerStaffTiming
}
```

#### 通过移动 Timing_translator 实现多节拍 🔧

```lilypond
global = {
  \time 3/4 s2.*3 \break
  s2.*3
}

\layout {
  \context {
    \Score
    \remove "Timing_translator"
    \remove "Bar_number_engraver"
    \override SpacingSpanner.uniform-stretching = ##t
    \override SpacingSpanner.strict-note-spacing = ##t
    proportionalNotationDuration = #1/64
  }
  \context {
    \Staff
    \consists "Timing_translator"
  }
  \context {
    \Voice
    \remove "Forbid_line_break_engraver"
    tupletFullLength = ##t
  }
}

Bassklarinette = \new Staff \with {
  \consists "Bar_number_engraver"
  barNumberVisibility = #(every-nth-bar-number-visible 2)
  \override BarNumber.break-visibility = #end-of-line-invisible
} <<
  \global
  {
    \clef treble
    \time 3/8 d''4. |
    \time 3/4 r8 des''2( c''8) |
    \time 7/8 r4. ees''2 ~ |
    \time 2/4 \tupletUp \tuplet 3/2 { ees''4 r4 d''4 ~ } |
    \time 3/8 \tupletUp \tuplet 4/3 { d''4 r4 } |
    \time 2/4 e''2 |
    \time 3/8 es''4. |
    \time 3/4 r8 d''2 r8 |
  }
>>

Perkussion = \new StaffGroup <<
  \new Staff <<
    \global
    {
      \clef percussion
      \time 3/4 r4 c'2 ~ |
      c'2. |
      R2. |
      r2 g'4 ~ |
      g'2. ~ |
      g'2. |
    }
  >>
  \new Staff <<
    \global {
      \clef percussion
      \time 3/4 R2. |
      g'2. ~ |
      g'2. |
      r4 g'2 ~ |
      g'2 r4 |
      g'2. |
    }
  >>
>>

\score {
  <<
    \Bassklarinette
    \Perkussion
  >>
}
```

**核心原理**：将 `Timing_translator` 从 `Score` 移到 `Staff`，使每个声部拥有独立小节线。

### 2.6 当代变音记号样式 🎵

使用 `\accidentalStyle` 函数指定变音记号风格。语法格式：

```lilypond
\accidentalStyle style
```

其中 `style` 可指定上下文前缀：

```lilypond
\accidentalStyle default
\accidentalStyle voice
\accidentalStyle StaffGroup.voice
\accidentalStyle Score.default
```

[补充] `default` 按声部处理变音记号，`voice` 类似但严格按声部分离，`StaffGroup.voice` 在谱表组内按声部分离，`Score.default` 在整个乐谱范围内使用默认规则。

#### 十二音风格（dodecaphonic）🎵

每个音符都获得一个变音记号，包括还原号：

```lilypond
\new Staff <<
  \accidentalStyle dodecaphonic
  { … }
>>
```

变体：
- **`dodecaphonic-no-repeat`**：同一声部中立即重复的音高抑制变音记号
- **`dodecaphonic-first`**：仅在小节内第一次出现时打印，贯穿所有声部

#### 新现代风格（neo-modern）🎵

同一小节中再次出现的同一音符会再次打印变音记号（除非是立即重复）：

```lilypond
\new Staff <<
  \accidentalStyle neo-modern
  { … }
>>
```

变体：
- **`neo-modern-cautionary`**：额外的变音记号作为警示变音记号（带括号）打印
- **`neo-modern-voice`**：多声部版本，声部之间会取消
- **`neo-modern-voice-cautionary`**：带警示的多声部版本

#### 其他当代相关风格

| 风格 | 说明 |
|------|------|
| `modern` / `modern-cautionary` | 20世纪常见做法 |
| `no-reset` | 变音记号永久持续 |
| `forget` | 完全不记忆变音记号 |
| `teaching` | 自动创建警示变音记号 |

替代结尾的变通方案：

```lilypond
forget = #(define-music-function (music) (ly:music?) #{
  \accidentalStyle forget
  #music
  \accidentalStyle modern
#})
{
  \accidentalStyle modern
  \time 2/4
  \repeat volta 2 {
    c'2
  }
  \alternative {
     \volta 1 { cis' }
     \volta 2 { \forget c' }
  }
}
```

防止自动添加额外还原号：

```lilypond
\relative c'' {
  aeses4 aes ais a
  \set Staff.extraNatural = ##f
  aeses4 aes ais a
}
```

### 2.7 当代调号与和声 🎵

#### 自定义调式

通过列出每个音阶步骤的变音记号来定义额外调式：

```lilypond
freygish = #`((0 . ,NATURAL) (1 . ,FLAT) (2 . ,NATURAL)
    (3 . ,NATURAL) (4 . ,NATURAL) (5 . ,FLAT) (6 . ,FLAT))

\relative {
  \key c \freygish c'4 des e f
  \bar "||" \key d \freygish d es fis g
}
```

#### 非标准调号 🎵

直接设置 `Staff.keyAlterations` 属性：

```lilypond
\set Staff.keyAlterations =
  #`(((octave . step) . alter) ((octave . step) . alter) ...)
```

其中 `octave` 为0表示从中音C到上方B的八度，`step` 为0-6（C到B），`alter` 为 `SHARP`、`FLAT`、`SEMI-FLAT` 等。

微音程调号示例：

```lilypond
\include "arabic.ly"

\relative do' {
  \set Staff.keyAlterations = #`((0 . ,SEMI-FLAT)
                                 (1 . ,SEMI-FLAT)
                                 (2 . ,FLAT)
                                 (5 . ,FLAT)
                                 (6 . ,SEMI-FLAT))
  re reb \dwn reb resd
  dod dob dosd \dwn dob |
  dobsb dodsd do do |
}
```

[补充] 调号位置控制——调整升降号在谱表上的位置：

```lilypond
\override Staff.KeySignature.flat-positions = #'((-5 . 5))
\override Staff.KeyCancellation.flat-positions = #'((-5 . 5))
\clef bass \key es \major es g bes d'
\clef treble \bar "||" \key es \major es' g' bes' d''

\override Staff.KeySignature.sharp-positions = #'(2)
\bar "||" \key b \major b' fis' b'2
```

调号间距控制（`padding-pairs`）：

```lilypond
\layout {
  \context {
    \Staff
    alterationGlyphs =
      #'((-1/2 . "accidentals.flat.arrowdown")
         (0 . "accidentals.natural.arrowup")
         (1/2 . "accidentals.sharp.arrowup"))
    \override KeySignature.padding-pairs =
      #'((("accidentals.sharp.arrowup" . "accidentals.sharp.arrowup")
            . 0.25)
         (("accidentals.flat.arrowdown" . "accidentals.flat.arrowdown")
            . 0.3))
    \override KeyCancellation.padding-pairs =
      #'((("accidentals.natural.arrowup" . "accidentals.natural.arrowup")
            . 0.7))
  }
}

{
  \key cis \major
  ces' c'
  \key ces \major
  cis'
}
```

简写格式（所有八度使用相同变音）：`(step . alter)`

防止调号更改时打印还原号：

```lilypond
\relative c' {
  \key d \major
  a4 b cis d
  \key g \minor
  a4 bes c d
  \set Staff.printKeyCancellation = ##f
  \key d \major
  a4 b cis d
  \key g \minor
  a4 bes c d
}
```

### 2.7b 音域标记（Ambitus）🎵

[补充] Ambitus 显示声部或谱表的音域范围。

按声部添加 Ambitus：

```lilypond
\layout {
  \context {
    \Voice
    \consists Ambitus_engraver
  }
}

\relative {
  aes' c e2
  cis,1
}
```

多声部独立 Ambitus：

```lilypond
\new Staff <<
  \new Voice \with {
    \consists "Ambitus_engraver"
  } \relative c'' {
    \override Ambitus.X-offset = 2.0
    \voiceOne
    c4 a d e
    f1
  }
  \new Voice \with {
    \consists "Ambitus_engraver"
  } \relative c' {
    \voiceTwo
    es4 f g as
    b1
  }
>>
```

按谱表添加 Ambitus（共享）：

```lilypond
\new Staff \with {
  \consists "Ambitus_engraver"
  }
<<
  \new Voice \relative c'' {
    \voiceOne
    c4 a d e
    f1
  }
  \new Voice \relative c' {
    \voiceTwo
    es4 f g as
    b1
  }
>>
```

修改 Ambitus 间距：

```lilypond
\layout {
  \context {
    \Voice
    \consists "Ambitus_engraver"
  }
}

\new Staff {
  \time 2/4
  % Default setting
  c'4 g''
}

\new Staff {
  \time 2/4
  \override AmbitusLine.gap = 0
  c'4 g''
}

\new Staff {
  \time 2/4
  \override AmbitusLine.gap = 1
  c'4 g''
}
```

[补充] `\ambitusAfter` 命令——将 Ambitus 放在调号之后：

```lilypond
\new Staff \with {
  \consists Ambitus_engraver
} \relative {
  \ambitusAfter key-signature
  \key d \major
  es'8 g bes cis d2
}
```

### 2.8 渐变连梁（Feathered Beams）🎵

渐变连梁表示音符以逐渐加快或减慢的速度演奏。

- `#LEFT`：渐宽（减速）
- `#RIGHT`：渐窄（加速）
- `#'()`：恢复非渐变

```lilypond
\relative c' {
  \override Beam.grow-direction = #LEFT
  \featherDurations 2/1
  { c16[ c c c c c c c] }
  \override Beam.grow-direction = #RIGHT
  \featherDurations 2/3
  { c32[ d e f] }
  \override Beam.grow-direction = #'()
  { g32[ a b c] }
}
```

> `\featherDurations` 仅适用于非常短的音乐片段，且分数中的数字较小时才能正常工作。

### 2.9 倒影/逆行/调式变换 🎵

#### 移调（\transpose）

```
\transpose frompitch topitch musicexpr
```

```lilypond
\transpose d e {
  \relative {
    \key d \major
    d'4 fis a d
  }
}
```

等音移调区别：

```lilypond
music = \relative { c' d e f }
\new Staff {
  \transpose c cis { \music }
  \transpose c des { \music }
}
```

"智能"移调——最少变音号 🔧：

```lilypond
#(define (naturalize-pitch p)
   (let ((o (ly:pitch-octave p))
         ;; `ly:pitch-alteration` returns quarter tone steps.
         (a (* 4 (ly:pitch-alteration p)))
         (n (ly:pitch-notename p)))
     (cond
      ((and (> a 1) (or (eqv? n 6) (eqv? n 2)))
       (set! a (- a 2)) (set! n (+ n 1)))
      ((and (< a -1) (or (eqv? n 0) (eqv? n 3)))
       (set! a (+ a 2)) (set! n (- n 1))))
     (cond
      ((> a 2) (set! a (- a 4)) (set! n (+ n 1)))
      ((< a -2) (set! a (+ a 4)) (set! n (- n 1))))
     (when (< n 0) (set! o (- o 1)) (set! n (+ n 7)))
     (when (> n 6) (set! o (+ o 1)) (set! n (- n 7)))
     (ly:make-pitch o n (/ a 4))))

#(define (naturalize music)
   (let ((es (ly:music-property music 'elements))
         (e (ly:music-property music 'element))
         (p (ly:music-property music 'pitch)))
     (when (pair? es)
       (ly:music-set-property! music 'elements (map naturalize es)))
     (when (ly:music? e)
       (ly:music-set-property! music 'element (naturalize e)))
     (when (ly:pitch? p)
       (set! p (naturalize-pitch p))
       (ly:music-set-property! music 'pitch p))
     music))

naturalizeMusic =
#(define-music-function (m) (ly:music?)
   (naturalize m))

music = \relative c' { c4 d e g }

\new Staff {
  \transpose c ais { \music }
  \naturalizeMusic \transpose c ais { \music }
  \transpose c deses { \music }
  \naturalizeMusic \transpose c deses { \music }
}
```

#### 倒影（\inversion）🎵

```
\inversion around-pitch to-pitch musicexpr
```

```lilypond
music = \relative { c' d e f }
\new Staff {
  \music
  \inversion d' d' \music
  \inversion d' ees' \music
}
```

#### 逆行（\retrograde）🎵

```lilypond
music = \relative { c'8. ees16( fis8. a16 b8.) gis16 f8. d16 }

\new Staff {
  \music
  \retrograde \music
}
```

#### 调式变换 🎵

调式移调：

```
\modalTranspose from-pitch to-pitch scale motif
```

```lilypond
diatonicScale = \relative { c' d e f g a b }
motif = \relative { c'8 d e f g a b c }

\new Staff {
  \motif
  \modalTranspose c f \diatonicScale \motif
  \modalTranspose c b, \diatonicScale \motif
}
```

五声音阶移调：

```lilypond
pentatonicScale = \relative { ges aes bes des ees }
motif = \relative { ees'8 des ges,4 <ges' bes,> <ges bes,> }

\new Staff {
  \motif
  \modalTranspose ges ees' \pentatonicScale \motif
}
```

调式倒影：

```
\modalInversion around-pitch to-pitch scale motif
```

```lilypond
octatonicScale = \relative { ees' f fis gis a b c d }
motif = \relative { c'8. ees16 fis8. a16 b8. gis16 f8. d16 }

\new Staff {
  \motif
  \modalInversion fis' fis' \octatonicScale \motif
}
```

逆行倒影组合 🎵：

```lilypond
\new Staff {
  \motif
  \retrograde \modalInversion c' c' \octatonicScale \motif
}
```

### 2.10 表情记号扩展 🎵

#### 琶音变体 [补充]

基本琶音：

```lilypond
\relative { <c' e g c>1\arpeggio }
```

跨谱表琶音：

```lilypond
\new PianoStaff \fixed c' <<
  \new Staff {
    \once \set PianoStaff.connectArpeggios = ##t
    <e g>2\arpeggio q\arpeggio
  }
  \new Staff {
    \clef "bass"
    <a, c>2\arpeggio q\arpeggio
  }
>>
```

6种琶音样式：

```lilypond
\relative {
  <c' e g c>2\arpeggio

  \arpeggioArrowUp
  <c e g c>2\arpeggio

  \arpeggioArrowDown
  <c e g c>2\arpeggio

  \arpeggioNormal
  <c e g c>2\arpeggio
}
```

方括号和圆括号琶音：

```lilypond
\relative {
  <c' e g c>2

  \arpeggioBracket
  <c e g c>2\arpeggio

  \arpeggioParenthesis
  <c e g c>2\arpeggio

  \arpeggioParenthesisDashed
  <c e g c>2\arpeggio

  \arpeggioNormal
  <c e g c>2\arpeggio
}
```

| 命令 | 说明 |
|------|------|
| `\arpeggio` | 普通琶音（波浪线） |
| `\arpeggioArrowUp` | 向上箭头琶音 |
| `\arpeggioArrowDown` | 向下箭头琶音 |
| `\arpeggioNormal` | 恢复普通琶音 |
| `\arpeggioBracket` | 方括号琶音 |
| `\arpeggioParenthesis` | 圆括号琶音 |
| `\arpeggioParenthesisDashed` | 虚线圆括号琶音 |

#### al niente 渐弱记号

```lilypond
\relative c'' {
  \override Hairpin.circled-tip = ##t
  c2\< c\!
  c4\> c\< c2\!
}
```

#### 各种渐变形状

```lilypond
\relative c'' {
  \override Hairpin.stencil = #flared-hairpin
  a4\< a a a\f
  \override Hairpin.stencil = #constante-hairpin
  a4\< a a a\f
}
```

#### 下降记号和上升记号（Falls & Doits）

```lilypond
\relative c'' {
  c2\bendAfter 4
  c2\bendAfter -4
  c2\bendAfter 6.5
  c2\bendAfter -6.5
}
```

#### 当代 Glissando

基本滑奏：

```lilypond
\relative {
  g'2\glissando g'
  c2\glissando c,
}
```

当代滑奏——无终止音：

```lilypond
\relative c'' {
  \time 3/4
  \override Glissando.style = #'zigzag
  c4 c
  \cadenzaOn
  c4\glissando
  \hideNotes
  c,,4
  \unHideNotes
  \cadenzaOff
  \bar "|"
}
```

长滑奏的时值标记：

```lilypond
glissandoSkipOn = {
  \override NoteColumn.glissando-skip = ##t
  \hide NoteHead
  \override NoteHead.no-ledgers = ##t
}

glissandoSkipOff = {
  \revert NoteColumn.glissando-skip
  \undo \hide NoteHead
  \revert NoteHead.no-ledgers
}

\relative c'' {
  r8 f8\glissando \glissandoSkipOn f4 g a |
  a8\noBeam \glissandoSkipOff a8
  r8 f8\glissando \glissandoSkipOn g4 a8 \glissandoSkipOff a8 |
}
```

和弦滑奏映射：

```lilypond
\relative {
  <c' e>2\glissando g'
  <c, e>\glissando <g' b>
  \set glissandoMap = #'((0 . 1) (1 . 0))
  <c, g'>\glissando <d a'>
}
```

[补充] Glissando 跨行属性：

```lilypond
\relative c'' {
  \override Glissando.breakable = ##t
  \override Glissando.after-line-breaking = ##t
  a1\glissando
  \break
  a1
}
```

Glissando 最小长度和弹簧控制：

```lilypond
\relative c' {
  \override Glissando.minimum-length = 5
  \override Glissando.springs-and-rods = #ly:spanner::set-spacing-rods
  f1\glissando f'
}
```

#### 新力度记号

使用 `\markup`：

```lilypond
moltoF = \markup { molto \dynamic f }
\relative {
  <d' e>16_\moltoF <d e>
  <d e>2..
}
```

使用 `make-dynamic-script`：

```lilypond
sfzp = #(make-dynamic-script "sfzp")
\relative {
  c'4 c c\sfzp c
}
```

复杂自定义力度记号：

```lilypond
roundF = \markup { \center-align \concat {
           \normal-text { \bold { \italic ( } }
           \dynamic f
           \normal-text { \bold { \italic ) } } } }
boxF = \markup { \bracket { \dynamic f } }
mfEspress = \markup { \center-align \line {
              \hspace #3.7 mf \normal-text \italic espress. } }
roundFdynamic = #(make-dynamic-script roundF)
boxFdynamic = #(make-dynamic-script boxF)
mfEspressDynamic = #(make-dynamic-script mfEspress)
\relative {
  c'4_\roundFdynamic\< d e f
  g,1~_\boxFdynamic\>
  g1
  g'1~\mfEspressDynamic
  g1
}
```

#### Sprechstimme（说唱声部标记）🔧🎵

```lilypond
speakOn = \override Stem.stencil =
  #(lambda (grob)
     (let* ((x-parent (ly:grob-parent grob X))
            (is-rest? (ly:grob? (ly:grob-object x-parent 'rest))))
       (if is-rest?
           empty-stencil
           (ly:stencil-combine-at-edge
            (ly:stem::print grob)
            Y
            (- (ly:grob-property grob 'direction))
            (grob-interpret-markup
	     grob
             (markup #:center-align #:fontsize -4
                     #:musicglyph "noteheads.s2cross"))
            -1.7))))

speakOff = \revert Stem.stencil

\new Staff {
  \relative c'' {
    a4 b a c
    \speakOn
    g4 f r g8 a
    b4 r r8 d e4
    \speakOff
    c4 a g f
  }
}
```

#### `\after` 延迟记号 🎵

```lilypond
<<
  \relative {
    \after 2 \< c'1
    d4\f\> e f g
    \after 2. \pp c,1
    \after 2. \fermata e
    \after 2. ^"Fine." f
  }
  \relative {
    \repeat unfold 12 c'4
    c c c c\fermata
    c c c c
  }
>>
```

#### 有音高颤音

```lilypond
\relative {
  \pitchedTrill
  d''2\startTrillSpan fis
  d2
  c2\stopTrillSpan
  r2
}
```

#### 扁平连线（Flat Ties）🔧🎵

```lilypond
#(define ((flared-tie coords) grob)
   (define (pair-to-list pair)
     (list (car pair) (cdr pair)))

   (define (normalize-coords goods x y dir)
     (map
      (lambda (coord)
        (cons (* x (car coord)) (* y dir (cdr coord))))
      goods))

   (define (my-c-p-s points thick)
     (make-connected-path-stencil points thick 1.0 1.0 #f #f))

   ;; Calling `ly:tie::print` and assigning its return value to a
   ;; variable in this outer `let` triggers LilyPond to position the
   ;; tie, allowing us to extract its extents.  We only proceed,
   ;; however, if the tie doesn't get discarded (for whatever reason).
   (let ((sten (ly:tie::print grob)))
     (if (grob::is-live? grob)
         (let* ((layout (ly:grob-layout grob))
                (line-thickness (ly:output-def-lookup layout
                                                      'line-thickness))
                (thickness (ly:grob-property grob 'thickness 0.1))
                (used-thick (* line-thickness thickness))
                (dir (ly:grob-property grob 'direction))
                (xex (ly:stencil-extent sten X))
                (yex (ly:stencil-extent sten Y))
                (lenx (interval-length xex))
                (leny (interval-length yex))
                (xtrans (car xex))
                (ytrans (if (> dir 0)(car yex) (cdr yex)))
                (coord-list (append coords '((1.0 . 0.0))))
                (uplist
                 (map pair-to-list
                      (normalize-coords coord-list lenx (* leny 2) dir))))
           (ly:stencil-translate
            (my-c-p-s uplist used-thick)
            (cons xtrans ytrans)))
         '())))

% 定义默认连线形状（三条直线段）
#(define flare-tie
   (flared-tie '((0.1 . 0.3) (0.9 . 0.3))))

\relative c' {
  a4~ a
  \once \override Tie.stencil = #flare-tie
  a4~ a \break

  <a c e a c e a c e>~ q
  \once \override Tie.stencil = #flare-tie
  q~ q\break

  <>^\markup \small \typewriter "height-limit = 14"
  \override Tie.details.height-limit = 14
  a'4~ a
  \once \override Tie.stencil = #flare-tie
  a4~ a \break

  <>^\markup \small \typewriter "height-limit = 0.5"
  \revert Tie.details.height-limit
  \override Tie.details.height-limit = 0.5
  a4~ a
  \once \override Tie.stencil = #flare-tie
  a4~ a \break

  <>^\markup \small \typewriter "flared-tie with \\shape"
  \revert Tie.details.height-limit
  a4~ a
  \once \override Tie.stencil = #flare-tie
  \shape #'((0.3 . 0.5) (0.3 . 0.5) (0.3 . 0.5) (0.3 . 0.5)) Tie
  a4~ a \break
}

% [补充] 使用 flared-tie 函数直接传入坐标参数
\relative c' {
  a4~ a
  \once \override Tie.stencil = #(flared-tie '((0.5 . 2)))
  a4~ a
}
```

### 2.11 无品弦乐扩展技法 🎵

#### 弓法指示

```lilypond
\relative { c''4(\downbow d) e(\upbow f) }
```

空弦指示：

```lilypond
a'4 \open
\romanStringNumbers
a'\2
a'2^\markup { \small "sul A" }
```

#### 泛音

自然泛音（菱形音符头）：

```lilypond
\relative d'' {
  d4 e4.
  \harmonicsOn
  d8 e e
  d4 e4.
  \harmonicsOff
  d8 e e
}
```

泛音圆圈标记：

```lilypond
d''2^\flageolet d''_\flageolet
```

人工泛音：

```lilypond
\relative e' {
  <e a\harmonic>2.  <c g'\harmonic>4
  \set harmonicDots = ##t
  <e a\harmonic>2.  <c g'\harmonic>4
}
```

#### Bartók 拨弦（Snap pizzicato）🎵

```lilypond
\relative {
  c'4\snappizzicato
  <c' e g>4\snappizzicato
  <c' e g>4^\snappizzicato
  <c, e g>4_\snappizzicato
}
```

#### 预定义命令速查

| 命令 | 用途 |
|---|---|
| `\upbow` | 上弓 |
| `\downbow` | 下弓 |
| `\open` | 空弦 |
| `\romanStringNumbers` | 罗马数字弦号 |
| `\harmonicsOn` / `\harmonicsOff` | 自然泛音开关 |
| `\flageolet` | 泛音圆圈标记 |
| `\harmonic` | 人工泛音 |
| `\snappizzicato` | Bartók 拨弦 |

### 2.12 阿拉伯音乐微音程 🎵

#### 使用 arabic.ly

```lilypond
\include "arabic.ly"
\relative {
  \key do \rast
  do' re misb fa | sol la sisb do | sib la sol fa | misb re do
}
```

英语音名版本：

```lilypond
\include "arabic.ly"
\language "english"
\relative {
  \key c \rast
  c' d eqf f | g a bqf c | bf a g f | eqf d c
}
```

#### 阿拉伯微音程变音记号

| 后缀 | 含义 |
|------|------|
| `sd` | 半升（Semi-sharp） |
| `sb` | 半降（Semi-flat） |
| `dsd` | 一又半升 |
| `bsb` | 一又半降 |

```lilypond
\include "arabic.ly"
\relative {
  \set Staff.extraNatural = ##f
  dod' dob dosd \dwn dob dobsb dodsd do do
}
```

#### 玛卡姆调号

常用玛卡姆分组：

| 玛卡姆组 | 调号名 | 终止音 |
|----------|--------|--------|
| ajam | `major` | sib |
| bayati | `bayati` | re |
| hijaz | `hijaz` | re |
| hijaz kar | `hijaz_kar` | do |
| huzam | `huzam` | misb |
| iraq | `iraq` | sisb |
| kurd | `kurd` | re |
| nahawand | `minor` | do |
| rast | `rast` | do |
| sikah | `sikah` | misb |

自定义玛卡姆：

```lilypond
\include "arabic.ly"

zanjaran = #`(
  (0 . ,NATURAL)
  (1 . ,FLAT)
  (2 . ,NATURAL)
  (3 . ,NATURAL)
  (4 . ,NATURAL)
  (5 . ,NATURAL)
  (6 . ,FLAT)
)

\relative {
  \key do \zanjaran
  do' reb mi fa sol la sib do
}
```

#### 阿拉伯即兴演奏

```lilypond
\include "arabic.ly"
\relative sol' {
  \key re \kurd
  \accidentalStyle forget
  \cadenzaOn
  sol4 sol sol sol fad mib sol1 fad8 mib re4. r8 mib1 fad sol
}
```

#### 阿拉伯音乐完整示例

```lilypond
\include "arabic.ly"
\score {
  \header {
    title = "Semai Muhayer"
    composer = "Jamil Bek"
  }
  \relative {
    \set Staff.extraNatural = ##f
    \set Staff.autoBeaming = ##f
    \key re \bayati
    \time 10/8

    re'4 re'8 re16 [misb re do] sisb [la sisb do] re4 r8
    re16 [misb do re] sisb [do] la [sisb sol8] la [sisb] do [re] misb
    fa4 fa16 [misb] misb8. [re16] re8 [misb] re  [do] sisb
    do4 sisb8 misb16 [re do sisb] la [do sisb la] la4 r8
  }
}
```

### 2.13 土耳其古典音乐微音程 🎵

土耳其古典音乐使用基于 **1/9音** 的微音程音程系统。

```lilypond
\include "turkish-makam.ly"

\relative {
  \set Staff.extraNatural = ##f
  \set Staff.autoBeaming = ##f

  \key a \huseyni
  \time 10/8

  a'4 g'16[ fb] e8.[ d16] d[ c d e] c[ d c8] bfc |
  a16[ bfc a8] bfc c16[ d c8] d16[ e d8] e4 fb8 |
  d4 a'8 a16[ g fb e] fb8[ g] a8.[ b16] a16[ g] |
  g4 g16[ fb] fb8.[ e16] e[ g fb e] e4 r8 |
}
```

关键要素：
- `\include "turkish-makam.ly"` 引入超过200个makam调号定义
- `fb`、`bfc` 等为土耳其微音程变音记号
- 每个makam都有其特定的"karar"（终止音/主音）

### 2.14 对象可见性控制与形状修改

#### 可见性控制

[补充] stencil、transparent、omit 三者的完整对比：

**方法一：`stencil = ##f`（原始写法）**

```lilypond
\override BarLine.stencil = ##f
a1 a
\revert BarLine.stencil
a a
```

**方法二：`\omit`（等价简写）**

```lilypond
\omit Staff.BarLine
a1 a
\undo \omit Staff.BarLine
a a
```

**方法三：`transparent = ##t`（原始写法）**

```lilypond
\override Staff.TimeSignature.transparent = ##t
a'4 a' a' a'
\revert Staff.TimeSignature.transparent
a'4 a' a' a'
```

**方法四：`\hide`（等价简写）**

```lilypond
\hide Staff.TimeSignature
a'4 a' a' a'
\undo \hide Staff.TimeSignature
a'4 a' a' a'
```

**方法五：`\omit` 对 TimeSignature 的效果**

```lilypond
\omit Staff.TimeSignature
a'4 a' a' a'
\undo \omit Staff.TimeSignature
a'4 a' a' a'
```

| 方法 | 效果 | 简写命令 | 保留占位空间 |
|------|------|---------|-------------|
| `stencil = ##f` | 对象完全消失 | `\omit` | 否 |
| `transparent = ##t` | 对象不可见但仍在 | `\hide` | 是 |
| `color = "white"` + `layer = -1` | 对象被白色覆盖 | — | 是 |

使对象透明（\hide）——对象不可见，保留占位空间：

```lilypond
a'4 a'
\once \hide NoteHead
a' a'
```

将对象涂白：

```lilypond
\override Staff.Clef.color = "white"
\override Staff.Clef.layer = -1
a'1
```

使用 whiteout 属性：

```lilypond
{
  \override Score.StaffSymbol.layer = 4
  \override Staff.TimeSignature.layer = 3
  b'2 b'~
  \once \override Staff.TimeSignature.whiteout = ##t
  \time 3/4
  b' r4
}
```

break-visibility 控制换行时可见性：

```lilypond
\relative {
  f'4 g a b
  \once \override Score.BarLine.break-visibility = ##(#f #t #t)
  \break
  f4 g a b
}
```

预定义函数对照表：

| 函数 | 向量 | 换行前 | 无换行处 | 换行后 |
|---|---|---|---|---|
| `all-visible` | `#(#t #t #t)` | 是 | 是 | 是 |
| `begin-of-line-visible` | `#(#f #f #t)` | 否 | 否 | 是 |
| `end-of-line-visible` | `#(#t #f #f)` | 是 | 否 | 否 |
| `begin-of-line-invisible` | `#(#t #t #f)` | 是 | 是 | 否 |
| `end-of-line-invisible` | `#(#f #t #t)` | 否 | 是 | 是 |
| `all-invisible` | `#(#f #f #f)` | 否 | 否 | 否 |

当设置 `stencil = ##f` 会导致错误时（如 NoteHead），使用 `#point-stencil`：

```lilypond
\relative {
  c''4 c
  \once \override NoteHead.stencil = #point-stencil
  c4 c
}
```

#### 修改形状（\shape 命令）

通过偏移量调整，位移参数为四个 `(dx . dy)` 值的列表：

```lilypond
<<
  {
    \shape #'((0 . 0.5) (0 . 0.5) (0 . 0.5) (0 . 0.5)) Tie
    e'1~ 1
  }
\\
  \relative { r4 <g' c,> <g c,> <g c,> }
>>
```

调试辅助（\vshape）：

```lilypond
\relative {
  c''8(\( a) e4 gis a\)
  \vshape #'((0 . -0.3) (0.5 . -0.2)
             (0.5 . -0.3) (0 . -0.7)) PhrasingSlur
  c8(\( a) e4 gis a\)
}
```

跨行断行的连奏线：

```lilypond
\relative c' {
  \shape #'(
             (( 0 . 0) (0 . 0) (0 . 0) (0 . 1))
             ((0.5 . 1.5) (1 . 0) (0 . 0) (0 . -1.5))
           ) Slur
  c4( f g c
  \break
  d,4 c' f, c)
}
```

S 形曲线 🎵：

```lilypond
\relative c'' {
  c8( e b-> f d' a e-> g)
  \shape #'((0 . -1) (5.5 . -0.5) (-5.5 . -10.5) (0 . -5.5))
         PhrasingSlur
  c8\( e b-> f d' a e-> g\)
}
```

---

## 第三部分：音高与节奏完整参考

### 3.1 音高输入详解

（音高输入的绝对/相对/fixed模式、变音号、语言等已在第一部分1.2节完整描述。）

教会调式名称：`\ionian`、`\dorian`、`\phrygian`、`\lydian`、`\mixolydian`、`\aeolian`、`\locrian`

八度移位记号（Ottava brackets）：

```lilypond
\relative c'' {
  a2 b
  \ottava 1
  a b
  \ottava 0
  a b
}
```

### 3.2 节奏输入详解

（节奏基本语法已在第一部分1.3节描述。）

时值范围：

```lilypond
\relative {
  \time 8/1
  c''\longa c\breve c1 c2
  c4 c8 c16 c32 c64 c128 c128
}
```

延音线详细用法：

```lilypond
{ a'2~ 4~ 16 r r8 }
```

Laissez vibrer 和重复延音线：

```lilypond
<c' f' g'>1\laissezVibrer
```

虚线/点线延音线 🎵：

```lilypond
\relative c' {
  \tieDotted
  c2~ 2
  \tieDashed
  c2~ 2
  \tieHalfDashed
  c2~ 2
  \tieHalfSolid
  c2~ 2
  \tieSolid
  c2~ 2
}
```

自定义虚线模式：

```lilypond
\relative c' {
  \tieDashPattern 0.3 0.75
  c2~ 2
  \tieDashPattern 0.7 1.5
  c2~ 2
  \tieSolid
  c2~ 2
}
```

### 3.3 拍号详解

基本设置：

```lilypond
\time 2/4
c''2
\time 3/4
c''2.
```

[补充] `\defaultTimeSignature` 恢复4/4默认拍号样式，`\numericTimeSignature` 仅显示数字拍号：

```lilypond
\relative c'' {
  \time 3/4 c4 c c
  \defaultTimeSignature
  \time 3/4 c4 c c
  \numericTimeSignature
  \time 3/4 c4 c c
}
```

[补充] `\overrideTimeSignatureSettings` 和 `\revertTimeSignatureSettings` 自定义拍号行为：

```lilypond
\overrideTimeSignatureSettings
  #'(4 . 4)           % timeSignatureFraction
  #'(1 . 4)           % baseMomentFraction
  #'(3 1)             % beatStructure
  #'()                % beamExceptions

\relative c'' {
  \time 4/4
  c8 c c c c c c c
}
```

复杂拍号 🎵：

```lilypond
\fixed c' {
  \time #'((2 2 2) . 8)
  \repeat unfold 6 c8
}
```

交替复合节拍：

```lilypond
\fixed c' {
  \time #'((1 . 4) (3 . 8))
  \repeat unfold 5 c8
}
```

子小节线：

```lilypond
\fixed c' {
  \time #'(((1 2 3) . 8) (3 . 4))
  \repeat unfold 12 c8
  \submeasureBarsOn
  \repeat unfold 12 c8
}
```

自定义节拍结构：

```lilypond
\score {
  \new Staff {
    \relative {
      \time 2,2,3 7/8
      \repeat unfold 7 { c'8 } |
      \time 3,2,2 7/8
      \repeat unfold 7 { c8 } |
    }
  }
}
```

仅打印拍号分子 🎵：

```lilypond
\relative c'' {
  \time 3/4
  c4 c c
  \override Staff.TimeSignature.style = #'single-number
  \time 2/4
  c4 c
}
```

### 3.4 速度标记

```lilypond
\relative {
  \tempo 4 = 120
  c'2 d
  e4. d8 c2
}
```

速度范围：

```lilypond
\relative {
  \tempo 4 = 40 - 46
  c'4. e8 a4 g
}
```

文字速度指示：

```lilypond
\relative {
  \tempo "Allegretto"
  c''4 e d c
}
```

使用 `\rhythm` 打印摇摆节奏标记 🎵：

```lilypond
\relative {
  \tempo \markup {
    Swing
    \hspace #0.4
    \rhythm { 8[ 8] } = \rhythm { \tuplet 3/2 { 4 8 } }
  }
  b8 g' c, d ees d16 ees d c r8
}
```

### 3.5 连奏线/延音线/连梁

（基本用法见第一部分1.9节和1.11节。）

多条同时连音线使用 `\=` 标记：

```lilypond
\fixed c' {
  <c~ f\=1( g\=2( >2 <c e\=1) a\=2) >
}
```

连音方括号替换为圆滑线 🎵：

```lilypond
\relative {
  \tuplet 3/2 4 {
    \override TupletBracket.tuplet-slur = ##t
    c'4 e8 d4 f8
    \override TupletBracket.bracket-visibility = ##t
    e f g f e d
  } c1
}
```

更改连音数字显示 🎵：

```lilypond
\relative c'' {
  \tuplet 3/2 { c8 c c }
  \tuplet 3/2 { c8 c c }
  \override TupletNumber.text = #tuplet-number::calc-fraction-text
  \tuplet 3/2 { c8 c c }
  \omit TupletNumber
  \tuplet 3/2 { c8 c c }
}
```

非默认连音数字 🎵：

```lilypond
\relative c'' {
  \once \override TupletNumber.text =
    #(tuplet-number::non-default-tuplet-denominator-text 7)
  \tuplet 3/2  { c4. c4. c4. c4. }
  \once \override TupletNumber.text =
    #(tuplet-number::non-default-tuplet-fraction-text 12 7)
  \tuplet 3/2  { c4. c4. c4. c4. }
  \once \override TupletNumber.text =
    #(tuplet-number::append-note-wrapper
      (tuplet-number::non-default-tuplet-fraction-text 12 7)
      (ly:make-duration 3 0))
  \tuplet 3/2  { c4. c4. c4. c4. }
  \once \override TupletNumber.text =
    #(tuplet-number::append-note-wrapper
      tuplet-number::calc-denominator-text
      (ly:make-duration 2 0))
  \tuplet 3/2  { c8 c8 c8 c8 c8 c8 }
  \once \override TupletNumber.text =
    #(tuplet-number::append-note-wrapper
      tuplet-number::calc-fraction-text
      (ly:make-duration 2 0))
  \tuplet 3/2  { c8 c8 c8 c8 c8 c8 }
  \once \override TupletNumber.text =
    #(tuplet-number::fraction-with-notes
      (ly:make-duration 2 1) (ly:make-duration 3 0))
  \tuplet 3/2  { c4. c4. c4. c4. }
  \once \override TupletNumber.text =
    #(tuplet-number::non-default-fraction-with-notes 12
      (ly:make-duration 3 0) 4 (ly:make-duration 2 0))
  \tuplet 3/2  { c4. c4. c4. c4. }
}
```

控制连音方括号可见性：

```lilypond
music = \relative c'' {
  \tuplet 3/2 { c16[ d e } f8]
  \tuplet 3/2 { c8 d e }
  \tuplet 3/2 { c4 d e }
}

\new Voice {
  \relative c' {
    \override Score.TextMark.non-musical = ##f
    \textMark "default" \music
    \override TupletBracket.bracket-visibility = #'if-no-beam
    \textMark \markup \typewriter "'if-no-beam" \music
    \override TupletBracket.bracket-visibility = ##t
    \textMark \markup \typewriter "#t" \music
    \override TupletBracket.bracket-visibility = ##f
    \textMark \markup \typewriter "#f" \music
    \omit TupletBracket
    \textMark \markup \typewriter "omit" \music
  }
}
```

允许在连梁连音内换行 🎵：

```lilypond
\layout {
  \context {
    \Voice
    \remove "Forbid_line_break_engraver"
    \override Beam.breakable = ##t
  }
}

\relative c'' {
  a8
  \repeat unfold 5 { \tuplet 3/2 { c8[ b g16 a] } }
  \tuplet 3/2 { c8[ b \break g16 a] }
  \repeat unfold 5 { \tuplet 3/2 { c8[ b g16 a] } }
  c8 \bar "||"
}
```

自动音符拆分：

```lilypond
\new Voice \with {
  \remove Note_heads_engraver
  \consists Completion_heads_engraver
  \remove Rest_engraver
  \consists completion_rest_engraver
}
\relative {
  c'2. c8 d4 e f g a b c8 c2 b4 a g16 f4 e d c8. c2 r1*2
}
```

[补充] 连梁细分控制——`subdivideBeams`、`beamMinimumSubdivision`、`beamMaximumSubdivision`、`respectIncompleteBeams`、`strictBeatBeaming`：

```lilypond
\relative c'' {
  \time 1/4

  <>^"default"
  c32 c c c c c c c

  <>^"with subdivision"
  \set subdivideBeams = ##t
  c32 c c c c c c c

  <>^"min 1/8"
  \once \set beamMinimumSubdivision = #1/8
  c32 c c c c c c c

  <>^"max 1/16"
  \once \set beamMaximumSubdivision = #1/16
  c32 c c c c c c c

  <>^"max 3/8"
  \once \set beamMaximumSubdivision = #3/8
  \repeat unfold 16 c64

  <>^"min 1/32, max 1/64"
  \once \set beamMinimumSubdivision = #1/32
  \once \set beamMaximumSubdivision = #1/64
  \repeat unfold 32 c128
  \break

  <>^"beams with incomplete subdivisions"
  c32 c c c c c c r32
  c32 c c c c r16.

  <>^\markup { "the same with"
               \typewriter { "respectIncomplete=#t" } }
  \set respectIncompleteBeams = ##t
  c32 c c c c c c r32
  c32 c c c c r16.
}
```

严格节拍连梁（`strictBeatBeaming`）：

```lilypond
\relative c'' {
  \time 6/8
  a8. a16 a a
  \set strictBeatBeaming = ##t
  a8. a16 a a
}
```

---

## 第四部分：谱表与多声部

### 4.1 谱表类型与谱表组

使用 `\new Staff` 创建每个谱表，用 `<< >>` 并行组合：

```lilypond
<<
  \new Staff { \clef treble c''4 }
  \new Staff { \clef bass c4 }
>>
```

谱表组类型：

| 类型 | 用途 |
|------|------|
| `\new PianoStaff << … >>` | 钢琴音乐，两行谱表通过花括号连接 |
| `\new GrandStaff << … >>` | 管弦乐总谱 |
| `\new ChoirStaff << … >>` | 声乐总谱 |

```lilypond
\new PianoStaff <<
  \new Staff \relative { \time 2/4 c''4 e | g g, | }
  \new Staff \relative { \clef bass c4 c' | e c | }
>>
```

### 4.2 多声部记谱

使用 `<< \\ >>` 构造输入多声部（`\\` 分隔不同声部）：

```lilypond
\relative {
  \key d \minor
  << { r4 g' g4. a8 }   \\ { d,2 d4 g }       >> |
  << { bes4 bes c bes } \\ { g4 g g8( a) g4 } >> |
  << { a2. r4 }         \\ { fis2. r4 }       >> |
}
```

也可将每个声部完整拆分。隐式创建的声部分别命名为 `"1"`、`"2"` 等，奇数声部符干朝上，偶数朝下。

三个或更多声部：

```lilypond
\new Staff \relative {
  c'16 d e f
  << { g4 f e } \\ { r8 e4 d c8~ } >> |
  << { d2 e }   \\ { c8 b16 a b8 g~ 2 } \\ { s4 b c2 } >> |
}
```

使用 `\voices` 命令指定声部编号顺序：

```lilypond
\new Staff \relative {
  c'16 d e f
  << { g4 f e } \\ { r8 e4 d c8~ } >> |
  \voices 1,3,2
  << { d2 e }   \\ { s4 b c2 } \\ { c8 b16 a b8 g~ 2 } >> |
}
```

`\relative` 在复调中的计算：每个音符相对于紧接在它前面的音符计算，跨声部不影响。

> 注意：歌词、连奏线、延音线、渐强渐弱等不能"跨越"声部创建。

### 4.3 上下文与雕刻器

上下文层级：`Score` → `Staff` → `Voice`

- `Score` 处理全局规则（如小节线同步）
- `Staff` 处理谱表级别规则（如变音号显示）
- `Voice` 处理声部级别规则（如引入变音号）

其他上下文：`PianoStaff`、`ChoirStaff`、`Lyrics`、`ChordNames` 等。

命名规则：每个单词首字母大写，无连字符或下划线，如 `GregorianTranscriptionStaff`。

#### 雕刻器

常见雕刻器：

| 雕刻器 | 功能 |
|--------|------|
| `Accidental_engraver` | 产生变音记号 |
| `Beam_engraver` | 产生连梁 |
| `Clef_engraver` | 产生谱号 |
| `Completion_heads_engraver` | 分割跨小节线的音符 |
| `Dynamic_engraver` | 产生渐强渐弱记号和力度文本 |
| `Key_engraver` | 产生调号 |
| `Note_heads_engraver` | 产生音符头 |
| `Rest_engraver` | 产生休止符 |
| `Staff_symbol_engraver` | 产生五线谱线 |
| `Stem_engraver` | 产生符干 |

命名规则：第一个单词首字母大写，其余单词用下划线连接。

#### 修改上下文属性

```
\set ContextName.propertyName = value
```

布尔值：`##t`（真）、`##f`（假）。必须指定正确的上下文。

#### 添加和移除雕刻器

从单个上下文移除：

```lilypond
\new Staff \with {
  \remove Staff_symbol_engraver
}
\relative { c'4 d e f | }
```

添加到单个上下文：

```lilypond
\new Staff \with {
  \consists Ambitus_engraver
}
<<
  \new Voice { \relative { \voiceOne c''4 a b g } }
  \new Voice { \relative { \voiceTwo c'4 e d f } }
>>
```

在 `\layout` 块中为所有同类上下文添加：

```lilypond
\score {
  <<
    \new Staff { \relative { c''4 a b g } }
    \new Staff { \relative { c'4 a b g } }
  >>
  \layout {
    \context {
      \Staff
      \consists Ambitus_engraver
    }
  }
}
```

### 4.4 歌词

（基本用法见第一部分1.16节。）

---

## 第五部分：Scheme编程与程序化生成 🔧

### 5.1 Scheme语言基础 🔧

LilyPond 使用 Scheme 编程语言（GNU Guile 实现，基于 R5RS 标准）。

#### Scheme 沙盒

```bash
lilypond scheme-sandbox
```

启动后得到 Guile 提示符：`lily-guile@()>`

可使用 `rlwrap` 改善编辑体验：`rlwrap lilypond scheme-sandbox`

#### 变量与数据类型

```scheme
guile> (define a 2)
guile> a
2
guile> (set! a 12345)
```

Pairs（对）：

```scheme
guile> (define mypair (cons 123 "hello there"))
guile> (car mypair)
123
guile> (cdr mypair)
"hello there"
```

Lists（列表）：

```scheme
guile> (list 1 2 3 "abc" 17.5)
(1 2 3 "abc" 17.5)
guile> '(17 23 "foo" "bar" "bazzle")
(17 23 "foo" "bar" "bazzle")
```

#### 计算

Scheme 使用**前缀语法**：

```scheme
guile> (+ 1 2)
3
guile> (+ 1 (* 3 4))
13
```

#### 过程定义

```scheme
(define (function-name arg1 arg2 … argn)
  scheme-expression-that-gives-a-return-value)

guile> (define (average x y) (/ (+ x y) 2))
guile> (average 3 12)
15/2
```

#### 条件表达式

`if` 条件：

```scheme
(if test-expression true-expression false-expression)

guile> (if (> a b) "a is greater than b" "a is not greater than b")
```

`cond` 条件：

```scheme
(cond (test-expression-1 result-expression-sequence-1)
      (test-expression-2 result-expression-sequence-2)
      …)
```

### 5.2 LilyPond中的Scheme 🔧

#### 语法标记

- `#` — 最推荐的方式，让词法分析器读取完整 Scheme 表达式，由解析器在适当时机求值
- `$` — 在词法分析器读取后**立即**求值，并创建值的副本
- `$@` 和 `#@` — 列表拼接操作符，将列表的所有元素插入到周围上下文中

Scheme 代码中的注释（LilyPond 注释 `%%` `%{ %}` **不能**在 Scheme 代码中使用）：

```scheme
; 单行注释

#!
  Guile 风格块注释
!#
```

合并顶层 Scheme 表达式：

```scheme
#(begin
  (define foo 0)
  (define bar 1))
```

#### LilyPond 变量与 Scheme 变量

LilyPond 变量在内部以 Scheme 变量存储：

```lilypond
twelve = 12
```

等价于：

```lilypond
#(define twelve 12)
```

可以在 Scheme 表达式中使用：

```lilypond
twentyFour = #(* 2 twelve)
```

当使用反斜杠 `\twentyFour` 引用音乐变量时，LilyPond 会创建该变量音乐值的**副本**。

#### 调试 Scheme 代码

```scheme
#(ly:set-option 'compile-scheme-code)   ;; Scheme 代码出错时显示精确行号
#(debug-enable 'backtrace)              ;; 启用详细回溯
```

命令行选项：`-dcompile-scheme-code`、`-ddebug-eval`

#### 输入变量与 Scheme

```lilypond
traLaLa = { c'4 d'4 }

#(define newLa (map ly:music-deep-copy
  (list traLaLa traLaLa)))
#(define twice
  (make-sequential-music newLa))

\twice
```

使用 `$` 将 Scheme 值导入为 LilyPond 语法：

```scheme
$(make-sequential-music newLa)
```

列表拼接：

```scheme
{ #@newLa }
```

#### 对象属性

对象属性以 alist-chains（关联列表链）形式存储：

```lilypond
\override Stem.thickness = #2.6
```

向属性列表添加条目 `'(thickness . 2.6)`。

#### LilyPond 的盒模型

经典盒模型包含 width、height、depth。LilyPond 增加了第四个维度 **breapth**（参考点左侧的水平大小）。

不参与间距算法的盒子，将范围设为 `#'(+inf.0 . -inf.0)`。

### 5.3 音乐函数/事件函数/标记函数 🔧

#### 音乐函数定义

```scheme
function =
#(define-music-function
     (arg1 arg2 …)
     (type1? type2? …)
   body)
```

| 组成部分 | 说明 |
|---|---|
| `function` | 函数名称 |
| `#(define-music-function` | Scheme 宏 |
| `(arg1 arg2 …)` | 参数列表 |
| `(type1? type2? …)` | 类型谓词列表 |
| `body` | 函数体，通常为 LilyPond 代码块 |

音乐函数必须返回与 `ly:music?` 匹配的表达式。

#### 事件函数

事件函数不需要方向指示符，适合力度命令等场景：

```scheme
dyn = #(define-event-function (arg) (markup?)
         (make-dynamic-script arg))
```

```lilypond
\relative { c'\dyn pfsss }
```

#### 标记函数定义

```scheme
(define-markup-command (command-name layout props arg1 arg2 …)
    (arg1-type? arg2-type? …)
    [ #:properties ((property1 default-value1) …) ]
    [ #:as-string expression ]
  …command body…)
```

### 5.4 回调函数与grob-transformer 🔧

属性可以设置为 Scheme 过程（回调），在排版时动态计算。

基本示例——根据符干方向改变粗细：

```lilypond
\override Stem.thickness = #(lambda (grob)
    (if (= UP (ly:grob-property grob 'direction))
        2.0
        7.0))
\relative { c'' b a g b a g b }
```

获取默认值——直接调用默认回调函数：

```lilypond
\relative {
  \override Flag.X-offset = #(lambda (flag)
    (let ((default (ly:flag::calc-x-offset flag)))
      (* default 4.0)))
  c''4. d8 a4. g8
}
```

使用 `grob-transformer` 获取默认值：

```lilypond
\relative {
  \override Flag.X-offset = #(grob-transformer 'X-offset
    (lambda (flag default) (* default 4.0)))
  c''4. d8 a4. g8
}
```

在回调中解释 Markup：

```lilypond
my-callback = #(lambda (grob)
                 (grob-interpret-markup grob (markup "foo")))
```

### 5.5 构建复杂函数实例 🔧

**问题**：使 espressivo 记号变宽但不变高。

**步骤 1**：用颜色验证正确的 grob

```lilypond
colorDefaultStencil =
\once \override Script.stencil =
  #(lambda (grob)
     (let ((stil (ly:script-interface::print grob)))
       (stencil-with-color stil red)))

{ \colorDefaultStencil f'\espressivo }
```

**步骤 2**：使用 `grob-transformer`

```lilypond
colorDefaultStencil =
\once \override Script.stencil =
  #(grob-transformer 'stencil
     (lambda (grob orig)
       (stencil-with-color orig red)))

{ \colorDefaultStencil f'\espressivo }
```

**步骤 3**：使用 `ly:stencil-scale` 缩放

```lilypond
scaleColorDefaultStencil =
\once \override Script.stencil =
  #(grob-transformer 'stencil
     (lambda (grob orig)
       (stencil-with-color
        (ly:stencil-scale orig 2 1)
        red)))

{ \scaleColorDefaultStencil f'\espressivo }
```

**步骤 4**：定义事件函数

```lilypond
#(define (longer-script x y)
   (grob-transformer 'stencil
     (lambda (grob orig)
       (ly:stencil-scale orig x y))))

longEspressivo =
#(define-event-function (x-val) (number?)
  #{
    \tweak stencil #(longer-script x-val 1)
    \espressivo
  #})

{ f'^\longEspressivo #2 _\fermata }
```

旋转 stencil 90度：

```lilypond
#(define rotate-90
   (grob-transformer 'stencil
     (lambda (grob orig)
       (ly:stencil-rotate orig 90 0 0))))

{ f'8( g')
  f'8-\tweak stencil #rotate-90 ( g')
  f'8[ g']
  f'8-\tweak stencil #rotate-90 [ g'] }
```

### 5.6 上下文编程 🔧

#### 上下文求值

语法：`\applyContext function`（LilyPond代码块中）

`function` 接受**单个参数**：调用时所在的上下文。

相关 Scheme 函数：

| 函数 | 用途 |
|---|---|
| `ly:context-property` | 查找 context 属性值 |
| `ly:context-set-property!` | 设置 context 属性 |
| `ly:context-grob-definition` + `ly:assoc-get` | 查找 grob 属性值 |
| `ly:context-pushpop-property` | 执行 `\temporary \override` 或 `\revert` |

示例：将 fontSize 值加倍

```lilypond
doubleFontSize =
\applyContext
  #(lambda (context)
     (let ((fontSize (ly:context-property context 'fontSize)))
       (ly:context-set-property! context 'fontSize (+ fontSize 6))))

{
  \set fontSize = #-3
  b'4
  \doubleFontSize
  b'
}
```

示例：降低 NoteHead、Stem、Beam 颜色饱和度

```lilypond
desaturate =
\applyContext
  #(lambda (context)
     (define (desaturate-grob grob)
       (let* ((grob-def (ly:context-grob-definition context grob))
              (color (ly:assoc-get 'color grob-def black))
              (new-color (map (lambda (x) (min 1 (/ (1+ x) 2))) color)))
         (ly:context-pushpop-property context grob 'color new-color)))
     (for-each desaturate-grob '(NoteHead Stem Beam)))

\relative {
  \time 3/4
  g'8[ g] \desaturate g[ g] \desaturate g[ g]
  \override NoteHead.color = "darkred"
  \override Stem.color = "darkred"
  \override Beam.color = "darkred"
  g[ g] \desaturate g[ g] \desaturate g[ g]
}
```

### 5.7 Unpure-pure 容器 🔧

用于使用 Scheme 函数覆盖 Y 轴间距计算（`Y-offset` 和 `Y-extent`）。

```scheme
(ly:make-unpure-pure-container f0 f1)
```

- `f0`：接受 n 个参数（n >= 1），第一个参数为 grob，给出实际结果
- `f1`：被标记为"纯"的函数，接受 n + 2 个参数（grob, start, end, ...），提供近似值

完整示例：

```lilypond
#(define (square-line-circle-space grob)
   (let* ((pitch (ly:event-property (ly:grob-property grob 'cause)
                                    'pitch))
         (notename (ly:pitch-notename pitch)))
     (if (= 0 (modulo notename 2))
         (make-circle-stencil 0.5 0.0 #t)
         (make-filled-box-stencil '(0 . 1.0)
                                  '(-0.5 . 0.5)))))

squareLineCircleSpace = {
  \override NoteHead.stencil = #square-line-circle-space
}

smartSquareLineCircleSpace = {
  \squareLineCircleSpace
  \override NoteHead.Y-extent =
   #(ly:make-unpure-pure-container
      ly:grob::stencil-height
      (lambda (grob start end) (ly:grob::stencil-height grob)))
}

\new Voice \with { \remove Stem_engraver }
\relative c'' {
  \squareLineCircleSpace
  cis4 ces disis d
  \smartSquareLineCircleSpace
  cis4 ces disis d
}
```

### 5.8 内部音乐表示 🔧

音乐在内部表示为 **Scheme 列表**，包含影响打印输出的各种元素。

#### 三种类型

1. **音乐名称**：每个表达式都有名称（如 `NoteEvent`、`SimultaneousMusic`）
2. **'type' 或接口**：如音符是 `event`、`note-event`、`rhythmic-event`、`melodic-event`
3. **C++ 对象**：由 `Music` 类表示

#### 属性

- `NoteEvent` 具有 `pitch` 和 `duration` 属性
- 复合表达式的子对象存储方式：
  - `elements`：存储子音乐对象的**列表**（如 `SequentialMusic`）
  - `element`：存储**单个**子音乐对象（如 `GraceMusic`）

#### 使用 `\displayMusic` 查看内部结构

```lilypond
{
  \displayMusic { c'4\f }
}
```

输出：

```scheme
(make-music
  'SequentialMusic
  'elements
  (list (make-music
          'NoteEvent
          'articulations
          (list (make-music
                  'AbsoluteDynamicEvent
                  'text
                  "f"))
          'duration
          (ly:make-duration 2 0 1/1)
          'pitch
          (ly:make-pitch 0 0 0))))
```

输出到文件：

```lilypond
{
  port = #(open-output-file "display.txt")
  \displayMusic \port { c'4\f }
  \displayMusic \port { d'4 }
  #(close-output-port port)
}
```

访问音高属性：

```scheme
#(display-scheme-music
   (ly:music-property (first (ly:music-property someNote 'elements))
                      'pitch))
===>
(ly:make-pitch 0 0 0)
```

修改音高属性：

```scheme
#(set! (ly:music-property (first (ly:music-property someNote 'elements))
                          'pitch)
       (ly:make-pitch 0 1 0)) ;; 设置音高为 d'
\displayLilyMusic \someNote
===>
d'4
```

#### 用连音加倍音符 🔧

```scheme
doubleSlur = #(define-music-function (note) (ly:music?)
  "Return: { note ( note) }.
  `note` is supposed to be a single note."
  (let ((note2 (ly:music-deep-copy note)))
    (set! (ly:music-property note 'articulations)
          (cons (make-music 'SlurEvent 'span-direction -1)
                (ly:music-property note 'articulations)))
    (set! (ly:music-property note2 'articulations)
          (cons (make-music 'SlurEvent 'span-direction 1)
                (ly:music-property note2 'articulations)))
    (make-music 'SequentialMusic 'elements (list note note2))))
```

#### 为音符添加奏法 🔧

```scheme
addAccent = #(define-music-function (note-event) (ly:music?)
  "Add an accent ArticulationEvent to the articulations of `note-event`,
  which is supposed to be a NoteEvent expression."
  (set! (ly:music-property note-event 'articulations)
        (cons (make-music 'ArticulationEvent
                'articulation-type 'accent)
              (ly:music-property note-event 'articulations)))
  note-event)
```

> **重要契约**：音乐函数被允许修改其参数，但返回值不会被复制。因此不能多次使用同一参数。

### 5.9 常用Scheme API速查 🔧

#### 音乐对象操作

| 函数 | 用途 |
|------|------|
| `ly:music-property music symbol` | 获取音乐属性 |
| `ly:music-deep-copy music` | 深度复制音乐对象 |
| `make-music name properties…` | 创建音乐对象 |
| `make-sequential-music list` | 创建顺序音乐 |
| `ly:make-pitch octave note alter` | 创建音高 |
| `ly:make-duration log dotcount` | 创建时长 |
| `scorify-music music` | 将音乐转换为 score |

#### Grob 操作

| 函数 | 用途 |
|------|------|
| `ly:grob-property grob symbol` | 获取 grob 属性 |
| `ly:grob-set-property! grob symbol value` | 设置 grob 属性 |
| `ly:grob-original grob` | 获取原始 grob（跨行时） |
| `ly:grob-parent grob axis` | 获取父 grob |
| `ly:grob-object grob symbol` | 获取关联 grob |
| `grob-interpret-markup grob markup` | 在回调中解释 markup |
| `grob-transformer property func` | 创建 grob 变换回调 |

#### Stencil 操作

| 函数 | 用途 |
|------|------|
| `ly:stencil-scale stil x y` | 缩放 stencil |
| `ly:stencil-rotate stil angle x y` | 旋转 stencil |
| `ly:stencil-extent stil axis` | 获取 stencil 范围 |
| `ly:stencil-combine-at-edge stil axis dir stil padding` | 组合 stencil |
| `stencil-with-color stil color` | 着色 stencil |
| `make-connected-path-stencil points thick x-scale y-scale connect fill` | 创建路径 stencil |
| `make-circle-stencil radius thickness fill` | 创建圆形 stencil |
| `make-filled-box-stencil xext yext` | 创建填充矩形 stencil |

#### 上下文操作

| 函数 | 用途 |
|------|------|
| `ly:context-property context symbol` | 获取上下文属性 |
| `ly:context-set-property! context symbol value` | 设置上下文属性 |
| `ly:context-grob-definition context symbol` | 获取 grob 定义 |
| `ly:context-pushpop-property context grob symbol value` | 临时覆盖/还原属性 |

#### 函数定义宏

| 宏 | 用途 |
|---|---|
| `define-music-function` | 定义音乐函数 |
| `define-event-function` | 定义事件函数 |
| `define-void-function` | 定义无返回值函数 |
| `define-markup-command` | 定义标记命令 |
| `grob-transformer` | 创建 grob 变换回调 |

#### 其他重要函数

| 函数 | 用途 |
|------|------|
| `display-scheme-music` | 显示 Scheme 音乐表示 |
| `ly:parser-define! symbol value` | 定义解析器变量 |
| `ly:parser-lookup symbol` | 查找解析器变量 |
| `ly:output-def-clone def` | 克隆输出定义 |
| `ly:score-add-output-def! score def` | 为 score 添加输出定义 |
| `ly:make-unpure-pure-container f0 f1` | 创建 unpure-pure 容器 |
| `ly:set-option symbol value` | 设置 LilyPond 选项 |

### 5.10 实用代码片段

#### 生成随机音符 🔧

```lilypond
randomNotes =
#(define-music-function (n from to dur)
   (integer? ly:pitch? ly:pitch? ly:duration?)
   (let ((from-step (ly:pitch-steps from))
         (to-step (ly:pitch-steps to)))
     (make-sequential-music
      (map (lambda (_)
             (let* ((step (+ from-step
                             (random (- to-step from-step))))
                    (pitch (ly:make-pitch 0 step 0)))
               #{ $pitch $dur #}))
           (iota n)))))

\randomNotes 24 c' g'' 8
```

参数：n=音符数量，from/to=音高范围，dur=时值

#### 根据音高为音符着色 🔧

```lilypond
% 音高到颜色的关联列表
#(define color-mapping
   (list
    (cons (ly:make-pitch 0 0 NATURAL) (x11-color 'red))
    (cons (ly:make-pitch 0 0 SHARP) (x11-color 'green))
    (cons (ly:make-pitch 0 1 FLAT) (x11-color 'green))
    (cons (ly:make-pitch 0 2 NATURAL) (x11-color 'red))
    (cons (ly:make-pitch 0 2 SHARP) (x11-color 'green))
    (cons (ly:make-pitch 0 3 FLAT) (x11-color 'red))
    (cons (ly:make-pitch 0 3 NATURAL) (x11-color 'green))
    (cons (ly:make-pitch 0 4 SHARP) (x11-color 'red))
    (cons (ly:make-pitch 0 5 NATURAL) (x11-color 'green))
    (cons (ly:make-pitch 0 5 FLAT) (x11-color 'red))
    (cons (ly:make-pitch 0 6 SHARP) (x11-color 'red))
    (cons (ly:make-pitch 0 1 NATURAL) (x11-color 'blue))
    (cons (ly:make-pitch 0 3 SHARP) (x11-color 'blue))
    (cons (ly:make-pitch 0 4 FLAT) (x11-color 'blue))
    (cons (ly:make-pitch 0 5 SHARP) (x11-color 'blue))
    (cons (ly:make-pitch 0 6 FLAT) (x11-color 'blue))))

% 比较音高和变音（忽略八度）
#(define (pitch-equals? p1 p2)
   (and
    (= (ly:pitch-alteration p1) (ly:pitch-alteration p2))
    (= (ly:pitch-notename p1) (ly:pitch-notename p2))))

#(define (pitch-to-color pitch)
   (let ((color (assoc pitch color-mapping pitch-equals?)))
     (if color
         (cdr color))))

#(define (color-notehead grob)
   (pitch-to-color
    (ly:event-property (event-cause grob) 'pitch)))

\score {
  \new Staff \relative c' {
    \override NoteHead.color = #color-notehead
    c8 b d dis ees f g aes
  }
}
```

#### 在 Scheme 中生成完整乐谱 🔧

```lilypond
#(define-public (add-score score)
   (ly:parser-define! 'toplevel-scores
                      (cons score (ly:parser-lookup 'toplevel-scores))))

#(define-public (add-text text)
   (add-score (list text)))

#(define-public (add-music music)
   (collect-music-aux (lambda (score)
                        (add-score score))
                      music))

#(define-public (toplevel-book-handler book)
   (map (lambda (score)
          (ly:book-add-score! book score))
        (reverse! (ly:parser-lookup 'toplevel-scores)))
   (ly:parser-define! 'toplevel-scores (list))
   (print-book-with-defaults book))

% 示例：生成一系列单音符乐谱
#(define add-one-note-score #f)
#(let ((pitch 0))
   (set! add-one-note-score
         (lambda ()
           (let* ((music
                   (make-music
                    'EventChord
                    'elements (list (make-music
                                     'NoteEvent
                                     'duration (ly:make-duration 2 0 1/1)
                                     'pitch (ly:make-pitch 0 pitch 0)))))
                  (score (scorify-music music))
                  (layout (ly:output-def-clone $defaultlayout))
                  (note-name (case pitch
                               ((0) "do")
                               ((1) "ré")
                               ((2) "mi")
                               ((3) "fa")
                               ((4) "sol")
                               ((5) "la")
                               ((6) "si")
                               (else "huh")))
                  (title (markup #:large #:line
                                 ("Score with a" note-name))))
             (ly:score-add-output-def! score layout)
             (add-text title)
             (add-score score))
           (set! pitch (modulo (1+ pitch) 7)))))

oneNoteScore =
#(define-void-function () ()
   (add-one-note-score))

\book {
  \oneNoteScore
  \paper { tagline = ##f }
}
```

#### 创建不同音高的音符序列 🔧

```lilypond
rhythm =
#(define-music-function (p) (ly:pitch?)
   "Make the rhythm in Mars (the Planets) at the given pitch"
  #{ \tuplet 3/2 { $p 8 8 8 } 4 4 8 8 4 #})

\new Staff {
  \time 5/4
  \rhythm c'
  \rhythm c''
  \rhythm g
}
```

---

## 第六部分：调整输出与布局

### 6.1 override/revert/once/tweak/offset/single

#### \override 命令

修改布局对象属性：

```
\override Context.LayoutObject.layout-property = value
```

```lilypond
\relative {
  c'4 d
  \override NoteHead.color = "red"
  e4 f |
  \override NoteHead.color = "green"
  g4 a b c |
}
```

#### \revert 命令

恢复属性到原始默认值：

```lilypond
\relative {
  c'4 d
  \override NoteHead.color = "red"
  e4 f |
  \override NoteHead.color = "green"
  g4 a
  \revert NoteHead.color
  b4 c |
}
```

#### \once 前缀

使命令仅在当前音乐时刻生效：

```lilypond
\relative {
  c'4 d
  \override NoteHead.color = "red"
  e4 f |
  \once \override NoteHead.color = "green"
  g4 a
  \once \revert NoteHead.color
  b c |
  \revert NoteHead.color
  f2 c |
}
```

也可用于预定义命令：

```lilypond
\relative {
  c'4( d)
  \once \slurDashed
  e4( f) |
  g4( a)
  \once \hideNotes
  b( c) |
}
```

#### \tweak 和 \offset 命令

用于同时刻的多个对象中只修改选定的对象（如和弦中的单个音符）。

[补充] `\override` 与 `\tweak` 的区别——`\once \override` 影响同时刻所有音符，`\tweak` 只影响指定对象：

```lilypond
\relative {
  <c' e g>4
  \once \override NoteHead.font-size = -3
  <c e g>4
  <c e g>4
}
```

`\tweak` 作用于输入流中紧随其后的项目：

```lilypond
\relative {
  <c' e g>4
  <c \tweak font-size -3 e g>4
}
```

[补充] `\tweak` 语法格式：

```lilypond
\tweak layout-property value
\offset layout-property value
```

[补充] `\tweak` 修饰文本标记：

```lilypond
a'4^"Black"
  -\tweak color "red" ^"Red"
  -\tweak color "green" _"Green"
```

[补充] 指定 LayoutObject 的 `\tweak`/`\offset` 语法：

```lilypond
\tweak LayoutObject.layout-property value
\offset LayoutObject.layout-property value
```

和弦中 `\tweak` 指定 Accidental 颜色（字符串形式）：

```lilypond
<\tweak Accidental.color "red"   cis''4
 \tweak Accidental.color "green" es''
 g''>
```

`\offset` 相对于默认值偏移：

```lilypond
\relative c'' {
  c4
  \breathe
  c4
  \offset Y-offset 2 \breathe
  c2
  \tweak Y-offset 4 \breathe
}
```

修改和弦中单个符头：

```lilypond
< c''
  \tweak color "red"
  d''
  g''
  \tweak duration-log 1
  a''
> 4
```

修改间接创建的对象（需显式指定 grob 名称）：

```lilypond
\tweak Stem.color #(universal-color 'orange)
\tweak Beam.color #(universal-color 'skyblue) c''8 e''
<c'' e'' \tweak Accidental.font-size -3 ges''>4
```

多个 tweak 叠加：

```lilypond
c'
  -\tweak springs-and-rods #ly:spanner::set-spacing-rods
  -\tweak minimum-length 15
  -\tweak style #'dashed-line
  -\tweak dash-fraction 0.2
  -\tweak thickness 3
  -\tweak color #red
  \glissando
f''
```

#### \single 前缀

将 override 转换为 tweak，仅影响特定 grob：

```lilypond
emphNoteHead = {
  \override NoteHead.color = "red"
  \override NoteHead.font-size = 2
}
\relative {
   <c'' a \single \emphNoteHead f d>4
}
```

`\single` **不会**将 `\revert`、`\set` 或 `\unset` 转换为 tweak。

### 6.2 可见性与颜色

（可见性控制的完整内容见第二部分2.14节。）

#### color 属性

使用预定义 CSS 颜色名称（字符串形式）：

```lilypond
\override Staff.BarLine.color = "white"
```

使用十六进制颜色代码：

```lilypond
\override Staff.BarLine.color = "#FFFFFF"
```

使用 `rgb-color` 函数（参数范围 0-1）：

```lilypond
\override Staff.BarLine.color = #(rgb-color 1 1 1)
```

使用 `x11-color` 函数（更多颜色选择，颜色名称为符号形式）：

```lilypond
\override Staff.StaffSymbol.color = #(x11-color 'grey30)
\override Staff.TimeSignature.color = #(x11-color 'grey60)
\override Staff.Clef.color = #(x11-color 'grey60)
\override Voice.NoteHead.color = #(x11-color 'grey85)
\override Voice.Stem.color = #(x11-color 'grey85)
\override Staff.BarLine.color = #(x11-color 'grey10)
```

将颜色定义为变量：

```lilypond
whiteVar = "#FFFFFF"

\relative {
  \override Staff.BarLine.color = \whiteVar
  c''4
  \override Staff.BarLine.color = #whiteVar
  c4
}
```

### 6.3 对象放置与碰撞修复

#### direction 属性

控制对象放置方向：

```lilypond
a'4( g') c''( a') |
\override Slur.direction = #DOWN
a'4( g') c''( a') |
```

方向常量：`DOWN` = `-1`，`UP` = `+1`，`CENTER` = `0`

预定义方向命令：

| 向下 | 向上 | 还原 | 效果 |
|------|------|------|------|
| `\dotsDown` | `\dotsUp` | `\dotsNeutral` | 附点方向 |
| `\dynamicDown` | `\dynamicUp` | `\dynamicNeutral` | 力度方向 |
| `\phrasingSlurDown` | `\phrasingSlurUp` | `\phrasingSlurNeutral` | 乐句连线方向 |
| `\slurDown` | `\slurUp` | `\slurNeutral` | 连奏线方向 |
| `\stemDown` | `\stemUp` | `\stemNeutral` | 符干方向 |
| `\textSpannerDown` | `\textSpannerUp` | `\textSpannerNeutral` | 文本跨度线方向 |
| `\tieDown` | `\tieUp` | `\tieNeutral` | 延音线方向 |
| `\tupletDown` | `\tupletUp` | `\tupletNeutral` | 连音组方向 |

方向指示符 `^`（强制向上）和 `_`（强制向下）：

```lilypond
a'4( g') c''( a') |
a'4^( g') c''_( a') |
```

#### 碰撞修复关键属性

- **`padding`**：增加对象周围的间距
- **`right-padding`**：控制对象右侧的间距
- **`staff-padding`**：控制对象与谱表之间的最小距离
- **`self-alignment-X`**：调整对象水平对齐方式
- **`staff-position`**：控制对象在谱表上的垂直位置
- **`extra-offset`**：直接手动设置 X/Y 偏移量（完全控制，但不改变原本占用的空间）
- **`positions`**：控制连音线、连杠等的垂直位移
- **`force-hshift`**：强制改变对象的水平位移

`extra-offset` 属性示例：

```lilypond
f'4-5
\once \override Fingering.extra-offset = #'(-0.3 . -1.8)
f'4-5
```

偏移量格式：`'(水平 . 垂直)`，单位为谱表空间。负值水平向左/垂直向下。

> **注意**：`extra-offset` 不改变对象原本占用的空间，可能导致重叠。建议在其他方法无法满足时使用。

### 6.4 垂直/水平间距

- **StaffGroup**（如 `GrandStaff`、`PianoStaff`）内部间距由 `StaffGrouper` 的间距变量控制
- **未分组的谱表**（如 `Lyrics` 和 `Staff`）之间的间距由 `VerticalAxisGroup` 的变量控制

调整歌词与谱表的间距：

```lilypond
\new Lyrics \with {
  \override VerticalAxisGroup
            .nonstaff-relatedstaff-spacing.padding = 2
  \override VerticalAxisGroup
            .nonstaff-unrelatedstaff-spacing.padding = 2
}
```

调整合唱谱与钢琴谱之间的间距：

```lilypond
\new ChoirStaff \with {
  \override StaffGrouper
            .staffgroup-staff-spacing.basic-distance = 15
}
```

调整钢琴谱内部两行谱表之间的间距：

```lilypond
\new PianoStaff \with {
  \override StaffGrouper.staff-staff-spacing =
              #'((basic-distance . 0)
                 (padding . 0))
}
```

关键属性汇总：

| 用途 | 属性路径 |
|------|---------|
| 歌词与关联谱表的间距 | `VerticalAxisGroup.nonstaff-relatedstaff-spacing.padding` |
| 歌词与非关联谱表的间距 | `VerticalAxisGroup.nonstaff-unrelatedstaff-spacing.padding` |
| 合唱组与下方谱表的间距 | `StaffGrouper.staffgroup-staff-spacing.basic-distance` |
| 钢琴谱内部谱表之间的间距 | `StaffGrouper.staff-staff-spacing` |

### 6.5 困难调整 🔧

#### 跨行断点延音对象调整

延音对象跨换行时会被克隆，`\override` 影响所有部分。使用 `after-line-breaking` 回调单独修改某一部分：

```scheme
#(define (my-callback grob)
   (let* (
          (orig (ly:grob-original grob))
          (siblings (if (ly:grob? orig)
                        (ly:spanner-broken-into orig)
                        '())))
     (if (and (>= (length siblings) 2)
              (eq? (car (last-pair siblings)) grob))
         (ly:grob-set-property! grob 'extra-offset '(1 . -4)))))
```

```lilypond
\relative {
  \override Tie.after-line-breaking =
  #my-callback
  c''1 ~ \break
  c2 ~ 2
}
```

#### 无法使用 `\override` 更改的对象

使用 `\overrideProperty`：

```lilypond
\overrideProperty [ContextName].GrobName.property-name[.subproperty-name] value
```

> `\overrideProperty` 正在被更常用的 `\once \override` 命令取代。

### 6.6 样式表工作流 🔧

[补充] 使用样式表集中管理排版设置，通过 `\include` 引入。

**definitions.ily** — 集中定义所有样式变量：

```lilypond
% definitions.ily
mpdolce = #(make-dynamic-script
  #{ \markup { \hspace #0.1 \italic mp \hspace #0.1 dolce } #})

inst =
#(define-music-function (string) (string?)
   #{ \markup \bold \box #string #})
```

**web-publish.ily** — 网页发布样式：

```lilypond
% web-publish.ily
\include "definitions.ily"

#(set-global-staff-size 23)

\paper {
  paper-height = 140
  paper-width = 200
  indent = 0
}

\layout {
  \context {
    \Score
    \override SpacingSpanner.spacing-increment = 5
  }
}

\header {
  tagline = ##f
}

\layout {
  \context {
    \Voice
    \override DynamicText.stencil =
      #(grob-transformer 'stencil
         (lambda (grob orig)
           (stencil-with-color orig "blue")))
  }
}
```

**global.ily** — 使用样式：

```lilypond
% global.ily
\include "web-publish.ily"

\layout {
  \context {
    \Voice
    \override TextScript.stencil =
      #(grob-transformer 'stencil
         (lambda (grob orig)
           (stencil-with-color orig "darkred")))
  }
}
```

**主文件** — 通过 `\include` 引入样式：

```lilypond
\include "global.ily"

\relative {
  c'4^\inst "Clarinet" d e f
  g,4\mpdolce a b c
}
```

> 这种方式将所有样式定义与音乐内容分离，方便在打印版和网络版之间切换。

---

## 第七部分：命令行与外部工具

### 7.1 运行lilypond

基本调用语法：

```bash
lilypond [option]… file…
```

**文件扩展名**：无扩展名时自动添加 `.ly`。从 stdin 读取使用 `-`。

**输出文件**：处理 `filename.ly` 时默认生成 `filename.pdf`。

**批量处理**：

```bash
lilypond *.ly
```

**重定向输出**：

```bash
lilypond file.ly 1> stdout.txt   # 仅正常输出
lilypond file.ly 2> stderr.txt   # 仅错误信息
lilypond file.ly &> all.txt      # 所有输出
```

**递归处理（macOS/Linux）**：

```bash
find . -name '*.ly' -exec lilypond '{}' \;
```

**递归处理（Windows）**：

```cmd
forfiles /s /M *.ly /c "cmd /c convert-ly -e @file"
forfiles /s /p C:\Documents\MyScores /M *.ly /c "cmd /c convert-ly -e @file"
forfiles /s /p "C:\Documents\My Scores" /M *.ly /c "cmd /c convert-ly -e @file"
```

> [校正] 原文误写为 `lilypond @file`，此命令属于 convert-ly 章节的批量更新示例，非 lilypond 基本用法。

### 7.2 命令行选项

#### `-d` 选项语法

- `-d`, `--define-default=`option-name[`=`value]
- `-d`, `--define-default=no-`option-name（关闭选项）

在 Scheme 代码中：

```scheme
#(ly:set-option 'backend 'svg)  ;; 等同于 -dbackend=svg
```

未提供 value 时默认使用 `#t`。`no-` 前缀等同于 `#f`。

#### 完整选项列表

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `anti-alias-factor` num | 高分辨率渲染并缩小 | `1` |
| `backend` symbol | 输出后端：`ps`/`cairo`/`svg` | `ps` |
| `clip-systems` bool | 提取音乐片段 | `#f` |
| `compile-scheme-code` bool | 使用 Guile 编译器运行 Scheme | `#f` |
| `crop` bool | 创建裁剪版PDF | `#f` |
| `debug-eval` bool | 调试 Scheme 评估器 | `#f` |
| `embed-source-code` bool | 嵌入源代码到PDF | `#f` |
| `eps-box-padding` num | EPS 包围盒额外内边距（pt） | `#f` |
| `font-export-dir` string | 导出字体的目录 | `#f` |
| `font-ps-resdir` string | PS 字体资源目录 | `#f` |
| `gs-never-embed-fonts` bool | 阻止 Ghostscript 嵌入字体 | `#f` |
| `job-count` num | 并行处理作业数 | `#f` |
| `loglevel` num | 日志级别（0=NONE, 1=ERROR, 2=WARN, 3=BASIC, 4=PROGRESS, 5=INFO, 6=DEBUG） | `BASIC` |
| `log-file` string | 输出到日志文件 | `#f` |
| `max-markup-depth` num | 递归标记求值最大深度 | `1024` |
| `midi-extension` string | MIDI 输出文件扩展名 | `"midi"` |
| `paper-size` string | 默认纸张大小 | `"a4"` |
| `point-and-click` value | 添加指向并点击链接 | `#t` |
| `preview` bool | 创建预览图像 | `#f` |
| `print-pages` bool | 生成完整页面 | `#t` |
| `resolution` num | PNG 分辨率（dpi） | `101` |
| `set-global-staff-size` num | 设置全局谱表大小 | `20` |
| `staff-size` num | 全局谱表大小（磅） | `20` |
| `strokeadjust` bool | 强制 PS 笔画调整 | `#f` |
| `warning-as-error` bool | 警告视为错误 | `#f` |

**后端说明**：
- `ps`：默认，PostScript 输出，PDF 通过 Ghostscript 生成
- `cairo`：通过 Cairo 库，支持 PS/EPS/PDF/PNG/SVG
- `svg`：SVG 输出，文本字体不嵌入

### 7.3 转换工具

| 工具 | 用途 |
|------|------|
| `midi2ly` | MIDI 文件转 LilyPond |
| `musicxml2ly` | MusicXML 转 LilyPond |
| `abc2ly` | ABC 记谱法转 LilyPond |
| `etf2ly` | ETF 格式转 LilyPond |

### 7.4 SVG输出

使用 `-dbackend=svg` 生成 SVG 输出：

```bash
lilypond -dbackend=svg file.ly
```

或在文件中：

```scheme
#(ly:set-option 'backend 'svg)
```

> 注意：SVG 输出中文本字体不嵌入，需要确保查看器可访问相应字体。

---

## 附录A：现代作曲关键命令速查表 🎵

| 需求 | 命令/技术 |
|------|----------|
| 微音程（四分音） | `ih`, `eh`, `isih`, `eseh` 后缀 |
| 微音程调号 | `\set Staff.keyAlterations = #\`((step . ,SEMI-FLAT) ...)` |
| 十二音变音记号 | `\accidentalStyle dodecaphonic` |
| 新现代变音记号 | `\accidentalStyle neo-modern` |
| 替代变音记号字形 | `alterationGlyphs` 属性 |
| 集群记谱 | `\makeClusters`, `ClusterSpanner.style` |
| DurationLine | `Duration_line_engraver`, `DurationLine.style` |
| 无节拍/华彩段 | `\cadenzaOn` / `\cadenzaOff` |
| 多节拍 | `\enablePerStaffTiming`, `\polymetric \time` |
| 渐变连梁 | `\featherDurations`, `Beam.grow-direction` |
| 倒影 | `\inversion around-pitch to-pitch music` |
| 逆行 | `\retrograde music` |
| 调式移调 | `\modalTranspose from to scale music` |
| 调式倒影 | `\modalInversion around to scale music` |
| Bartók 拨弦 | `\snappizzicato` |
| 泛音 | `\harmonicsOn`, `\harmonic`, `\flageolet` |
| 近似音高 | `\approximatePitch` |
| 自定义力度 | `make-dynamic-script` |
| al niente | `Hairpin.circled-tip = ##t` |
| Falls/Doits | `\bendAfter N` |
| 当代滑奏 | `\override Glissando.style = #'zigzag` + `\hideNotes` |
| 曲线形状修改 | `\shape #'((dx . dy) ...) Slur/Tie` |
| 对象可见性 | `\omit`, `\hide`, `break-visibility` |
| 摇摆节奏标记 | `\rhythm { 8[ 8] }` markup |
| 隐形速度变化 | `\set Score.tempoHideNote = ##t` |
| 阿拉伯微音程 | `\include "arabic.ly"`, `sd`/`sb`/`dsd`/`bsb` |
| 土耳其微音程 | `\include "turkish-makam.ly"` |
| Sprechstimme | 自定义 `Stem.stencil` 回调（见2.10节） |
| 扁平连线 | 自定义 `Tie.stencil`（见2.10节） |
| 复杂拍号 | `\time #'((2 2 2) . 8)` |
| 自定义调式 | `#`((step . ,NATURAL) ...)` + `\key c \custommode` |
| 非标准调号 | `\set Staff.keyAlterations` |
| 多节拍（移动Timing） | `\remove "Timing_translator"` + `\consists "Timing_translator"` |
| 华彩段不规则小节 | `cadenzaMeasure = { \cadenzaOff \partial 1024 s1024 \cadenzaOn }` |
| 延迟记号 | `\after duration command note` |
| 仅拍号分子 | `\override Staff.TimeSignature.style = #'single-number` |
| 禁止额外还原号 | `\set Staff.extraNatural = ##f` |

---

## 附录B：全部命令速查表

### 基本记谱

| 命令/语法 | 说明 |
|-----------|------|
| `\version "2.26.0"` | 版本声明 |
| `\relative { … }` | 相对音高模式 |
| `\fixed c'' { … }` | 固定参考音高模式 |
| `\key g \major` | 调号 |
| `\time 3/4` | 拍号 |
| `\clef treble` | 谱号 |
| `\tempo "Andante"` | 速度标记 |
| `\partial 8` | 弱起小节 |
| `\tuplet 3/2 { … }` | 连音符 |
| `\grace { … }` | 装饰音 |
| `\appoggiatura` | 长倚音 |
| `\acciaccatura` | 短倚音 |
| `\repeat volta 2 { … }` | 反复 |

### 修饰记号

| 命令/语法 | 说明 |
|-----------|------|
| `~` | 延音线 |
| `()` | 连奏线 |
| `\(\)` | 乐句连奏线 |
| `[]` | 手动连梁 |
| `-^` | 强调记号 |
| `--` | 保持音 |
| `->` | 重音 |
| `-.` | 跳音 |
| `-_` | 断连音 |
| `-数字` | 指法 |
| `\<` / `\>` / `\!` | 渐强/渐弱/结束 |
| `\ff` / `\mf` / `\p` / `\pp` | 力度记号 |

### 结构与布局

| 命令/语法 | 说明 |
|-----------|------|
| `\score { … }` | 乐谱块 |
| `\header { … }` | 标题信息 |
| `\layout { … }` | 布局设置 |
| `\midi { … }` | MIDI输出设置 |
| `\new Staff { … }` | 创建谱表 |
| `\new Voice { … }` | 创建声部 |
| `\new PianoStaff << … >>` | 钢琴谱表组 |
| `\new ChoirStaff << … >>` | 合唱谱表组 |
| `\new GrandStaff << … >>` | 大谱表组 |
| `\addlyrics { … }` | 添加歌词 |
| `\lyricmode { … }` | 歌词模式 |
| `\markup { … }` | 文本标记 |
| `<< >>` | 同时进行的音乐 |
| `{ }` | 顺序排列的音乐 |
| `\\` | 声部分隔符 |
| `< >` | 和弦 |
| `q` | 重复前一和弦 |

### 调整命令

| 命令 | 说明 |
|------|------|
| `\override Grob.property = value` | 覆盖布局对象属性 |
| `\revert Grob.property` | 恢复默认值 |
| `\once \override …` | 仅当前时刻生效 |
| `\tweak property value` | 微调下一个对象 |
| `\offset property value` | 相对偏移 |
| `\single` | 将 override 转为 tweak |
| `\set Context.property = value` | 设置上下文属性 |
| `\unset Context.property` | 取消上下文属性设置 |
| `\omit Grob` | 完全隐藏对象（stencil=#f） |
| `\hide Grob` | 使对象透明（保留空间） |
| `\with { … }` | 创建上下文时设置属性 |
| `\remove Engraver` | 移除雕刻器 |
| `\consists Engraver` | 添加雕刻器 |

### 方向命令

| 向下 | 向上 | 还原 |
|------|------|------|
| `\slurDown` | `\slurUp` | `\slurNeutral` |
| `\stemDown` | `\stemUp` | `\stemNeutral` |
| `\dynamicDown` | `\dynamicUp` | `\dynamicNeutral` |
| `\tieDown` | `\tieUp` | `\tieNeutral` |
| `\phrasingSlurDown` | `\phrasingSlurUp` | `\phrasingSlurNeutral` |
| `\tupletDown` | `\tupletUp` | `\tupletNeutral` |

### 连梁控制

| 命令 | 说明 |
|------|------|
| `\autoBeamOn` | 开启自动连梁 |
| `\autoBeamOff` | 关闭自动连梁 |
| `[` / `]` | 手动连梁起止 |
| `\noBeam` | 禁止当前音符连梁 |

### 现代作曲 🎵

| 命令/语法 | 说明 |
|-----------|------|
| `\cadenzaOn` / `\cadenzaOff` | 华彩段开关 |
| `\makeClusters { … }` | 集群记谱 |
| `\featherDurations fraction { … }` | 渐变连梁 |
| `\transpose from to music` | 移调 |
| `\inversion around to music` | 倒影 |
| `\retrograde music` | 逆行 |
| `\modalTranspose from to scale music` | 调式移调 |
| `\modalInversion around to scale music` | 调式倒影 |
| `\accidentalStyle style` | 变音记号风格（`default`, `voice`, `StaffGroup.voice`, `Score.default` 等） |
| `\ambitusAfter key-signature` | 将 Ambitus 放在调号之后 |
| `\arpeggio` / `\arpeggioArrowUp` / `\arpeggioArrowDown` / `\arpeggioNormal` / `\arpeggioBracket` / `\arpeggioParenthesis` / `\arpeggioParenthesisDashed` | 琶音样式 |
| `\defaultTimeSignature` | 恢复默认拍号样式 |
| `\numericTimeSignature` | 仅显示数字拍号 |
| `\overrideTimeSignatureSettings` | 自定义拍号行为 |
| `\revertTimeSignatureSettings` | 恢复默认拍号行为 |
| `\enablePerStaffTiming` | 启用多节拍 |
| `\polymetric \time …` | 多节拍拍号 |
| `\approximatePitch` | 近似音高 |
| `\bendAfter N` | 下降/上升记号 |
| `\snappizzicato` | Bartók拨弦 |
| `\harmonicsOn` / `\harmonicsOff` | 泛音开关 |
| `\harmonic` | 人工泛音 |
| `\flageolet` | 泛音圆圈标记 |
| `\shape displacements item` | 修改形状 |
| `\after duration command note` | 延迟记号 |
| `\allowBreak` | 允许换行 |
| `\ottava N` | 八度移位 |

### Scheme编程 🔧

| 命令/语法 | 说明 |
|-----------|------|
| `#(scheme-expr)` | 嵌入Scheme表达式 |
| `$(scheme-expr)` | 立即求值Scheme表达式 |
| `#{ LilyPond code #}` | LilyPond代码块 |
| `#(define-music-function …)` | 定义音乐函数 |
| `#(define-event-function …)` | 定义事件函数 |
| `#(define-void-function …)` | 定义无返回值函数 |
| `#(define-markup-command …)` | 定义标记命令 |
| `\displayMusic` | 显示音乐内部表示 |
| `\applyContext function` | 上下文求值 |
| `#(grob-transformer …)` | 创建grob变换回调 |
