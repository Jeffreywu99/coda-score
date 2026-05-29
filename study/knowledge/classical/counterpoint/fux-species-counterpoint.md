---
category: counterpoint
source: "Fux, Gradus ad Parnassum (1725), trans. Alfred Mann"
confidence: text_derived
tags: [对位, 物种对位, 文艺复兴, 帕莱斯特里那, 声部进行, 协和, 不协和, 延留音]
title: Fux 物种对位法
title_en: Fux Species Counterpoint
difficulty: intermediate
contexts: [counterpoint, fugue, two-voice, three-voice, species-counterpoint]
---

# Fux 物种对位法

Johann Joseph Fux 1725 年 *Gradus ad Parnassum* 将多声部写作分解为五种"物种"（Species），以帕莱斯特里那风格为模型，逐步训练对位思维。本文涵盖全部五类对位的规则与完整 LilyPond 示例。

> **音域、谱号、`\relative` 参考、通用禁止项（平行五/八度等）、声部进行优先级、终止式** → 见 shared-rules.md

---

## 协和与不协和分类

| 分类 | 音程 | 使用频率 |
|------|------|---------|
| **完全协和** | P1、P5、P8 | 少用（首尾必用，内部慎用） |
| **不完全协和** | M3/m3、M6/m6 | 主要材料，应占多数 |
| **不协和** | M2/m2、P4、M7/m7、A4/d5 | 仅允许在特定条件下出现（见各物种规则） |

---

## 第一类对位 — 一音对一音 (1:1)

**节奏**：定旋律（Cantus Firmus, C.F.）用全音符，对位声部也用全音符。

**规则**：
1. **仅允许协和音程** — 完全协和（P1/P5/P8）和不完全协和（3 度/6 度）
2. **纯一度仅允许在首尾小节**，内部小节避免
3. **内部小节以不完全协和（3 度/6 度）为主**
4. **反向进行优先**，同向进行仅用于进入不完全协和音程
5. **开始**：必须用完全协和音程（P1/P5/P8）
6. **结束**：必须用 P1 或 P8

**终止式**：
- 定旋律在下：倒数第二小节用 **大六度** → 解决到八度（需升高第七级导音）
- 定旋律在上：倒数第二小节用 **小三度** → 解决到一度或八度

