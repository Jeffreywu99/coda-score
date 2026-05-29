import { EditorView, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter, drawSelection, dropCursor } from "@codemirror/view";
import { EditorState, type Extension } from "@codemirror/state";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { bracketMatching, foldGutter, foldKeymap, indentOnInput } from "@codemirror/language";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { closeBrackets, closeBracketsKeymap, autocompletion, completionKeymap } from "@codemirror/autocomplete";

import lilypondLanguage from "./lilypond-lang";
import { lilypondTheme } from "./lilypond-theme";

export interface EditorCallbacks {
  /** Called when editor content changes */
  onChange?: (code: string) => void;
  /** Called on Ctrl+S / Cmd+S save */
  onSave?: () => void;
}

/**
 * Create the full set of CodeMirror extensions for the LilyPond editor.
 * Pass optional callbacks for change/save events.
 */
export function createEditorExtensions(
  callbacks: EditorCallbacks = {}
): Extension[] {
  const { onChange, onSave } = callbacks;

  // Change listener extension
  const changeListener = onChange
    ? EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          onChange(update.state.doc.toString());
        }
      })
    : [];

  // Save keymap (Ctrl+S / Cmd+S)
  const saveKeymap = onSave
    ? keymap.of([
        {
          key: "Mod-s",
          run: () => {
            onSave();
            return true;
          },
        },
      ])
    : [];

  return [
    // Language
    lilypondLanguage.extension,

    // Theme
    ...lilypondTheme,

    // Editor state config
    EditorState.tabSize.of(2),
    EditorView.lineWrapping,

    // Line numbers + gutter
    lineNumbers(),
    highlightActiveLineGutter(),
    foldGutter(),

    // Active line highlight
    highlightActiveLine(),

    // History (undo/redo)
    history(),

    // Bracket matching & auto-close
    bracketMatching(),
    closeBrackets(),

    // Indentation
    indentOnInput(),

    // Selection
    drawSelection(),
    dropCursor(),
    highlightSelectionMatches(),

    // Autocompletion
    autocompletion(),

    // Keymaps
    keymap.of([
      ...closeBracketsKeymap,
      ...defaultKeymap,
      ...searchKeymap,
      ...historyKeymap,
      ...foldKeymap,
      ...completionKeymap,
      indentWithTab,
    ]),

    // Save shortcut
    saveKeymap,

    // Change listener
    changeListener,
  ];
}

/**
 * Create a CodeMirror EditorView attached to a DOM element.
 */
export function createEditor(
  parent: HTMLElement,
  initialCode: string = "",
  callbacks: EditorCallbacks = {}
): EditorView {
  return new EditorView({
    state: EditorState.create({
      doc: initialCode,
      extensions: createEditorExtensions(callbacks),
    }),
    parent,
  });
}

export { lilypondLanguage, lilypondTheme };
