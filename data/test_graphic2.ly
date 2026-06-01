\version "2.24.0"

\header { tagline = ##f }

\score {
  \new Staff \with {
    \remove "Time_signature_engraver"
    \remove "Bar_engraver"
  } {
    \clef treble
    \relative c'' {
      % 1. 扇形连音符 (Feathered Beams) 代表渐快和渐慢
      \override Beam.grow-direction = #RIGHT
      \featherDurations 2/1 { c16[ d e f] }
      \override Beam.grow-direction = #LEFT
      \featherDurations 1/2 { g16[ f e d] }
      \revert Beam.grow-direction
      
      % 2. 自定义特殊符头
      \override NoteHead.style = #'cross
      c4^\markup { "noise" }
      \override NoteHead.style = #'harmonic-mixed
      d4^\markup { "harm." }
      \revert NoteHead.style

      % 3. 悬空自由滑音 (隐藏首尾音符实体，只留线条)
      \once \override NoteHead.transparent = ##t
      \once \override Stem.transparent = ##t
      e4\glissando
      \once \override NoteHead.transparent = ##t
      \once \override Stem.transparent = ##t
      b'4

      % 4. 贝塞尔曲线 (平滑的不规则图形，比如随机控制参数)
      g4^\markup {
        \with-color #black
        \path #0.25 #'((moveto 0 0) 
                       (curveto 1 5 4 5 5 0) 
                       (curveto 6 -5 9 -5 10 0))
      }
      
      % 5. 盒装或带圈的随机演奏指令，常用于偶然音乐
      s4^\markup { \box \pad-markup #0.5 { \circle "Free Improvisation 5''" } }
    }
  }
  \layout {
    \context {
      \Score
      proportionalNotationDuration = #1/16
      \override SpacingSpanner.strict-note-spacing = ##t
    }
  }
}
