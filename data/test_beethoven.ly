\version "2.24.0"

\header {
  title = "Sonata in C Minor"
  subtitle = "Style: L. van Beethoven"
  composer = "AI System"
  tagline = ##f
}

\score {
  \new PianoStaff \with {
    instrumentName = "Piano"
  } <<
    \new Staff = "upper" {
      \clef treble
      \key c \minor
      \time 4/4
      \tempo "Allegro con brio" 4 = 144
      
      % Measure 1: 贝多芬标志性的突强 (sfz) 开头与动机
      <c' es' g' c''>2\f\sfz ~ <c' es' g' c''>8 r8 g'8.\p ( c''16 ) |
      
      % Measure 2: 旋律向上的紧张感
      es''4. ( c''8 ) g''4. ( es''8 ) |
      
      % Measure 3: 属和弦的重复动机
      <d'' f'' b''>2\sfz ~ <d'' f'' b''>8 r8 g'8.\p ( d''16 ) |
      
      % Measure 4
      f''4. ( d''8 ) b''4. ( g''8 ) |
      
      % Measure 5: 力量积聚的连续八分音符敲击
      <es'' g'' c'''>4\ff <es'' g'' c'''>8 <es'' g'' c'''> <e'' g'' c'''>4 <e'' g'' c'''>8 <e'' g'' c'''> |
      
      % Measure 6: 三连音的爆发下行
      <f'' aes'' c'''>2 \tuplet 3/2 { <f'' aes''>8\sfz ( <g'' bes''> <aes'' c'''> ) } \tuplet 3/2 { <d'' f''>8\sfz ( <es'' g''> <f'' aes''> ) } |
      
      % Measure 7: 经典的古典终止式颤音
      <es'' g''>4 <c'' es''> <b' d''>2\trill |
      
      % Measure 8: 强有力的结束双和弦
      <c'' es''>4 r4 <c' es' g' c''>4\ff r4 |
      \bar "|."
    }
    
    \new Staff = "lower" {
      \clef bass
      \key c \minor
      \time 4/4
      
      % Measure 1: 强有力的底鼓式低音
      <c, c>4\f\sfz <g c'>8-. <g c'>-. <g c'>4 r4 |
      
      % Measure 2: 典型的阿尔贝蒂式低音变体
      <c, c>4 <g c'> <c, c> <g c'> |
      
      % Measure 3
      <g,, g,>4\sfz <g b>8-. <g b>-. <g b>4 r4 |
      
      % Measure 4
      <g,, g,>4 <g b> <g,, g,> <g b> |
      
      % Measure 5
      <c, c>4\ff <g c'> <c, c> <bes c'> |
      
      % Measure 6
      <f, c>4 <aes c'> r2 |
      
      % Measure 7
      <g,, g,>4 <c, c> <g,, g,>2 |
      
      % Measure 8
      <c, c>4 r4 <c,, c,>4\ff r4 |
    }
  >>
  \layout {
    % 在这里我们完全不使用现代比例排版
    % 而是让 LilyPond 采用其默认的经典美学算法，自动松紧音符间距
  }
}
