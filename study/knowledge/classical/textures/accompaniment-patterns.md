---
category: classical
source: "Accompaniment pattern reference"
confidence: text_derived
tags: [织体, 伴奏, accompaniment, Alberti, 分解和弦, 琶音, 固定音型, 钢琴]
title: 伴奏织体模式大全
title_en: Accompaniment Patterns Catalog
difficulty: intermediate
contexts: [classical, texture, piano, accompaniment]
---

# 伴奏织体模式大全

本文件收录 10 种常见伴奏织体模式，每种均含完整 LilyPond 示例。
钢琴左手音域与 `\relative` 参考 → 见 shared-rules.md §一（键盘乐器）。

> **通用约定**：所有低音谱表示例均使用 `\relative c`（对应 C3），右手旋律省略，
> 仅展示伴奏织体本身。实际生成时可将右手旋律层叠在 `<< >>` 中。

---

## 1. Alberti Bass（阿尔贝蒂低音）

**最经典的古典钢琴伴奏音型**。莫扎特、海顿奏鸣曲中无处不在。
核心逻辑：低-高-中-高（root–5th–3rd–5th）交替循环，产生持续流动感。

### 1a. C 大调 Alberti Bass（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% Alberti Bass — C 大调
% 音型：低-高-中-高 (root-5th-3rd-5th)
% 和声：I - IV - V7 - I
% 适用于：古典奏鸣曲快板乐章、小步舞曲伴奏
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % I 和弦 (C-E-G)：经典 Alberti 音型
    c8 g' e g c g e g |
    % IV 和弦 (F-A-C)：保持同样的交替模式
    f,8 c' a c f c a c |
    % V7 和弦 (G-B-D-F)：属七和弦的 Alberti
    g8 d' b d g d b d |
    % 回到 I 和弦，完满终止
    c,8 g' e g c4 r4 |
  }
  \layout { }
}
```

### 1b. G 大调 Alberti Bass 变体（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% Alberti Bass — G 大调变体
% 音型同上，调性移至 G 大调
% 和声：I - vi - IV - V - I
% 适用于：小奏鸣曲、古典风格小品
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key g \major

    % I 和弦 (G-B-D)
    g8 d' b d g d b d |
    % vi 和弦 (E-G-B)
    e,8 b' g b e b g b |
    % IV 和弦 (C-E-G) → V 和弦 (D-F#-A)
    c,8 g' e g d'8 a' fis a |
    % 回到 I 和弦
    g,8 d' b d g4 r4 |
  }
  \layout { }
}
```

**要点**：
- 八分音符持续流动，与右手旋律形成节奏互补（→ shared-rules.md §3.9 钢琴节奏分配）
- 低音根音始终在最下方，三音和五音在上方交替
- 速度通常 ♩ = 100-132（快板乐章），保持轻盈均匀

---

## 2. Broken Chords（分解和弦）

比 Alberti Bass 更灵活，音型不限于低-高-中-高，可以是纯上行、上行+下行、
或其他排列。浪漫派和印象派常用。

### 2a. 上行分解和弦（C 大调 4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 分解和弦 — 纯上行型
% 音型：1-3-5-8（根音到八度）
% 和声：I - V - vi - IV - V - I
% 适用于：练习曲、抒情小品
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % I：C-E-G-C（上行一个八度）
    c8 e g c e g c4 |
    % V：G-B-D-G
    g,8 b d g b d g4 |
    % vi (Am) → IV (F)
    a,8 c e a c e a4 |
    f,8 a c f a c f4 |
    % V → I 终止
    g,8 b d g d b g4 |
    c,8 e g c4 r4 |
  }
  \layout { }
}
```

### 2b. 上行+下行分解和弦（F 大调 4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 分解和弦 — 上行+下行（波浪型）
% 音型：1-3-5-8-5-3（一个八度波浪）
% 和声：I - IV - V7 - I
% 适用于：浪漫派夜曲、抒情段落
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key f \major

    % I (F-A-C)：上行到八度再回落
    f8 a c f c a f4 |
    % IV (Bb-D-F)
    bes,8 d f bes f d bes4 |
    % V7 (C-E-G-Bb)：七和弦分解
    c,8 e g bes g e c4 |
    % 回到 I
    f,8 a c f4 r4 |
  }
  \layout { }
}
```

