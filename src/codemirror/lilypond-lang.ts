import { StreamLanguage } from "@codemirror/language";
import type { StringStream } from "@codemirror/language";

/** LilyPond music command keywords */
const MUSIC_COMMANDS = new Set([
  "relative", "absolute", "fixed", "score", "header", "layout", "midi",
  "paper", "book", "bookpart", "markup", "markuplist", "transpose",
  "clef", "key", "time", "tempo", "set", "unset", "override", "revert",
  "once", "repeat", "alternative", "tuplet", "times", "scaleDurations",
  "grace", "acciaccatura", "appoggiatura", "slashedGrace",
  "stemUp", "stemDown", "stemNeutral",
  "voiceOne", "voiceTwo", "voiceThree", "voiceFour",
  "oneVoice",
  "new", "context", "with",
  "include", "language", "version",
  "bar", "break", "noBreak", "pageBreak", "noPageBreak",
  "tag", "removeWithTag",
  "displayMusic", "displayScheme",
  "autochange",
  "partcombine", "partcombineUp", "partcombineDown",
  "lyricsto", "addlyrics", "lyricmode",
  "chordmode", "notemode", "drummode",
  "figuremode",
  "hide", "omit",
  "shape",
  "partial", "incomplete",
]);

/** Dynamic marks */
const DYNAMICS = new Set([
  "f", "ff", "fff", "ffff", "fp", "fz",
  "mf", "mp",
  "p", "pp", "ppp", "pppp",
  "rfz", "sf", "sff", "sfp", "sfz", "sp", "spp",
  "cr", "decr", "cresc", "decresc", "dim",
]);

/** Articulation marks */
const ARTICULATIONS = new Set([
  "accent", "espressivo", "fermata", "longfermata", "shortfermata",
  "verylongfermata", "flageolet", "lheel", "ltoe", "marcato",
  "mordent", "portato", "prall", "prallmordent", "prallprall",
  "pralldown", "prallup", "upbow", "downbow",
  "shortfermata", "signumcongruentiae", "snappizzicato",
  "sostenuto", "staccatissimo", "staccato", "stopped",
  "tenuto", "thumb", "trill", "turn", "varcoda",
]);

/** Scheme keywords in LilyPond */
const SCHEME_KEYWORDS = new Set([
  "define", "let", "if", "cond", "begin", "lambda",
  "define-music-function", "define-scheme-function",
]);

/** Note name base letters */
const NOTE_LETTERS = /^[a-g]/;

interface LilyPondState {
  inBlockComment: boolean;
  inString: boolean;
  inScheme: number; // nesting depth of parens in scheme mode
}

