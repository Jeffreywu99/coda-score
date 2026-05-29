import { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { useScoreStore } from "../stores/scoreStore";
import * as ai from "../lib/ai";
import type { AIGenerateRequest } from "../types/lilypond";

/**
 * Hook for AI-powered LilyPond code generation.
 *
 * v0.5.0: Single API call, no review loop.
 * Review runs once in background (advisory only — shows warnings, never re-generates).
 */
export function useAIGenerate() {
  const { t } = useTranslation();
  const isAIGenerating = useScoreStore((s) => s.isAIGenerating);
  const isOnline = useScoreStore((s) => s.isOnline);

  const generate = useCallback(
    async (
      prompt: string,
      options: {
        context?: string;
        knowledgeSections?: string;
      } = {}
    ) => {
      const store = useScoreStore.getState();

      // Guard against rapid double-clicks: check and set atomically
      if (store.isAIGenerating) {
        return false; // already generating, prevent concurrent calls
      }
      store.setIsAIGenerating(true);

      if (!store.isOnline) {
        store.setIsAIGenerating(false);
        store.setCompileErrors([
          { line: 0, column: 0, message: t("lilypond.ai.offlineError"), severity: "error" },
        ]);
        return false;
      }

      store.clearReviewWarnings();

      // Single API call — model handles music planning + code generation internally
      const request: AIGenerateRequest = {
        prompt,
        context: options.context,
        knowledgeSections: options.knowledgeSections,
      };

      let lastCode: string;
      try {
        const result = await ai.generateLilypondCode(request);

        if (!result.success) {
          store.setCompileErrors([
            { line: 0, column: 0, message: result.errorMessage ?? t("lilypond.ai.generateFailed"), severity: "error" },
          ]);
          store.setIsAIGenerating(false);
          return false;
        }

        lastCode = result.lilypondCode;
        store.setLilypondCode(result.lilypondCode);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        store.setCompileErrors([
          { line: 0, column: 0, message, severity: "error" },
        ]);
        store.setIsAIGenerating(false);
        return false;
      }

      // Release button immediately — code is in the editor
      store.setIsAIGenerating(false);

      // Advisory review: runs once in background, shows warnings only, never re-generates
      runAdvisoryReview(lastCode, options.knowledgeSections);

      return true;
    },
    [t]
  );

  return {
    isAIGenerating,
    isOnline,
    generate,
  };
}

/**
 * Advisory review — runs once in the background after generation.
 * Shows warnings in the sidebar but never triggers re-generation.
 */
async function runAdvisoryReview(
  code: string,
  knowledgeSections?: string
) {
  try {
    const review = await ai.reviewLilypondCode(code, knowledgeSections);
    const store = useScoreStore.getState();
    if (review.violations && review.violations.length > 0) {
      store.setReviewWarnings(review.violations);
    }
  } catch {
    // Review failure is non-blocking — silently accept current code
  }
}
