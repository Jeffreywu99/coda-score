import { EditorView } from "@codemirror/view";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

/**
 * CodeMirror theme for LilyPond editor.
 * Colors are derived from the project's CSS custom properties
 * defined in index.css:
 *   --color-paper:     #fbf7f0  (background)
 *   --color-surface:   #fffcf7  (gutter / panels)
 *   --color-parchment: #f2ebe0  (active line)
 *   --color-accent:    #c0854a  (cursor, accent highlights)
 *   --color-ink:       #2c1810  (main text)
 *   --color-ink-muted: #8b6f5c  (comments, muted text)
 *   --color-divider:   #e5d5c0  (borders, line numbers)
 *   --color-danger:    #c46a5a  (errors)
 *   --color-success:   #7a8b5e  (strings / positive)
 */

const baseTheme = EditorView.theme(
  {
    // Editor container
    "&": {
      backgroundColor: "var(--color-paper)",
      color: "var(--color-ink)",
      fontSize: "14px",
      height: "100%",
    },

    // Content area
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Consolas', monospace",
      padding: "12px 0",
      caretColor: "var(--color-accent)",
    },

    // Cursor
    ".cm-cursor, .cm-dropCursor": {
      borderLeftColor: "var(--color-accent)",
      borderLeftWidth: "2px",
    },

    // Selection
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground": {
      backgroundColor: "rgba(192, 133, 74, 0.15) !important",
    },

    // Active line highlight
    ".cm-activeLine": {
      backgroundColor: "var(--color-parchment)",
    },

    // Gutter
    ".cm-gutters": {
      backgroundColor: "var(--color-surface)",
      color: "var(--color-ink-muted)",
      borderRight: "1px solid var(--color-divider)",
    },

    ".cm-activeLineGutter": {
      backgroundColor: "var(--color-parchment)",
      color: "var(--color-accent)",
    },

    // Line numbers
    ".cm-lineNumbers .cm-gutterElement": {
      padding: "0 12px 0 8px",
      minWidth: "40px",
    },

    // Fold gutter
    ".cm-foldGutter .cm-gutterElement": {
      color: "var(--color-ink-muted)",
    },

    // Matching brackets
    "&.cm-focused .cm-matchingBracket": {
      backgroundColor: "rgba(192, 133, 74, 0.2)",
      outline: "1px solid var(--color-accent)",
      borderRadius: "2px",
    },

    "&.cm-focused .cm-nonmatchingBracket": {
      backgroundColor: "rgba(196, 106, 90, 0.2)",
      outline: "1px solid var(--color-danger)",
      borderRadius: "2px",
    },

    // Search match
    ".cm-searchMatch": {
      backgroundColor: "rgba(192, 133, 74, 0.25)",
      borderRadius: "2px",
    },

    ".cm-searchMatch.cm-searchMatch-selected": {
      backgroundColor: "rgba(192, 133, 74, 0.4)",
    },

    // Panels (search bar, etc.)
    ".cm-panels": {
      backgroundColor: "var(--color-surface)",
      color: "var(--color-ink)",
      borderBottom: "1px solid var(--color-divider)",
    },

    ".cm-panels.cm-panels-top": {
      borderBottom: "1px solid var(--color-divider)",
    },

    ".cm-panels.cm-panels-bottom": {
      borderTop: "1px solid var(--color-divider)",
    },

    // Tooltip / autocomplete
    ".cm-tooltip": {
      backgroundColor: "var(--color-surface)",
      border: "1px solid var(--color-divider)",
      borderRadius: "6px",
      boxShadow: "0 4px 12px rgba(44, 24, 16, 0.1)",
    },

    ".cm-tooltip-autocomplete > ul > li[aria-selected]": {
      backgroundColor: "var(--color-parchment)",
      color: "var(--color-ink)",
    },

    // Scrollbar inside editor
    ".cm-scroller::-webkit-scrollbar": {
      width: "5px",
      height: "5px",
    },
    ".cm-scroller::-webkit-scrollbar-thumb": {
      background: "var(--color-divider)",
      borderRadius: "3px",
    },
    ".cm-scroller::-webkit-scrollbar-thumb:hover": {
      background: "var(--color-accent)",
    },
  },
  { dark: false }
);

/** Syntax highlighting using project-aligned colors */
const highlightStyle = HighlightStyle.define([
  // Keywords: \relative, \score, \header, etc. — warm accent
  { tag: t.keyword, color: "var(--color-accent)", fontWeight: "600" },

  // Note names and type names — deep ink
  { tag: t.typeName, color: "#3a5a2a" },

  // Functions (unknown backslash commands) — muted accent
  { tag: t.function(t.variableName), color: "#8a5a2a" },

  // Attribute names (dynamics, articulations) — deep accent
  { tag: t.attributeName, color: "var(--color-accent-deep)" },

  // Strings — success green
  { tag: t.string, color: "var(--color-success)" },

  // Numbers (durations) — teal-ish
  { tag: t.number, color: "#5a7a6a" },

  // Booleans (#t, #f) — teal
  { tag: t.bool, color: "#5a7a6a" },

  // Comments — muted ink
  { tag: t.comment, color: "var(--color-ink-muted)", fontStyle: "italic" },

  // Brackets, braces, punctuation — divider color
  { tag: t.bracket, color: "var(--color-ink-muted)" },
  { tag: t.brace, color: "var(--color-ink-muted)" },
  { tag: t.squareBracket, color: "var(--color-ink-muted)" },

  // Operators (ties, slurs)
  { tag: t.operator, color: "var(--color-accent)" },

  // Meta / scheme markers — muted
  { tag: t.meta, color: "#7a5a8a" },

  // Variables in scheme mode
  { tag: t.variableName, color: "var(--color-ink)" },

  // Separators (bar lines |)
  { tag: t.separator, color: "var(--color-divider)" },

  // Punctuation
  { tag: t.punctuation, color: "var(--color-ink-muted)" },
]);

export const lilypondTheme = [baseTheme, syntaxHighlighting(highlightStyle)];
