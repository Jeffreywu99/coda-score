---
category: orchestration
source: "Nikolai Rimsky-Korsakov, Principles of Orchestration, 1912 (posthumous, ed. Maximilian Steinberg)"
confidence: text_derived
tags: [配器, 音色, 俄罗斯学派, 里姆斯基-科萨科夫, 纯音色, 混合音色, 浪漫派]
title: Rimsky-Korsakov — 配器法原理
title_en: Principles of Orchestration (Rimsky-Korsakov)
difficulty: intermediate
contexts: [orchestra, ensemble, chamber]
---

# Rimsky-Korsakov — 配器法原理

基于里姆斯基-科萨科夫本人作品的配器经验总结。原书以俄语写成，1912 年遗作出版。核心贡献：**纯音色 vs 混合音色**的分类体系——这是配器初学者最重要的概念框架。所有音域数据参见 shared-rules.md。

---

## 一、纯音色与混合音色

### 1.1 纯音色 (Pure Timbre)

纯音色指旋律由**单一乐器**或**同族同类乐器**演奏，音色统一、特征明确。

**使用场景**：
- 主题的**首次陈述**——让听众先认识旋律本身的性格
- 需要明确音色辨识度的段落
- 亲密、私密的音乐表达

**典型纯音色**：
- 独奏长笛旋律
- 弦乐齐奏（Vln I 全体）
- 独奏双簧管旋律
- 圆号独奏

### 1.2 混合音色 (Mixed Timbre)

混合音色指旋律由**跨乐器族**的不同乐器组合演奏，音色丰富、层次复杂。

**使用场景**：
- 主题的**重复陈述**——在已认识旋律后增添色彩变化
- 需要增加厚度和庄严感的段落
- 高潮段落的力度加强

**典型混合音色**：

| 组合 | 效果 |
|------|------|
| 长笛 + 双簧管（同度） | 柔和的复合音色，兼具清澈与甜美 |
| 小提琴 + 长笛（八度） | 弦乐的温暖 + 长笛的光辉 |
| 弦乐 + 木管（同度/八度） | 丰满厚实的管弦乐旋律 |
| 圆号 + 弦乐 | 温暖的融合，圆号几乎融入弦乐 |

### 1.3 选择原则

**核心决策逻辑**：

```
首次陈述 → 纯音色（明确性格）
重复陈述 → 混合音色（增添变化）
高潮段落 → 多族混合（最大厚度）
独奏段落 → 纯音色（亲密表达）
```

**LilyPond 示例**（纯音色 vs 混合音色对比，d 小调）：

```lilypond
\version "2.24.0"
\score {
  <<
    % === 第 1-4 小节：纯音色 — 独奏双簧管 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \minor
      \time 4/4
      ^\markup { "纯音色：双簧管独奏" }
      % 双簧管在最佳音区演奏抒情旋律
      d4( f a d' a f d2~ d4 r)
      a'( f d a f' d a2~ a4 r)
    }

    % 弦乐休止（为双簧管让出空间）
    \new Staff \relative c'' {
      \clef treble
      \key d \minor
      \time 4/4
      ^\markup { "小提琴（休止）" }
      R1*2
    }

    % === 第 5-8 小节：混合音色 — 双簧管 + 小提琴 + 中提琴 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \minor
      \time 4/4
      ^\markup { "混合音色：双簧管 + 弦乐" }
      % 双簧管与小提琴同度演奏
      d4-\tenuto( f-\tenuto a-\tenuto d'-\tenuto a f d2~ d4 r)
      a'( f d a f' d a2~ a4 r)
    }

    \new Staff \relative c' {
      \clef alto
      \key d \minor
      \time 4/4
      ^\markup { "中提琴（和声填充）" }
      % 中提琴提供温暖的内声部
      d2( f4 d d2 f4 d)
      c2( d4 c c2 d4 c)
    }
  >>
  \layout { }
}
```

