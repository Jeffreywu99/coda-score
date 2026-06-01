\version "2.24.0"

\header {
  title = "AI Graphic Score Experiment"
  subtitle = "Coda-Score Test #1"
  tagline = ##f
}

\score {
  \new Staff \with {
    % 隐藏传统排版元素
    \remove "Time_signature_engraver"
    \remove "Bar_engraver"
  } {
    \clef treble
    \relative c'' {
      % 开始的一段自由音符，附加文字
      c4^\markup { \italic "senza misura, freely" } 
      
      % 注入一个代表特殊音高起伏或噪音的折线
      d4^\markup {
        \with-color #black
        \path #0.3 #'((moveto 0 0)
                      (lineto 2 4)
                      (lineto 3 -2)
                      (lineto 5 3)
                      (lineto 7 0))
      }
      
      e8 f16 g r4
      
      % 极端的音簇标记，表示钢琴上用手掌压下
      \makeClusters { 
        <c e>4 <g' b> <d f> <c a'> 
      }
      
      % 结尾的长音，上面带有一个表示特殊敲击位置的多边形
      c1^\markup {
        \with-color #black
        \polygon #'((0 . 0) (2 . 3) (4 . 0) (2 . -3))
      }
    }
  }
  \layout {
    \context {
      \Score
      % 开启严格比例记谱：每个十六分音符分配固定的物理空间
      proportionalNotationDuration = #1/16
      \override SpacingSpanner.strict-note-spacing = ##t
      \override SpacingSpanner.strict-grace-spacing = ##t
    }
  }
}
