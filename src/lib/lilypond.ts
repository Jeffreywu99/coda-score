import type {
  LilyPondCompileResult,
  LilyPondInfo,
  Template,
  ReferenceSection,
  KnowledgeSection,
  ExportFormat,
} from "../types/lilypond";

interface TauriGlobal {
  core: {
    invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
  };
}

function getTauri(): TauriGlobal | undefined {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (window as any).__TAURI__;
}

/** Compile LilyPond code and return result (PNG/SVG/PDF output as base64 or path) */
export async function compileLilypond(
  code: string,
  outputFormat: ExportFormat = "png"
): Promise<LilyPondCompileResult> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("compile_lilypond", {
    code,
    outputFormat,
  }) as Promise<LilyPondCompileResult>;
}

/** Check if LilyPond is installed and return version info */
export async function checkLilypondInstallation(): Promise<LilyPondInfo> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("check_lilypond_installation") as Promise<LilyPondInfo>;
}

/** Get available LilyPond code templates */
export async function getLilypondTemplates(): Promise<Template[]> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("get_lilypond_templates") as Promise<Template[]>;
}

/** Search reference documentation by keywords */
export async function searchReference(
  keywords: string[],
  maxSections: number = 5
): Promise<ReferenceSection[]> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("search_reference", {
    keywords,
    maxSections,
  }) as Promise<ReferenceSection[]>;
}

/** Search the embedded knowledge base for composition rules matching the query. */
export async function searchKnowledge(
  query: string,
  maxResults: number = 5,
  templateId?: string
): Promise<KnowledgeSection[]> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("search_knowledge", {
    query,
    maxResults,
    templateId: templateId ?? null,
  }) as Promise<KnowledgeSection[]>;
}

/** Export a compiled LilyPond file to a user-selected path */
export async function exportLilypondFile(
  sourcePath: string,
  targetPath: string
): Promise<void> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("export_lilypond_file", {
    sourcePath,
    targetPath,
  }) as Promise<void>;
}
