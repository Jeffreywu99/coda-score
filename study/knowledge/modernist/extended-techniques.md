---
category: modernist
source: "Extended Techniques and Modern Notation — 20th/21st century practice"
confidence: text_derived
tags: [演奏法, 图形记谱, 偶然音乐, 弦乐技法, 木管技法, 铜管技法, 打击乐, 人声, Cage, Crumb]
title: 现代演奏法与图形记谱
title_en: Extended Techniques and Graphic Notation
difficulty: advanced
contexts: [modernist, extended-techniques, graphic-notation, aleatoric, experimental]
---

# 现代演奏法与图形记谱 — Extended Techniques and Graphic Notation

本文件涵盖 20/21 世纪音乐中的扩展演奏技法记谱、图形记谱近似、偶然音乐记谱，以及现代特殊符号。
乐器音域数据 → 见 `classical/common/shared-rules.md`。
LilyPond 基础语法 → 见 `syntax/lilypond-core-syntax.md`。

---

## 分类说明：标准记谱 vs 扩展记谱

| 类别 | 定义 | LilyPond 支持度 |
|------|------|----------------|
| **标准记谱** | 古典/浪漫时期已有的常规演奏法（pizz., con sord., trill 等） | ✅ 原生支持 |
| **扩展记谱** | 20 世纪后发展的特殊技法（multiphonics, col legno, Sprechstimme 等） | ⚠️ 部分原生 + 文本注释近似 |
| **图形/偶然记谱** | 非传统图形化或不确定性记谱 | ⚠️ 有限近似（LilyPond 非图形工具） |

---

## PART A: 现代演奏法记谱

---

## 1. 弦乐特殊技法 (String Extended Techniques)

> 音域 → 见 `shared-rules.md` 弦乐器表

### 1.1 弓弦位置变化

#### Sul ponticello（靠近琴马演奏）

音色尖锐、泛音丰富。标准记法为文字标注 + 虚线延续。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    % 靠近琴马演奏——文字标注，虚线表示延续
    g4^\markup { \italic "sul ponticello" } a b c |
    d e f g |
    % 恢复正常演奏位置
    a^\markup { \italic "ord." } b c d |
  }
  \layout { }
}
```

#### Sul tasto（靠近指板演奏）

音色柔和、朦胧，泛音较少。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    % 靠近指板——柔和朦胧的音色
    g4^\markup { \italic "sul tasto" } a b c |
    d2 e2 |
    % 恢复正常
    f4^\markup { \italic "ord." } g a b |
  }
  \layout { }
}
```

### 1.2 Col legno（用弓杆演奏）

| 技法 | 意大利语 | 效果 | 记法 |
|------|---------|------|------|
| **Col legno battuto** | 弓杆击弦 | 干涩的打击声 | 文字标注 + 跳音点 |
| **Col legno tratto** | 弓杆拉弦 | 嘶哑的摩擦声 | 文字标注 + 连线 |

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % Col legno battuto——弓杆击弦，干涩打击声
    g4-.^\markup { \italic "col legno battuto" } a-. b-. c-. |
    % Col legno tratto——弓杆拉弦，嘶哑摩擦声
    g2(^\markup { \italic "col legno tratto" } a) |
    % 恢复正常弓法
    b2(^\markup { \italic "arco" } c) |
  }
  \layout { }
}
```

### 1.3 泛音 (Harmonics)

#### 自然泛音 (Natural Harmonics)

在空弦泛音节点轻触产生的音。LilyPond 提供两种记法：
- `\flageolet`：音符上方加小圆圈符号（○）
- `\tweak NoteHead.style #'harmonic`：菱形符头

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 自然泛音——小圆圈符号
    g4\flageolet d'\flageolet g'\flageolet d''\flageolet |
    % 自然泛音——菱形符头（更现代的记法）
    \tweak NoteHead.style #'harmonic g4
    \tweak NoteHead.style #'harmonic d'
    \tweak NoteHead.style #'harmonic g'
    \tweak NoteHead.style #'harmonic d'' |
  }
  \layout { }
}
```

**常用自然泛音音程**：

| 触点位置 | 泛音序数 | 与空弦音程 | 记谱符号 |
|---------|---------|-----------|---------|
| 1/2 弦长 | 第 2 泛音 | 纯八度 | ○ |
| 1/3 弦长 | 第 3 泛音 | 纯八度 + 纯五度 | ○ |
| 1/4 弦长 | 第 4 泛音 | 两个纯八度 | ○ |
| 1/5 弦长 | 第 5 泛音 | 两个八度 + 大三度 | ○ |

#### 人工泛音 (Artificial Harmonics)

实按一指 + 轻触一指。通常记为：实按音用普通符头，轻触音用菱形符头。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 人工泛音——实按音（普通符头）+ 轻触音（菱形符头）
    % 下方音为实按指，上方音为轻触指（通常高纯四度）
    <g \tweak NoteHead.style #'harmonic c'>2
    <a \tweak NoteHead.style #'harmonic d'>2 |
    <b \tweak NoteHead.style #'harmonic e'>2
    <c \tweak NoteHead.style #'harmonic f'>2 |
  }
  \layout { }
}
```

### 1.4 Bartók pizzicato（巴托克拨弦 / 弹拨拨弦）

用力拨弦使其弹击指板，产生尖锐的打击效果。记法为音符上方加圆圈符号 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c' {
    \clef bass
    \time 3/4
    % Bartók pizzicato——弹拨拨弦
    % 用 \stopped（+号）近似，加文字标注
    g4\stopped^\markup { \bold "Bartók pizz." }
    c\stopped g\stopped |
    % 或用 \flageolet（○）近似
    c,2\flageolet^\markup { \italic "snap pizz." }
    g4\flageolet |
  }
  \layout { }
}
```

### 1.5 琴马后方演奏 (Behind the Bridge)

在琴马与拉弦板之间的弦段演奏，产生高频率的金属声。无标准记法，用文字标注 + 特殊符头近似。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 琴马后方演奏——用 × 符头 + 文字标注近似
    \override NoteHead.style = #'cross
    g4^\markup { \italic "dietro il ponticello" }
    a b c |
    d e f g |
    \revert NoteHead.style
    % 恢复正常
    a4^\markup { \italic "ord." } b c d |
  }
  \layout { }
}
```

