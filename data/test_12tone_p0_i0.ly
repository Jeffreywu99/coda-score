\version "2.24.0"

\header {
  title = "Klavierstück III"
  subtitle = "P0 & I0 (Prime and Inversion) with Hyper-Complexity"
  composer = "AI System"
  tagline = ##f
}

\score {
  \new PianoStaff \with {
    instrumentName = "Piano"
    \override StaffGrouper.staff-staff-spacing.basic-distance = #15
  } <<
    \new Staff = "upper" {
      \clef treble
      \time 4/4
      \tempo "Lento, rigoroso" 4 = 40
      
      % Measure 1 (P0: 1-6)
      \tuplet 5/4 { r16 g''8.\sfz\> ~ g''16 \! } 
      \tuplet 3/2 { a'8\p\staccatissimo ( r4 } 
      r4 
      \tuplet 7/4 { r8 cis'''16\f\> c''16\pp r8. } |
      
      % Measure 2 (P0: 7-12)
      r4 
      \tuplet 6/4 { f''16\p bes'16\f\> e''16\pp r8. } 
      \tuplet 3/2 { fis''8\mp\espressivo r4 } 
      r4 |
      
      % Measure 3 (I0: 1-6)
      r4 
      \tuplet 7/4 { r8 b'16\mf a'16\p r8. } 
      r4 
      \tuplet 5/4 { f''16\sfz fis''16\ppp r8. } |
      
      % Measure 4 (I0: 7-12)
      \tuplet 6/4 { e''16\p cis'''16\f as''16\pp r8. } 
      r4 
      r8 \tuplet 3/2 { c''16\mp bes'8\ppp } 
      r4 \fermata |
      
      \bar "|."
    }
    
    \new Staff = "lower" {
      \clef bass
      \time 4/4
      
      % Measure 1 (P0: 1-6)
      \tuplet 3/2 { es8\mp ( r4 } 
      r4 
      \tuplet 5/4 { r16 b,8.\mf r16 } 
      r4 |
      
      % Measure 2 (P0: 7-12)
      \tuplet 7/4 { d'16\sfz^\markup { 
        \with-color #black 
        \path #0.15 #'((moveto 0 2) (curveto 2 5 -2 6 3 8)) 
      } r8. r8. } 
      r4 
      r4 
      \tuplet 5/4 { r8 as,16\ppp r8 } |
      
      % Measure 3 (I0: 1-6)
      \tuplet 3/2 { es8\f r4 } 
      r4 
      \tuplet 5/4 { g,16\mp r8. r16 } 
      r4 |
      
      % Measure 4 (I0: 7-12)
      r4 
      \tuplet 7/4 { d'16\mf r8. r8. } 
      r4 
      r4 \fermata |
    }
  >>
  \layout {
    \context {
      \Score
      proportionalNotationDuration = #1/32
      \override SpacingSpanner.strict-note-spacing = ##t
      \override TupletBracket.bracket-visibility = ##t
      % 让连音符号显示为分数形式，比如 5:4，增加谱面的复杂度
      \override TupletNumber.text = #tuplet-number::calc-fraction-text
    }
  }
}

\markup { 
  \vspace #2
  \fill-line {
    \center-column {
      \italic "Prime (P0): Eb - G - A - B - C# - C - D - F - Bb - E - F# - Ab"
      \vspace #0.5
      \italic "Inversion (I0): Eb - B - A - G - F - F# - E - C# - Ab - D - C - Bb"
    }
  }
}