**要点**：
- 上行型给人积极、推进的感觉；波浪型更抒情柔和
- 分解和弦跨越的音域越宽，音响越丰满（但注意不要侵入右手旋律区域）
- 左手舒适音域：C2–C4（→ shared-rules.md 钢琴左手）

---

## 3. Arpeggio Accompaniment（琶音伴奏）

跨越两个八度以上的大范围琶音，肖邦夜曲的标志性织体。
通常配合延音踏板使用，营造朦胧丰满的音响。

### 3a. 肖邦风格夜曲琶音（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 琶音伴奏 — 肖邦夜曲风格
% 跨越两个八度的宽幅琶音
% 和声：I - vi - IV - V7（经典浪漫进行）
% 配合 \sustainOn 延音踏板，每小节换一次踏板
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % === 第 1 小节：I 和弦 (C-E-G) ===
    % 踏板开 → 琶音从低到高横跨两个八度 → 踏板关
    \sustainOn
    c,8 g' e' c'' e'' c'' e' g |
    \sustainOff
    % === 第 2 小节：vi 和弦 (A-C-E) ===
    \sustainOn
    a,8 e' c' a' c'' a' c' e |
    \sustainOff
    % === 第 3 小节：IV 和弦 (F-A-C) ===
    \sustainOn
    f,8 c' a' f'' a'' f'' a' c |
    \sustainOff
    % === 第 4 小节：V7 → I ===
    \sustainOn
    g,8 d' b' f'' d'' b' f' d |
    \sustainOff
    \sustainOn
    c,8 g' e' c'' e' g c4 |
    \sustainOff
  }
  \layout { }
}
```

### 3b. 简化琶音（适合初级作品，4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 琶音伴奏 — 简化版（一个半八度）
% 适合初级到中级难度的作品
% 和声：I - V - vi - IV - V - I
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 3/4
    \key c \major

    % I 和弦：C-G-E'-G-C'-E'（每拍 3 个八分音符）
    \sustainOn
    c8 g' e' g c' e' |
    % V 和弦：G-D-B-D-G-B
    g,8 d' b d g b |
    % vi (Am) → IV (F)
    a,8 e' c' e a c |
    f,8 c' a' c f a |
    % V → I
    g,8 d' b' d g d |
    c,8 g' e'4 r4 |
    \sustainOff
  }
  \layout { }
}
```

**要点**：
- 踏板标记 `\sustainOn` / `\sustainOff` 控制延音踏板
- 每换一个和弦需更换踏板（先关后开，即"踏板切换"）
- 琶音音域宽时，最低音与最高音之间不超过两个半八度为宜
- 肖邦常用十度以上的跨度，但需根据难度等级调整

---

## 4. Oom-pah-pah / Waltz Bass（圆舞曲低音）

3/4 拍的标志性伴奏模式。第一拍低音，第二、三拍和弦。
华尔兹、小步舞曲、兰德勒舞曲的标准织体。

### 4a. 标准圆舞曲伴奏（8 小节）

```lilypond
\version "2.24.0"
% ============================================
% 圆舞曲低音（Oom-pah-pah）
% 拍号：3/4
% 模式：低音(强拍) + 和弦(弱拍) + 和弦(弱拍)
% 和声：I - V - vi - IV - ii - V7 - I
% 适用于：华尔兹、小步舞曲、施特劳斯风格
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 3/4
    \key c \major

    % === I 和弦 ===
    % 低音 C（第一拍强），和弦（第二、三拍轻）
    c,4 <c' e g>4 <c e g>4 |
    % === V 和弦 ===
    g,4 <g' b d>4 <g b d>4 |
    % === vi (Am) ===
    a,4 <a' c e>4 <a c e>4 |
    % === IV (F) ===
    f,4 <f' a c>4 <f a c>4 |
    % === ii (Dm) ===
    d,4 <d' f a>4 <d f a>4 |
    % === V7 ===
    g,4 <g' b f'>4 <g b f'>4 |
    % === I（终止） ===
    c,4 <c' e g>4 <c e g>4 |
    c,4 <c' e g>2 |
  }
  \layout { }
}
```

