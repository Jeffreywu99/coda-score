---
category: syntax
source: "LilyPond 2.24 Notation Reference — 纯语法层（两分支共享）"
confidence: canonical
tags: [LilyPond语法, 括号, 音高, 时值, 排版, 编译, 谱号, 调号, 拍号]
title: LilyPond 核心语法
title_en: LilyPond Core Syntax (Base Layer)
contexts: [all]
---

# LilyPond 核心语法 — Base Layer

本文件是**纯语法参考**，不包含任何音乐风格知识。古典分支和现代分支共享此文件。
乐器的音域数据见 `classical/common/shared-rules.md`。

---

## 一、文件结构

每个 LilyPond 文件必须包含以下结构：

```lilypond
\version "2.24.0"

\header {
  title = "曲目标题"
  composer = "作曲者"
}

\score {
  % 音乐内容放在这里
  \layout { }
}
```

| 元素 | 必须？ | 说明 |
|------|--------|------|
| `\version` | ✅ | 声明 LilyPond 版本 |
| `\header` | ❌ | 标题、作曲者等元信息 |
| `\score` | ✅ | 音乐的顶层容器 |
| `\layout` | ✅（在 `\score` 内） | 触发排版 |
| `\midi` | ❌ | 触发 MIDI 输出 |

---

## 二、括号系统（最常见的编译错误来源）

LilyPond 使用三种括号，**必须正确配对**：

| 括号 | 用途 | 示例 |
|------|------|------|
| `{ }` | 顺序音乐块（音符按时间先后排列） | `{ c4 d e f }` |
| `<< >>` | 并行音乐块（多个声部同时演奏） | `<< { 高声部 } \\ { 低声部 } >>` |
| `< >` | 和弦（多个音同时发声） | `<c e g>4` |

### 嵌套规则

```lilypond
% 正确：花括号内可以嵌套尖括号
\score {
  { c4 <e g b> d f }
}

% 正确：双尖括号内可以嵌套花括号
\score {
  << { c'4 d' e' f' } \\ { c4 b a g } >>
}

% 正确：多层嵌套
\score {
  \new PianoStaff <<
    \new Staff { \relative c'' { c4 d e f } }
    \new Staff { \relative c  { c4 b a g } }
  >>
  \layout { }
}
```

### 常见括号错误

| 错误 | 修复 |
|------|------|
| `{ c4 d e f` — 缺少右括号 | 添加 `}` |
| `<< { c d } { e f } >>` — 并行块内两个声部缺少 `\\` | 改为 `<< { c d } \\ { e f } >>` |
| `<c e g}4` — 和弦括号不匹配 | 改为 `<c e g>4` |
| 花括号和双尖括号混用 | `{ }` = 顺序，`<< >>` = 并行 |

### 括号配对检查方法

编译前数括号：
- 每个 `{` 必须有对应的 `}`
- 每个 `<<` 必须有对应的 `>>`
- 每个 `<`（和弦开始）必须有对应的 `>`

---

## 三、音高输入

### 3.1 `\relative` 模式（推荐）

```lilypond
\relative c'' {
  c4 d e f    % C5 D5 E5 F5（以 c'' = C5 为参考，级进自动保持八度）
  g a b c     % G5 A5 B5 C6（继续上行级进）
}
```

**规则**：
- `\relative` 后跟一个参考音，设定第一个音符的八度
- 之后每个音符的八度由**与前一个音符的距离**自动判定
- 纯四度以内的进行自动保持八度
- 超过纯四度的跳进需要显式八度标记

### 3.2 八度标记

| 标记 | 含义 |
|------|------|
| `'`（单引号） | 比自然八度高一个八度 |
| `''`（双引号） | 高两个八度 |
| `,`（逗号） | 低一个八度 |
| `,,`（双逗号） | 低两个八度 |
| 无标记 | 自然八度（纯四度内自动判定） |

### 3.3 音名与变化音

| 写法 | 含义 |
|------|------|
| `c d e f g a b` | C D E F G A B（自然音） |
| `cis dis eis` | C# D# E#（升号，后缀 `-is`） |
| `des ees fes` | Db Eb Fb（降号，后缀 `-es`；E→ees，A→aes） |
| `cisis` | C##（重升） |
| `deses` | Dbb（重降） |

