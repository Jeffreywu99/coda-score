---
category: counterpoint
source: "Kent Kennan, Counterpoint (Based on Eighteenth-Century Practice), 4th Edition, 1999"
confidence: text_derived
tags: [对位, 巴赫, 创意曲, 赋格, 18世纪, 二声部, 赋格主题, 答题, 紧接段]
title: Kennan — 18 世纪对位法
title_en: Counterpoint — 18th Century Practice (Kennan)
difficulty: intermediate
contexts: [counterpoint, fugue, invention, two-voice, three-voice, baroque]
---

# Kennan — 18 世纪对位法

Kennan 以 **J.S. Bach 的实际作品**为核心，教授巴洛克对位法。与 Fux（帕莱斯特里那风格为模型）不同，Kennan 更注重实用性——更自由的跳进、更多的半音化、更接近真实器乐作品的写法。

> **音域、谱号、`\relative` 参考、通用禁止项、声部进行优先级** → 见 shared-rules.md
> **物种对位基础（协和/不协和分类、延留音规则）** → 见 fux-species-counterpoint.md

---

## 一、18 世纪对位风格特征

### 与 Fux 的关键差异

| 方面 | Fux（16 世纪模型） | Kennan（18 世纪模型） |
|------|-------------------|---------------------|
| 风格模型 | 帕莱斯特里那 | J.S. Bach |
| 不协和处理 | 严格限制（仅弱拍经过音） | 更自由——经过音、助音、倚音、延留均可 |
| 半音化 | 极少 | 自然音半音化是常规手法 |
| 节奏 | 固定比例（1:1, 2:1, 4:1） | 灵活——附点、切分、三连音 |
| 旋律跳进 | 最大纯五度 | 允许六度、八度跳进 |
| 和声暗示 | 仅两声部音程关系 | 明确暗示功能和声进行 |
| 教学目标 | 训练对位纪律 | 写出像 Bach 那样的对位作品 |

### 18 世纪对位的自由度

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 18 世纪风格的二声部片段：
      % 允许六度跳进、附点节奏、半音经过音
      c'4. d8 e4 g  a2 g4 e
      f4. e8 d4 cis  d2. r4
    }
    \new Staff \relative c {
      \clef bass
      % 低音暗示和声进行: I - V - vi - V - I
      c2 g4 e  f2 c4 g'
      d2 a4 f  g2 d4 g
    }
  >>
  \layout { }
}
```

**18 世纪特征标注**：
- 小节 1：六度跳进 c'→a（Fux 不允许）
- 小节 2：附点节奏 4. 8（Fux 仅用均匀节奏）
- 小节 3：半音经过音 cis（D 大调导音）
- 整体：明确暗示 I→V→vi→V→I 和声进行

---

## 二、二声部创意曲结构

### 结构模板

```
呈示部 (Exposition) — 2-4 小节
  ├── 主题 (Subject) — 在主调上，第一声部单独陈述
  ├── 主题模仿 — 第二声部在主调上应答
  └── 对题 (Countermotive) — 与主题形成对位的固定旋律素材

插句 (Episode) — 2-4 小节
  ├── 从主题/对题中抽取动机
  ├── 模进发展（最多 3 次迭代）
  └── 转调到近关系调

中间进入 (Middle Entry) — 2-4 小节
  └── 主题在近关系调上再次出现

再现/尾声 (Return / Coda) — 2-4 小节
  └── 主题在主调上的最后陈述，结束