**正确示例**（C 多利亚调式，8 小节二声部对位）：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 对位声部（上方）：拱形旋律，以级进为主
      c1 e f d e f e c
    }
    \new Staff \relative c {
      \clef bass
      % 定旋律（下方）：C 多利亚
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**音程分析**（逐小节验证）：

| 小节 | 低音 | 高音 | 音程 | 运动类型 |
|------|------|------|------|---------|
| 1 | c | c' | P8 | — |
| 2 | b, | e' | m3（复音=m10） | 反向 ✓ |
| 3 | a, | f' | m6（复音=m13） | 同向 ✓（不完全协和） |
| 4 | d | d' | P8 | 反向 ✓ |
| 5 | e | e' | P8 | 斜向 ✓ |
| 6 | f | f' | P8 | 斜向 ✓ |
| 7 | g | e' | M6 | 反向 ✓ |
| 8 | c | c' | P8 | 反向 ✓ |

**常见错误 — 平行五度**：

```lilypond
% WRONG: 小节 2-3 之间出现平行五度
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 高音 d'→g' = 上行四度
      c1 d' g f e a g c
    }
    \new Staff \relative c {
      \clef bass
      % 低音 g,→c = 上行四度
      % g,→d' = P5, c→g' = P5 → 平行五度！
      c1 g, c d e f g c
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 改变高音避免平行五度
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 高音 d'→e' = 上行二度（非五度）
      c1 d' e f e c d c
    }
    \new Staff \relative c {
      \clef bass
      c1 g, c d e f g c
    }
  >>
  \layout { }
}
```

**常见错误 — 纯一度出现在内部小节**：

```lilypond
% WRONG: 小节 3 出现纯一度（a, 对 a）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      c1 e a d e f g c  % 小节3: a, 对 a = P1（内部禁用！）
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

---

## 第二类对位 — 二音对一音 (2:1)

**节奏**：定旋律全音符，对位声部两个二分音符。

**规则**：
1. **强拍（第 1 个二分音符）** — 必须与定旋律**协和**
2. **弱拍（第 2 个二分音符）** — 协和则自由；不协和则**仅允许经过音**
3. **经过音条件**：弱拍出现、前后强拍音协和、级进（二度）运动、前后三音同方向
4. 禁止连续强拍出现相同的完全协和音程
5. 不允许连线、不允许重复音

**正确示例**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 对位声部：每小节两个二分音符
      % 强拍协和，弱拍可经过音或协和
      e2 g d2 f e2 g a2 c''
      b2 a g2 f a2 f g2 e
      c1
    }
    \new Staff \relative c {
      \clef bass
      % 定旋律
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**逐小节分析**：

| 小节 | 低音 | 强拍音/音程 | 弱拍音/音程 | 说明 |
|------|------|------------|------------|------|
| 1 | c | e'/M3 ✓ | g'/P5 ✓ | 两拍皆协和 |
| 2 | b, | d'/M3 ✓ | f'/P5 ✓ | 两拍皆协和 |
| 3 | a, | e'/P5 ✓ | g'/m7 | g' 为不协和弱拍音 |
| 4 | d | a'/P5 ✓ | c''/m7 | c'' 经过音 d'→?→? |
| 5 | e | b'/P5 ✓ | a'/P4 | a' 经过音 |
| 6 | f | a'/M3 ✓ | f'/P8 ✓ | 两拍皆协和 |
| 7 | g | g'/P8 ✓ | b'/M3 ✓ | 协和 |
| 8 | c | e'/M3 ✓ | — | 终止 |

**常见错误 — 不协和音出现在强拍**：

```lilypond
% WRONG: 小节 2 强拍 f' 与低音 d 形成 m3（协和）但弱拍 e' 与 d 形成 M2（不协和且非经过音）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      c2 e f2 e d2 f e2 g
      f2 a g2 b a2 c' g2 e
      c1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 弱拍不协和音必须是经过音（级进、同方向）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 小节2: d'(强拍,M3) → f'(弱拍,P5,协和) ✓
      % 小节3: e'(强拍,P5) → f'(弱拍,m6,经过音 e'→f'→g') ✓
      c2 e d2 f e2 f g2 a
      f2 g a2 b g2 a e2 g
      c1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

---

## 第三类对位 — 四音对一音 (4:1)

**节奏**：定旋律全音符，对位声部四个四分音符。

**规则**：
1. **第 1、3 拍（强拍）** — 必须协和
2. **第 2、4 拍（弱拍）** — 可协和，可不协和
3. **不协和处理**有五种模式：

| 模式 | 缩写 | 说明 |
|------|------|------|
| **经过音** | P | 级进填充两个协和音之间，同方向 |
| **上助音** | UN | 级进上行到不协和音，再级进返回同一协和音 |
| **下助音** | LN | 级进下行到不协和音，再级进返回 |
| **换音** | Camb | 协和→下行级进→三度跳进→级进返回 |
| **双助音** | DN | 协和→上助不协和→下助不协和→返回协和 |

4. 不允许多于一个连续的不协和音（除换音模式外）
5. 若四个以上音符同向级进，第三个音必须协和（"五音规则"）

**正确示例**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 每小节四个四分音符
      % 强拍(1,3拍)协和，弱拍(2,4拍)可为经过音/助音
      c'4 d e d  d c b c  d c d c  f d f f
      g a b a  f e d c  d c b d  c1    }
    \new Staff \relative c {
      \clef bass
      % 定旋律
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**逐小节分析**：

| 小节 | 低音 | 拍 1 | 拍 2 | 拍 3 | 拍 4 | 说明 |
|------|------|------|------|------|------|------|
| 1 | c | c'/P8 ✓ | d'/M2 | e'/M3 ✓ | d'/M2 | 经过音模式 |
| 2 | b, | d'/M3 ✓ | c'/M2 | b/M3 ✓ | c'/M2 | 下助音模式 |
| 3 | a, | d'/P5 ✓ | c'/m6 | d'/P5 ✓ | c'/m6 | 下助音模式 |
| 4 | d | f/m3 ✓ | d/P8 ✓ | f/m3 ✓ | f/P8 ✓ | 全协和音程 |
| 5 | e | g/m3 ✓ | a/P4 | b/P5 ✓ | a/P4 | 上助音模式 |
| 6 | f | f/P8 ✓ | e/M7 | d/M6 ✓ | c/m7 | 经过音下行 |
| 7 | g | d/P5 ✓ | c/P4 | b/M6 ✓ | d/P5 | 经过音 c |
| 8 | c | c'/P8 | — | — | — | 终止 |

**常见错误 — 强拍不协和**：

```lilypond
% WRONG: 小节 2 第 1 拍 c' 与低音 b, 形成 M2（不协和！强拍必须协和）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      c4 d e d  c b a b  d e f e  g f e f
      f g a g  a b c' b  a g f g  c1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

---

## 第四类对位 — 切分/延留对位

**节奏**：定旋律全音符，对位声部用二分音符跨小节连线（切分）。

**延留音三阶段**：

| 阶段 | 位置 | 性质 | 要求 |
|------|------|------|------|
| **准备** (Prep) | 前小节弱拍 | 协和 | 必须与定旋律协和 |
| **延留** (Susp) | 当前小节强拍 | 不协和 | 由连线产生 |
| **解决** (Res) | 当前小节弱拍 | 协和 | **向下级进**解决 |

**允许的延留类型**（对位在上方时）：

| 类型 | 强拍（不协和） | 弱拍（解决） | 说明 |
|------|---------------|-------------|------|
| **4-3** | 纯四度 | 大三/小三度 | 最常用 |
| **7-6** | 小七度 | 大六/小六度 | 常用 |
| **9-8** | 大九度 | 纯八度 | 较空，慎用 |

**正确示例**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 延留对位：连线跨小节
      % 模式：准备(协和) → 延留(不协和) → 解决(下行级进)
      r2 g'~  % 小节1: 弱拍 g' 为准备音（与 c 形成 P5，协和）
      g f~    % 小节2: g=4-3延留(与d=P4→不协和), f=解决(与d=m3→协和)
      f e~    % 小节3: f=延留(与e=m2→不协和), e=解决(与e=P1→协和)
      e d~    % 小节4: e=9-8延留(与f=M2→不协和), d=解决(与f=M6→协和)
      d b~    % 小节5: d=延留(与g=P5→协和,无延留), b=弱拍(与g=M3→协和)
      b a~    % 小节6: b=9-8延留(与a=M2→不协和), a=解决(与a=P1→协和)
      a g~    % 小节7: a=9-8延留(与g=M2→不协和), g=解决(与g=P1→协和)
      g c'2   % 小节8: g=延留(与c=P5→协和), 终止
    }
    \new Staff \relative c {
      \clef bass
      % 定旋律
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**延留分析**：

| 小节 | 低音 | 强拍音程 | 类型 | 弱拍解决 |
|------|------|---------|------|---------|
| 2 | b,→d | g' 与 d = P4 | **4-3** | f（与 d = m3）|
| 3 | d→e | f 与 e = m2 | **2-1** | e（与 e = P1）|
| 4 | e→f | e 与 f = M2 | **9-8** | d（与 f = M6）|
| 6 | f→a | b 与 a = M2 | **9-8** | a（与 a = P1）|
| 7 | a→g | a 与 g = M2 | **9-8** | g（与 g = P1）|

**常见错误 — 延留音上行解决**：

```lilypond
% WRONG: 小节 2 延留音 g 上行解决到 a（必须下行级进！）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      r2 g'~
      g a~   % WRONG: g(延留) → a(上行解决)。应下行到 f
      a g~
      g f~
      f e~
      e d~
      d c'~
      c'1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 延留音 g 下行解决到 f
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      r2 g'~
      g f~   % CORRECT: g(4-3延留) → f(下行级进解决到 m3)
      f e~
      e d~
      d b~
      b a~
      a g~
      g c'1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**常见错误 — 缺少准备阶段**：

```lilypond
% WRONG: 小节 2 强拍延留音 g 未在前小节弱拍准备
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      r2 e'   % 弱拍是 e' 而不是 g
      g f~    % WRONG: g 突然出现，未经准备！
      f e~
      e d~
      d b~
      b a~
      a g~
      g c'1
    }
    \new Staff \relative c {
      \clef bass
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

---

## 第五类对位 — 华丽对位

**节奏**：前四类物种的自由组合。全音符、二分音符、四分音符、连线切换使用。

**规则**：
1. **每种节奏型适用对应物种的规则**：
   - 全音符段落 → 第一类规则
   - 二分音符段落 → 第二类规则
   - 四分音符段落 → 第三类规则
   - 连线/切分段落 → 第四类规则
2. **八分音符**可偶尔使用（通常一对），必须级进
3. **目标**：接近真实声乐复调——可唱、节奏多样、音乐性强

**正确示例**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 华丽对位：混合各物种节奏
      % 小节1: 第二类（二分音符）
      c'2 e
      % 小节2: 第三类（四分音符）
      d4 c b c
      % 小节3: 第三类（四分音符，含经过音）
      e4 f g f
      % 小节4: 第二类 + 第四类（连线延留）
      a2 g~
      % 小节5: 第四类（延留解决）+ 第三类
      g4 f e2
      % 小节6: 第三类（四分音符）
      a4 g f e
      % 小节7: 第四类（延留）
      d2~ d4 c
      % 小节8: 终止（全音符）
      c1
    }
    \new Staff \relative c {
      \clef bass
      % 定旋律
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**节奏类型标注**：

| 小节 | 节奏型 | 适用规则 |
|------|--------|---------|
| 1 | 二分音符 ×2 | 第二类：强拍协和 |
| 2 | 四分音符 ×4 | 第三类：拍 1,3 协和 |
| 3 | 四分音符 ×4 | 第三类：经过音 f |
| 4 | 二分 + 连线 | 第二/四类：a 协和，g 为延留准备 |
| 5 | 连线解决 + 二分 | 第四类：g→f 延留解决 |
| 6 | 四分音符 ×4 | 第三类：级进下行 |
| 7 | 延留 + 四分 | 第四类：d 延留→c 解决 |
| 8 | 二分 + 全音符 | 终止 |

---

## 二声部对位通则

以下为所有物种共享的二声部写作规则（补充 shared-rules.md 中的通用规则）。

### 旋律规则

| 规则 | 说明 |
|------|------|
| **旋律范围 ≤ 十度** | 对位声部全曲音域不超过十度 |
| **级进为主** | 跳进应占少数，且跳进后应反方向级进 |
| **禁止旋律跳进** | 七度、三全音、所有增减音程 |
| **避免重复音** | 不连续重复同一音（第四类连线除外） |
| **旋律轮廓** | 应有明确的拱形（先升后降或先降后升） |

### 声部间距

| 规则 | 说明 |
|------|------|
| **两声部间距** | 一般不超过两个八度，优先保持在一个八度内 |
| **声部交错** | 上方声部不可低于下方声部 |
| **声部超越** | 不可越过相邻声部前一音的位置 |

### 终止式处理

**定旋律在下方的终止**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 上方对位声部的终止：导音(b) → 主音(c')
      % 倒数第二小节: 大六度(f' 与 a,)
      % 最后一小节: 八度(c' 与 c)
      s1*6 a'1 c1
    }
    \new Staff \relative c {
      \clef bass
      % 定旋律终止: 2̂→1̂ (d→c)
      % 需升高第七级导音: b, → c (b, 为导音)
      s1*6 d1 c1
    }
  >>
  \layout { }
}
```

**定旋律在上方的终止**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 定旋律在上方终止: 2̂→1̂ (d'→c')
      s1*6 d'1 c'1
    }
    \new Staff \relative c {
      \clef bass
      % 下方对位声部终止:
      % 倒数第二小节: 小三度(b, 与 d')
      % 最后一小节: 八度(c 与 c')
      s1*6 b,1 c1
    }
  >>
  \layout { }
}
```

---

## 三声部对位

三声部对位在一对一基础上增加一个声部，规则扩展如下。

**附加规则**：
1. 每对声部之间均需遵守二声部规则（无平行五/八度等）
2. 不完全协和音程（3 度/6 度）应占多数
3. 三声部中允许不完全三和弦（省略五音，重复根音或三音）
4. 声部间距：上方两声部间距 ≤ 八度，低音与中间声部可更宽

**三声部第一类对位示例**：

```lilypond
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % 高音声部 (Soprano)
      c'1 d' e' f' g' a' g' c''
    }
    \new Staff \relative c' {
      \clef treble
      % 中间声部 (Alto)
      e1 d e f d c e c'
    }
    \new Staff \relative c {
      \clef bass
      % 低音声部 (Bass / C.F.)
      c1 b, a, d e f g c
    }
  >>
  \layout { }
}
```

**逐小节音程验证**（B=低音, A=中音, S=高音）：

| 小节 | B-A | A-S | B-S | 运动 |
|------|-----|-----|-----|------|
| 1 | M3 | m6 | P8 | — |
| 2 | M3 | M6 | m10 | 混合 |
| 3 | M3 | P8 | m10 | 混合 |
| 4 | M3 | M6 | M10 | 同向（不完全协和，可接受）|
| 5 | M3 | P4 | P5 | 需注意 B-S 平行 |
| 6 | m3 | m6 | m10 | 同向 |
| 7 | m3 | m3 | P5 | 混合 |
| 8 | M3 | m6 | P8 | 反向 ✓ |

**注意**：小节 4-5 的低音-高音从 M10 到 P5，非平行完全协和，可接受。小节 5 中 A-S 的 P4 在三声部中被中间声部缓冲，实际可接受度取决于上下文。

---

## 常见错误总结

| 错误类型 | 说明 | 出现物种 |
|---------|------|---------|
| 平行五度 | 两个声部连续纯五度同向 | 所有 |
| 平行八度 | 两个声部连续纯八度同向 | 所有 |
| 隐伏五度 | 外声部同向进入 P5，高音跳进 | 所有 |
| 隐伏八度 | 外声部同向进入 P8，高音跳进 | 所有 |
| 强拍不协和 | 非延留音的不协和音出现在强拍 | 2, 3, 5 |
| 延留音上行解决 | 延留音必须向下级进解决 | 4, 5 |
| 延留音无准备 | 延留音必须在前小节弱拍协和准备 | 4, 5 |
| 内部纯一度 | 非首尾小节使用纯一度 | 1, 2, 3 |
| 旋律跳进七度 | 旋律中禁止七度、增减音程跳进 | 所有 |
| 连续同向跳进 | 除非分解三和弦，避免连续同向跳进 | 所有 |
| 声部交错 | 高音部低于低音部 | 所有 |
| 超范围 | 对位声部旋律范围超过十度 | 所有 |

### 平行五度 vs 平行三度对比

```lilypond
% WRONG: 平行五度（禁止）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % g'→a' 上行二度
      g'1 a
    }
    \new Staff \relative c {
      \clef bass
      % c→d 上行二度
      % c→g' = P5, d→a' = P5 → 平行五度！
      c1 d
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 平行三度（允许，最多连续 3 个）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % e'→f' 上行二度
      e'1 f
    }
    \new Staff \relative c {
      \clef bass
      % c→d 上行二度
      % c→e' = M3, d→f' = m3 → 平行三度 ✓
      c1 d
    }
  >>
  \layout { }
}
```

### 隐伏五度 vs 反向进入五度

```lilypond
% WRONG: 隐伏五度（外声部同向进入 P5，高音跳进）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % c'→g' 跳进上行（纯五度跳进）
      c'1 g'
    }
    \new Staff \relative c {
      \clef bass
      % e→d 下行级进
      % 但高音跳进 + 进入 P5(d→g' = P11 = P4?)
      e1 d
    }
  >>
  \layout { }
}
```

```lilypond
% CORRECT: 反向运动进入五度（安全）
\version "2.24.0"
\score {
  \new ChoirStaff <<
    \new Staff \relative c'' {
      \clef treble
      % e'→d' 下行级进
      e'1 d'
    }
    \new Staff \relative c {
      \clef bass
      % c→g 下行
      % g→d' = P5, 反向进入 ✓
      c1 g
    }
  >>
  \layout { }
}
```

---

## AI 生成约束总结

在生成 LilyPond 对位代码时，注入以下约束（补充 shared-rules.md 的通用规则）：

1. **第一类**：仅协和音程，纯一度仅首尾
2. **第二类**：强拍协和，弱拍不协和须为经过音（级进、同方向）
3. **第三类**：拍 1/3 协和，拍 2/4 可为经过音/助音/换音
4. **第四类**：延留三阶段（准备→延留→下行级进解决），不可缺一
5. **第五类**：每种节奏型适用对应物种规则
6. **旋律范围 ≤ 10 度**，级进为主，跳进后反向
7. **开始/结束完全协和**（P1/P5/P8），终止式正确
8. **反向进行优先**，声部独立性最大化
9. **三声部**：每对声部均遵守二声部规则
