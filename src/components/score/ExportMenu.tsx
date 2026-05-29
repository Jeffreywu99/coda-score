import { useState, useRef, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { save } from "@tauri-apps/plugin-dialog";
import { compileLilypond, exportLilypondFile } from "../../lib/lilypond";
import { useScoreStore } from "../../stores/scoreStore";
import type { ExportFormat } from "../../types/lilypond";
import { Download, ChevronDown, FileText, Image, Music, AlertCircle } from "lucide-react";

interface ExportMenuProps {
  onCompileStart?: () => void;
  onCompileEnd?: (success: boolean) => void;
}

interface FormatOption {
  format: ExportFormat;
  labelKey: string;
  extension: string;
  filterName: string;
  icon: React.ReactNode;
}

const FORMAT_OPTIONS: FormatOption[] = [
  {
    format: "pdf",
    labelKey: "lilypond.export.pdf",
    extension: "pdf",
    filterName: "PDF Document",
    icon: <FileText size={14} />,
  },
  {
    format: "png",
    labelKey: "lilypond.export.png",
    extension: "png",
    filterName: "PNG Image",
    icon: <Image size={14} />,
  },
  {
    format: "svg",
    labelKey: "lilypond.export.svg",
    extension: "svg",
    filterName: "SVG Image",
    icon: <Image size={14} />,
  },
  {
    format: "midi",
    labelKey: "lilypond.export.midi",
    extension: "mid",
    filterName: "MIDI File",
    icon: <Music size={14} />,
  },
];

export function ExportMenu({ onCompileStart, onCompileEnd }: ExportMenuProps) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [exportError, setExportError] = useState<string | null>(null);
  const menuRef = useRef<HTMLDivElement>(null);

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

  async function handleExport(option: FormatOption) {
    setOpen(false);
    const store = useScoreStore.getState();
    const code = store.lilypondCode;

    if (!code.trim()) return;

    setExporting(true);
    onCompileStart?.();

    try {
      // 1. Compile to the target format
      const result = await compileLilypond(code, option.format);

      if (!result.success) {
        onCompileEnd?.(false);
        setExportError(t("lilypond.export.compileFailed"));
        return;
      }

      setExportError(null);
      onCompileEnd?.(true);

      // 2. Open save dialog
      const filePath = await save({
        defaultPath: `score.${option.extension}`,
        filters: [
          { name: option.filterName, extensions: [option.extension] },
        ],
      });

      // 3. Export the file
      if (filePath) {
        await exportLilypondFile(result.outputPath, filePath);
      }
    } catch (err) {
      console.error("Export failed:", err);
      onCompileEnd?.(false);
    } finally {
      setExporting(false);
    }
  }

  return (
    <div ref={menuRef} style={{ position: "relative" }}>
      <button
        onClick={() => setOpen((prev) => !prev)}
        disabled={exporting}
        style={{
          display: "flex",
          alignItems: "center",
          gap: 6,
          padding: "7px 12px",
          fontSize: 13,
          fontWeight: 500,
          borderRadius: 8,
          border: "1px solid var(--color-divider)",
          backgroundColor: "var(--color-surface)",
          color: exporting ? "var(--color-ink-muted)" : "var(--color-ink)",
          cursor: exporting ? "default" : "pointer",
          opacity: exporting ? 0.6 : 1,
          transition: "all 0.15s ease",
        }}
      >
        <Download size={14} />
        <span>{t("lilypond.export.title")}</span>
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
            right: 0,
            minWidth: 180,
            backgroundColor: "var(--color-surface)",
            border: "1px solid var(--color-divider)",
            borderRadius: 8,
            boxShadow: "0 4px 16px rgba(0,0,0,0.08)",
            zIndex: 100,
            overflow: "hidden",
          }}
        >
          {FORMAT_OPTIONS.map((option) => (
            <button
              key={option.format}
              onClick={() => handleExport(option)}
              disabled={exporting}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 8,
                width: "100%",
                padding: "9px 16px",
                fontSize: 13,
                textAlign: "left",
                border: "none",
                backgroundColor: "transparent",
                color: "var(--color-ink)",
                cursor: exporting ? "default" : "pointer",
                transition: "background-color 0.1s ease",
              }}
              onMouseEnter={(e) => {
                if (!exporting) {
                  e.currentTarget.style.backgroundColor =
                    "rgba(196,149,106,0.06)";
                }
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "transparent";
              }}
            >
              <span style={{ color: "var(--color-accent)" }}>
                {option.icon}
              </span>
              {t(option.labelKey)}
            </button>
          ))}
        </div>
      )}

      {/* Export error feedback */}
      {exportError && (
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 6,
            marginTop: 6,
            fontSize: 11,
            color: "var(--color-danger)",
          }}
        >
          <AlertCircle size={12} />
          {exportError}
        </div>
      )}
    </div>
  );
}