```

### 完整二声部创意曲示例（C 大调，16 小节）

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff = "upper" \relative c'' {
      \clef treble
      % ===== 呈示部 (小节 1-4) =====
      % 小节 1-2: 主题 — 上行分解和弦 + 级进下行
      c'4 d e g  f e d c
      % 小节 3-4: 自由对位（第二声部即将进入）
      d4 e f a  g f e d

      % ===== 插句 (小节 5-8) =====
      % 小节 5-6: 主题动机模进（下行二度模进）
      e4 f g b  a g f e
      % 小节 7-8: 继续模进，转调暗示
      d4 e f a  g2. r4

      % ===== 中间进入 (小节 9-12) =====
      % 小节 9-10: 主题在 a 小调上（关系小调）
      a4 b c e  d c b a
      % 小节 11-12: 对题发展，准备回主调
      g4 a b d  c2. r4

      % ===== 再现/尾声 (小节 13-16) =====
      % 小节 13-14: 主题回到 C 大调
      c4 d e g  f e d c
      % 小节 15-16: 终止扩展
      e4 d c e  d2 c
    }
    \new Staff = "lower" \relative c {
      \clef bass
      % ===== 呈示部 (小节 1-4) =====
      % 小节 1-2: 休止，等待主题陈述
      R1*2
      % 小节 3-4: 主题模仿（第二声部进入）
      c4 d e g  f e d c

      % ===== 插句 (小节 5-8) =====
      % 小节 5-6: 对题（与主题动机对应）
      c4 b a g  f e d c
      % 小节 7-8: 低音模进
      b4 c d f  e2. r4

      % ===== 中间进入 (小节 9-12) =====
      % 小节 9-10: 对位旋律（a 小调区域）
      e4 f g b  a g f e
      % 小节 11-12: 和声准备回主调
      d2 e4 c  d2. r4

      % ===== 再现/尾声 (小节 13-16) =====
      % 小节 13-14: 对题（倒置的主题动机）
      g4 f e c  d e f g
      % 小节 15-16: 终止式
      c4 g a e  f2 c
    }
  >>
  \layout { }
}
```

**结构分析**：

| 段落 | 小节 | 调性 | 上方声部 | 下方声部 |
|------|------|------|---------|---------|
| 呈示部 | 1-2 | C 大调 | 主题陈述 | 休止 |
| 呈示部 | 3-4 | C 大调 | 自由对位 | 主题模仿 |
| 插句 | 5-6 | C→G | 动机模进 | 对题 |
| 插句 | 7-8 | G 大调 | 继续模进 | 低音模进 |
| 中间进入 | 9-10 | a 小调 | 主题(Am) | 对位 |
| 中间进入 | 11-12 | a→C | 发展 | 准备回主调 |
| 再现 | 13-14 | C 大调 | 主题 | 对题 |
| 尾声 | 15-16 | C 大调 | 终止扩展 | 终止式 |

---

## 三、赋格结构

### Kennan 的核心论断

> "只有赋格的呈示部按照固定形式进行；在这之后发生什么，取决于音乐材料的性质、以及作曲家的品味和想象力。"

### 赋格完整模板

```
呈示部 (Exposition)
  声部 1: 主题（主调）
  声部 2: 答题（属调）+ 对题伴随声部 1
  [更多声部交替进入...]

插句 1 (Episode 1)
  从主题/对题抽取动机 → 模进 → 转调

中间进入 (Middle Entry)
  主题在近关系调上出现（1-3 次）

插句 2 (Episode 2)
  进一步发展，推向高潮

紧接段 + 结尾 (Stretto + Final Section)
  主题叠置进入 → 持续音 → 主调终止
```