const lilypondLanguage = StreamLanguage.define<LilyPondState>({
  name: "lilypond",

  startState(): LilyPondState {
    return {
      inBlockComment: false,
      inString: false,
      inScheme: 0,
    };
  },

  token(stream: StringStream, state: LilyPondState): string | null {
    // Block comment continuation
    if (state.inBlockComment) {
      while (!stream.eol()) {
        if (stream.match("%}")) {
          state.inBlockComment = false;
          return "comment";
        }
        stream.next();
      }
      return "comment";
    }

    // String continuation
    if (state.inString) {
      while (!stream.eol()) {
        const ch = stream.next();
        if (ch === "\\") {
          stream.next(); // skip escaped char
        } else if (ch === '"') {
          state.inString = false;
          return "string";
        }
      }
      return "string";
    }

    // Scheme mode (inside parentheses after #)
    if (state.inScheme > 0) {
      if (stream.match("(")) {
        state.inScheme++;
        return "punctuation";
      }
      if (stream.match(")")) {
        state.inScheme--;
        return "punctuation";
      }
      if (stream.match(";")) {
        stream.skipToEnd();
        return "comment";
      }
      if (stream.match('"')) {
        state.inString = true;
        return "string";
      }
      if (stream.match("'")) {
        return "operator";
      }
      // Scheme keyword/symbol
      if (stream.match(/[a-zA-Z][\w-]*/)) {
        const word = stream.current();
        if (SCHEME_KEYWORDS.has(word)) return "keyword";
        return "variableName";
      }
      if (stream.match(/-?\d+(\.\d+)?/)) {
        return "number";
      }
      stream.next();
      return null;
    }

    // Skip whitespace
    if (stream.eatSpace()) return null;

    // Line comment
    if (stream.match("%{")) {
      state.inBlockComment = true;
      while (!stream.eol()) {
        if (stream.match("%}")) {
          state.inBlockComment = false;
          return "comment";
        }
        stream.next();
      }
      return "comment";
    }

    if (stream.match("%")) {
      stream.skipToEnd();
      return "comment";
    }

    // String literal
    if (stream.match('"')) {
      state.inString = true;
      while (!stream.eol()) {
        const ch = stream.next();
        if (ch === "\\") {
          stream.next();
        } else if (ch === '"') {
          state.inString = false;
          return "string";
        }
      }
      return "string";
    }

    // Scheme expression: #( or #'
    if (stream.match("#(")) {
      state.inScheme = 1;
      return "meta";
    }
    if (stream.match("#'")) {
      // Quoted scheme expression — read the next word/s-expr
      if (stream.match(/[a-zA-Z][\w-]*/)) {
        return "meta";
      }
      return "meta";
    }
    if (stream.match("#")) {
      // Inline scheme value
      if (stream.match(/-?\d+(\.\d+)?/)) {
        return "number";
      }
      if (stream.match(/[tf]/)) {
        return "bool";
      }
      return "meta";
    }

    // Backslash commands: \keyword
    if (stream.match("\\")) {
      // Dynamic marks like \f, \p, \mf, etc.
      if (stream.match(/[a-zA-Z]+/)) {
        const cmd = stream.current().slice(1); // remove backslash
        if (DYNAMICS.has(cmd)) return "attributeName";
        if (ARTICULATIONS.has(cmd)) return "attributeName";
        if (MUSIC_COMMANDS.has(cmd)) return "keyword";
        return "function";
      }
      return "keyword";
    }

    // Angle brackets for chords <c e g>
    if (stream.match("<") || stream.match(">")) {
      return "bracket";
    }

    // Curly braces for music expressions
    if (stream.match("{") || stream.match("}")) {
      return "brace";
    }

    // Square brackets for beams and slurs
    if (stream.match("[") || stream.match("]")) {
      return "squareBracket";
    }

    // Bar lines and pipe
    if (stream.match("|")) {
      return "separator";
    }

    // Note names: c d e f g a b followed by optional accidentals + octave
    if (stream.match(NOTE_LETTERS)) {
      // Consume accidentals: is, es, isis, eses
      stream.match(/(isis|eses|is|es)?/);
      // Consume octave markers: ' and ,
      stream.match(/['’,]+/);
      // Consume duration number
      stream.match(/\d*/);
      // Consume dots (dotted notes)
      stream.match(/\.*/);
      return "typeName";
    }

    // Duration numbers (standalone, e.g. after a rest "r4")
    if (stream.match(/\d+/)) {
      return "number";
    }

    // Rest marks: r, R, s
    if (stream.match(/[rRs]/)) {
      // Consume duration
      stream.match(/\d*/);
      stream.match(/\.*/);
      return "typeName";
    }

    // Ties and slurs
    if (stream.match("~") || stream.match("(") || stream.match(")")) {
      return "operator";
    }

    // Dynamics hairpin (when not in scheme)
    if (stream.match("\\<") || stream.match("\\>")) {
      return "attributeName";
    }

    // Punctuation
    stream.next();
    return null;
  },

  languageData: {
    commentTokens: { line: "%" },
    closeBrackets: { brackets: ["(", "[", "{", '"', "<"] },
  },
});

export default lilypondLanguage;
