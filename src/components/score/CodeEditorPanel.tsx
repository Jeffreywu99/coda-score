import { useRef, useEffect, useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { useScoreStore } from "../../stores/scoreStore";
import { useLilyPondCompile } from "../../hooks/useLilyPondCompile";
import { Code2, Trash2, Copy, Check } from "lucide-react";

// Lazy type — only loaded when component mounts
type EditorView = import("@codemirror/view").EditorView;

export function CodeEditorPanel() {
  const { t } = useTranslation();
  const editorRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  const lilypondCode = useScoreStore((s) => s.lilypondCode);
  const { compile } = useLilyPondCompile();
  const [copied, setCopied] = useState(false);

  // Initialize CodeMirror editor via dynamic import (code-splitting)
  useEffect(() => {
    let destroyed = false;

    async function initEditor() {
      if (!editorRef.current || viewRef.current) return;

      const { createEditor } = await import("../../codemirror");

      if (destroyed || !editorRef.current) return;

      viewRef.current = createEditor(editorRef.current, lilypondCode, {
        onChange: (code: string) => {
          useScoreStore.getState().setLilypondCode(code);
        },
        onSave: () => {
          compile();
        },
      });
    }

    initEditor();

    return () => {
      destroyed = true;
      viewRef.current?.destroy();
      viewRef.current = null;
    };
    // Only run once on mount
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Sync external code changes (AI generation, cross-tool import) into CodeMirror
  useEffect(() => {
    const view = viewRef.current;
    if (!view) return;
    const currentCode = view.state.doc.toString();
    if (lilypondCode && lilypondCode !== currentCode) {
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: lilypondCode,
        },
      });
    }
  }, [lilypondCode]);

  const handleClear = useCallback(() => {
    if (viewRef.current) {
      viewRef.current.dispatch({
        changes: {
          from: 0,
          to: viewRef.current.state.doc.length,
          insert: "",
        },
      });
    }
    useScoreStore.getState().setLilypondCode("");
  }, []);

  const handleCopy = useCallback(async () => {
    const code = useScoreStore.getState().lilypondCode;
    if (!code.trim()) return;
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      // clipboard API not available
    }
  }, []);

  return (
    <div style={{ display: "flex", flexDirection: "column", flex: 1, minHeight: 0 }}>
      {/* Section header */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginBottom: 8,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 6,
            fontSize: 12,
            fontWeight: 600,
            letterSpacing: "0.05em",
            textTransform: "uppercase",
            color: "var(--color-ink-muted)",
          }}
        >
          <Code2 size={13} style={{ color: "var(--color-accent)" }} />
          {t("lilypond.editor.title")}
        </div>

        <div style={{ display: "flex", gap: 4 }}>
          <IconButton
            onClick={handleCopy}
            title={t("lilypond.editor.copy")}
            disabled={!lilypondCode.trim()}
          >
            {copied ? <Check size={13} /> : <Copy size={13} />}
          </IconButton>
          <IconButton
            onClick={handleClear}
            title={t("lilypond.editor.clear")}
            disabled={!lilypondCode.trim()}
          >
            <Trash2 size={13} />
          </IconButton>
        </div>
      </div>

      {/* CodeMirror mount point */}
      <div
        ref={editorRef}
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: 8,
          border: "1px solid var(--color-divider)",
          overflow: "hidden",
          fontSize: 13,
        }}
      />
    </div>
  );
}

function IconButton({
  children,
  onClick,
  title,
  disabled,
}: {
  children: React.ReactNode;
  onClick: () => void;
  title: string;
  disabled?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      title={title}
      disabled={disabled}
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: 28,
        height: 28,
        padding: 0,
        border: "1px solid var(--color-divider)",
        borderRadius: 6,
        backgroundColor: "var(--color-surface)",
        color: disabled ? "var(--color-ink-muted)" : "var(--color-ink)",
        cursor: disabled ? "default" : "pointer",
        opacity: disabled ? 0.4 : 1,
        transition: "all 0.15s ease",
      }}
    >
      {children}
    </button>
  );
}
