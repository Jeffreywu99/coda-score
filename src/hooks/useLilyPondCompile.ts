import { useCallback, useEffect } from "react";
import { useScoreStore } from "../stores/scoreStore";
import * as lilypond from "../lib/lilypond";
import type { ExportFormat } from "../types/lilypond";

/**
 * Hook for compiling LilyPond code and managing preview state.
 * Handles:
 *   - Invoking the Rust compile command
 *   - Updating store with preview image / errors / status
 *   - Checking LilyPond installation on mount
 *   - Online/offline detection
 */
export function useLilyPondCompile() {
  const compileStatus = useScoreStore((s) => s.compileStatus);
  const previewImage = useScoreStore((s) => s.previewImage);
  const previewFormat = useScoreStore((s) => s.previewFormat);
  const compileErrors = useScoreStore((s) => s.compileErrors);
  const lilypondVersion = useScoreStore((s) => s.lilypondVersion);
  const lastCompileTime = useScoreStore((s) => s.lastCompileTime);
  const isOnline = useScoreStore((s) => s.isOnline);

  // Check LilyPond installation on mount
  useEffect(() => {
    let cancelled = false;

    async function checkInstallation() {
      try {
        const info = await lilypond.checkLilypondInstallation();
        if (!cancelled && info.available) {
          useScoreStore.getState().setLilypondVersion(info.version);
        }
      } catch {
        // LilyPond not installed or check failed — silent
      }
    }

    checkInstallation();

    return () => {
      cancelled = true;
    };
  }, []);

  // Online/offline detection
  useEffect(() => {
    const handleOnline = () => useScoreStore.getState().setIsOnline(true);
    const handleOffline = () => useScoreStore.getState().setIsOnline(false);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  /** Compile the current LilyPond code and update preview */
  const compile = useCallback(
    async (code?: string, format: ExportFormat = "png") => {
      const store = useScoreStore.getState();
      const lilyCode = code ?? store.lilypondCode;

      if (!lilyCode.trim()) {
        store.setCompileStatus("IDLE");
        store.setPreviewImage("", "png");
        store.setCompileErrors([]);
        return;
      }

      store.setCompileStatus("COMPILING");
      store.setCompileErrors([]);

      try {
        const result = await lilypond.compileLilypond(lilyCode, format);

        if (result.success) {
          // Build data URL for preview
          const mimeType =
            result.outputFormat === "svg" ? "image/svg+xml" : "image/png";
          const dataUrl = `data:${mimeType};base64,${result.outputBase64}`;

          store.setPreviewImage(dataUrl, result.outputFormat);
          store.setCompileStatus("SUCCESS");
          store.setCompileErrors(result.errors); // may contain warnings
          store.setLastCompileTime(result.durationMs);
        } else {
          store.setPreviewImage("", format);
          store.setCompileStatus("FAILED");
          store.setCompileErrors(result.errors);
          store.setLastCompileTime(result.durationMs);
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        store.setCompileStatus("FAILED");
        store.setCompileErrors([
          { line: 0, column: 0, message, severity: "error" },
        ]);
      }
    },
    []
  );

  return {
    compileStatus,
    previewImage,
    previewFormat,
    compileErrors,
    lilypondVersion,
    lastCompileTime,
    isOnline,
    compile,
  };
}