### 1.6 弦乐扩展技法综合示例

```lilypond
\version "2.24.0"

\header {
  title = "弦乐扩展技法示例"
  composer = "教学示范"
}

\score {
  \new Staff \relative c' {
    \clef bass
    \time 4/4

    % 第 1 小节：Sul ponticello
    c4^\markup { \italic "sul pont." } d e f |

    % 第 2 小节：Sul tasto
    g4^\markup { \italic "sul tasto" } a b c |

    % 第 3 小节：Col legno battuto
    d4-.^\markup { \italic "col legno batt." } e-. f-. g-. |

    % 第 4 小节：自然泛音
    \tweak NoteHead.style #'harmonic c4
    \tweak NoteHead.style #'harmonic g'
    \tweak NoteHead.style #'harmonic c''
    \tweak NoteHead.style #'harmonic g'' |

    % 第 5 小节：Bartók pizzicato
    \revert NoteHead.style
    c,4\stopped^\markup { \bold "B.pizz." } d\stopped e\stopped f\stopped |

    % 第 6 小节：恢复正常
    g2(^\markup { \italic "arco ord." } a) |
  }
  \layout { }
}
```

---

## 2. 木管特殊技法 (Woodwind Extended Techniques)

> 音域 → 见 `shared-rules.md` 木管乐器表

### 2.1 复音 (Multiphonics)

同时吹出多个音高，通过特殊指法实现。记法通常为和弦 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 复音——和弦记法 + 文字标注
    % 实际音高取决于指法，此处为近似记谱
    <g bes d'>1^\markup { \italic "multiphonic" } |
    <a c' e'>1^\markup { \italic "multiph." } |
    % 也可以用文字标注描述具体指法
    g1^\markup { \italic "multiph. (fingering: T-1-2-3)" } |
  }
  \layout { }
}
```

**注意**：复音的实际可用组合高度依赖乐器和指法，需参考专门的指法表（如 Carin Levine 的 *The Techniques of Flute Playing*）。

### 2.2 花舌 (Flutter-tonguing / Frullato)

舌尖快速颤动产生的滚音效果。记法：颤音线 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 花舌——用 :32 表示快速震音 + 文字标注
    g2:32^\markup { \italic "flatterzunge" }
    a:32^\markup { \italic "frull." } |
    % 也可用 \trill 近似
    b2\trill^\markup { \italic "flutter" }
    c\trill |
  }
  \layout { }
}
```

**演奏说明**：花舌本质上是舌尖的滚音（类似发"r"音），不是真正的颤音（trill 是音高交替），因此 `:32` 震音记号比 `\trill` 更准确。

### 2.3 键击声 (Key Clicks)

按下按键但不吹气产生的机械打击声。记法：`×` 符头 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 键击声——× 符头表示无音高的打击效果
    \override NoteHead.style = #'cross
    g4^\markup { \italic "key clicks" } a b c |
    g8 a b c g a b c |
    \revert NoteHead.style
    % 恢复正常吹奏
    g4^\markup { \italic "norm." } a b c |
  }
  \layout { }
}
```

### 2.4 气声 (Breath Sounds / Air Sounds)

仅吹气不产生明确音高的效果。记法：`/` 符头（slash）+ 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 气声——斜线符头表示不确定音高
    \override NoteHead.style = #'slash
    \override Stem.transparent = ##t
    g4^\markup { \italic "air sounds" } a b c |
    g2 a2 |
    \revert NoteHead.style
    \revert Stem.transparent
    % 恢复正常
    g4^\markup { \italic "norm." } a b c |
  }
  \layout { }
}
```

### 2.5 循环呼吸 (Circular Breathing)

通过鼻腔储气实现不间断持续吹奏。记法：呼吸记号 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 循环呼吸——标注提示演奏者在此处换气（但不间断）
    g4^\markup { \italic "circular breathing" }
    \breathe a \breathe b \breathe c |
    d \breathe e \breathe f \breathe g |
    % 连续不间断长音
    a1^\markup { \italic "senza respirare (circ. breath.)" } |
  }
  \layout { }
}
```

### 2.6 音色颤音 (Bisbigliando)

在同一音高上交替使用不同指法，产生微妙的音色变化。记法：颤音线 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 音色颤音——同一音高上交替指法
    g2\trill^\markup { \italic "bisbigliando" }
    g2\trill |
    % 也可以用两个音表示不同指法的交替
    g4^\markup { \italic "bisb. (alt. fing.)" }
    g^\markup { \italic "(alt.)" }
    g g |
  }
  \layout { }
}
```

### 2.7 木管扩展技法综合示例

```lilypond
\version "2.24.0"

\header {
  title = "木管扩展技法示例"
  composer = "教学示范"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4

    % 花舌
    g2:32^\markup { \italic "flatterz." } a:32 |

    % 复音
    <b d' g'>1^\markup { \italic "multiph." } |

    % 键击声
    \override NoteHead.style = #'cross
    c4^\markup { \italic "key clicks" } c c c |
    \revert NoteHead.style

    % 气声
    \override NoteHead.style = #'slash
    \override Stem.transparent = ##t
    c2^\markup { \italic "air" } c |
    \revert NoteHead.style
    \revert Stem.transparent

    % 恢复正常 + 音色颤音
    g2\trill^\markup { \italic "bisbigl." } g |
  }
  \layout { }
}
```

---

## 3. 铜管特殊技法 (Brass Extended Techniques)

> 音域 → 见 `shared-rules.md` 铜管乐器表

### 3.1 弱音器类型 (Mute Types)

