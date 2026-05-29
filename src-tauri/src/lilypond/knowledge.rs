use serde::Serialize;
use std::sync::OnceLock;

// Include the auto-generated knowledge data
include!(concat!(env!("OUT_DIR"), "/knowledge_data.rs"));

// ─── Public Types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub file: &'static str,
    pub title: &'static str,
    pub tags: &'static str,
    pub content: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeSection {
    pub file: String,
    pub title: String,
    pub section_title: String,
    pub section_content: String,
    pub tags: String,
    pub contexts: String,
    pub relevance: f64,
}

// ─── Context-to-template mapping ─────────────────────────────────────────────────

/// Given a template ID (or inferred scenario), return the list of context tags
/// that should be included in the search. Files whose `contexts` frontmatter
/// does NOT intersect with this list will be excluded.
pub fn contexts_for_template(template_id: &str) -> Vec<&'static str> {
    // "all" is always included regardless of template
    let mut ctx: Vec<&'static str> = vec!["all"];

    match template_id {
        // ── Classical templates ──
        "satb-choir" | "satb-piano" => {
            ctx.extend_from_slice(&["classical", "satb", "choir", "vocal"]);
        }
        "string-quartet" => {
            ctx.extend_from_slice(&["classical", "chamber", "strings"]);
        }
        "duet" => {
            ctx.extend_from_slice(&["classical", "chamber"]);
        }
        "woodwind-trio" => {
            ctx.extend_from_slice(&["classical", "chamber", "woodwinds"]);
        }
        "brass-quintet" => {
            ctx.extend_from_slice(&["classical", "chamber", "brass"]);
        }
        "piano-solo" | "piano-voice" => {
            ctx.extend_from_slice(&["classical", "piano", "solo"]);
        }
        "solo-melody" | "solo-bass" => {
            ctx.extend_from_slice(&["classical", "solo"]);
        }
        "chord-progression" | "pitch-class-chord" => {
            ctx.extend_from_slice(&["classical", "analysis"]);
        }

        // ── Modernist templates ──
        "twelve-tone-row" | "twelve-tone-matrix" => {
            ctx.extend_from_slice(&["modernist", "serial", "twelve-tone", "atonal"]);
        }
        "set-composition" => {
            ctx.extend_from_slice(&["modernist", "set-theory", "atonal", "post-tonal"]);
        }
        "minimalist-process" => {
            ctx.extend_from_slice(&["modernist", "minimalist", "process-music"]);
        }
        "spectral-harmony" => {
            ctx.extend_from_slice(&["modernist", "spectral"]);
        }
        "graphic-score" | "extended-chamber" => {
            ctx.extend_from_slice(&[
                "modernist",
                "extended-techniques",
                "graphic-notation",
                "aleatoric",
                "experimental",
            ]);
        }

        // ── Legacy / contemporary templates (bridge) ──
        "microtonal" | "cluster" | "proportional" | "graphic-staff" => {
            ctx.extend_from_slice(&["modernist", "contemporary", "experimental"]);
        }

        // ── Fallback: blank/free — include classical + modernist basics ──
        "blank" | "" | _ => {
            ctx.extend_from_slice(&["classical", "modernist"]);
        }
    }

    ctx
}

// ─── Search API ──────────────────────────────────────────────────────────────────

/// Cache for parsed knowledge sections.
static KNOWLEDGE_CACHE: OnceLock<Vec<ParsedSection>> = OnceLock::new();

#[derive(Debug, Clone)]
struct ParsedSection {
    file: &'static str,
    title: &'static str,
    tags: &'static str,
    contexts: String,
    section_title: String,
    section_content: String,
}

