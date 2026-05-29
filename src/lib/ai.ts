import type { AIGenerateRequest, AIGenerateResult, ReviewResult } from "../types/lilypond";

interface TauriGlobal {
  core: {
    invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
  };
}

function getTauri(): TauriGlobal | undefined {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (window as any).__TAURI__;
}

/** Generate LilyPond code from natural language prompt via AI */
export async function generateLilypondCode(
  request: AIGenerateRequest
): Promise<AIGenerateResult> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("generate_lilypond_code", {
    request: {
      prompt: request.prompt,
      context: request.context ?? null,
      knowledgeSections: request.knowledgeSections ?? null,
      previousCode: request.previousCode ?? null,
      previousErrors: request.previousErrors ?? null,
      retryCount: request.retryCount ?? 0,
    },
  }) as Promise<AIGenerateResult>;
}

/** Check if AI service is available (API key configured, network reachable) */
export async function checkAIAvailable(): Promise<boolean> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("check_ai_available") as Promise<boolean>;
}

/** AI config display info returned from Rust */
export interface AiConfigDisplay {
  apiKey: string;
  baseUrl: string;
  model: string;
  source: "env" | "file" | "none";
}

/** Get current AI config (API key masked for security) */
export async function getAIConfig(): Promise<AiConfigDisplay> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("get_ai_config") as Promise<AiConfigDisplay>;
}

/** Save AI config to app data directory */
export async function saveAIConfig(
  apiKey: string,
  baseUrl: string,
  model: string
): Promise<void> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  await tauri.core.invoke("save_ai_config", { apiKey, baseUrl, model });
}

/** Test AI API connection with current config */
export async function testAIConnection(): Promise<string> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("test_ai_connection") as Promise<string>;
}

/** Review LilyPond code against music theory rules */
export async function reviewLilypondCode(
  lilypondCode: string,
  knowledgeSections?: string
): Promise<ReviewResult> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("review_lilypond_code", {
    request: {
      lilypondCode,
      knowledgeSections: knowledgeSections ?? null,
    },
  }) as Promise<ReviewResult>;
}