| 弱音器 | 英文 | 音色效果 | LilyPond 记法 |
|--------|------|---------|--------------|
| 直排弱音器 | Straight mute | 尖锐、金属感 | `\stopped` + `straight mute` 标注 |
| 杯状弱音器 | Cup mute | 柔和、闷暗 | `\stopped` + `cup mute` 标注 |
| 哈门弱音器 | Harmon mute | 遥远、嗡鸣感 | `\stopped` + `harmon mute` 标注 |
| 柱塞弱音器 | Plunger mute | "哇哇"声 | `\stopped` + `plunger` 标注 |
| 桶状弱音器 | Bucket mute | 柔和、类似弦乐 | `\stopped` + `bucket mute` 标注 |

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4

    % 直排弱音器
    c4\stopped^\markup { \italic "straight mute" } d e f |

    % 杯状弱音器
    g4\stopped^\markup { \italic "cup mute" } a b c |

    % 哈门弱音器
    d4\stopped^\markup { \italic "harmon mute" } e f g |

    % 去除弱音器
    a4^\markup { \italic "open" } b c d |
  }
  \layout { }
}
```

#### 柱塞弱音器开合 (Plunger Open/Close)

柱塞弱音器可以半开半合，产生"哇哇"效果。用 `+`（闭合）和 `o`（开放）标记。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 柱塞弱音器——开合交替
    g4\stopped^\markup { "+" }
    g^\markup { "o" }
    g\stopped^\markup { "+" }
    g^\markup { "o" } |
    % 快速开合产生"哇哇"声
    g8\stopped g g\stopped g g\stopped g g\stopped g |
  }
  \layout { }
}
```

### 3.2 踏板音 (Pedal Tones)

铜管乐器的超低音区（低于正常音域基音），主要在长号和大号上使用。音色粗糙、有力。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    % 长号踏板音——低于正常音域的超低音
    % 长号正常最低音为 E2（e,），踏板音在其下方
    e,4^\markup { \italic "pedal tones" } d, c, b,, |
    % 逐渐上行回到正常音域
    a,,2 g,,2 |
    e,1^\markup { \italic "ped." } |
  }
  \layout { }
}
```

**注意**：踏板音非常消耗气息，不宜长时间持续。→ 音域极限见 `shared-rules.md`。

### 3.3 咆哮 / 边吹边唱 (Growling / Singing while Playing)

#### 咆哮 (Growling)

同时吹奏和哼唱，产生粗糙的复合音色。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 咆哮——用 × 符头 + 文字标注近似
    \override NoteHead.style = #'cross
    g4^\markup { \bold "growl" } a b c |
    d e f g |
    \revert NoteHead.style
    a4^\markup { \italic "norm." } b c d |
  }
  \layout { }
}
```

#### 边吹边唱 (Singing while Playing)

演奏者同时吹奏乐器和用人声唱出另一个音高。记法：两条谱线（或文字标注描述人声音高）。

```lilypond
\version "2.24.0"

\score {
  <<
    % 乐器声部
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      g4^\markup { \italic "sing while playing ↓" } a b c |
      d e f g |
    }
    % 人声声部（演唱音高）
    \new Staff \relative c' {
      \clef treble
      \time 4/4
      c4^\markup { \italic "voice" } b a g |
      f e d c |
    }
  >>
  \layout { }
}
```

### 3.4 滑音 (Glissando)

#### 长号滑音

长号可通过拉管实现连续滑音，这是其独有的技法。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    % 长号滑音——连续的音高滑动
    g4\glissando c |
    e,\glissando b' |
    % 长音滑音
    c2\glissando g' |
    d,\glissando a' |
  }
  \layout { }
}
```

#### 按键铜管滑音

小号、圆号等按键铜管的滑音幅度有限（通常不超过半音），通过半按活塞实现。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 小号半音滑音——幅度有限
    g4\glissando^\markup { \italic "half-valve gliss." } aes |
    c4\glissando b |
    % 唇部滑音（lip bend）
    e4^\markup { \italic "lip bend ↓" } \glissando ees |
  }
  \layout { }
}
```

### 3.5 半按活塞 (Half-valve Effects)

部分按下活塞，产生模糊的、介于两个音之间的音高。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 半按活塞——音高模糊、不稳定
    g4^\markup { \italic "half-valve" }
    \override NoteHead.style = #'slash
    a b c |
    \revert NoteHead.style
    % 恢复正常
    d4^\markup { \italic "norm." } e f g |
  }
  \layout { }
}
```

### 3.6 铜管扩展技法综合示例

```lilypond
\version "2.24.0"

\header {
  title = "铜管扩展技法示例"
  composer = "教学示范"
}

\score {
  \new Staff \relative c' {
    \clef treble
    \time 4/4

    % 哈门弱音器
    g4\stopped^\markup { \italic "harmon" } a b c |

    % 咆哮
    \override NoteHead.style = #'cross
    d4^\markup { \bold "growl" } e f g |
    \revert NoteHead.style

    % 滑音
    a4\glissando d |

    % 半按活塞
    e4^\markup { \italic "half-valve" } f g a |

    % 开放音
    b4^\markup { \italic "open" } c d e |
  }
  \layout { }
}
```

---

## 4. 打击乐记谱 (Percussion Notation)

### 4.1 无音高打击乐 (Non-pitched Percussion)

LilyPond 提供 `DrumStaff` 和 `DrumVoice` 上下文，内置打击乐音名映射。

#### 基本鼓组记谱

```lilypond
\version "2.24.0"

\score {
  \new DrumStaff {
    \drummode {
      % 基本鼓组节奏
      % bd = bass drum, sn = snare, hh = hi-hat
      bd4 sn bd sn |
      bd8 hh sn hh bd hh sn hh |
      bd4 sn bd8 sn bd sn |
    }
  }
  \layout { }
}
```

#### LilyPond 打击乐音名速查

| 音名 | 乐器 | 符头位置 |
|------|------|---------|
| `bd` | 低音鼓 (Bass Drum) | 第一线下方 |
| `sn` | 小军鼓 (Snare Drum) | 第三线 |
| `tomh` `toml` `tomfh` | 高音/低音/落地通鼓 | 不同线位 |
| `hh` | 踩镲 (Hi-hat) | 第五线上方 |
| `cymc` `cymr` | 碎音镲/叮叮镲 | 第五线上方（× 符头） |
| `tamb` | 铃鼓 | 第四线 |
| `tri` | 三角铁 | 第四线（× 符头） |
| `wb` `wh` | 木鱼高/低 | 第二/三线 |

#### 自定义无音高打击乐

使用标准 `Staff` + `×` 符头，适合非标准打击乐器。

```lilypond
\version "2.24.0"