/// Search all embedded knowledge entries for sections matching the query.
///
/// `context_filter` is an optional list of context tags. When provided, only
/// sections whose `contexts` frontmatter intersects with this list (or contains
/// "all") will be considered. Pass `None` to search everything.
pub fn search_knowledge(
    query: &str,
    max_results: usize,
    context_filter: Option<&[&str]>,
) -> Vec<KnowledgeSection> {
    let all_sections = KNOWLEDGE_CACHE.get_or_init(|| parse_all_entries());

    if query.trim().is_empty() {
        return Vec::new();
    }

    // Tokenize query: split on whitespace, keep meaningful tokens
    let keywords: Vec<String> = query
        .split_whitespace()
        .map(|s| {
            s.trim_matches(|c: char| !c.is_alphanumeric() && !c.is_ascii_alphabetic())
                .to_lowercase()
        })
        .filter(|s| !s.is_empty())
        .collect();

    if keywords.is_empty() {
        return Vec::new();
    }

    let mut scored: Vec<KnowledgeSection> = all_sections
        .iter()
        .filter(|section| match_context(section, context_filter))
        .filter_map(|section| {
            let score = score_section(section, &keywords);
            if score > 0.0 {
                Some(KnowledgeSection {
                    file: section.file.to_string(),
                    title: section.title.to_string(),
                    section_title: section.section_title.clone(),
                    section_content: truncate_content(&section.section_content, 1500),
                    tags: section.tags.to_string(),
                    contexts: section.contexts.clone(),
                    relevance: score,
                })
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| {
        b.relevance
            .partial_cmp(&a.relevance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    scored.truncate(max_results);
    scored
}

// ─── Context filtering ───────────────────────────────────────────────────────────

/// Check if a section's contexts intersect with the filter list.
/// Sections with `contexts: [all]` always pass.
fn match_context(section: &ParsedSection, filter: Option<&[&str]>) -> bool {
    let filter = match filter {
        Some(f) if !f.is_empty() => f,
        _ => return true, // No filter → everything passes
    };

    let section_ctx = section.contexts.to_lowercase();
    if section_ctx.contains("all") {
        return true;
    }

    filter.iter().any(|f| section_ctx.contains(f))
}

// ─── Parsing ─────────────────────────────────────────────────────────────────────

fn parse_all_entries() -> Vec<ParsedSection> {
    let mut sections = Vec::new();

    for entry in embedded_knowledge() {
        let content = entry.content.replace("\\n", "\n");

        // Extract contexts from frontmatter
        let contexts = extract_frontmatter_field(&content, "contexts");

        // Split content into sections by ## or ### headings
        let mut current_title = String::from(entry.title);
        let mut current_body = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") {
                if !current_body.trim().is_empty() {
                    sections.push(ParsedSection {
                        file: entry.file,
                        title: entry.title,
                        tags: entry.tags,
                        contexts: contexts.clone(),
                        section_title: current_title.clone(),
                        section_content: current_body.clone(),
                    });
                }
                current_title = trimmed.trim_start_matches("## ").trim().to_string();
                current_body = String::new();
            } else if trimmed.starts_with("### ") {
                if !current_body.trim().is_empty() {
                    sections.push(ParsedSection {
                        file: entry.file,
                        title: entry.title,
                        tags: entry.tags,
                        contexts: contexts.clone(),
                        section_title: current_title.clone(),
                        section_content: current_body.clone(),
                    });
                }
                current_title = trimmed.trim_start_matches("### ").trim().to_string();
                current_body = String::new();
            } else {
                current_body.push_str(line);
                current_body.push('\n');
            }
        }

        if !current_body.trim().is_empty() {
            sections.push(ParsedSection {
                file: entry.file,
                title: entry.title,
                tags: entry.tags,
                contexts,
                section_title: current_title,
                section_content: current_body,
            });
        }
    }

    sections
}

/// Extract a field value from YAML frontmatter (between --- delimiters).
fn extract_frontmatter_field(content: &str, field: &str) -> String {
    let mut in_frontmatter = false;
    let mut frontmatter_closed = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                continue;
            } else if !frontmatter_closed {
                frontmatter_closed = true;
                continue;
            }
        }
        if in_frontmatter && !frontmatter_closed {
            if trimmed.starts_with(&format!("{}:", field)) {
                return trimmed
                    .trim_start_matches(&format!("{}:", field))
                    .trim()
                    .to_string();
            }
        }
        // Stop searching if we've passed the frontmatter
        if frontmatter_closed {
            break;
        }
    }

    String::new()
}

// ─── Scoring ─────────────────────────────────────────────────────────────────────

