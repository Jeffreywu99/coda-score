import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useScoreStore } from "../../stores/scoreStore";
import { useAIGenerate } from "../../hooks/useAIGenerate";
import { TemplateMenu } from "./TemplateMenu";
import { loadTemplates, findTemplate, buildTemplateContext } from "../../lib/lilypond-template";
import { searchKnowledge } from "../../lib/lilypond";
import type { Template, KnowledgeSection } from "../../types/lilypond";
import { Sparkles, Send, AlertCircle, BookOpen, ChevronDown } from "lucide-react";

interface AIInputPanelProps {
  onTemplateSelect: (template: Template) => void;
}

export function AIInputPanel({ onTemplateSelect }: AIInputPanelProps) {
  const { t } = useTranslation();
  const aiInput = useScoreStore((s) => s.aiInput);
  const setAiInput = useScoreStore((s) => s.setAiInput);
  const isOnline = useScoreStore((s) => s.isOnline);
  const selectedTemplate = useScoreStore((s) => s.selectedTemplate);
  const triggerAutoCompile = useScoreStore((s) => s.triggerAutoCompile);
  const { isAIGenerating, generate } = useAIGenerate();

  const [knowledgeSections, setKnowledgeSections] = useState<KnowledgeSection[]>([]);
  const [knowledgeExpanded, setKnowledgeExpanded] = useState(false);
  const [contextError, setContextError] = useState(false);

  async function handleGenerate() {
    if (!aiInput.trim() || isAIGenerating) return;

    let context: string | undefined;
    if (selectedTemplate) {
      try {
        const templates = await loadTemplates();
        const tpl = findTemplate(templates, selectedTemplate);
        if (tpl) {
          context = buildTemplateContext(tpl);
        }
      } catch (e) {
        console.warn("Failed to load templates:", e);
        setContextError(true);
      }
    }

    // v0.5.0: Only search knowledge when user has selected a template (on-demand injection)
    let knowledgeCtx: string | undefined;
    if (selectedTemplate) {
      try {
        const ks = await searchKnowledge(aiInput.trim(), 5, selectedTemplate);
        if (ks.length > 0) {
          setKnowledgeSections(ks);
          setKnowledgeExpanded(false);
          knowledgeCtx = ks.map((k) =>
            `【${k.title} — ${k.sectionTitle}】\n${k.sectionContent}`
          ).join("\n\n");
        }
      } catch (e) {
        console.warn("Failed to search knowledge:", e);
        setContextError(true);
      }
    } else {
      setKnowledgeSections([]);
    }

    // Pass the raw user request — knowledge is injected ONLY via the system prompt (Rust side).
    await generate(aiInput.trim(), { context, knowledgeSections: knowledgeCtx || undefined });
    triggerAutoCompile();
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleGenerate();
    }
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
      {/* Section title */}
      <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 12, fontWeight: 600, letterSpacing: "0.05em", textTransform: "uppercase", color: "var(--color-ink-muted)" }}>
        <Sparkles size={13} style={{ color: "var(--color-accent)" }} />
        {t("lilypond.ai.title")}
      </div>

      {/* Textarea */}
      <textarea
        value={aiInput}
        onChange={(e) => setAiInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={t("lilypond.ai.inputPlaceholder")}
        disabled={isAIGenerating || !isOnline}
        rows={4}
        style={{
          width: "100%", padding: "10px 12px", fontSize: 13, lineHeight: 1.6,
          fontFamily: "inherit", borderRadius: 8, border: "1px solid var(--color-divider)",
          backgroundColor: "var(--color-surface)", color: "var(--color-ink)",
          resize: "vertical", outline: "none", transition: "border-color 0.15s ease",
          boxSizing: "border-box",
        }}
        onFocus={(e) => { e.currentTarget.style.borderColor = "var(--color-accent)"; }}
        onBlur={(e) => { e.currentTarget.style.borderColor = "var(--color-divider)"; }}
      />

      {/* Offline notice */}
      {!isOnline && (
        <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 11, color: "var(--color-danger)", padding: "4px 0" }}>
          <AlertCircle size={12} />
          {t("lilypond.ai.offline")}
        </div>
      )}

      {/* Context load failure warning */}
      {contextError && (
        <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 11, color: "var(--color-accent-deep)", padding: "4px 0" }}>
          <AlertCircle size={12} />
          {t("lilypond.ai.contextLoadFailed")}
        </div>
      )}

      {/* Matched knowledge sections (auto-populated after generate) */}
      {knowledgeSections.length > 0 && (
        <div style={{ border: "1px solid var(--color-divider)", borderRadius: 8, overflow: "hidden" }}>
          <button
            onClick={() => setKnowledgeExpanded(!knowledgeExpanded)}
            style={{
              display: "flex", alignItems: "center", justifyContent: "space-between",
              width: "100%", padding: "5px 10px", fontSize: 11, fontWeight: 600,
              border: "none", backgroundColor: "rgba(196,149,106,0.06)", color: "var(--color-accent-deep)",
              cursor: "pointer",
            }}
          >
            <span style={{ display: "flex", alignItems: "center", gap: 4 }}>
              <BookOpen size={11} />
              {t("lilypond.ai.knowledgeInjected", { count: knowledgeSections.length })}
            </span>
            <ChevronDown size={11} style={{ transform: knowledgeExpanded ? "rotate(180deg)" : "rotate(0deg)", transition: "transform 0.15s ease" }} />
          </button>
          {knowledgeExpanded && (
            <div style={{ padding: "6px 10px", maxHeight: 150, overflowY: "auto" }}>
              {knowledgeSections.map((ks, i) => (
                <div key={i} style={{ marginBottom: 4, fontSize: 11, lineHeight: 1.4 }}>
                  <div style={{ color: "var(--color-accent-deep)", fontWeight: 600 }}>
                    {ks.title} › {ks.sectionTitle}
                  </div>
                  <div style={{ color: "var(--color-ink-muted)", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                    {ks.sectionContent.slice(0, 120)}...
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Action row */}
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", gap: 8 }}>
        <TemplateMenu onSelectTemplate={onTemplateSelect} />

        <button
          onClick={handleGenerate}
          disabled={!aiInput.trim() || isAIGenerating || !isOnline}
          style={{
            display: "flex", alignItems: "center", gap: 6, padding: "7px 16px",
            fontSize: 13, fontWeight: 600, borderRadius: 8, border: "none",
            backgroundColor: !aiInput.trim() || isAIGenerating || !isOnline ? "var(--color-divider)" : "var(--color-accent)",
            color: !aiInput.trim() || isAIGenerating || !isOnline ? "var(--color-ink-muted)" : "#fff",
            cursor: !aiInput.trim() || isAIGenerating || !isOnline ? "default" : "pointer",
            transition: "all 0.15s ease", whiteSpace: "nowrap",
          }}
        >
          <Send size={13} />
          {isAIGenerating ? t("lilypond.ai.generating") : t("lilypond.ai.generate")}
        </button>
      </div>
    </div>
  );
}