### 4b. 带低音变化的华尔兹（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 圆舞曲变体 — 低音交替（根音与五音交替）
% 更精致的华尔兹写法，避免单调
% 第 1 拍：根音 | 第 2 拍：和弦 | 第 3 拍：和弦
% 下一小节第 1 拍换为五音
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 3/4
    \key c \major

    % I 和弦：根音低音 + 和弦
    c,4 <c' e g>4 <c e g>4 |
    % I 和弦：五音低音 + 和弦（变化低音，更有趣）
    g,4 <c' e g>4 <c e g>4 |
    % V 和弦：根音低音
    g,4 <g' b d>4 <g b d>4 |
    % V 和弦：三音低音 + 解决到 I
    b,4 <g' b d>4 <g d'>4 |
    c,4 <c' e g>2 |
  }
  \layout { }
}
```

**要点**：
- 第一拍低音应比后两拍和弦强（力度层次：`f` vs `p`）
- 低音通常在 C2–G3 之间，和弦在 C3–C4 之间
- 变体写法：低音在根音与五音之间交替，增加趣味
- 小步舞曲用相同织体，但速度更慢、更端庄

---

## 5. March Pattern（进行曲模式）

坚定有力的柱式和弦，强调节拍重音。常用于进行曲、颂歌、庄严段落。

### 5a. 标准进行曲伴奏（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 进行曲伴奏模式
% 特征：强拍柱式和弦 + 重音记号
% 拍号：4/4（或 2/4）
% 和声：I - IV - V - I（简洁有力）
% 适用于：进行曲、颂歌、庆典音乐
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % 强拍和弦带重音（-^），短促有力
    % I 和弦：强-弱-次强-弱
    <c e g>4-^ <c e g>8 r8 <c e g>4-^ r4 |
    % IV 和弦
    <f a c>4-^ <f a c>8 r8 <f a c>4-^ r4 |
    % V 和弦（属和弦，增加紧张度）
    <g b d>4-^ <g b d>8 r8 <g b d>4-> <g b d>8 r8 |
    % I 和弦（终止，最后和弦更长）
    <c e g>4-^ <c e g>8 r8 <c e g>2-^ |
  }
  \layout { }
}
```

### 5b. 带低音进行曲（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 进行曲变体 — 低音 + 和弦交替
% 第 1 拍：低音八度 | 第 2-3 拍：和弦
% 更具冲击力的军乐风格
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 2/4
    \key c \major

    % 低音八度 + 和弦交替（2/4 拍，紧凑有力）
    <c, c'>8-^ <c' e g>8-^ <c, c'>8-^ <c' e g>8-^ |
    <f, f'>8-^ <f' a c>8-^ <f, f'>8-^ <f' a c>8-^ |
    <g, g'>8-^ <g' b d>8-^ <g, g'>8-^ <g' b d>8-^ |
    <c, c'>8-^ <c' e g>4-^ r8 |
  }
  \layout { }
}
```

**要点**：
- 重音记号 `-^`（marcato）是进行曲的灵魂，每个强拍必须标注
- 休止符制造"呼吸"空间，体现进行曲的果断干脆
- 低音八度 `<c c'>` 增加厚度与冲击力
- 速度通常 ♩ = 110-120（行板/进行速度）

---

## 6. Repeated Chords / Tremolo（重复和弦 / 震音）

持续重复的和弦或震音，营造紧张感或推动力。
贝多芬"悲怆"奏鸣曲、舒伯特艺术歌曲中常见。

### 6a. 重复柱式和弦（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 重复和弦伴奏
% 八分音符柱式和弦持续重复
% 制造持续的紧张感和推进力
% 和声：i - iv - V - i（小调，戏剧性）
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \minor

    % i 和弦 (C-Eb-G)：8 个八分音符持续重复
    <c ees g>8 <c ees g> <c ees g> <c ees g>
    <c ees g>8 <c ees g> <c ees g> <c ees g> |
    % iv 和弦 (F-Ab-C)
    <f aes c>8 <f aes c> <f aes c> <f aes c>
    <f aes c>8 <f aes c> <f aes c> <f aes c> |
    % V 和弦 (G-B-D)：大调属和弦（和声小调）
    <g b d>8 <g b d> <g b d> <g b d>
    <g b d>8 <g b d> <g b d> <g b d> |
    % 回到 i，渐弱收尾
    <c ees g>8 <c ees g> <c ees g> <c ees g>
    <c ees g>4 r4 |
  }
  \layout { }
}
```

### 6b. 震音（Tremolo）效果（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 震音（Tremolo）伴奏
% 使用 :32 记号表示三十二分音符震音
% 两个音之间快速交替，制造管弦乐般的持续音响
% 适用于：戏剧性段落、高潮推进
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \minor

    % 震音记号 c8:32 表示将八分音符演奏为三十二分音符震音
    % c:32 与 ees:32 交替 = C 与 Eb 之间快速颤动
    c8:32 ees:32 g:32 ees:32 c4:32 g4:32 |
    f8:32 aes:32 c:32 aes:32 f4:32 c4:32 |
    g8:32 b:32 d:32 b:32 g4:32 d4:32 |
    c8:32 ees:32 g:32 c:32 c2:32 |
  }
  \layout { }
}
```