---

## 二、音色对比原则

Rimsky-Korsakov 的核心配器技巧：**通过段落间的乐器配置变化实现音色对比**。

### 2.1 对比方式

| 对比方式 | 说明 | 效果 |
|---------|------|------|
| **族切换** | 段落 A 用弦乐 → 段落 B 用木管 | 鲜明的音色转换 |
| **纯→混切换** | 独奏 → 多乐器合奏 | 从亲密到宏大 |
| **厚度渐变** | 从 1 件乐器逐步增至全奏 | 渐强不仅是力度，更是音色厚度 |
| **音色剥离** | 从全奏逐步减至独奏 | 渐弱配合音色变薄，极为有效 |

### 2.2 段落配器的决策框架

```
乐句 A（8小节）→ 弦乐齐奏（温暖统一）
乐句 A'（8小节）→ 木管+弦乐混合（色彩变化）
乐句 B（8小节）→ 木管独奏+弦乐伴奏（亲密对比）
乐句 A''（8小节）→ 全奏 tutti（高潮释放）
```

**关键原则**：
- 相邻段落**必须**改变乐器配置——否则听觉疲劳
- 音色变化应与曲式结构同步——每个新段落对应新的配器
- 全奏是**稀缺资源**——留到结构高潮处，不可滥用

### 2.3 音色渐变的层次

从最薄到最厚的音色渐变（以 C 大调为例）：

| 层次 | 乐器配置 | 效果 |
|------|---------|------|
| 1（最薄） | 独奏长笛 | 清澈孤寂 |
| 2 | 长笛 + 双簧管（同度） | 复合色彩 |
| 3 | 木管四重奏 | 室内乐般的亲密 |
| 4 | 木管 + 圆号 | 加入铜管温暖感 |
| 5 | 弦乐 + 木管 | 管弦乐的基础厚度 |
| 6 | 全弦乐 + 木管 + 圆号 | 丰满的管弦乐 |
| 7（最厚） | 全奏 tutti | 最大的力量与厚度 |

---

## 三、弦乐组的旋律写作

### 3.1 弦乐是管弦乐的骨架

Rimsky-Korsakov 反复强调：**弦乐是管弦乐的基础**。

- 弦乐齐奏（tutti strings）是最强大、最灵活的基础音色
- 弦乐可以不间断地长时间演奏（不像铜管需要休息）
- 弦乐的五声部（Vln I、Vln II、Vla、Vc、Cb）本身就构成完整的和声织体

### 3.2 旋律分配原则

| 声部 | 旋律角色 | 说明 |
|------|---------|------|
| **Vln I** | 主旋律 | 最常用，音区明亮 |
| **Vln II** | 副旋律/和声 | 支撑 Vln I 或演奏对位旋律 |
| **Vla** | 内声部旋律 | 偶尔承担旋律，音色温暖忧郁 |
| **Vc** | 低音旋律/对位 | C 弦旋律极有表现力（男中音般的歌唱） |
| **Cb** | 低音基础 | 极少独奏旋律，与大提琴八度重叠 |

### 3.3 弦乐分奏 (Divisi)

- **divisi**：一个声部一分为二，增加和声厚度
- 效果：声部变薄（每组人数减半），但和声更丰富
- 用法：需要密集和声但又不想用更多乐器时

**LilyPond 示例**（弦乐 divisi 写法）：

```lilypond
\version "2.24.0"
\score {
  \new Staff \relative c'' {
    \clef treble
    % 小提琴 divisi：同一谱表上两个声部
    \key d \major
    <<
      { d4( fis a d' a fis d2~ d4 r) }
      \\
      { d,4( a' d a' d a' d2~ d4 r) }
    >>
    ^\markup { "div." }
  }
  \layout { }
}
```

### 3.4 俄罗斯浪漫派弦乐风格

