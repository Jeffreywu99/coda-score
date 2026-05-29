import type { Template } from "../types/lilypond";
import { getLilypondTemplates } from "./lilypond";

/**
 * Fill a template's slots with provided values.
 * Unfilled optional slots use their default value.
 * Music slots default to rests; text slots default to empty string.
 */
export function fillTemplate(
  template: Template,
  values: Record<string, string>
): string {
  let result = template.lilypondCode;

  for (const slot of template.slots) {
    const value = values[slot.name] ?? slot.default;
    result = result.replaceAll(`{{${slot.name}}}`, value);
  }

  // Clean up empty header fields (title = "", composer = "", etc.)
  result = cleanEmptyHeaders(result);

  return result;
}

/** Get all slot names from a template */
export function getSlotNames(template: Template): string[] {
  return template.slots.map((s) => s.name);
}

/** Find a template by ID from a template list */
export function findTemplate(
  templates: Template[],
  id: string
): Template | undefined {
  return templates.find((t) => t.id === id);
}

/** Check if a template has unfilled required slots */
export function hasMissingSlots(
  values: Record<string, string>,
  template: Template
): string[] {
  return template.slots
    .filter((s) => s.required && !values[s.name])
    .map((s) => s.name);
}

/**
 * Build AI context string from a template.
 * NOTE: The instructional text is intentionally in Chinese — the default AI
 * backend (DeepSeek) responds best to Chinese system prompts. The LilyPond
 * code and slot names are language-neutral.
 */
export function buildTemplateContext(template: Template): string {
  const slotList = template.slots
    .map((s) => `  {{${s.name}}} — ${s.description}${s.required ? " (必填)" : ""}`)
    .join("\n");

  return `使用以下 LilyPond 模板结构。只替换 {{SLOT_NAME}} 占位符，不要修改模板结构（五线谱配置、layout、header）。未提供的可选插槽用休止符填充。

${template.lilypondCode}

模板插槽说明：
${slotList}`;
}

/** Extract slot references from LilyPond code */
export function extractSlotRefs(code: string): string[] {
  const re = /\{\{(\w+)\}\}/g;
  const names = new Set<string>();
  let match: RegExpExecArray | null;
  while ((match = re.exec(code)) !== null) {
    names.add(match[1]);
  }
  return Array.from(names);
}

// ─── Template cache ────────────────────────────────────────────────────────────

let templateCache: Template[] | null = null;

/** Load templates from Rust backend, cached after first call */
export async function loadTemplates(): Promise<Template[]> {
  if (templateCache) return templateCache;
  templateCache = await getLilypondTemplates();
  return templateCache;
}

/** Get cached templates (synchronous, may be null if not loaded) */
export function getCachedTemplates(): Template[] | null {
  return templateCache;
}

// ─── Internal helpers ──────────────────────────────────────────────────────────

/** Remove header lines with empty values (e.g., title = "") */
function cleanEmptyHeaders(code: string): string {
  return code
    .replace(/^\s*(title|composer|subtitle|subsubtitle|poet|meter|arranger|opus|piece|instrument|dedication|copyright|tagline)\s*=\s*""\s*\n/gm, "")
    .replace(/\n{3,}/g, "\n\n");
}