\score {
  \new Staff {
    \clef percussion
    \time 4/4
    \relative c' {
      % 自定义打击乐——× 符头在中间线
      \override NoteHead.style = #'cross
      c4 c c c |
      c8 c c c c c c c |
      c4. c8 c2 |
    }
  }
  \layout { }
}
```

### 4.2 有音高打击乐 (Pitched Percussion)

有音高打击乐使用标准记谱法，谱号根据乐器选择。

```lilypond
\version "2.24.0"

\score {
  <<
    % 马林巴——双谱表
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      c4 d e f |
      g a b c |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      c4 b a g |
      f e d c |
    }
  >>
  \layout { }
}
```

> 有音高打击乐音域 → 见 `shared-rules.md` 打击乐表

### 4.3 扩展打击乐技法

#### 弓奏镲片 (Bowed Cymbal)

用弓拉奏吊镲边缘，产生持续的金属共鸣声。

```lilypond
\version "2.24.0"

\score {
  \new Staff {
    \clef percussion
    \time 4/4
    \relative c' {
      % 弓奏镲片——× 符头 + 连线 + 文字标注
      \override NoteHead.style = #'cross
      c1(^\markup { \italic "bowed suspended cymbal" } |
      c1) |
      % 弓奏镲片渐强渐弱
      c2(\< c\> |
      c2 c\!) |
    }
  }
  \layout { }
}
```

#### 预置钢琴 (Prepared Piano)

在琴弦间插入各种物件（螺丝钉、橡皮、纸片等）改变音色。记法：标准音符 + 详细文字标注。

```lilypond
\version "2.24.0"

\header {
  title = "预置钢琴片段"
  composer = "教学示范"
}

\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % 预置钢琴右手——标注具体预置物
      c4^\markup { \fontsize #-3 "bolt on C5" }
      d^\markup { \fontsize #-3 "rubber on D5" }
      e f |
      <c e g>2^\markup { \fontsize #-3 "screw on G4" }
      <d f a>2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      % 预置钢琴左手
      c4^\markup { \fontsize #-3 "chain on C3" }
      d e f |
      c2 d2 |
    }
  >>
  \layout { }
}
```

**预置钢琴标注规范**（John Cage 传统）：
- 在每个受影响的音符上方标注预置物类型
- 在乐谱开头附"预置表"（preparation table），列出每个音的预置物、尺寸和放置位置
- 常用标注：`bolt`（螺栓）、`screw`（螺丝）、`rubber`（橡皮）、`weather stripping`（密封条）、`chain`（链条）

### 4.4 打击乐综合示例

```lilypond
\version "2.24.0"

\header {
  title = "打击乐综合示例"
  composer = "教学示范"
}

\score {
  <<
    % 定音鼓
    \new Staff \relative c {
      \clef bass
      \time 4/4
      d4^\markup { "Timp." } e f g |
      d2 g2 |
    }
    % 小军鼓
    \new DrumStaff {
      \drummode {
        sn4 sn sn sn |
        sn8 sn sn sn sn sn sn sn |
      }
    }
  >>
  \layout { }
}
```

---

## 5. 人声特殊技法 (Extended Vocal Techniques)

> 音域 → 见 `shared-rules.md` 人声表

### 5.1 说唱式歌唱 (Sprechstimme / Sprechgesang)

介于说话和歌唱之间的发声方式。Arnold Schoenberg 在《月迷彼埃罗》中首创此技法。

#### Schoenberg 记法

使用 `×` 符头表示近似音高的说唱，符干保留以指示节奏。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 3/4
    % Sprechstimme——× 符头 + 符干（表示节奏）
    \override NoteHead.style = #'cross
    g4^\markup { \bold "Sprechstimme" } a b |
    c4. b8 a4 |
    b4 c b |
    \revert NoteHead.style
    % 恢复正常歌唱
    a4^\markup { \italic "cantando" } b c |
  }
  \layout { }
}
```

#### 现代记法变体

| 记法 | 说明 | 使用场景 |
|------|------|---------|
| `×` 符头 | 近似音高，节奏精确 | Schoenberg 传统 |
| 无符头、仅符干 | 完全自由音高 | 更现代的写法 |
| 普通符头 + `Sprech.` | 精确音高，说唱音色 | 折中方案 |

### 5.2 耳语 (Whispering)

不出声的气声说话。记法：`×` 符头 + 去符干或 `p` 力度 + 文字标注。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 耳语——× 符头，极弱力度
    \override NoteHead.style = #'cross
    \override Stem.transparent = ##t
    g4\p^\markup { \italic "whisper" } a b c |
    g2\pp a2 |
    \revert NoteHead.style
    \revert Stem.transparent
    % 恢复正常
    g4^\markup { \italic "norm. voice" } a b c |
  }
  \layout { }
}
```

### 5.3 喊叫 (Shouting)

非歌唱性的大声喊叫。记法：文字标注 + 特殊符头。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 喊叫——× 符头 + 强力度 + 文字标注
    \override NoteHead.style = #'cross
    g4\ff^\markup { \bold "shout!" } a\ff b\ff c\ff |
    \revert NoteHead.style
    % 恢复正常
    g4\p^\markup { \italic "subito piano, cantando" } a b c |
  }
  \layout { }
}
```

### 5.4 气泡音 (Vocal Fry)