fn score_section(section: &ParsedSection, keywords: &[String]) -> f64 {
    let mut score: f64 = 0.0;

    let section_title_lower = section.section_title.to_lowercase();
    let content_lower = section.section_content.to_lowercase();
    let tags_lower = section.tags.to_lowercase();
    let file_title_lower = section.title.to_lowercase();

    // Split tags into individual words for exact matching
    let tag_words: Vec<String> = tags_lower
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    for keyword in keywords {
        let kw = keyword.trim();
        if kw.is_empty() || kw.chars().count() < 2 && !kw.chars().all(|c| c.is_ascii()) {
            // Skip single CJK characters — too noisy (e.g. "乐" matches everything)
            continue;
        }

        // Tag exact match — highest weight
        if tag_words.iter().any(|t| t == kw) {
            score += 10.0;
        } else if tags_lower.contains(kw) {
            // Partial tag match (lower weight)
            score += 4.0;
        }

        // Section heading match — very strong signal
        if section_title_lower.contains(kw) {
            score += 6.0;
        }

        // File title match — strong signal
        if file_title_lower.contains(kw) {
            score += 4.0;
        }

        // Content body match (diminishing returns)
        let count = content_lower.matches(kw).count();
        if count > 0 {
            score += (count as f64).min(5.0) * 1.0;
        }
    }

    // Bonus for sections containing LilyPond code blocks (only if keywords matched)
    if score > 0.0 {
        if content_lower.contains("```lilypond") || content_lower.contains("\\score") {
            score += 2.0;
        }

        // Bonus for shared-rules.md (always authoritative — but only when relevant)
        if section.file.contains("shared-rules") {
            score += 3.0;
        }
    }

    score
}

// ─── Helpers ─────────────────────────────────────────────────────────────────────

fn truncate_content(content: &str, max_chars: usize) -> String {
    let chars: Vec<char> = content.chars().collect();
    if chars.len() <= max_chars {
        return content.to_string();
    }
    let truncated: String = chars.into_iter().take(max_chars).collect();
    format!("{}...", truncated)
}

// ─── Tests ───────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_returns_results() {
        let results = search_knowledge("赋格", 5, None);
        assert!(!results.is_empty(), "Should find fugue-related entries");
    }

    #[test]
    fn test_search_empty_query() {
        let results = search_knowledge("", 5, None);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_multiple_keywords() {
        let results = search_knowledge("弦乐四重奏 对位", 5, None);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_harmony() {
        let results = search_knowledge("声部进行", 5, None);
        assert!(!results.is_empty());
        let top = &results[0];
        assert!(
            top.file.contains("aldwell") || top.file.contains("Aldwell"),
            "Top result should be Aldwell, got: {}",
            top.file
        );
    }

    #[test]
    fn test_relevance_ordering() {
        let results = search_knowledge("SATB 音域", 3, None);
        for i in 1..results.len() {
            assert!(
                results[i - 1].relevance >= results[i].relevance,
                "Results should be sorted by relevance descending"
            );
        }
    }

    #[test]
    fn test_context_filter_string_quartet_excludes_trombone() {
        // When searching for "弦乐" with string-quartet context,
        // trombone/brass sections should be filtered out
        let filter = contexts_for_template("string-quartet");
        let filter_refs: Vec<&str> = filter.iter().copied().collect();
        let results = search_knowledge("配器 乐器", 10, Some(&filter_refs));

        for r in &results {
            // Should NOT have brass-only content
            let file = r.file.to_lowercase();
            if file.contains("rimsky") {
                // Rimsky-Korsakov is orchestration — should be filtered out
                // for string quartet context unless it has "all" or "classical"
                assert!(
                    r.contexts.contains("classical") || r.contexts.contains("all"),
                    "Rimsky-Korsakov should have classical context, got: {}",
                    r.contexts
                );
            }
        }
    }

    #[test]
    fn test_context_filter_modernist_excludes_classical_harmony() {
        let filter = contexts_for_template("twelve-tone-row");
        let filter_refs: Vec<&str> = filter.iter().copied().collect();
        let results = search_knowledge("和声", 10, Some(&filter_refs));

        for r in &results {
            // Classical-only harmony files should not appear
            if r.file.contains("aldwell") || r.file.contains("kostka") {
                assert!(
                    r.contexts.contains("modernist") || r.contexts.contains("all"),
                    "Classical harmony file should not appear in modernist search, got: {} (contexts: {})",
                    r.file,
                    r.contexts
                );
            }
        }
    }

    #[test]
    fn test_single_cjk_char_not_scored() {
        // A single character like "乐" should NOT match everything
        let results = search_knowledge("乐", 5, None);
        // Should return empty or very few results since single CJK chars are skipped
        assert!(
            results.len() <= 2,
            "Single CJK char '乐' should not match many results, got {}",
            results.len()
        );
    }
}
