import { useState } from "react";
import { Settings as SettingsIcon } from "lucide-react";
import { SettingsDialog } from "../common/SettingsDialog";

interface AppShellProps {
  children: React.ReactNode;
}

export function AppShell({ children }: AppShellProps) {
  const [settingsOpen, setSettingsOpen] = useState(false);

  return (
    <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
      {/* Top-right settings button */}
      <div style={{ display: "flex", justifyContent: "flex-end", padding: "8px 16px 0" }}>
        <button
          onClick={() => setSettingsOpen(true)}
          style={{
            padding: 6,
            borderRadius: 6,
            border: "none",
            backgroundColor: "transparent",
            color: "var(--color-ink-muted)",
            cursor: "pointer",
            transition: "color 0.15s ease",
          }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--color-accent-deep)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--color-ink-muted)"; }}
          title="Settings"
        >
          <SettingsIcon size={18} />
        </button>
      </div>
      <main style={{ flex: 1, overflow: "hidden", padding: "16px 32px 24px" }}>
        {children}
      </main>
      <SettingsDialog open={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </div>
  );
}