### 3.4 `\absolute` 模式

```lilypond
\absolute {
  c'4    % 明确的 C4（中央 C）
  c''4   % 明确的 C5
  c,4    % 明确的 C2
}
```

每个音符的八度都是绝对的，不受前一个音影响。适合音程跳进很大的现代音乐。

---

## 四、时值

### 4.1 基本时值

| 写法 | 含义 | 等值 |
|------|------|------|
| `c\breve` | 二全音符 | = 2 × 全音符 |
| `c\longa` | 四全音符 | = 4 × 全音符 |
| `c1` | 全音符 | |
| `c2` | 二分音符 | |
| `c4` | 四分音符 | |
| `c8` | 八分音符 | |
| `c16` | 十六分音符 | |
| `c32` | 三十二分音符 | |
| `c64` | 六十四分音符 | |

### 4.2 附点

| 写法 | 含义 |
|------|------|
| `c4.` | 附点四分音符 = 四分 + 八分 |
| `c2..` | 双附点二分音符 |

### 4.3 连音线

```lilypond
c2~ c4    % 二分音符连到四分音符（同音高）
c4.~ c8   % 附点四分连到八分
```

### 4.4 休止符

| 写法 | 含义 |
|------|------|
| `r4` | 四分休止符 |
| `r2` | 二分休止符 |
| `R1` | 全小节休止（大写 R，自动适应拍号） |
| `R1*3` | 连续 3 小节休止 |
| `s4` | 隐形休止（占位但不显示） |

### 4.5 连音符（Tuplets）

```lilypond
\tuplet 3/2 { c8 d e }     % 三连音（3 个八分 = 2 个八分的时值）
\tuplet 5/4 { c16 d e f g } % 五连音
```

---

## 五、谱号、拍号、调号

### 5.1 谱号

```lilypond
\clef treble       % 高音谱号
\clef bass         % 低音谱号
\clef alto         % 中音谱号（中提琴）
\clef tenor        % 次中音谱号
\clef "treble_8"   % 低八度高音谱号（男高音）
\clef "bass_8"     % 低八度低音谱号
\clef percussion   % 打击乐谱号
```

### 5.2 拍号

```lilypond
\time 4/4          % 四四拍
\time 3/4          % 三四拍
\time 6/8          % 八六拍
\time 5/4          % 五四拍
\time 7/8          % 七八拍
```

### 5.3 调号

```lilypond
\key c \major      % C 大调（无升降号）
\key g \major      % G 大调（1 个升号）
\key d \minor      % D 小调（1 个降号）
\key bes \major    % Bb 大调
\key fis \minor    % F# 小调
```

### 5.4 临时记号

```lilypond
\time 4/4
c4 d e f           % 无临时记号
cis4 des eis fes   % 有临时记号
```

临时记号默认在同一小节内有效，跨小节自动还原。

---

## 六、多声部与多谱表

### 6.1 同一谱表上的两个声部

```lilypond
\new Staff {
  << { c''4 d'' e'' f'' } \\ { c'4 b a g } >>
}
```

`\\` 是关键——它将 `<< >>` 内的两个花括号块分为独立的声部。

### 6.2 多谱表

```lilypond
\score {
  \new PianoStaff <<
    \new Staff { \clef treble \relative c'' { c4 d e f } }
    \new Staff { \clef bass   \relative c   { c4 b a g } }
  >>
  \layout { }
}
```

### 6.3 常见多谱表容器

| 容器 | 用途 | 是否自动加括号 |
|------|------|---------------|
| `\new PianoStaff` | 钢琴（两个谱表用大括号连接） | 是 |
| `\new GrandStaff` | 大谱表（多个谱表用大括号连接） | 是 |
| `\new ChoirStaff` | 合唱（多个谱表不加括号） | 否 |
| `\new StaffGroup` | 乐器组（多个谱表用括号连接） | 是 |

### 6.4 完整的四部合唱布局

```lilypond
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' { \clef treble       % Soprano
      c4 d e f
    }
    \new Staff \relative c'  { \clef treble       % Alto
      g4 a b c
    }
    \new Staff \relative c'  { \clef "treble_8"   % Tenor
      e4 f g a
    }
    \new Staff \relative c   { \clef bass          % Bass
      c4 b a g
    }
  >>
  \layout { }
}
```