### 完整二声部赋格示例（C 大调，24 小节）

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff = "soprano" \relative c'' {
      \clef treble

      % ===== 呈示部 (小节 1-8) =====
      % 小节 1-2: 上方声部休止，低音先陈述主题
      R1*2

      % 小节 3-4: 答题（属调 G 大调）
      % 主题移高纯五度：c→g, d→a, e→b, g→d'...
      g4 a b d'  c' b a g
      % 小节 5-6: 对题（伴随低音的主题陈述）
      f4 e d c  b a g f
      % 小节 7-8: 主题在上方声部（回到 C 大调）
      c4 d e g  f e d c

      % ===== 第一插句 (小节 9-12) =====
      % 小节 9-10: 主题头部动机模进（下行）
      b4 c d f  e d c b
      % 小节 11-12: 继续模进，转向 a 小调
      a4 b c e  d2. r4

      % ===== 中间进入 (小节 13-16) =====
      % 小节 13-14: 主题在 a 小调上
      a4 b c e  d c b a
      % 小节 15-16: 自由对位，准备回主调
      g4 a b d  c2. r4

      % ===== 第二插句 (小节 17-18) =====
      % 主题尾部动机模进
      b4 a g f  e d c b

      % ===== 紧接段 + 结尾 (小节 19-24) =====
      % 小节 19-20: 紧接段 — 主题叠置进入
      % 上方声部比下方声部提前 1 小节进入
      c4 d e g  f e d c
      % 小节 21-22: 继续紧接
      e4 d c b  a2 g4 e
      % 小节 23-24: 终止（主调持续音暗示）
      f4 e d c  d2 c
    }
    \new Staff = "bass" \relative c {
      \clef bass

      % ===== 呈示部 (小节 1-8) =====
      % 小节 1-2: 主题（C 大调）— 上行分解和弦 + 级进下行
      c4 d e g  f e d c
      % 小节 3-4: 对题（伴随答题）
      c'4 b a g  f e d c
      % 小节 5-6: 主题再次在低音（C 大调）
      c4 d e g  f e d c
      % 小节 7-8: 对题
      f4 e d b  c2. r4

      % ===== 第一插句 (小节 9-12) =====
      % 小节 9-10: 对题动机模进（a 小调区域）
      e4 d c a  b c d e
      % 小节 11-12: 低音支持转调
      f4 e d c  b2. r4

      % ===== 中间进入 (小节 13-16) =====
      % 小节 13-14: 对题（a 小调区域）
      e4 d c a  b c d e
      % 小节 15-16: 准备回主调
      e4 f g e  f2. r4

      % ===== 第二插句 (小节 17-18) =====
      % 低音模进
      g4 f e d  c d e f

      % ===== 紧接段 + 结尾 (小节 19-24) =====
      % 小节 19-20: 紧接段 — 低音延迟 1 小节进入主题
      r4 c d e  g f e d
      % 小节 21-22: 低音继续
      c4 d e g  f2 e4 c
      % 小节 23-24: 持续音 C + 终止
      c4 g' f e  d2 c
    }
  >>
  \layout { }
}
```

**结构元素标注**：

| 小节 | 段落 | 上方声部 | 下方声部 | 调性 |
|------|------|---------|---------|------|
| 1-2 | 呈示部 | 休止 | **主题**(C) | C |
| 3-4 | 呈示部 | **答题**(G) | 对题 | G |
| 5-6 | 呈示部 | 对题 | **主题**(C) | C |
| 7-8 | 呈示部 | **主题**(C) | 对题 | C |
| 9-10 | 插句 1 | 动机模进 | 动机模进 | C→G |
| 11-12 | 插句 1 | 继续模进 | 转调 | →a |
| 13-14 | 中间进入 | **主题**(Am) | 对题 | a |
| 15-16 | 中间进入 | 发展 | 准备回主调 | a→C |
| 17-18 | 插句 2 | 动机模进 | 模进 | C |
| 19-20 | **紧接段** | 主题(提前) | 主题(延迟) | C |
| 21-22 | 结尾 | 自由发展 | 继续 | C |
| 23-24 | 终止 | 终止式 | **持续音 C** | C |

---

## 四、赋格主题写作

### 好主题的五个特征

| 特征 | 说明 | 检验方法 |
|------|------|---------|
| **特征鲜明** | 有辨识度的节奏型或旋律轮廓 | 听一遍就能记住 |
| **调性明确** | 清楚建立主调（通常包含主、属音） | 能判断是大调还是小调 |
| **适度长度** | 通常 2-4 小节 | 太长则难以发展 |
| **可发展性** | 包含可抽取的动机片段 | 能拆分为更小的动机单元 |
| **起止清晰** | 通常开始于主/属音，结束于主/属音 | 有明确的开始和结束感 |

### 好主题示例

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c' {
    \clef treble
    \key c \major
    % 主题 A: 分解和弦上行 + 级进下行（Bach 风格）
    % 特征：清晰的 C 大调三和弦轮廓，节奏有变化
    \mark \markup { \bold "主题 A — 分解和弦型" }
    c4 e g e  f d e c  d2 c \bar "||"

    % 主题 B: 级进为主 + 跳进点缀
    % 特征：级进上行创造张力，六度跳进释放
    \mark \markup { \bold "主题 B — 级进型" }
    c4 d e f  g a f d  e2 c \bar "||"

    % 主题 C: 附点节奏 + 半音化
    % 特征：附点节奏赋予动力，半音经过音增加色彩
    \mark \markup { \bold "主题 C — 节奏型" }
    c4. d8 e4 g  f4. e8 d4 c  d2 c \bar "||"
  }
  \layout { }
}
```