**特征**：
- 宽广的旋律线条，长气息乐句
- 浓密的弦乐织体，大量 divisi
- Vln I 在高音区歌唱，Vc 在中高音区对位
- 温暖的内声部（Vla + Vln II）填充和声
- 低音提琴与大提琴八度重叠，提供坚实基础

**LilyPond 示例**（弦乐齐奏，D 大调，12 小节 — 俄罗斯浪漫派风格）：

```lilypond
\version "2.24.0"
\score {
  <<
    % 第一小提琴：宽广的主旋律
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "第一小提琴 — 主旋律" }
      % 第 1-4 小节：主题呈示
      d4(\p\< f' a d' a f d'2\!~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      fis( a d a fis' d a2~ a4 r)
      % 第 5-8 小节：展开
      b,\mf( d' g b g d' b2~ b4 r)
      a,( d fis a fis d' a2~ a4 r)
      g,( b e g e b' g2~ g4 r)
      fis( a d fis d a' d,2~ d4 r)
      % 第 9-12 小节：高潮与收束
      g'4(\f b d' g d' b g2~ g4 r)
      fis( a d a fis' d a2~ a4 r)
      e(\mf g b e b g' e2~ e4 r)
      d,(\> f' a d' d2\!~ d4 r) \bar "|."
    }

    % 第二小提琴：和声支撑与对位
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "第二小提琴 — 和声填充" }
      a4( d fis a fis d a2~ a4 r)
      a( d fis d a' fis d2~ d4 r)
      b( d' g b g d' b2~ b4 r)
      a( d fis d a' fis d2~ d4 r)
      g,( b d' g d' b g2~ g4 r)
      fis( a d fis d a' fis2~ fis4 r)
      e( g b e b g' e2~ e4 r)
      d( fis a d a fis' d2~ d4 r)
      b'( d g b g d' b2~ b4 r)
      a( d fis d a' fis d2~ d4 r)
      g,( b e g e b' g2~ g4 r)
      a,( d fis a a'2~ a'4 r) \bar "|."
    }

    % 中提琴：温暖的内声部
    \new Staff \relative c' {
      \clef alto
      \key d \major
      \time 4/4
      ^\markup { \bold "中提琴 — 内声部" }
      d,( fis a d a fis d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      d'( g b d b g d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      g,( b d' g d' b g2~ g4 r)
      d( fis a d a fis' d2~ d4 r)
      b( e g b g e b2~ b4 r)
      d,( fis a d a fis' d2~ d4 r)
      g,( b d' g d' b g2~ g4 r)
      d( fis a d a fis' d2~ d4 r)
      b( e g b g e b2~ b4 r)
      fis( a d fis fis'2~ fis'4 r) \bar "|."
    }

    % 大提琴：低音线与对位旋律
    \new Staff \relative c {
      \clef bass
      \key d \major
      \time 4/4
      ^\markup { \bold "大提琴 — 低音与对位" }
      d,( fis a d a fis d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      g,( b d' g d' b g2~ g4 r)
      d( fis a d a fis d2~ d4 r)
      g,( b d' g d' b g2~ g4 r)
      d( fis a d a fis d2~ d4 r)
      e,( g b e b g e2~ e4 r)
      d( fis a d a fis d2~ d4 r)
      g,( b d' g d' b g2~ g4 r)
      d( fis a d a fis d2~ d4 r)
      e,( g b e b g e2~ e4 r)
      d( fis a d d2~ d4 r) \bar "|."
    }

    % 低音提琴：低音基础（记谱高八度）
    \new Staff \relative c {
      \clef bass
      \key d \major
      \time 4/4
      ^\markup { \bold "低音提琴 — 低音基础" }
      % 低音提琴记谱比实际音高高八度
      d,2( a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      e,2( b'4 e e,2 b'4 e,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      e,2( b'4 e e,2 b'4 e,)
      d( a' d2~ d4 r) \bar "|."
    }
  >>
  \layout { }
}
```

