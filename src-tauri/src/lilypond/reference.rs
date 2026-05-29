use regex::Regex;
use serde::Serialize;
use std::sync::OnceLock;

/// Reference document embedded at compile time.
/// The file must exist at `src-tauri/lilypond-complete-reference.md`.
const EMBEDDED_REFERENCE: &str = include_str!("../../lilypond-complete-reference.md");

/// Cache for parsed sections — parsed once, reused for every search.
static PARSED_SECTIONS: OnceLock<Vec<ReferenceSection>> = OnceLock::new();

// ─── Public Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceSection {
    pub title: String,
    pub content: String,
    pub relevance: f64,
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Search the LilyPond reference document for sections matching the given keywords.
///
/// Returns up to `max_sections` results, sorted by relevance (descending).
pub fn search(keywords: &[String], max_sections: usize) -> Vec<ReferenceSection> {
    let sections = PARSED_SECTIONS.get_or_init(|| parse_reference(EMBEDDED_REFERENCE));

    if keywords.is_empty() {
        return sections.iter().take(max_sections).cloned().collect();
    }

    let keywords_lower: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();

    let mut scored: Vec<ReferenceSection> = sections
        .iter()
        .filter_map(|section| {
            let score = calculate_relevance(section, &keywords_lower);
            if score > 0.0 {
                Some(ReferenceSection {
                    title: section.title.clone(),
                    content: section.content.clone(),
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
    scored.truncate(max_sections);
    scored
}

// ─── Document Parsing ────────────────────────────────────────────────────────

/// Parse the markdown reference document into sections.
/// Splits on `##` (level-2) and `###` (level-3) headings.
fn parse_reference(content: &str) -> Vec<ReferenceSection> {
    let mut sections = Vec::new();
    let mut current_title = String::new();
    let mut current_content = String::new();

    for line in content.lines() {
        if line.starts_with("## ") {
            // Flush previous section
            if !current_title.is_empty() {
                sections.push(ReferenceSection {
                    title: current_title.clone(),
                    content: current_content.trim().to_string(),
                    relevance: 0.0,
                });
            }
            current_title = line.trim_start_matches("## ").trim().to_string();
            current_content = String::new();
        } else if line.starts_with("### ") {
            // Flush previous section
            if !current_title.is_empty() {
                sections.push(ReferenceSection {
                    title: current_title.clone(),
                    content: current_content.trim().to_string(),
                    relevance: 0.0,
                });
            }
            current_title = line.trim_start_matches("### ").trim().to_string();
            current_content = String::new();
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Flush final section
    if !current_title.is_empty() {
        sections.push(ReferenceSection {
            title: current_title,
            content: current_content.trim().to_string(),
            relevance: 0.0,
        });
    }

    sections
}

// ─── Relevance Scoring ──────────────────────────────────────────────────────

/// Calculate how relevant a section is to the given keywords.
/// Title matches are weighted 5x, code block matches 2x, body matches 1x.
fn calculate_relevance(section: &ReferenceSection, keywords: &[String]) -> f64 {
    let mut score = 0.0;
    let title_lower = section.title.to_lowercase();
    let content_lower = section.content.to_lowercase();

    for keyword in keywords {
        let kw = keyword.trim();
        if kw.is_empty() {
            continue;
        }
        let kw_lower = kw.to_lowercase();

        // Title match (highest value)
        if title_lower.contains(&kw_lower) {
            score += 5.0;
        }

        // Exact keyword match in content
        if content_lower.contains(&kw_lower) {
            score += 1.0;

            // Bonus for matching LilyPond command syntax (e.g. \relative)
            let cmd_pattern = format!("\\{}", kw_lower);
            if content_lower.contains(&cmd_pattern) {
                score += 2.0;
            }

            // Bonus for matches inside code blocks
            let code_block_re = Regex::new(r"```lilypond\s*([\s\S]*?)```").unwrap();
            for cap in code_block_re.captures_iter(&section.content) {
                if let Some(code) = cap.get(1) {
                    if code.as_str().to_lowercase().contains(&kw_lower) {
                        score += 2.0;
                        break;
                    }
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference_basic() {
        let content = "## Section One\nSome content\n### Subsection\nMore content\n## Section Two\nFinal";
        let sections = parse_reference(content);
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].title, "Section One");
        assert_eq!(sections[1].title, "Subsection");
        assert_eq!(sections[2].title, "Section Two");
    }

    #[test]
    fn test_search_empty_keywords() {
        let sections = search(&[], 5);
        assert!(!sections.is_empty());
    }

    #[test]
    fn test_calculate_relevance_title_match() {
        let section = ReferenceSection {
            title: "音高输入".to_string(),
            content: "Some content about pitch".to_string(),
            relevance: 0.0,
        };
        let score = calculate_relevance(&section, &["音高".to_string()]);
        assert!(score >= 5.0);
    }

    #[test]
    fn test_calculate_relevance_no_match() {
        let section = ReferenceSection {
            title: "Something else".to_string(),
            content: "Nothing relevant here".to_string(),
            relevance: 0.0,
        };
        let score = calculate_relevance(&section, &["microtone".to_string()]);
        assert_eq!(score, 0.0);
    }
}
