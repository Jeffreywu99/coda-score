import { useState, useRef, useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useScoreStore } from "../../stores/scoreStore";
import { getLilypondTemplates } from "../../lib/lilypond";
import type { Template, TemplateCategory } from "../../types/lilypond";
import { TEMPLATE_CATEGORY_META } from "../../types/lilypond";
import { ChevronDown, FileText } from "lucide-react";

interface TemplateMenuProps {
  onSelectTemplate: (template: Template) => void;
}

export function TemplateMenu({ onSelectTemplate }: TemplateMenuProps) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const [templates, setTemplates] = useState<Template[]>([]);
  const [loading, setLoading] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const selectedTemplate = useScoreStore((s) => s.selectedTemplate);
  const setSelectedTemplate = useScoreStore((s) => s.setSelectedTemplate);

  // Group templates by category, sorted by category order
  const groupedTemplates = useMemo(() => {
    const groups: { category: TemplateCategory; templates: Template[] }[] = [];
    const seen = new Set<string>();
    for (const tpl of templates) {
      if (!seen.has(tpl.category)) {
        seen.add(tpl.category);
        groups.push({ category: tpl.category as TemplateCategory, templates: [] });
      }
      const group = groups.find((g) => g.category === tpl.category);
      if (group) group.templates.push(tpl);
    }
    groups.sort(
      (a, b) =>
        (TEMPLATE_CATEGORY_META[a.category]?.order ?? 99) -
        (TEMPLATE_CATEGORY_META[b.category]?.order ?? 99)
    );
    return groups;
  }, [templates]);

  // Close dropdown on outside click
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    if (open) {
      document.addEventListener("mousedown", handleClickOutside);
    }
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open]);

  async function handleToggle() {
    if (!open && templates.length === 0) {
      setLoading(true);
      try {
        const list = await getLilypondTemplates();
        setTemplates(list);
      } catch {
        // silently fail
      } finally {
        setLoading(false);
      }
    }
    setOpen((prev) => !prev);
  }

  function handleSelect(template: Template) {
    setSelectedTemplate(template.id);
    onSelectTemplate(template);
    setOpen(false);
  }

  const selectedName =
    selectedTemplate && templates.find((t) => t.id === selectedTemplate);

  return (
    <div ref={menuRef} style={{ position: "relative" }}>
      <button
        onClick={handleToggle}
        style={{
          display: "flex",
          alignItems: "center",
          gap: 6,
          padding: "6px 10px",
          fontSize: 12,
          fontWeight: 500,
          borderRadius: 6,
          border: "1px solid var(--color-divider)",
          backgroundColor: "var(--color-surface)",
          color: "var(--color-ink-muted)",
          cursor: "pointer",
          transition: "all 0.15s ease",
          whiteSpace: "nowrap",
        }}
      >
        <FileText size={13} />
        <span>
          {selectedName
            ? t(`lilypond.templates.${selectedName.id}`, selectedName.name)
            : t("lilypond.ai.selectTemplate")}
        </span>
        <ChevronDown
          size={12}
          style={{
            transition: "transform 0.15s ease",
            transform: open ? "rotate(180deg)" : "rotate(0deg)",
          }}
        />
      </button>

      {open && (
        <div
          style={{
            position: "absolute",
            top: "calc(100% + 4px)",
            left: 0,
            minWidth: 240,
            maxHeight: 380,
            overflowY: "auto",
            backgroundColor: "var(--color-surface)",
            border: "1px solid var(--color-divider)",
            borderRadius: 8,
            boxShadow: "0 4px 16px rgba(0,0,0,0.08)",
            zIndex: 100,
          }}
        >
          {loading ? (
            <div
              style={{
                padding: "12px 16px",
                fontSize: 13,
                color: "var(--color-ink-muted)",
              }}
            >
              {t("common.loading")}
            </div>
          ) : templates.length === 0 ? (
            <div
              style={{
                padding: "12px 16px",
                fontSize: 13,
                color: "var(--color-ink-muted)",
              }}
            >
              —
            </div>
          ) : (
            groupedTemplates.map((group) => (
              <div key={group.category}>
                {/* Category header */}
                <div
                  style={{
                    padding: "6px 16px 2px",
                    fontSize: 10,
                    fontWeight: 700,
                    letterSpacing: "0.05em",
                    textTransform: "uppercase",
                    color: "var(--color-accent)",
                    opacity: 0.7,
                  }}
                >
                  {t(`templateCategory.${group.category}`)}
                </div>
                {group.templates.map((tpl) => (
                  <button
                    key={tpl.id}
                    onClick={() => handleSelect(tpl)}
                    style={{
                      display: "block",
                      width: "100%",
                      padding: "7px 16px",
                      fontSize: 13,
                      textAlign: "left",
                      border: "none",
                      backgroundColor:
                        selectedTemplate === tpl.id
                          ? "rgba(196,149,106,0.08)"
                          : "transparent",
                      color: "var(--color-ink)",
                      cursor: "pointer",
                      transition: "background-color 0.1s ease",
                    }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.backgroundColor =
                        "rgba(196,149,106,0.06)";
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.backgroundColor =
                        selectedTemplate === tpl.id
                          ? "rgba(196,149,106,0.08)"
                          : "transparent";
                    }}
                  >
                    <div style={{ fontWeight: 500 }}>{t(`lilypond.templates.${tpl.id}`, tpl.name)}</div>
                  </button>
                ))}
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
}