---

## 四、木管的色彩运用

### 4.1 木管是色彩大师

Rimsky-Korsakov 认为每件木管乐器都是**独奏家**——它们不应该被当作合唱团员来使用。

**木管独奏的黄金法则**：
- 木管独奏时，弦乐伴奏应在**不同音区**或降低力度（≤ p）
- 避免弦乐在同一音区以同等力度覆盖木管独奏
- 木管独奏段落的伴奏织体应简洁——长音、持续音、简单的拨弦

### 4.2 各木管乐器的色彩角色

| 乐器 | 色彩特征 | 最佳独奏场景 |
|------|---------|------------|
| **长笛** | 清澈、空灵、田园 | 宁静的田园段落、轻盈的装饰 |
| **双簧管** | 甜美、哀愁、田园 | 哀歌、田园牧歌、东方风格旋律 |
| **单簧管** | 灵活、多变、抒情 | 快速灵活的旋律、抒情歌唱 |
| **大管** | 温暖、哀愁、喜剧 | 低音旋律、幽默段落、庄严的低音 |

### 4.3 木管四重奏

木管四重奏（长笛 + 双簧管 + 单簧管 + 大管）是管弦乐中最完美的室内乐组合：
- 音色各异但和谐共存
- 可独立演奏和声段落
- 加入圆号 = 木管五重奏（更温暖）

**LilyPond 示例**（木管旋律 + 弦乐伴奏，a 小调，8 小节）：

展示"木管独奏 + 弦乐轻柔伴奏"的经典配器手法。

```lilypond
\version "2.24.0"
\score {
  <<
    % 双簧管独奏：抒情旋律
    \new Staff \relative c'' {
      \clef treble
      \key a \minor
      \time 4/4
      ^\markup { \bold "双簧管独奏" }
      % 第 1-4 小节：第一乐句
      e4(\mf f8 e d4 e c' b a2~ a4 r)
      a4( b8 a g4 a f' e d2~ d4 r)
      % 第 5-8 小节：第二乐句（略加装饰）
      e4( f8 e d4 c' b8 a g4 f e2~ e4 r)
      d'( c8 b a4 g f8 e d4 c b2 a4 r) \bar "|."
    }

    % 第一小提琴：轻柔伴奏（pp，低于双簧管音区）
    \new Staff \relative c'' {
      \clef treble
      \key a \minor
      \time 4/4
      ^\markup { \bold "第一小提琴（pp 伴奏）" }
      % 长音和声铺垫
      a2\pp( e' a, e')
      d( g, d' g,)
      a( e' a, e')
      d( g, a2 e'4 r) \bar "|."
    }

    % 第二小提琴：轻柔伴奏
    \new Staff \relative c'' {
      \clef treble
      \key a \minor
      \time 4/4
      ^\markup { \bold "第二小提琴（pp 伴奏）" }
      e2\pp( b' e, b')
      a( d, a' d,)
      e( b' e, b')
      a( d, e2 b'4 r) \bar "|."
    }

    % 中提琴：轻柔伴奏
    \new Staff \relative c' {
      \clef alto
      \key a \minor
      \time 4/4
      ^\markup { \bold "中提琴（pp 伴奏）" }
      c2\pp( g' c, g')
      f( c' f, c')
      c( g' c, g')
      f( c' c2 g'4 r) \bar "|."
    }

    % 大提琴：拨弦低音
    \new Staff \relative c {
      \clef bass
      \key a \minor
      \time 4/4
      ^\markup { \bold "大提琴（拨弦）" }
      \pizzicato
      a,4\pp-. r a-. r e'-. r e,-. r
      d-. r d-. r a'-. r a,-. r
      a-. r a-. r e'-. r e,-. r
      d-. r d-. r a'-. r a,-. r \bar "|."
    }
  >>
  \layout { }
}
```

---

## 五、铜管的和声支撑