---

## 七、表情记号与演奏法

### 7.1 力度

```lilypond
c4\ppp c\pp c\p c\mp c\mf c\f c\ff c\fff
```

### 7.2 渐强渐弱

```lilypond
c4\< c c c\!          % 渐强后终止
c4\> c c c\!          % 渐弱后终止
c4\cr c c c\decr      % 文字形式
```

### 7.3 发音法

```lilypond
c4->    % 重音（accent）
c4-.    % 跳音（staccato）
c4--    % 保持音（tenuto）
c4-^    % 强音（marcato）
c4-_    % 延音（portato）
```

### 7.4 连线

```lilypond
c4( d e f)    % 圆滑线（slur）
c4[ d e f]    % 手动连梁（beam）
c2~ c2        % 连音线（tie）
```

### 7.5 装饰音

```lilypond
\grace { a16 b } c4          % 倚音
\appoggiatura { a16 b } c4   % 长倚音
\acciaccatura { a16 b } c4   % 短倚音（带斜线）
```

---

## 八、反复与结构

### 8.1 反复

```lilypond
\repeat volta 2 {
  c4 d e f
}
```

### 8.2 反复 + 结尾

```lilypond
\repeat volta 2 {
  c4 d e f
}
\alternative {
  { g a b c }    % 第一结尾
  { g f e d }    % 第二结尾
}
```

### 8.3 小节线

```lilypond
c4 d e f |       % 普通小节线
c4 d e f \bar "||"   % 双纵线
c4 d e f \bar "|."   % 终止线
```

---

## 九、变量与复用

```lilypond
% 定义音乐变量
melody = \relative c'' {
  c4 d e f | g a b c
}

bass = \relative c {
  c4 b a g | f e d c
}

% 使用变量
\score {
  \new PianoStaff <<
    \new Staff { \melody }
    \new Staff { \clef bass \bass }
  >>
  \layout { }
}
```

---

## 十、常见编译错误与修复

### 10.1 错误诊断表

| 错误信息 | 原因 | 修复 |
|---------|------|------|
| `syntax error, unexpected '}'` | 括号不配对 | 检查所有 `{` `}` `<<` `>>` `<` `>` |
| `not a duration` | 时值写法错误 | 检查数字是否合法（1/2/4/8/16/32/64） |
| `unexpected \new` | `\new Staff` 位置不对 | 确保在 `\score` 内部 |
| `unknown escaped string` | 未知的 `\命令` | 检查拼写，确保命令存在 |
| `Unattached SlurEvent` | 圆滑线没有连接到音符 | 确保 `()` 内有音符 |
| `Junk after note name` | 音名后有多余字符 | 检查音名拼写（只允许 a-g + is/es） |

### 10.2 调试技巧

1. **逐段注释**：用 `%` 注释掉可疑段落，定位问题
2. **最小化**：删除所有非必要的代码，只保留出问题的一小节
3. **括号计数**：数 `{` 和 `}` 的数量是否相等
4. **检查 `\relative`**：确保每个 `\new Staff` 都有自己的 `\relative`

### 10.3 编译检查清单

生成 LilyPond 代码前，对照此清单：

- [ ] 有 `\version "2.24.0"`
- [ ] 有 `\score { ... \layout { } }`
- [ ] 所有 `{` 有对应的 `}`
- [ ] 所有 `<<` 有对应的 `>>`
- [ ] 并行块内多个声部之间有 `\\`
- [ ] 每个 `\new Staff` 有自己的 `\clef` 和 `\relative`
- [ ] 每个声部的小节线 `|` 对齐（非必须但便于调试）
- [ ] 和弦用 `< >` 而非 `{ }`

---

## 十一、移调

```lilypond
% 将音乐移调
\transpose c d { \relative c' { c4 d e f } }
% 结果：d4 e fis g（从 C 移到 D = 上移大二度）
```

`\transpose` 的第一个参数是原调，第二个参数是目标调。

---

## 十二、Markup 文本

```lilypond
c4^\markup { \bold "forte" }     % 粗体文本
c4^\markup { \italic "dolce" }   % 斜体文本
c4^\markup { "espr." }           % 普通文本
```