**要点**：
- 重复和弦注意力度变化：可做 `p` → `f` 渐强推进
- 震音 `:32` 在 LilyPond 中自动生成快速交替效果
- 重复和弦持续 4 小节以上容易单调，应配合和声变化或力度变化
- 小调重复和弦特别适合戏剧性、悲壮的场景

---

## 7. Ostinato / Ground Bass（固定低音）

不断重复的低音模式（通常 4 或 8 小节一个循环），上方声部自由发展。
巴洛克时期的标志性技法，帕赫贝尔、珀塞尔、巴赫均大量使用。

### 7a. 帕赫贝尔风格固定低音（8 小节 = 2 次循环）

```lilypond
\version "2.24.0"
% ============================================
% 固定低音（Ground Bass）— 帕赫贝尔卡农风格
% 低音模式：I - V - vi - iii - IV - I - IV - V
% 4 小节一个循环，使用 \repeat unfold 重复
% 适用于：巴洛克风格、帕萨卡利亚、恰空
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % 固定低音模式（4 小节循环）
    % 使用 unfold 重复 2 次（实际演奏 8 小节）
    \repeat unfold 2 {
      % 第 1 小节：I (C) → V (G)
      c4 g, a, e, |
      % 第 2 小节：IV (F) → I (C) → IV (F) → V (G)
      f, c, f, g, |
    }
  }
  \layout { }
}
```

### 7b. 半音化固定低音（4 小节循环，附上方和声提示）

```lilypond
\version "2.24.0"
% ============================================
% 固定低音变体 — 带半音下行的低音线条
% "lament bass"（哀歌低音）：巴洛克悲歌常用
% 模式：下行半音阶（从主音到属音）
% 适用于：悲伤、哀悼、帕萨卡利亚
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 3/2
    \key c \minor

    % Lament Bass：c - b - bes - a - ab - g（半音下行）
    % 每个音持续一个二分音符
    \repeat unfold 2 {
      c2 b, bes, |
      a,2 aes, g, |
    }
  }
  \layout { }
}
```

**要点**：
- `\repeat unfold N { ... }` 展开重复 N 次，适合固定音型
- 固定低音的关键：模式本身要有足够的旋律趣味和和声逻辑
- 帕赫贝尔式低音每个音对应一个和弦，上方声部可以自由发挥
- Lament Bass（半音下行低音）是表达悲伤的经典公式
- 循环次数通常 4-8 次，过长会令听众疲劳

---

## 8. Syncopated Accompaniment（切分伴奏）

和弦落在弱拍或弱位上，制造"错位"的节奏感。
爵士、流行、拉丁音乐的核心织体，古典音乐中也有运用。

### 8a. 反拍和弦伴奏（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 切分伴奏 — 反拍和弦
% 和弦落在每拍的"后半拍"（off-beat）
% 模式：休止-和弦-休止-和弦-休止-和弦-休止-和弦
% 适用于：爵士、流行、雷鬼、斯卡
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % 所有和弦都在弱位（每拍的后八分音符）
    % I 和弦
    r8 <c' e g> r <c e g> r <c e g> r <c e g> |
    % IV 和弦
    r8 <f a c> r <f a c> r <f a c> r <f a c> |
    % V 和弦
    r8 <g b d> r <g b d> r <g b d> r <g b d> |
    % I 和弦（终止）
    r8 <c e g> r <c e g> <c e g>4 r4 |
  }
  \layout { }
}
```

### 8b. 拉丁风格切分（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 切分伴奏变体 — 拉丁/波萨诺瓦风格
% 混合长短节奏的切分模式
% 节奏：短-长-短-短-长（典型的 bossa nova 节奏）
% 适用于：波萨诺瓦、桑巴、拉丁爵士
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % Bossa nova 节奏型：八分-附点四分-八分-八分-附点四分
    % I 和弦
    <c' e g>8 <c e g>4. <c e g>8 <c e g> <c e g>4. |
    % ii 和弦 (Dm)
    <d f a>8 <d f a>4. <d f a>8 <d f a> <d f a>4. |
    % V 和弦
    <g b d>8 <g b d>4. <g b d>8 <g b d> <g b d>4. |
    % I 和弦（终止）
    <c e g>8 <c e g>4. <c e g>2 |
  }
  \layout { }
}
```

