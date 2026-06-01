\version "2.24.0"

\header {
  title = "Klavierstück I"
  subtitle = "Based on a 12-Tone Row (Pointillism)"
  composer = "AI System"
  tagline = ##f
}

\score {
  \new PianoStaff \with {
    instrumentName = "Piano"
  } <<
    \new Staff = "upper" {
      \clef treble
      \time 4/4
      \tempo "Äußerst langsam (极慢)" 4 = 40
      
      % Measure 1: G, A, C#, C
      r4 g''8\p\< ( a'8-. ) \! r4 cis'''8\> ( c''8-. ) \! |
      
      % Measure 2: F, E, F#
      r4 f''8\pp r8 e''4\sfz fis''4\ppp \fermata |
      
      \bar "|."
    }
    
    \new Staff = "lower" {
      \clef bass
      \time 4/4
      
      % Measure 1: Eb, B
      es4\mf\> r4 \! b,4\p r4 |
      
      % Measure 2: D, Bb, Ab
      d'4\mp r8 bes,8\mf r4 as,4\ppp \fermata |
    }
  >>
  \layout {
    \context {
      \Score
      % 开启比例排版，让空间更加冷峻、理性，符合序列主义的审美
      proportionalNotationDuration = #1/16
      \override SpacingSpanner.strict-note-spacing = ##t
    }
  }
}

\markup { 
  \vspace #2
  \fill-line {
    \center-column {
      \italic "Prime Row (P0): Eb - G - A - B - C# - C - D - F - Bb - E - F# - Ab"
    }
  }
}