### 差主题示例

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c' {
    \clef treble
    % WRONG: 主题太长（8 小节），难以记忆和发展
    \mark \markup { \bold "✗ 过长" }
    c4 d e f g a b c'  b a g f e d c b
    a g f e d c b a  g f e d c2 \bar "||"

    % WRONG: 无特征（纯音阶级进，无辨识度）
    \mark \markup { \bold "✗ 无特征" }
    c4 d e f  e d c2 \bar "||"

    % WRONG: 调性模糊（不含主音或属音）
    \mark \markup { \bold "✗ 调性模糊" }
    f4 a d' f  a d f2 \bar "||"
  }
  \layout { }
}
```

### 主题中的动机抽取

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c' {
    \clef treble
    % 完整主题
    \mark \markup { \bold "完整主题" }
    c4 d e g  f e d c \bar "||"

    % 动机 a: 主题头部（上行四音）
    \mark \markup { \bold "动机 a — 头部" }
    c4 d e g \bar "|"
    % 动机 a 的模进
    d4 e f a  \bar "|"
    e4 f g b \bar "||"

    % 动机 b: 主题尾部（下行四音）
    \mark \markup { \bold "动机 b — 尾部" }
    f4 e d c \bar "|"
    % 动机 b 的模进
    g'4 f e d \bar "|"
    a'4 g f e \bar "||"
  }
  \layout { }
}
```

---

## 五、答题规则

### 真答题 (Real Answer)

**定义**：主题的**严格五度移调**——所有音程关系保持不变。

**使用条件**：主题的首音**不强调属音**，且主题开头不包含属→主的运动。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 答题（G 大调）：主题严格移高纯五度
      % c→g, d→a, e→b, g→d', f→c', e→b, d→a, c→g
      \mark \markup { \bold "答题 (Real)" }
      g4 a b d'  c' b a g
    }
    \new Staff \relative c {
      \clef bass
      % 主题（C 大调）：从主音开始，上行分解三和弦
      \mark \markup { \bold "主题" }
      c4 d e g  f e d c
    }
  >>
  \layout { }
}
```

**音程验证**（主题 vs 答题）：

| 位置 | 主题音程 | 答题音程 | 一致？ |
|------|---------|---------|--------|
| 1→2 | M2 (c→d) | M2 (g→a) | ✓ |
| 2→3 | M2 (d→e) | M2 (a→b) | ✓ |
| 3→4 | m3 (e→g) | m3 (b→d') | ✓ |
| 4→5 | M2 (g→f)↓ | M2 (d'→c')↓ | ✓ |
| 5→6 | M2 (f→e)↓ | M2 (c'→b)↓ | ✓ |
| 6→7 | M2 (e→d)↓ | M2 (b→a)↓ | ✓ |
| 7→8 | M2 (d→c)↓ | M2 (a→g)↓ | ✓ |

### 调性答题 (Tonal Answer)

**定义**：五度移调但**局部调整音程**，使答题保持在合理的调性范围内。

**使用条件**（满足任一即需调性答题）：
1. 主题**首音为属音**（如 G 在 C 大调中）
2. 主题开头包含**属→主的跳进**（如 g→c）
3. 主题开头**强调属音**的反复

**调整规则**：主题中的五度跳进在答题中变为四度，反之亦然。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 调性答题：主题首音为属音 g
      % 真答题会是 d'→g 但太高
      % 调性答题将五度 g→c' 调整为四度 g→c'
      \mark \markup { \bold "答题 (Tonal)" }
      g4 a b c'  b a g2

      % 对比：如果是真答题（严格五度移调）
      % g→d', a→e', b→f#', c'→g'... 音域过高！
      \mark \markup { \bold "✗ 真答题（不合适）" }
      d'4 e' fis' g'  fis' e' d'2
    }
    \new Staff \relative c' {
      \clef bass
      % 主题：从属音 g 开始，强调 g→c' 的五度
      \mark \markup { \bold "主题" }
      g4 a b c'  b a g2
    }
  >>
  \layout { }
}
```