**要点**：
- 反拍和弦 (`r8` 开头) 是爵士/流行最基础的伴奏节奏
- 拉丁风格的切分更复杂，通常混合不同时值
- 切分伴奏通常搭配 Walking Bass 或简单低音线条
- 注意：切分节奏不宜持续太久不加变化，每 4-8 小节应有节奏变化

---

## 9. Walking Bass（行走低音）

爵士乐标志性的低音线条。以级进为主，穿插半音经过音，
形成持续"行走"的四分音符旋律线。

### 9a. 基本行走低音（8 小节）

```lilypond
\version "2.24.0"
% ============================================
% Walking Bass（行走低音）
% 每拍一个四分音符，以级进和跳进交替
% 半音经过音（chromatic passing tones）连接和弦音
% 和声：I - vi - ii - V - I - vi - ii - V - I
% 适用于：爵士标准曲、布鲁斯、摇摆乐
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % === 第 1-2 小节：I → vi ===
    % C 和弦音 (C-E-G) + 经过音 A 连接到 Am
    c4 e g a |
    % Am 和弦音 + 经过音 B 连接到 Dm
    a4 c e b |
    % === 第 3-4 小节：ii → V ===
    % Dm 和弦音 + 经过音连接到 G
    d4 f a bes |
    % G 和弦音 + 经过音连接到 C
    g4 b d fis |
    % === 第 5-6 小节：I → vi（重复模式） ===
    c4 e g a |
    a4 c e bes, |
    % === 第 7-8 小节：ii → V → I ===
    d4 f a bes, |
    g4 b d c |
  }
  \layout { }
}
```