### 5.1 圆号——万能桥梁

Rimsky-Korsakov 将圆号视为管弦乐中最重要的**桥梁乐器**：
- 连接弦乐和铜管的音色
- 可以融入任何乐器组而不突兀
- 4 支圆号构成完整的和声填充
- 弱奏时几乎融入弦乐，强奏时具有铜管的辉煌

### 5.2 铜管合唱

铜管合唱（圆号 + 小号 + 长号 + 大号）是管弦乐中最庄严的和声支撑：
- 用于宗教风格的段落、庄严的进行曲、英雄性的主题
- 铜管长音和弦是管弦乐最宏伟的背景
- 注意：铜管需要呼吸和休息，不能无限持续

### 5.3 铜管与弦乐的结合

| 组合 | 效果 |
|------|------|
| 弦乐 + 圆号 | 最温暖的融合，圆号几乎"消失"在弦乐中 |
| 弦乐 + 小号 | 弦乐旋律获得光辉的铜管色彩 |
| 弦乐 + 全铜管 | 辉煌的管弦乐高潮 |

### 5.4 俄罗斯浪漫派铜管写法

- 圆号以持续长音和弦提供温暖背景
- 小号在结构高点做号角式的强调
- 长号以庄严的合唱支撑和声
- 大号以持续低音加固基础
- 铜管**不喧宾夺主**——在俄罗斯风格中，弦乐永远是主角

---

## 六、俄罗斯浪漫派配器风格

### 6.1 风格特征

| 特征 | 说明 |
|------|------|
| **弦乐至上** | 弦乐承载绝大部分旋律，音色温暖浓密 |
| **木管点缀** | 木管独奏段落穿插其中，增添色彩变化 |
| **圆号铺垫** | 圆号持续和声背景，温暖地连接一切 |
| **力度渐变** | 大量 cresc./dim.，配合音色的增厚与减薄 |
| **Divisi 织体** | 弦乐大量分奏，产生浓密的和声层次 |
| **段落对比** | 相邻段落明显改变配器，避免单调 |

### 6.2 典型配器布局（以三段体为例）

```
A 段（弦乐为主）：
  - Vln I：主旋律
  - Vln II + Vla：和声填充（divisi）
  - Vc：对位旋律或低音
  - Cb：低音基础

B 段（木管为主）：
  - 双簧管/单簧管：独奏旋律
  - 弦乐：pp 长音伴奏或拨弦
  - 圆号：柔和的和声铺垫

A' 段（全奏再现）：
  - 所有弦乐：主旋律（加厚）
  - 木管：加倍旋律 + 装饰
  - 铜管：和声支撑
  - 定音鼓：节奏强调
```

### 6.3 管弦乐渐强示例（D 大调，16 小节）

展示从 pp 独奏到 ff 全奏的完整渐强过程，同时展示音色从薄到厚的渐变。

**配器层次递进**：
- 第 1-4 小节（pp）：独奏长笛
- 第 5-8 小节（mp）：长笛 + 双簧管 + 单簧管 + 大管（木管四重奏）
- 第 9-12 小节（f）：弦乐 + 木管 + 圆号（管弦乐基础厚度）
- 第 13-16 小节（ff）：全奏 tutti（最大力量与厚度）

