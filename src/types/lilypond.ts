/** Compilation status */
export type CompileStatus = "IDLE" | "COMPILING" | "SUCCESS" | "FAILED";

/** Compile error info */
export interface CompileError {
  line: number;
  column: number;
  message: string;
  severity: "error" | "warning";
}

/** Compile result from Rust */
export interface LilyPondCompileResult {
  success: boolean;
  outputFormat: "png" | "svg" | "pdf" | "midi";
  outputBase64: string;
  outputPath: string;
  errors: CompileError[];
  stderr: string;
  durationMs: number;
}

/** AI generate request */
export interface AIGenerateRequest {
  prompt: string;
  context?: string;
  knowledgeSections?: string;
  previousCode?: string;
  previousErrors?: string;
  retryCount?: number;
}

/** AI generate result */
export interface AIGenerateResult {
  success: boolean;
  lilypondCode: string;
  errorMessage?: string;
}

/** Cross-tool import data */
export interface SharedImportData {
  sourceToolId: string;
  lilypondCode: string;
  description: string;
}

/** Template slot (placeholder) definition */
export interface TemplateSlot {
  name: string;
  type: "music" | "text" | "markup";
  required: boolean;
  description: string;
  default: string;
}

/** Code template */
export interface Template {
  id: string;
  category: TemplateCategory;
  name: string;
  nameEn: string;
  description: string;
  slots: TemplateSlot[];
  lilypondCode: string;
}

/** Template category */
export type TemplateCategory =
  | "basic"
  | "piano"
  | "chamber"
  | "vocal"
  | "contemporary"
  | "analysis";

/** Category display metadata */
export const TEMPLATE_CATEGORY_META: Record<TemplateCategory, { name: string; nameEn: string; order: number }> = {
  basic: { name: "基础", nameEn: "Basic", order: 0 },
  piano: { name: "钢琴", nameEn: "Piano", order: 1 },
  chamber: { name: "室内乐", nameEn: "Chamber", order: 2 },
  vocal: { name: "声乐", nameEn: "Vocal", order: 3 },
  contemporary: { name: "现代/当代", nameEn: "Contemporary", order: 4 },
  analysis: { name: "分析/教学", nameEn: "Analysis", order: 5 },
};

/** Export format */
export type ExportFormat = "pdf" | "png" | "svg" | "midi";

/** LilyPond installation info */
export interface LilyPondInfo {
  available: boolean;
  version: string;
  path: string;
}

/** Reference search request */
export interface ReferenceSearchRequest {
  keywords: string[];
  maxSections: number;
}

/** Reference section */
export interface ReferenceSection {
  title: string;
  content: string;
  relevance: number;
}

/** Knowledge section from the knowledge engine */
export interface KnowledgeSection {
  file: string;
  title: string;
  sectionTitle: string;
  sectionContent: string;
  tags: string;
  contexts: string;
  relevance: number;
}

/** Review result from the theory reviewer agent */
export interface ReviewResult {
  passed: boolean;
  score: number;           // 0-100
  violations: ReviewViolation[];
  summary: string;
}

export interface ReviewViolation {
  severity: string;
  rule: string;
  location: string;
  description: string;
}