### 9b. 带半音经过音的行走低音（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% Walking Bass — 半音经过音强化版
% 更多半音进行，爵士味道更浓
% 半音邻音（chromatic approach）从下方或上方趋近目标和弦音
% ============================================
\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    \key c \major

    % I → ii：用半音经过音连接
    % C - D - D# - E（半音上行趋近 F）
    c4 d dis e |
    % ii → V：F - Ab(经过) - G - Bb(经过)
    f4 aes g bes |
    % V → I：A - Bb - B - C（半音上行趋近目标）
    a4 bes b c |
    % 终止
    c2 r2 |
  }
  \layout { }
}
```

**要点**：
- 四分音符持续"行走"，不能停顿（除终止外）
- 和弦音（根音、三音、五音、七音）落在强拍（第 1、3 拍）
- 经过音和邻音落在弱拍（第 2、4 拍），包括半音经过音
- 半音趋近（chromatic approach）：从半音下方或上方趋近下一个和弦的根音
- 行走低音通常搭配反拍和弦（→ 见第 8 节切分伴奏）

---

## 10. Chordal Homophony（和弦式织体）

所有声部以相同或相近的节奏同时运动，形成"柱式"音响。
圣咏、赞美诗、颂歌的典型织体。

### 10a. 圣咏风格和弦织体（4 小节）

```lilypond
\version "2.24.0"
% ============================================
% 和弦式织体（Chordal Homophony）
% 圣咏/赞美诗风格
% 所有声部节奏基本一致，但各声部仍有细微节奏差异
% 和声：I - IV - V - I（庄严的教会进行）
% 适用于：圣咏、赞美诗、颂歌、庄严段落
% ============================================
\score {
  \new PianoStaff <<
    % === 右手：上方三声部（S-A-T） ===
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      \key c \major

      % S-A-T 三声部，节奏基本同步但有小变化
      % I 和弦：S=E, A=C, T=G（高音区）
      <e c g>4 <e c g> <e c g> <e c g> |
      % IV 和弦：S=F, A=C, T=A
      <f c a>4 <f c a> <f c a> <f c a> |
      % V 和弦：S=G, A=D, T=B
      <g d b>4 <g d b> <g d b> <g d b> |
      % I 和弦（终止）：S=C, A=E, T=G → 延长
      <c e g>2 <c e g>2 |
    }
    % === 左手：低音 ===
    \new Staff \relative c {
      \clef bass
      \time 4/4
      \key c \major

      % 低音线条：节奏与上方声部一致
      c4 c c c |
      f4 f f f |
      g4 g g g |
      c2 c2 |
    }
  >>
  \layout { }
}
```

### 10b. 带节奏变化的和弦织体（更好的写法）

```lilypond
\version "2.24.0"
% ============================================
% 和弦式织体 — 改良版（声部有节奏独立性）
% 避免"大齐奏"：上方声部偶尔有附点或延长
% 低音略有变化，增加层次感
% 这是更专业的写法（→ shared-rules.md §3.9 节奏独立性）
% ============================================
\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      \key c \major

      % 上方声部：附点变化增加节奏趣味
      <e c g>4. <e c g>8 <e c g>4 <e c g>4 |
      <f c a>4 <f c a> <f c a>4. <f c a>8 |
      <g d b>4. <g d b>8 <g d b>4 <g d b>4 |
      <c e g>2 <c e g>2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      \key c \major

      % 低音：二分音符为主，比上方声部更稳定
      c2 c2 |
      f2 f2 |
      g2 g2 |
      c2 c2 |
    }
  >>
  \layout { }
}
```

**要点**：
- **纯柱式和弦的风险**：所有声部完全同节奏 = "大齐奏"，缺乏层次
- **改良方法**：让低音用更长的时值（二分音符），上方声部用附点变化
- 赞美诗风格中，每个和弦对应一个歌词音节，所以节奏通常简单统一
- 声部间距 → shared-rules.md §3.7（上三声部相邻 ≤ 八度）
- 和弦连接时注意共同音保持（→ shared-rules.md §3.4）

---

## Pattern Selection Guide（织体选择指南）

根据用户请求关键词，选择最合适的伴奏织体：

| 用户请求关键词 | 推荐织体 | 参考小节 |
|--------------|---------|---------|
| 圆舞曲、华尔兹、小步舞曲 | Oom-pah-pah（圆舞曲低音） | §4 |
| 夜曲、抒情、浪漫 | 琶音伴奏 | §3 |
| 进行曲、军队、庆典 | 进行曲模式 | §5 |
| 练习曲、小奏鸣曲 | Alberti Bass 或分解和弦 | §1, §2 |
| 圣咏、赞美诗、颂歌 | 和弦式织体 | §10 |
| 爵士、流行、摇摆 | 切分伴奏 / Walking Bass | §8, §9 |
| 古典奏鸣曲、莫扎特、海顿 | Alberti Bass | §1 |
| 巴洛克、帕赫贝尔、恰空 | Ostinato / Ground Bass | §7 |
| 戏剧性、紧张、悲壮 | 重复和弦 / 震音 | §6 |
| 拉丁、波萨诺瓦、桑巴 | 切分伴奏（拉丁变体） | §8b |
| 布鲁斯、12-bar blues | Walking Bass | §9 |
| 肖邦、李斯特、浪漫派钢琴 | 琶音伴奏（宽幅） | §3a |

---

## 织体组合建议

实际作品中，伴奏织体经常**组合使用**或**交替变化**：

| 组合 | 效果 | 典型场景 |
|------|------|---------|
| Alberti Bass + 旋律 | 经典古典钢琴 | 莫扎特奏鸣曲 |
| Walking Bass + 反拍和弦 | 爵士钢琴三重奏 | 爵士标准曲 |
| Ostinato + 自由旋律 | 巴洛克变奏 | 帕赫贝尔卡农 |
| 琶音 + 延音踏板 + 旋律 | 浪漫派夜曲 | 肖邦夜曲 |
| 圆舞曲低音 + 抒情旋律 | 华尔兹 | 肖邦华尔兹 |
| 进行曲 + 重复和弦 | 庆典/军队 | 进行曲 |

**核心原则**：伴奏织体的选择应服务于音乐的性格（character）、
时代风格（period style）和技术难度（difficulty level）。
当用户描述模糊时，优先使用上表中的关键词匹配推荐织体。
