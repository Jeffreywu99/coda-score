\version "2.24.0"

% --- 核心配置：十二音无调性记谱法标准 ---
\layout {
  \context {
    \Staff
    % 强制每一个音符都显示升降号或还原号（现代音乐绝对标准）
    \accidentalStyle dodecaphonic
    % 隐藏默认的拍号，因为序列音乐常打破传统节拍感
    \omit TimeSignature
  }
}

% --- 原形音列 (Prime Row Example: Webern Op.27) ---
primeRow = \relative c'' {
  % 12个音不重复出现
  dis8 e cis c d f 
  fis g ais a gis b
}

% --- 倒影音列 (Inversion Example) ---
inversionRow = \relative c'' {
  % 音程反转
  dis8 d f fis e cis 
  c b gis a ais g
}

\score {
  \new Staff {
    \clef treble
    \tempo "Äußerst ruhig" 4 = 48
    
    % 使用标记说明这是什么序列
    \mark \markup { \box "P0" }
    \primeRow
    \bar "||"
    
    \mark \markup { \box "I0" }
    \inversionRow
    \bar "|."
  }
}
