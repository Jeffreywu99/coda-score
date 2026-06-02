# 图形记谱：线条延续 (DurationLine)

## 1. 概念与适用场景
在现代音乐（如电声音乐记谱、前卫合唱、极简主义图形谱）中，经常需要放弃传统的符头、符干和符梁，转而用**一根长线条**来表示声音的延续状态。
LilyPond 从 v2.23+ 开始原生支持 `DurationLine`，这是实现现代图形记谱的核心。

## 2. 启用 DurationLine
必须在 `\layout` 中为 `\Voice` 上下文启用 `Duration_line_engraver`，并同时隐藏传统的符头属性。

**基础启用模板：**
```lilypond
\layout {
  \context {
    \Voice
    \consists "Duration_line_engraver"
    \omit Stem
    \omit Flag
    \omit Beam
    % 将符头转换为简单的黑点，或如果需要完全隐藏可以使用 \omit NoteHead
    \override NoteHead.duration-log = 2
  }
}
```

## 3. 语法与线条样式
使用反斜杠加减号 `\-` 触发 DurationLine（注意它必须写在音符后面）。
例如：`c1\-` 表示从这个音符开始画一条线条，直到下一个音符。

可以覆盖 `DurationLine.style` 属性来改变线条类型。

### 支持的样式列表：
1. **实线** (默认)：`\override DurationLine.style = #'line`
2. **虚线**：`\override DurationLine.style = #'dashed-line` (配合 `\override DurationLine.dash-period = 2` 控制虚线密度)
3. **点线**：`\override DurationLine.style = #'dotted-line`
4. **锯齿线 (常用于颤音/震音表达)**：`\override DurationLine.style = #'zigzag`

## 4. 线条终止符 (End Styles)
线条结尾可以加上特定的几何图形以表达音符结束的方式。
- **带箭头 (Arrow)**：`\override DurationLine.bound-details.right.end-style = #'arrow`
- **带竖钩 (Hook)**：`\override DurationLine.bound-details.right.end-style = #'hook`
  *可以控制钩子的朝向：`\override DurationLine.details.hook-direction = #DOWN`*

## 5. 示例 Snippet 代码
以下代码展示了如何生成一行具有现代感的虚线与锯齿线记谱：
```lilypond
\version "2.24.0"

\score {
  \new Staff {
    \clef treble
    % 开启实线
    \once \override DurationLine.style = #'line
    c''1\- s2 r2
    
    % 开启锯齿线，并在线条末尾加箭头
    \once \override DurationLine.style = #'zigzag
    \once \override DurationLine.bound-details.right.end-style = #'arrow
    g''1\- s2 r2
  }
  
  \layout {
    \context {
      \Voice
      \consists "Duration_line_engraver"
      \omit Stem
      \omit Beam
    }
  }
}
```
