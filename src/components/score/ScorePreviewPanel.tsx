import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useScoreStore } from "../../stores/scoreStore";
import { useLilyPondCompile } from "../../hooks/useLilyPondCompile";
import { ExportMenu } from "./ExportMenu";
import {
  Music,
  Loader2,
  AlertTriangle,
  ChevronRight,
  Eye,
  ChevronDown,
} from "lucide-react";

export function ScorePreviewPanel() {
  const { t } = useTranslation();
  const { compileStatus, previewImage, previewFormat, compileErrors, compile } =
    useLilyPondCompile();
  const lilypondCode = useScoreStore((s) => s.lilypondCode);
  const autoCompile = useScoreStore((s) => s.autoCompile);
  const clearAutoCompile = useScoreStore((s) => s.clearAutoCompile);
  const reviewWarnings = useScoreStore((s) => s.reviewWarnings);
  const [reviewExpanded, setReviewExpanded] = useState(false);

  useEffect(() => {
    if (autoCompile > 0 && lilypondCode.trim()) {
      compile().finally(() => {
        clearAutoCompile();
      });
    }
  }, [autoCompile, lilypondCode, compile, clearAutoCompile]);

  const isCompiling = compileStatus === "COMPILING";
  const hasFailed = compileStatus === "FAILED";
  const hasPreview = compileStatus === "SUCCESS" && previewImage;
  const hasErrors = compileErrors.length > 0;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100%",
      }}
    >
      {/* Section title */}
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
          marginBottom: 8,
        }}
      >
        <Music size={13} style={{ color: "var(--color-accent)" }} />
        {t("lilypond.preview.title")}
      </div>

      {/* Preview area */}
      <div
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: 8,
          border: "1px solid var(--color-divider)",
          backgroundColor: "var(--color-surface)",
          overflow: "auto",
          display: "flex",
          flexDirection: "column",
        }}
      >
        {isCompiling && (
          <div
            style={{
              flex: 1,
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              gap: 12,
              color: "var(--color-ink-muted)",
            }}
          >
            <Loader2 size={28} className="animate-spin" />
            <span style={{ fontSize: 13 }}>
              {t("lilypond.preview.compiling")}
            </span>
          </div>
        )}

        {!isCompiling && hasPreview && (
          <div
            style={{
              flex: 1,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              padding: 16,
            }}
          >
            {previewFormat === "svg" ? (
              <img
                src={previewImage}
                alt="Score preview"
                style={{
                  maxWidth: "100%",
                  maxHeight: "100%",
                  objectFit: "contain",
                }}
              />
            ) : (
              <img
                src={previewImage}
                alt="Score preview"
                style={{
                  maxWidth: "100%",
                  maxHeight: "100%",
                  objectFit: "contain",
                  borderRadius: 4,
                }}
              />
            )}
          </div>
        )}

        {!isCompiling && hasFailed && (
          <div
            style={{
              flex: 1,
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              justifyContent: "center",
              gap: 12,
              padding: 24,
            }}
          >
            <AlertTriangle size={28} style={{ color: "var(--color-danger)" }} />
            <span
              style={{
                fontSize: 13,
                fontWeight: 500,
                color: "var(--color-danger)",
              }}
            >
              {t("lilypond.preview.error")}
            </span>
          </div>
        )}

        {!isCompiling && !hasPreview && !hasFailed && (
          <div
            style={{
              flex: 1,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              padding: 24,
            }}
          >
            <span
              style={{
                fontSize: 13,
                color: "var(--color-ink-muted)",
                opacity: 0.6,
              }}
            >
              {t("lilypond.preview.noPreview")}
            </span>
          </div>
        )}
      </div>

      {/* Compile errors / warnings list */}
      {hasErrors && (
        <div
          style={{
            marginTop: 8,
            maxHeight: 100,
            overflowY: "auto",
            borderRadius: 6,
            border: `1px solid ${
              compileErrors.some((e) => e.severity === "error")
                ? "rgba(196,106,90,0.3)"
                : "rgba(196,149,106,0.3)"
            }`,
            backgroundColor: compileErrors.some((e) => e.severity === "error")
              ? "rgba(196,106,90,0.04)"
              : "rgba(196,149,106,0.04)",
            padding: "6px 10px",
          }}
        >
          {compileErrors.map((err, i) => (
            <div
              key={i}
              style={{
                fontSize: 11,
                lineHeight: 1.6,
                fontFamily: "monospace",
                color:
                  err.severity === "error"
                    ? "var(--color-danger)"
                    : "var(--color-accent)",
                display: "flex",
                gap: 6,
                alignItems: "flex-start",
              }}
            >
              <ChevronRight
                size={11}
                style={{ marginTop: 2, flexShrink: 0, opacity: 0.5 }}
              />
              <span>
                {err.line > 0 && (
                  <span style={{ opacity: 0.6 }}>
                    L{err.line}
                    {err.column > 0 && `:${err.column}`}{" "}
                  </span>
                )}
                {err.message}
              </span>
            </div>
          ))}
        </div>
      )}

      {/* Review warnings (advisory only) */}
      {reviewWarnings.length > 0 && (
        <div
          style={{
            marginTop: 8,
            borderRadius: 6,
            border: "1px solid rgba(106,149,196,0.3)",
            backgroundColor: "rgba(106,149,196,0.04)",
            overflow: "hidden",
          }}
        >
          <button
            onClick={() => setReviewExpanded(!reviewExpanded)}
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
              padding: "5px 10px",
              fontSize: 11,
              fontWeight: 600,
              border: "none",
              backgroundColor: "transparent",
              color: "rgb(106,149,196)",
              cursor: "pointer",
            }}
          >
            <span style={{ display: "flex", alignItems: "center", gap: 4 }}>
              <Eye size={11} />
              {t("lilypond.review.title", { count: reviewWarnings.length, defaultValue: `审查建议 (${reviewWarnings.length})` })}
            </span>
            <ChevronDown
              size={11}
              style={{
                transform: reviewExpanded ? "rotate(180deg)" : "rotate(0deg)",
                transition: "transform 0.15s ease",
              }}
            />
          </button>
          {reviewExpanded && (
            <div style={{ padding: "6px 10px", maxHeight: 120, overflowY: "auto" }}>
              {reviewWarnings.map((w, i) => (
                <div
                  key={i}
                  style={{
                    fontSize: 11,
                    lineHeight: 1.6,
                    color: w.severity === "error" ? "var(--color-danger)" : "rgb(106,149,196)",
                    display: "flex",
                    gap: 6,
                    alignItems: "flex-start",
                    marginBottom: 2,
                  }}
                >
                  <ChevronRight
                    size={11}
                    style={{ marginTop: 2, flexShrink: 0, opacity: 0.5 }}
                  />
                  <span>
                    <span style={{ fontWeight: 600 }}>{w.rule}</span>
                    {w.location && <span style={{ opacity: 0.6 }}> ({w.location})</span>}
                    {" — "}
                    {w.description}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Action bar */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginTop: 10,
          gap: 8,
        }}
      >
        <button
          onClick={() => compile()}
          disabled={isCompiling || !lilypondCode.trim()}
          style={{
            display: "flex",
            alignItems: "center",
            gap: 6,
            padding: "8px 18px",
            fontSize: 13,
            fontWeight: 600,
            borderRadius: 8,
            border: "none",
            backgroundColor:
              isCompiling || !lilypondCode.trim()
                ? "var(--color-divider)"
                : "var(--color-accent)",
            color:
              isCompiling || !lilypondCode.trim()
                ? "var(--color-ink-muted)"
                : "#fff",
            cursor:
              isCompiling || !lilypondCode.trim() ? "default" : "pointer",
            transition: "all 0.15s ease",
          }}
        >
          {isCompiling ? (
            <Loader2 size={14} className="animate-spin" />
          ) : (
            <Music size={14} />
          )}
          {isCompiling
            ? t("lilypond.preview.compiling")
            : t("lilypond.preview.compile")}
        </button>

        <ExportMenu />
      </div>
    </div>
  );
}