极低音区的声带松弛振动，产生颗粒感的"咔咔"声。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    % 气泡音——低音区 × 符头 + 震音
    \override NoteHead.style = #'cross
    e,4:32^\markup { \italic "vocal fry" }
    e,:32 e,:32 e,:32 |
    \revert NoteHead.style
    % 恢复正常
    e,4^\markup { \italic "norm." } f g a |
  }
  \layout { }
}
```

### 5.5 泛音歌唱 (Overtone Singing)

通过调整口腔共鸣同时发出基音和泛音（如蒙古喉音唱法）。记法：低音为基音，高音为泛音旋律。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c {
    \clef bass
    \time 4/4
    % 泛音歌唱——基音 + 泛音旋律
    % 基音持续（低音声部）
    <<
      { c1 c | c c | }
      \\
      {
        % 泛音旋律（用菱形符头表示泛音）
        \tweak NoteHead.style #'harmonic c''4^\markup { \italic "overtones" }
        \tweak NoteHead.style #'harmonic g''
        \tweak NoteHead.style #'harmonic c'''
        \tweak NoteHead.style #'harmonic e''' |
        \tweak NoteHead.style #'harmonic g''
        \tweak NoteHead.style #'harmonic c'''
        \tweak NoteHead.style #'harmonic e'''
        \tweak NoteHead.style #'harmonic g''' |
      }
    >>
  }
  \layout { }
}
```

### 5.6 弹舌音 / 嗒舌音 (Click Consonants)

南非科萨语等语言中的弹舌音，在现代声乐作品中偶有使用。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 弹舌音——用 × 符头 + 文字标注
    \override NoteHead.style = #'cross
    g4^\markup { \italic "click (dental)" }
    a^\markup { \italic "click (lateral)" }
    b^\markup { \italic "click (alveolar)" }
    c |
    \revert NoteHead.style
    % 恢复正常
    g4 a b c |
  }
  \layout { }
}
```

### 5.7 人声扩展技法综合示例

```lilypond
\version "2.24.0"

\header {
  title = "人声扩展技法示例"
  composer = "教学示范"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4

    % Sprechstimme
    \override NoteHead.style = #'cross
    g4^\markup { \bold "Sprechstimme" } a b c |
    \revert NoteHead.style

    % 耳语
    \override NoteHead.style = #'cross
    \override Stem.transparent = ##t
    g4\pp^\markup { \italic "whisper" } a b c |
    \revert NoteHead.style
    \revert Stem.transparent

    % 喊叫
    \override NoteHead.style = #'cross
    g4\ff^\markup { \bold "shout!" } a\ff b\ff c\ff |
    \revert NoteHead.style

    % 恢复正常歌唱
    g4\mp^\markup { \italic "cantando" } a b c |
  }
  \layout { }
}
```

---

## PART B: 图形记谱与偶然音乐

---

## 6. 图形记谱 (Graphic Notation)

### LilyPond 的局限性

LilyPond 是**排版引擎**，不是图形设计工具。它擅长精确的传统音乐记谱，但无法绘制自由图形。

| 能力 | LilyPond 可以做 | LilyPond 不能做 |
|------|----------------|----------------|
| ✅ | 无小节线、无拍号的自由节奏 | 自由手绘图形 |
| ✅ | 比例记谱（时间=空间） | 非五线谱的图形谱 |
| ✅ | 华彩乐段式自由节奏 | 颜色渐变表示力度 |
| ✅ | 文字标注替代传统符号 | 自定义图形符号库 |
| ✅ | 近似音高（× 符头、slash 符头） | 频谱图、声波形记谱 |

### 6.1 华彩乐段模式 (Cadenza Mode)

`\cadenzaOn` 取消小节线和拍号约束，允许自由节奏。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    % 进入华彩模式——无拍号、无小节线
    \cadenzaOn
    g4 a b c d e f g |
    a b c d e f g a |
    % 可以自由使用任意时值
    g2 a4. b8 c1 d4 e |
    f2. g4 a1 |
    % 退出华彩模式
    \cadenzaOff
    % 恢复标准记谱
    \time 4/4
    g4 a b c |
  }
  \layout { }
}
```

### 6.2 去除小节线和拍号

比 `\cadenzaOn` 更精细的控制：逐项移除排版元素。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    % 隐藏拍号
    \omit TimeSignature
    % 隐藏小节线
    \omit BarLine
    % 自由节奏——无视觉节拍约束
    g4 a b c d e f g |
    a2 b4 c d1 |
    e4 f g a b2 c |
    % 恢复小节线和拍号
    \undo \omit TimeSignature
    \undo \omit BarLine
    \time 4/4
    g4 a b c |
  }
  \layout { }
}
```

### 6.3 比例记谱 (Proportional Notation)

在比例记谱中，**水平空间与时间成正比**：长音符占更多空间，短音符占更少空间。这在现代音乐中常见（如 Feldman、Ligeti）。

```lilypond
\version "2.24.0"

\header {
  title = "比例记谱"
  subtitle = "空间 = 时间"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \cadenzaOn

    % 比例记谱——水平空间与时间成正比
    \override Score.SpacingSpanner.common-shortest-duration = #(ly:make-moment 1/8)
    \override Score.SpacingSpanner.base-shortest-duration = #(ly:make-moment 1/8)
    \override Score.SpacingSpanner.spacing-increment = 2.0

    % 长音占更多空间
    g1 a2 b4 c8 d16 e16 |
    f4 g2 a1 |
    b8 c4 d2 e1 |
  }
  \layout {
    % 去除节拍线等，增强图形感
    \context {
      \Staff
      \omit TimeSignature
      \omit BarLine
    }
  }
}
```

### 6.4 "图形谱"近似示例

用 LilyPond 的最大能力模拟图形谱效果：自由节奏 + 无小节线 + 比例间距 + 音高区域 + 文字描述。

```lilypond
\version "2.24.0"

\header {
  title = "近似图形谱"
  subtitle = "LilyPond 图形记谱极限"
}