### 真答题 vs 调性答题 判断流程

```
主题首音是否强调属音？
├── 否 → 主题开头是否有属→主运动？
│   ├── 否 → ✅ 真答题（严格五度移调）
│   └── 是 → ✅ 调性答题（调整五度/四度）
└── 是 → ✅ 调性答题（调整五度/四度）
```

**正确示例 — 判断并使用调性答题**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 主题: g c' e' d' | c'2
      % 首音 g = 属音 → 需要调性答题
      % g→c' 的五度在答题中变为 c'→f 的四度
      \mark \markup { \bold "主题" }
      g4 c' e' d'  c'2 \bar "||"

      % 调性答题: c' f a g | f2
      % g→c' (P5) 变为 c'→f (P4)
      % 其他音程相应调整
      \mark \markup { \bold "答题 (Tonal)" }
      c'4 f a g  f2 \bar "||"
    }
    \new Staff \relative c {
      \clef bass
      % 低音自由对位
      c2 g4 b  c2
      e2 f4 d  c2
    }
  >>
  \layout { }
}
```

---

## 六、赋格对题写作

### 对题的要求

| 要求 | 说明 |
|------|------|
| **可转位** | 对题既可在主题上方也可在下方 |
| **节奏互补** | 主题长音时对题活跃，反之亦然 |
| **动机独立** | 有自己的旋律特征，非简单和声填充 |
| **协和优先** | 与主题形成协和音程为主 |

### 对题与主题的配合示例

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 对题（上方）：当主题有长音时，对题活跃
      \mark \markup { \bold "对题" }
      e'4 f g e  d c b a  g2 r
    }
    \new Staff \relative c {
      \clef bass
      % 主题（下方）：前半活跃，后半长音
      \mark \markup { \bold "主题" }
      c4 d e g  f2 e2  c2 r
    }
  >>
  \layout { }
}
```

**节奏互补分析**：
- 小节 1：主题 4 个四分音符 ↔ 对题 4 个四分音符（两者均活跃）
- 小节 2：主题 2 个二分音符（长音）↔ 对题 4 个四分音符（活跃，补偿主题的静止）
- 小节 3：主题 1 个二分音符 ↔ 对题 1 个二分音符（同时收束）

---

## 七、紧接段 (Stretto)

### 定义与技法

紧接段是赋格中**主题叠置进入**的段落——后一声部在前一声部的主题尚未结束时即开始模仿。

| 要素 | 说明 |
|------|------|
| 时间间距 | 通常 1-2 小节（比呈示部的间距更短） |
| 音程 | 可在五度、四度、八度上模仿 |
| 功能 | 制造高潮张力，通常出现在结尾部分 |
| 调整 | 主题的音程可能需要微调以适应叠置 |

