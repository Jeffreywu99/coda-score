\version "2.24.0"

\header {
  title = "Klavierstück II"
  subtitle = "New Complexity Rhythms"
  composer = "AI System"
  tagline = ##f
}

\score {
  \new PianoStaff \with {
    instrumentName = "Piano"
  } <<
    \new Staff = "upper" {
      \clef treble
      \time 3/4
      \tempo "Fluido, quasi rubato" 4 = 52
      
      % Beat 1: 5/4 连音内嵌 3/2 连音
      \tuplet 5/4 { r16 g''8\p\< ~ g''16 \tuplet 3/2 { a'32 ( cis'''32 c''32-. ) \! } } 
      % Beat 2: 7/4 连音
      \tuplet 7/4 { r8 f''16\mf\> ( e''16 ) r8. \! } 
      % Beat 3:
      r4 |
      
      % Measure 2
      \tuplet 3/2 { r8 fis''4\sfz \fermata } r4 r4 |
      \bar "|."
    }
    
    \new Staff = "lower" {
      \clef bass
      \time 3/4
      
      % Beat 1: 3/2 连音
      \tuplet 3/2 { es8\mf ( r4 } 
      % Beat 2: 5/4 连音
      \tuplet 5/4 { b,16\p d'16\mp r8. } ) 
      % Beat 3:
      r4 |
      
      % Measure 2
      r4 \tuplet 7/4 { r16 bes,8\> ( as,4 ) \ppp } r4 \fermata |
    }
  >>
  \layout {
    \context {
      \Score
      % 更精细的比例空间，凸显复杂节奏的视觉错位感
      proportionalNotationDuration = #1/32
      \override SpacingSpanner.strict-note-spacing = ##t
      % 强制显示所有的连音中括号，增加谱面的复杂度
      \override TupletBracket.bracket-visibility = ##t
    }
  }
}
