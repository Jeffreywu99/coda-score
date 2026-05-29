import { useTranslation } from "react-i18next";
import { useScoreStore } from "../../stores/scoreStore";
import {
  CircleCheck,
  CircleX,
  Loader2,
  Circle,
  Wifi,
  WifiOff,
} from "lucide-react";

export function StatusBar() {
  const { t } = useTranslation();
  const compileStatus = useScoreStore((s) => s.compileStatus);
  const lilypondVersion = useScoreStore((s) => s.lilypondVersion);
  const lastCompileTime = useScoreStore((s) => s.lastCompileTime);
  const isOnline = useScoreStore((s) => s.isOnline);

  const statusConfig: Record<
    string,
    { icon: React.ReactNode; color: string; text: string }
  > = {
    IDLE: {
      icon: <Circle size={12} />,
      color: "var(--color-ink-muted)",
      text: t("lilypond.status.idle"),
    },
    COMPILING: {
      icon: <Loader2 size={12} className="animate-spin" />,
      color: "var(--color-accent)",
      text: t("lilypond.status.compiling"),
    },
    SUCCESS: {
      icon: <CircleCheck size={12} />,
      color: "var(--color-success)",
      text: t("lilypond.status.success"),
    },
    FAILED: {
      icon: <CircleX size={12} />,
      color: "var(--color-danger)",
      text: t("lilypond.status.failed"),
    },
  };

  const current = statusConfig[compileStatus] ?? statusConfig["IDLE"];

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "6px 16px",
        fontSize: 11,
        borderTop: "1px solid var(--color-divider)",
        backgroundColor: "var(--color-surface)",
        color: "var(--color-ink-muted)",
        flexShrink: 0,
      }}
    >
      {/* Left: status indicator */}
      <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
        <span style={{ color: current.color, display: "flex", alignItems: "center" }}>
          {current.icon}
        </span>
        <span style={{ color: current.color, fontWeight: 500 }}>
          {current.text}
        </span>
        {lastCompileTime > 0 && compileStatus !== "IDLE" && (
          <span style={{ opacity: 0.6, marginLeft: 4 }}>
            {t("lilypond.status.duration", { ms: lastCompileTime })}
          </span>
        )}
      </div>

      {/* Right: version + online status */}
      <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
        {lilypondVersion && (
          <span style={{ opacity: 0.7 }}>
            {t("lilypond.status.version", { version: lilypondVersion })}
          </span>
        )}
        <span style={{ display: "flex", alignItems: "center", gap: 4 }}>
          {isOnline ? (
            <Wifi size={11} style={{ color: "var(--color-success)" }} />
          ) : (
            <WifiOff size={11} style={{ color: "var(--color-danger)" }} />
          )}
          <span style={{ opacity: 0.6 }}>
            {isOnline ? t("lilypond.status.online") : t("lilypond.status.offline")}
          </span>
        </span>
      </div>
    </div>
  );
}