```lilypond
\version "2.24.0"
\score {
  <<
    % === 长笛 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "长笛" }
      % 第 1-4 小节：pp 独奏
      d4(\pp f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      % 第 5-8 小节：mp 木管合奏中继续
      d4(\mp f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      % 第 9-12 小节：f 与弦乐和木管合奏
      d4(\f f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      fis( a d a fis' d a2~ a4 r)
      % 第 13-16 小节：ff 全奏
      d,4(\ff f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      a,( d fis a d2~ d4 r) \bar "|."
    }

    % === 双簧管 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "双簧管" }
      % 第 1-4 小节：休止（让长笛独奏）
      R1*2
      % 第 5-8 小节：mp 加入木管合奏
      d4(\mp f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      % 第 9-12 小节：f
      d4(\f f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      fis( a d a fis' d a2~ a4 r)
      % 第 13-16 小节：ff 全奏
      d,4(\ff f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      a,( d fis a d2~ d4 r) \bar "|."
    }

    % === 单簧管（Bb）===
    \new Staff \relative c'' {
      \clef treble
      \transposition bes
      \key e \major
      \time 4/4
      ^\markup { \bold "Bb 单簧管" }
      % 记谱 E 大调 → 实际 D 大调
      % 第 1-4 小节：休止
      R1*2
      % 第 5-8 小节：mp
      e4(\mp g' b e b g' e2~ e4 r)
      e,( g b g e' b g2~ g4 r)
      % 第 9-12 小节：f
      e4(\f g' b e b g' e2~ e4 r)
      e,( g b g e' b g2~ g4 r)
      a,( c' e a e c' a2~ a4 r)
      gis( b e b gis' b, gis2~ gis4 r)
      % 第 13-16 小节：ff
      e,4(\ff g' b e b g' e2~ e4 r)
      e,( g b g e' b g2~ g4 r)
      a,( c' e a e c' a2~ a4 r)
      b,( e gis b e2~ e4 r) \bar "|."
    }

    % === 大管 ===
    \new Staff \relative c {
      \clef bass
      \key d \major
      \time 4/4
      ^\markup { \bold "大管" }
      % 第 1-4 小节：休止
      R1*2
      % 第 5-8 小节：mp 低音
      d,2(\mp a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      % 第 9-12 小节：f
      d,2(\f a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      % 第 13-16 小节：ff
      d,2(\ff a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      a,2( d4 fis a2~ a4 r) \bar "|."
    }

    % === 圆号（F）===
    \new Staff \relative c' {
      \clef treble
      \transposition f
      \key a \major
      \time 4/4
      ^\markup { \bold "F 圆号" }
      % 记谱 A 大调 → 实际 D 大调
      % 第 1-8 小节：休止
      R1*4
      % 第 9-12 小节：f 加入
      a2(\f e'4 a a,2 e'4 a,)
      a2( e'4 a a,2 e'4 a,)
      d,2( a'4 d d,2 a'4 d,)
      a2( e'4 a a,2 e'4 a,)
      % 第 13-16 小节：ff
      a,2(\ff e'4 a a,2 e'4 a,)
      a2( e'4 a a,2 e'4 a,)
      d,2( a'4 d d,2 a'4 d,)
      e2( a4 e' a,2~ a4 r) \bar "|."
    }

    % === 小号（Bb）===
    \new Staff \relative c'' {
      \clef treble
      \transposition bes
      \key e \major
      \time 4/4
      ^\markup { \bold "Bb 小号" }
      % 记谱 E 大调 → 实际 D 大调
      % 第 1-12 小节：休止（全奏才加入）
      R1*6
      % 第 13-16 小节：ff 全奏
      e4(\ff g' b e b g' e2~ e4 r)
      e,( g b g e' b g2~ g4 r)
      a,( c' e a e c' a2~ a4 r)
      b,( e gis b e2~ e4 r) \bar "|."
    }

    % === 定音鼓 ===
    \new Staff \relative c {
      \clef bass
      \time 4/4
      ^\markup { \bold "定音鼓 (D & A)" }
      % 第 1-8 小节：休止
      R1*4
      % 第 9-12 小节：f 加入
      d,4(\f r a' r d, r a' r)
      d,4( r a' r d, r a' r)
      d,( r a' r d, r a' r)
      d,( r a' r d, r a' r)
      % 第 13-16 小节：ff
      d,4(\ff r a' r d, r a' r)
      d,4( r a' r d, r a' r)
      d,( r a' r d, r a' r)
      d,4(-> a'-> d,2 r) \bar "|."
    }

    % === 第一小提琴 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "第一小提琴" }
      % 第 1-8 小节：休止（木管先行）
      R1*4
      % 第 9-12 小节：f 弦乐加入
      d4(\f f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      fis( a d a fis' d a2~ a4 r)
      % 第 13-16 小节：ff 全奏
      d,4(\ff f' a d' a f d'2~ d4 r)
      d,( f a f d' a f2~ f4 r)
      g( b d' g d' b g2~ g4 r)
      a,( d fis a d2~ d4 r) \bar "|."
    }

    % === 第二小提琴 ===
    \new Staff \relative c'' {
      \clef treble
      \key d \major
      \time 4/4
      ^\markup { \bold "第二小提琴" }
      R1*4
      % 第 9-12 小节：f
      a4(\f d fis a fis d a2~ a4 r)
      a( d fis d a' fis d2~ d4 r)
      b( d' g b g d' b2~ b4 r)
      a( d fis d a' fis d2~ d4 r)
      % 第 13-16 小节：ff
      a4(\ff d fis a fis d a2~ a4 r)
      a( d fis d a' fis d2~ d4 r)
      b( d' g b g d' b2~ b4 r)
      a( d fis a a'2~ a'4 r) \bar "|."
    }

    % === 中提琴 ===
    \new Staff \relative c' {
      \clef alto
      \key d \major
      \time 4/4
      ^\markup { \bold "中提琴" }
      R1*4
      % 第 9-12 小节：f
      d,4(\f fis a d a fis d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      d'( g b d b g d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      % 第 13-16 小节：ff
      d,4(\ff fis a d a fis d2~ d4 r)
      d( fis a fis d' a fis2~ fis4 r)
      d'( g b d b g d2~ d4 r)
      fis,( a d fis fis'2~ fis'4 r) \bar "|."
    }

    % === 大提琴 ===
    \new Staff \relative c {
      \clef bass
      \key d \major
      \time 4/4
      ^\markup { \bold "大提琴" }
      R1*4
      % 第 9-12 小节：f
      d,2(\f a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      % 第 13-16 小节：ff
      d,2(\ff a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      a,2( d4 fis a2~ a4 r) \bar "|."
    }

    % === 低音提琴 ===
    \new Staff \relative c {
      \clef bass
      \key d \major
      \time 4/4
      ^\markup { \bold "低音提琴" }
      R1*4
      % 第 9-12 小节：f（记谱高八度）
      d,2(\f a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      d2( a'4 d d,2 a'4 d,)
      % 第 13-16 小节：ff
      d,2(\ff a'4 d d,2 a'4 d,)
      d2( a'4 d d,2 a'4 d,)
      g,2( d'4 g g,2 d'4 g,)
      a,2( d4 fis a2~ a4 r) \bar "|."
    }
  >>
  \layout { }
}
```

---

## 七、AI 生成约束总结

1. **首次陈述用纯音色**：主题第一次出现应使用明确的单一音色（如独奏双簧管或弦乐齐奏）
2. **重复用混合音色**：主题再现时混合不同族乐器，增添色彩变化
3. **段落间音色对比**：相邻段落必须改变乐器配置——族切换、纯→混切换、厚度渐变
4. **弦乐是骨架**：永远不要长时间不用弦乐，弦乐承载大部分旋律
5. **木管是独奏员**：木管独奏时弦乐应降低力度（≤ p）或让出相同音区
6. **圆号是桥梁**：需要连接弦乐和铜管的音色时使用圆号，圆号可以融入任何组合
7. **全奏是稀缺资源**：tutti 应留到结构高潮处使用，不可滥用
8. **力度渐变 = 音色渐变**：crescendo 不仅是音量增大，更是音色从薄到厚的层次递进
9. **音域检查**：→ 见 shared-rules.md 音域速查表
10. **移调乐器**：→ 见 shared-rules.md 移调规则