\score {
  <<
    \new Staff \relative c''' {
      \clef treble
      \cadenzaOn
      \omit TimeSignature
      \omit BarLine
      % 高音区——稀疏的、飘浮的音
      g2^\markup { \italic "high, ethereal" }
      c1 d2 |
      b4 a1 g2 |
      % 渐密
      c8 d e f g4 a b |
    }
    \new Staff \relative c {
      \clef bass
      \cadenzaOn
      \omit TimeSignature
      \omit BarLine
      % 低音区——持续的低频嗡鸣
      c1^\markup { \italic "low drone" }
      c1 c1 |
      g2 c1 g2 |
    }
  >>
  \layout {
    \context {
      \Score
      \override SpacingSpanner.common-shortest-duration = #(ly:make-moment 1/4)
      \override SpacingSpanner.spacing-increment = 2.5
    }
  }
}
```

---

## 7. 偶然音乐记谱 (Aleatoric / Indeterminate Notation)

偶然音乐（Aleatoric music）赋予演奏者在一定范围内的选择自由。记谱方式因作曲家而异。

### 7.1 John Cage 方法：限定范围内的演奏者选择

John Cage 的偶然音乐通常使用**时间括号**（time brackets）：在指定时间窗口内，演奏者自由选择演奏内容。

```lilypond
\version "2.24.0"

\header {
  title = "时间括号记谱"
  subtitle = "受 John Cage Number Pieces 启发"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \cadenzaOn
    \omit TimeSignature
    \omit BarLine

    % 时间括号 1: 0'00" — 0'30"
    g4^\markup {
      \column {
        \bold "时间括号 I: 0'00\" — 0'30\""
        \italic "在此时间窗内自由演奏以下音"
      }
    }
    a2 b4 |

    % 时间括号 2: 0'20" — 1'00"（与 I 重叠）
    c1^\markup {
      \column {
        \bold "时间括号 II: 0'20\" — 1'00\""
        \italic "可选择一个音或多个音，自由时值"
      }
    }
    d2 |

    % 时间括号 3: 0'50" — 1'30"
    e1^\markup {
      \column {
        \bold "时间括号 III: 0'50\" — 1'30\""
      }
    }
    f2 g4 |

    % 时间括号 4: 1'20" — 2'00"
    a1^\markup {
      \column {
        \bold "时间括号 IV: 1'20\" — 2'00\""
        \italic "渐弱至无声"
      }
    } |
  }
  \layout {
    \context {
      \Score
      \override SpacingSpanner.common-shortest-duration = #(ly:make-moment 1/2)
      \override SpacingSpanner.spacing-increment = 3.0
    }
  }
}
```

### 7.2 移动曲式 (Mobile Form)

多个段落可以按任意顺序演奏（如 Stockhausen 的 *Klavierstück XI*、Boulez 的第三钢琴奏鸣曲）。

```lilypond
\version "2.24.0"

\header {
  title = "移动曲式"
  subtitle = "段落 A–D，演奏者可任意排列顺序"
}

% 段落 A
segmentA = \relative c'' {
  \time 3/4
  g4 a b | c2. |
}

% 段落 B
segmentB = \relative c'' {
  \time 4/4
  d4 e f g | a2 b2 |
}

% 段落 C
segmentC = \relative c' {
  \time 2/4
  e8 f g a | b4 c |
}

% 段落 D
segmentD = \relative c'' {
  \cadenzaOn
  g2 a4 b c1 |
}

\score {
  \new Staff {
    \clef treble
    % 段落 A
    \mark \markup { \bold "A" }
    \segmentA
    \bar "||"

    % 段落 B
    \mark \markup { \bold "B" }
    \segmentB
    \bar "||"

    % 段落 C
    \mark \markup { \bold "C" }
    \segmentC
    \bar "||"

    % 段落 D
    \mark \markup { \bold "D" }
    \segmentD
    \bar "|."
  }
  \layout { }
}

% 演奏指示：
% 演奏者可选择 A-B-C-D 的任意排列（如 B-D-A-C、C-A-D-B 等）
% 每次演奏的顺序不同，构成不同的"作品"
```

### 7.3 百分号反复 (Percent Repeat) 与演奏者变奏

`\repeat percent` 产生百分号反复符号（%），常用于 jazz 和偶然音乐中表示"以类似方式重复"。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 百分号反复——演奏者每次重复可做微小变化
    \repeat percent 4 {
      g4 a b c |
    }
    % 新的素材
    d2 e |
    % 再次百分号反复
    \repeat percent 3 {
      f4 g a b |
    }
  }
  \layout { }
}
```

**偶然音乐应用**：在百分号反复中，演奏者被鼓励在每次重复时做微小的即兴变化（如力度、articulation、微小的节奏偏移）。

### 7.4 偶然音乐综合示例：演奏者选择的段落

```lilypond
\version "2.24.0"

\header {
  title = "偶然音乐片段"
  subtitle = "演奏者选择"
}

\score {
  \new Staff \relative c'' {
    \clef treble

    % 引子——自由节奏
    \cadenzaOn
    \omit TimeSignature
    g2^\markup { \italic "自由节奏，任意时值" } a4 b c1 |
    d2 e f g |

    % 主体——可选段落
    \undo \omit TimeSignature
    \time 4/4
    \bar "||"

    % 选择 1 或 2（不可同时演奏）
    \mark \markup { \bold "选择 A 或 B" }
    <<
      % 选择 A
      \new Staff \relative c'' {
        \clef treble
        \time 4/4
        g4^\markup { \bold "选择 A:" } a b c |
        d e f g |
      }
    >>

    % 百分号反复——每次可变化
    \repeat percent 3 {
      g4 a b c |
    }
  }
  \layout { }
}
```

---

## 8. 现代装饰与符号 (Modern Symbols and Signs)

### 8.1 四分之一音变音记号 (Quarter-tone Accidentals)

四分之一音是半音的一半（50 音分），在 20 世纪音乐中广泛使用（如 Alois Hába、Iannis Xenakis、György Ligeti）。

#### LilyPond 四分之一音记法

| 音名 | 音高偏移 | 符号 |
|------|---------|------|
| `c` | 0 | 自然音 |
| `cih` | +50 音分 | 四分之一升 |
| `cis` | +100 音分 | 半升（标准升号） |
| `cisih` | +150 音分 | 四分之三升 |
| `ceh` | −50 音分 | 四分之一降 |
| `ces` | −100 音分 | 半降（标准降号） |
| `ceseh` | −150 音分 | 四分之三降 |

```lilypond
\version "2.24.0"

\header {
  title = "四分之一音音阶"
  composer = "教学示范"
}

\score {
  \new Staff \relative c' {
    \clef treble
    \time 4/4

    % 上行四分之一音音阶：C → C# 的四分之一音细分
    c4 cih cis cisih |
    d dih dis disih |
    e eih eis eisih |
    f fih fis fisih |

    % 下行四分之一音音阶
    g4 gih ges geh |
    f feh fes fehes |
    ees4 ees eh e |
  }
  \layout { }
}
```

#### 箭头变音记号 (Arrow Accidentals)

箭头变音记号用箭头方向表示音高偏移方向，常用于近似音高或微分音记谱。LilyPond 无原生箭头变音记号，需通过 markup 近似。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 箭头变音记号近似——用四分之一音记号 + 文字说明
    gih4^\markup { \fontsize #-2 "↑≈+50¢" }
    aeh^\markup { \fontsize #-2 "↓≈−50¢" }
    b c |
    % 近似音高区域
    dih2^\markup { \italic "approx. pitch" }
    eeh2 |
  }
  \layout { }
}
```

### 8.2 音簇记谱 (Cluster Notation)

音簇（tone cluster）是相邻半音或全音密集叠加的和声块。

#### 方法一：`\makeClusters`（LilyPond 原生）

`\makeClusters` 将连续的级进音符自动转化为音簇方块。

```lilypond
\version "2.24.0"

\header {
  title = "音簇记谱 — makeClusters"
}

\score {
  \new Staff \relative c' {
    \clef treble
    \time 4/4

    % 音簇——用 \makeClusters 将级进音自动转化为方块
    \makeClusters {
      g4 a b c |
      d e f g |
    }

    % 普通音符（对比）
    g4 a b c |
  }
  \layout { }
}
```

#### 方法二：密集和弦手动音簇

直接用密集和弦表示音簇，适合精确控制的场合。

```lilypond
\version "2.24.0"

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4
    % 半音音簇——所有相邻半音同时发声
    <c cis d dis e f>2^\markup { \italic "chromatic cluster" } |
    % 全音音簇
    <c d e fis gis>2^\markup { \italic "whole-tone cluster" } |
    % 白键音簇
    <c d e f g a b>1^\markup { \italic "white-key cluster" } |
  }
  \layout { }
}
```

### 8.3 变格定弦 (Scordatura)

改变弦乐器的标准定弦（如小提琴 G-D-A-E → 其他音高），以获得特殊音色或方便特定指法。

```lilypond
\version "2.24.0"

\header {
  title = "变格定弦 (Scordatura)"
  subtitle = "小提琴：G-D-A-E → G-D-A-D"
}

\score {
  \new Staff \relative c'' {
    \clef treble
    \time 4/4

    % 设定新的弦定弦
    \set Staff.stringTunings = \stringTuning <g d' a' d''>

    % 标注变格定弦
    g4^\markup {
      \column {
        \bold "Scordatura: G-D-A-D"
        \italic "(E 弦降低至 D)"
      }
    }
    d' a' d'' |
    g,2 d'2 |
    a'2 d''2 |
    g,1 |
  }
  \layout { }
}
```

**常见变格定弦**：

| 乐器 | 标准定弦 | 变格定弦示例 | 出处 |
|------|---------|-------------|------|
| 小提琴 | G-D-A-E | G-D-A-D | Biber, 巴洛克 |
| 大提琴 | C-G-D-A | Bb-F-D-A | Bach 第5号大提琴组曲 |
| 吉他 | E-A-D-G-B-E | D-A-D-G-B-E (Drop D) | 广泛使用 |
| 低音提琴 | E-A-D-G | C-E-A-D | 管弦乐扩展音域 |

### 8.4 预置乐器记谱 (Prepared Instrument Notation)

预置钢琴的记谱需要同时传达：(1) 演奏的音符，(2) 预置物的详细信息。

```lilypond
\version "2.24.0"

\header {
  title = "预置钢琴记谱"
  subtitle = "含预置表"
}

% 预置表以 markup 形式附在乐谱上方
\markup {
  \column {
    \bold "预置表 (Preparation Table):"
    \line { "C4: 1\" steel bolt, between strings 2-3, at 3\" from tuning pins" }
    \line { "D4: 1/2\" rubber eraser, woven between strings" }
    \line { "E4: 1\" wood screw, lightly touching middle string" }
    \line { "G3: bamboo strip, between all three strings" }
    \line { "A3: weather stripping, 2\" piece" }
  }
}

\score {
  \new PianoStaff <<
    \new Staff \relative c'' {
      \clef treble
      \time 4/4
      % 标注受预置影响的音
      c4^\markup { \fontsize #-4 "bolt" }
      d^\markup { \fontsize #-4 "rubber" }
      e^\markup { \fontsize #-4 "screw" }
      f |
      <c e>2^\markup { \fontsize #-4 "bolt + screw" }
      <d f>2 |
    }
    \new Staff \relative c {
      \clef bass
      \time 4/4
      g4^\markup { \fontsize #-4 "bamboo" }
      a^\markup { \fontsize #-4 "weather strip" }
      b c |
      g2 a2 |
    }
  >>
  \layout { }
}
```

### 8.5 现代符号综合示例

```lilypond
\version "2.24.0"

\header {
  title = "现代符号综合示例"
  composer = "教学示范"
}

\score {
  \new Staff \relative c' {
    \clef treble
    \time 4/4

    % 四分之一音
    cih4 dih eih fih |

    % 音簇
    <g gis a ais b>2^\markup { \italic "cluster" } |

    % 自由节奏段落
    \cadenzaOn
    \omit TimeSignature
    \omit BarLine
    c2^\markup { \italic "libero" } d4 e f1 g2 |
    \undo \omit TimeSignature
    \undo \omit BarLine

    % 恢复
    \time 3/4
    c4 d e |
  }
  \layout { }
}
```

---

## 综合示例：多技法组合

以下示例展示一首短曲中综合运用多种现代记谱技法。

```lilypond
\version "2.24.0"

\header {
  title = "现代室内乐片段"
  subtitle = "多技法综合示范"
  composer = "教学示范"
  instrument = "长笛 + 小提琴 + 大提琴 + 钢琴 + 打击乐"
}

% ========== 长笛 ==========
flutePart = \relative c'' {
  \clef treble
  \time 5/8

  % A 段：花舌 + 复音
  g8:32^\markup { \italic "flatterz." }
  a:32 b:32 c:32 d:32 |

  % B 段：键击声
  \override NoteHead.style = #'cross
  g8^\markup { \italic "key clicks" } a b c d |
  \revert NoteHead.style

  % C 段：气声
  \override NoteHead.style = #'slash
  \override Stem.transparent = ##t
  g4^\markup { \italic "air" } a8 b c |
  \revert NoteHead.style
  \revert Stem.transparent

  % D 段：自由华彩
  \cadenzaOn
  g4^\markup { \italic "cadenza ad lib." }
  a8 b c d e f g |
  a2 g1 |
  \cadenzaOff
}

% ========== 小提琴 ==========
violinPart = \relative c'' {
  \clef treble
  \time 5/8

  % A 段：Sul ponticello 泛音
  \tweak NoteHead.style #'harmonic g4^\markup { \italic "sul pont., harm." }
  \tweak NoteHead.style #'harmonic d'
  \tweak NoteHead.style #'harmonic g'8 |

  % B 段：Bartók pizzicato
  g,4\stopped^\markup { \bold "B.pizz." } a\stopped b\stopped c\stopped d\stopped |

  % C 段：Col legno battuto
  e4-.^\markup { \italic "col legno batt." } f-. g-. a-. b-. |

  % D 段：四分之一音滑奏
  \cadenzaOn
  cih4^\markup { \italic "¼-tone gliss." }
  \glissando c4
  dih4 \glissando d4 |
  \cadenzaOff
}

% ========== 大提琴 ==========
celloPart = \relative c {
  \clef bass
  \time 5/8

  % A 段：Sul tasto 长音
  c2(^\markup { \italic "sul tasto" } d4) e f |

  % B 段：音簇滑音
  <c cis d>4(^\markup { \italic "cluster gliss." }
  \glissando <e f fis>) <g gis a> |

  % C 段：弓奏琴马后方
  \override NoteHead.style = #'cross
  c4^\markup { \italic "behind bridge" } d e f g |
  \revert NoteHead.style

  % D 段：踏板音
  c,2(^\markup { \italic "pedal tone" } b,4) a, g, |
}

% ========== 钢琴 ==========
pianoRH = \relative c'' {
  \clef treble
  \time 5/8

  % A 段：预置钢琴
  c4^\markup { \fontsize #-4 "bolt" } d^\markup { \fontsize #-4 "rubber" }
  e f g |

  % B 段：音簇
  <c cis d dis e>4^\markup { \italic "fist cluster" } r r r r |

  % C 段：弦上拨奏
  \override NoteHead.style = #'cross
  g8^\markup { \italic "pluck strings" } a b c d e f g a b |
  \revert NoteHead.style

  % D 段：自由
  \cadenzaOn
  <c e g>1^\markup { \italic "let ring" } |
  \cadenzaOff
}

pianoLH = \relative c {
  \clef bass
  \time 5/8

  c4^\markup { \fontsize #-4 "chain" } g c g c |
  <c, d e f g>4^\markup { \italic "forearm cluster" } r r r r |
  c4 c c c c |
  \cadenzaOn
  c1 |
  \cadenzaOff
}

% ========== 打击乐 ==========
percPart = {
  \drummode {
    \time 5/8

    % A 段：颤音琴
    cymc8 cymc cymc cymc cymc |

    % B 段：弓奏镲片
    cymc1(^\markup { \italic "bowed cymbal" } |

    % C 段：回到碎音镲
    cymc8) cymc cymc cymc cymc |

    % D 段：自由
    cymc1 |
  }
}

% ========== 总谱 ==========
\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Fl."
      \flutePart
    }
    \new Staff {
      \set Staff.instrumentName = "Vln."
      \violinPart
    }
    \new Staff {
      \set Staff.instrumentName = "Vc."
      \celloPart
    }
    \new PianoStaff {
      \set PianoStaff.instrumentName = "Pf."
      <<
        \new Staff { \pianoRH }
        \new Staff { \pianoLH }
      >>
    }
    \new DrumStaff {
      \set Staff.instrumentName = "Perc."
      \percPart
    }
  >>
  \layout {
    indent = 20
    short-indent = 10
  }
}
```

---

## 参考与对照

| 主题 | 参考来源 |
|------|---------|
| 乐器音域 | → `classical/common/shared-rules.md` |
| LilyPond 基础语法 | → `syntax/lilypond-core-syntax.md` |
| 弦乐配器 | → `adler.md`（管弦乐法） |
| 十二音技法 | → `modernist/schoenberg.md` |
| 微分音体系 | → `modernist/microtonal.md`（如存在） |

### LilyPond 扩展技法命令速查

| 命令 | 效果 | 适用技法 |
|------|------|---------|
| `\flageolet` | 小圆圈符号 ○ | 泛音 |
| `\tweak NoteHead.style #'harmonic` | 菱形符头 | 泛音 |
| `\stopped` | 加号符号 + | 弱音器、Bartók pizz. |
| `\trill` | 颤音记号 | 花舌、bisbigliando |
| `\glissando` | 滑音线 | 滑音 |
| `\breathe` | 呼吸记号 | 循环呼吸 |
| `\cadenzaOn` / `\cadenzaOff` | 开关华彩模式 | 自由节奏 |
| `\omit TimeSignature` | 隐藏拍号 | 图形记谱 |
| `\omit BarLine` | 隐藏小节线 | 图形记谱 |
| `\makeClusters { ... }` | 音簇方块 | 音簇 |
| `\repeat percent N { ... }` | 百分号反复 | 偶然音乐 |
| `\override NoteHead.style = #'cross` | × 符头 | 无音高打击、气声、Sprechstimme |
| `\override NoteHead.style = #'slash` | 斜线符头 | 不确定音高 |
| `\stringTuning` | 变格定弦 | Scordatura |
| `:32` | 震音记号 | 花舌、快速震音 |
| `cih` / `ceh` 等 | 四分之一音 | 微分音 |
