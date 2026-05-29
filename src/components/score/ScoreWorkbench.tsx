import { useEffect, useState } from "react";
import { useScoreStore } from "../../stores/scoreStore";
import { AIInputPanel } from "./AIInputPanel";
import { CodeEditorPanel } from "./CodeEditorPanel";
import { ScorePreviewPanel } from "./ScorePreviewPanel";
import { StatusBar } from "./StatusBar";
import type { Template } from "../../types/lilypond";

export function ScoreWorkbench() {
  const setLilypondCode = useScoreStore((s) => s.setLilypondCode);

  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  useEffect(() => {
    const handleResize = () => setWindowWidth(window.innerWidth);
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);
  const isNarrow = windowWidth < 900;

  function handleTemplateSelect(template: Template) {
    setLilypondCode(template.lilypondCode);
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100%",
        maxWidth: 1400,
        margin: "0 auto",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: isNarrow ? "column" : "row",
          flex: 1,
          minHeight: 0,
          gap: 24,
        }}
      >
        <div
          style={{
            width: isNarrow ? "100%" : "40%",
            maxHeight: isNarrow ? "50%" : undefined,
            flexShrink: 0,
            display: "flex",
            flexDirection: "column",
            gap: 20,
            minWidth: 0,
          }}
        >
          <div
            style={{
              padding: 16,
              borderRadius: 10,
              backgroundColor: "var(--color-parchment)",
              border: "1px solid var(--color-divider)",
            }}
          >
            <AIInputPanel onTemplateSelect={handleTemplateSelect} />
          </div>
          <div
            style={{
              flex: 1,
              minHeight: 0,
              display: "flex",
              flexDirection: "column",
            }}
          >
            <CodeEditorPanel />
          </div>
        </div>
        <div
          style={{
            width: isNarrow ? "100%" : "60%",
            maxHeight: isNarrow ? "50%" : undefined,
            flexShrink: 0,
            minWidth: 0,
            display: "flex",
            flexDirection: "column",
          }}
        >
          <ScorePreviewPanel />
        </div>
      </div>
      <div style={{ marginTop: 12 }}>
        <StatusBar />
      </div>
    </div>
  );
}