### 紧接段示例

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 上方声部：主题提前进入
      \mark \markup { \bold "主题（提前）" }
      c'4 d e g  f e d c  d e f a  g2
    }
    \new Staff \relative c' {
      \clef bass
      % 下方声部：主题延迟 1 小节后进入
      \mark \markup { \bold "主题（延迟 1 小节）" }
      r4 r r r  c4 d e g  f e d c  e2
    }
  >>
  \layout { }
}
```

**叠置分析**：
- 小节 1：上方声部开始主题，下方声部休止
- 小节 2：上方声部继续主题后半部分，下方声部**同时开始主题**
- 小节 2-3：两个声部的主题**重叠进行**
- 小节 4：两声部同时收束

---

## 八、持续音 (Pedal Point)

### 定义

持续音是在赋格结尾（或其他段落）中，低音持续演奏一个音（通常是主音或属音），上方声部自由发展的技法。

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 上方声部：自由发展，暗示和声变化
      e'4 f g e  d e f d  e d c e  d2 c
    }
    \new Staff \relative c {
      \clef bass
      % 持续音：低音 C 持续 4 小节
      \mark \markup { \bold "持续音 C（主音）" }
      c1 c c c
    }
  >>
  \layout { }
}
```

**持续音上的和声暗示**：

| 小节 | 持续音 | 上方暗示和声 | 功能 |
|------|--------|-------------|------|
| 1 | C | C 大三 (I) | 主和弦 |
| 2 | C | F 大三/C (IV⁶₄) | 下属功能 |
| 3 | C | G 大三/C (V⁶₄) | 属功能 |
| 4 | C | C 大三 (I) | 回归主和弦 |

---

## 九、常见错误总结

| 错误 | 说明 | 正确做法 |
|------|------|---------|
| 主题无特征 | 纯音阶级进，无法辨识 | 加入跳进或特殊节奏型 |
| 答题类型错误 | 主题首音为属音却用真答题 | 检查首音 → 选择答题类型 |
| 对题不可转位 | 对题只能在主题上方/下方 | 写作时测试两个位置 |
| 插句材料与主题无关 | 使用全新素材 | 从主题/对题中抽取动机 |
| 模进超过 3 次 | 过多模进导致单调 | 最多 3 次迭代后变化 |
| 紧接段主题变形过大 | 叠置时主题面目全非 | 微调音程，保持主题可辨识 |
| 持续音上方和声冲突 | 持续音与上方形成不协和 | 确保每拍的主要音与持续音协和 |

### 答题类型选择错误示例

```lilypond
% WRONG: 主题首音为属音 g，却使用真答题
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 真答题：g→d', 但 d' 太高，且 F# 不在 C 大调中
      \mark \markup { \bold "✗ 真答题（不合适）" }
      d'4 e' fis' g'  fis' e' d'2
    }
    \new Staff \relative c' {
      \clef bass
      % 主题：首音 g = 属音
      \mark \markup { \bold "主题（首音 = 属音）" }
      g4 a b c'  b a g2
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 使用调性答题，将五度调整为四度
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 调性答题：g→c'（四度代替五度），保持在合理音域
      \mark \markup { \bold "✓ 调性答题" }
      c'4 d' e' f'  e' d' c'2
    }
    \new Staff \relative c' {
      \clef bass
      \mark \markup { \bold "主题" }
      g4 a b c'  b a g2
    }
  >>
  \layout { }
}
```

---

## AI 生成约束总结

生成巴洛克风格对位/LilyPond 代码时：

1. **主题**：2-4 小节，特征鲜明，调性明确，可抽取动机
2. **答题**：首音为属音 → 调性答题，否则 → 真答题
3. **呈示部**：声部依次进入，主题-答题交替，对题伴随
4. **对题**：可转位、节奏互补、动机独立
5. **插句**：从主题抽取动机 → 模进（≤3 次）→ 转调
6. **中间进入**：主题在近关系调上出现 1-3 次
7. **紧接段**：主题叠置进入，可微调音程
8. **持续音**：结尾低音持续主音或属音
9. **终止**：返回主调 + 完满正格终止 (V→I)
10. **18 世纪自由度**：允许六度/八度跳进、附点节奏、半音经过音
