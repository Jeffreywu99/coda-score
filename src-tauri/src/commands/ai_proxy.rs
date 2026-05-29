use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use tauri::Manager;

// ─── Compiled Regexes (OnceLock) ───────────────────────────────────────────────

static RE_THINK: OnceLock<Regex> = OnceLock::new();
fn re_think() -> &'static Regex {
    RE_THINK.get_or_init(|| Regex::new(r"(?s)<think\s*>.*?</think\s*>").unwrap())
}

static RE_THINKING: OnceLock<Regex> = OnceLock::new();
fn re_thinking() -> &'static Regex {
    RE_THINKING.get_or_init(|| Regex::new(r"(?s)<thinking\s*>.*?</thinking\s*>").unwrap())
}

static RE_FENCE_OPEN: OnceLock<Regex> = OnceLock::new();
fn re_fence_open() -> &'static Regex {
    RE_FENCE_OPEN.get_or_init(|| Regex::new(r"(?m)^\s*```\s*(?:lilypond|lilypond\s*)?\s*$").unwrap())
}

static RE_FENCE_CLOSE: OnceLock<Regex> = OnceLock::new();
fn re_fence_close() -> &'static Regex {
    RE_FENCE_CLOSE.get_or_init(|| Regex::new(r"(?m)^\s*```\s*$").unwrap())
}

static RE_FENCE_STRIP_OPEN: OnceLock<Regex> = OnceLock::new();
fn re_fence_strip_open() -> &'static Regex {
    RE_FENCE_STRIP_OPEN.get_or_init(|| Regex::new(r"^\s*```\s*(?:lilypond\s*)?\s*\n?").unwrap())
}

static RE_FENCE_STRIP_CLOSE: OnceLock<Regex> = OnceLock::new();
fn re_fence_strip_close() -> &'static Regex {
    RE_FENCE_STRIP_CLOSE.get_or_init(|| Regex::new(r"\n?\s*```\s*$").unwrap())
}

static RE_NOTE_DURATION: OnceLock<Regex> = OnceLock::new();
fn re_note_duration() -> &'static Regex {
    RE_NOTE_DURATION.get_or_init(|| Regex::new(r"([a-grsA-GRS])\s+([1-9]\d*(?:\.\\.?)?)\\b").unwrap())
}

static RE_ACCIDENTAL: OnceLock<Regex> = OnceLock::new();
fn re_accidental() -> &'static Regex {
    RE_ACCIDENTAL.get_or_init(|| Regex::new(r"([a-gA-G])\s+(is(?:ih)?|es(?:eh)?|isis|eses)\b").unwrap())
}

static RE_TRAILING: OnceLock<Regex> = OnceLock::new();
fn re_trailing() -> &'static Regex {
    RE_TRAILING.get_or_init(|| Regex::new(r"(?m)\n\s*[rRsS]\s*$").unwrap())
}

static RE_DOUBLE_SPACE: OnceLock<Regex> = OnceLock::new();
fn re_double_space() -> &'static Regex {
    RE_DOUBLE_SPACE.get_or_init(|| Regex::new(r"([a-grs0-9'\.,\[\]\(\)~<>\\|])\s{2,}([a-grs0-9'\.,\[\]\(\)~<>\\|])").unwrap())
}

// ─── Shared HTTP Client ──────────────────────────────────────────────────────

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("failed to build HTTP client")
    })
}

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiGenerateRequest {
    pub prompt: String,
    pub context: Option<String>,
    pub knowledge_sections: Option<String>,
    pub previous_code: Option<String>,
    pub previous_errors: Option<String>,
    pub retry_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiGenerateResult {
    pub success: bool,
    pub lilypond_code: String,
    pub error_message: Option<String>,
}

// ─── Review types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRequest {
    pub lilypond_code: String,
    pub knowledge_sections: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResult {
    pub passed: bool,
    pub score: u32,              // 0-100, higher = better
    pub violations: Vec<ReviewViolation>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewViolation {
    pub severity: String,        // "error" | "warning"
    pub rule: String,            // e.g. "禁止平行五度"
    pub location: String,        // e.g. "mm.3-4, 女高音与男低音之间"
    pub description: String,     // detailed description
}

/// AI API configuration, loaded from env vars or a config file.
#[derive(Debug, Clone, Deserialize)]
struct AiConfig {
    api_key: String,
    base_url: String,
    model: String,
}

// ─── Configuration ───────────────────────────────────────────────────────────

const DEFAULT_BASE_URL: &str = "https://api.deepseek.com";
const DEFAULT_MODEL: &str = "deepseek-chat";

/// Load AI configuration.
/// Priority: environment variables > config file in app data dir > defaults.
fn load_ai_config(app_data_dir: Option<&std::path::Path>) -> Option<AiConfig> {
    // 1. Try environment variables (DEEPSEEK_API_KEY takes priority, falls back to OPENAI_API_KEY)
    let env_key = std::env::var("DEEPSEEK_API_KEY")
        .ok()
        .or_else(|| std::env::var("OPENAI_API_KEY").ok());
    if let Some(key) = env_key {
        if !key.is_empty() {
            return Some(AiConfig {
                api_key: key,
                base_url: std::env::var("DEEPSEEK_BASE_URL")
                    .or_else(|_| std::env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
                model: std::env::var("DEEPSEEK_MODEL")
                    .or_else(|_| std::env::var("OPENAI_MODEL"))
                    .unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
            });
        }
    }

    // 2. Try config file
    if let Some(dir) = app_data_dir {
        let config_path = dir.join("ai-config.json");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let api_key = json["apiKey"]
                        .as_str()
                        .or_else(|| json["api_key"].as_str())
                        .unwrap_or("");
                    if !api_key.is_empty() {
                        return Some(AiConfig {
                            api_key: api_key.to_string(),
                            base_url: json["baseUrl"]
                                .as_str()
                                .or_else(|| json["base_url"].as_str())
                                .unwrap_or(DEFAULT_BASE_URL)
                                .to_string(),
                            model: json["model"]
                                .as_str()
                                .unwrap_or(DEFAULT_MODEL)
                                .to_string(),
                        });
                    }
                }
            }
        }
    }

    None
}

// ─── Unified Prompt (v0.5.0) ─────────────────────────────────────────────────

/// Build the unified system prompt for single-call generation.
///
/// v0.5.0 design: the model handles music planning internally and outputs
/// LilyPond code directly. This replaces the old two-stage pipeline
/// (planning + translation) with a single, concise prompt (~1500 tokens).
fn build_unified_system_prompt(knowledge_sections: Option<&str>) -> String {
    let mut prompt = String::from(
        "你是 LilyPond 乐谱代码生成器。用户描述音乐需求，你直接输出完整可编译的 LilyPond 代码。\n\n\
         ## 输出格式（严格遵守）\n\n\
         用 ```lilypond ... ``` 代码块包裹你的代码。代码块之外不写任何解释。\n\n\
         ## LilyPond 音符命名规则（默认荷兰语命名，不需要 \\language 声明）\n\n\
         - 基本音：c d e f g a b\n\
         - 升号加 is：cis dis fis gis ais（升C/D/F/G/A）\n\
         - 降号加 es：des es fes ges as bes（降D/E/F/G/A/B）\n\
         - 注意特殊的：E♭=es（非ees），A♭=as（非aes），B♭=bes\n\
         - 重升加 isis：cisis disis fisis gisis\n\
         - 重降加 eses：deses eses feses geses\n\
         - 还原号不加后缀，直接用基本音名\n\n\
         ## 代码骨架模板\n\n\
         ```lilypond\n\
         \\version \"2.24.0\"\n\n\
         \\score {\n\
           <<\n\
             \\new Staff {\n\
               \\clef treble\n\
               \\relative c'' {\n\
                 % 旋律在这里\n\
               }\n\
             }\n\
           >>\n\
           \\layout {}\n\
         }\n\
         ```\n\n\
         ## 强制规则\n\n\
         1. 第一行必须是 \\version \"2.24.0\"\n\
         2. 不要写 \\language 声明，使用 LilyPond 默认荷兰语命名\n\
         3. 必须有 \\score { ... \\layout {} }，\\layout {} 在 \\score 内\n\
         4. 钢琴谱使用两个 Staff：\\new Staff = \"right\" 和 \\new Staff = \"left\"，用 << >> 包裹\n\
         5. 每个 \\new Staff 必须包含 \\clef 和 \\relative\n\
         6. 花括号 { } 和尖括号 << >> 必须严格配对\n\
         7. 每个音符必须带时值：c4 d8 e16 fis4 — 不能有没带时值的裸音符\n\
         8. 小节线用 | 分隔，每小节时值加起来必须等于拍号指定的总时值\n\
         9. 禁止用块和弦 <c e g>1 凑数，每个声部必须写独立的旋律线\n\n\
         ## 钢琴谱正确模板\n\n\
         ```lilypond\n\
         \\version \"2.24.0\"\n\n\
         \\score {\n\
           <<\n\
             \\new Staff = \"right\" {\n\
               \\clef treble\n\
               \\relative c'' {\n\
                 \\key c \\major\n\
                 \\time 3/4\n\
                 e4.( dis8 e4)\n\
                 g2.\n\
               }\n\
             }\n\
             \\new Staff = \"left\" {\n\
               \\clef bass\n\
               \\relative c {\n\
                 \\key c \\major\n\
                 \\time 3/4\n\
                 <c e g>2.\n\
                 <b d g>2.\n\
               }\n\
             }\n\
           >>\n\
           \\layout {}\n\
         }\n\
         ```\n\n\
         ## 音区参考\n\n\
         - 钢琴右手/小提琴/女高音：\\relative c''\n\
         - 中提琴/女中音：\\relative c'\n\
         - 钢琴左手/大提琴/男低音：\\relative c + \\clef bass\n",
    );

    if let Some(knowledge) = knowledge_sections {
        if !knowledge.is_empty() {
            prompt.push_str("\n## 作曲知识参考\n\n");
            prompt.push_str(knowledge);
            prompt.push('\n');
        }
    }

    prompt
}

/// Build the user message for single-call generation.
fn build_unified_user_message(request: &AiGenerateRequest) -> String {
    let mut message = String::new();

    // Retry context
    if request.retry_count.unwrap_or(0) > 0 {
        if let Some(ref code) = request.previous_code {
            message.push_str("上次生成的代码编译失败，代码如下：\n```lilypond\n");
            message.push_str(code);
            message.push_str("\n```\n\n");
        }
        if let Some(ref errors) = request.previous_errors {
            message.push_str("错误信息：\n");
            message.push_str(errors);
            message.push_str("\n\n请修复上述错误，重新生成完整的LilyPond代码。\n\n");
        }
    }

    // Template / context — emphasize structure preservation
    if let Some(ref ctx) = request.context {
        if !ctx.is_empty() {
            message.push_str("【模板结构 — 你必须保留以下模板的 \\score 骨架、声部数量、谱号配置，只填充音乐内容】\n");
            message.push_str(ctx);
            message.push_str("\n\n");
        }
    }

    // User's natural language request
    message.push_str(&format!("请根据以下描述生成LilyPond代码：\n{}", request.prompt));

    message
}

// ─── Thinking Tag Stripping ────────────────────────────────────────────────────

/// Remove thinking/reasoning tags that some models (DeepSeek V4 Flash, etc.)
/// may include in the `content` field. Handles both `<think>...</think>` and
/// `<thinking>...</thinking>` patterns, including multi-line content.
fn strip_thinking_tags(text: &str) -> String {
    let mut cleaned = text.to_string();
    cleaned = re_think().replace_all(&cleaned, "").to_string();
    cleaned = re_thinking().replace_all(&cleaned, "").to_string();
    cleaned.trim().to_string()
}

// ─── Code Extraction ─────────────────────────────────────────────────────────

/// Extract LilyPond code from AI response, handling fenced code blocks.
fn extract_lilypond_code(response: &str) -> String {
    // Strategy: find the first line that looks like the start of a LilyPond file,
    // then collect everything until the end or a closing fence.
    let text = response.trim();

    // Try to find a ``` fence open and extract its content
    let code = if let Some(open_match) = re_fence_open().find(text) {
        let start = open_match.end();
        let after_open = &text[start..];
        if let Some(close_match) = re_fence_close().find(after_open) {
            let code = after_open[..close_match.start()].trim().to_string();
            if !code.is_empty() && (code.contains('\\') || code.contains('{')) {
                code
            } else {
                // No closing fence — use everything after the opening fence
                after_open.trim().to_string()
            }
        } else {
            after_open.trim().to_string()
        }
    } else {
        // Fallback: try to strip leading/trailing ``` fences from the whole text
        let mut cleaned = text.to_string();
        cleaned = re_fence_strip_open()
            .replace(&cleaned, "").to_string();
        cleaned = re_fence_strip_close()
            .replace(&cleaned, "").to_string();
        let cleaned = cleaned.trim().to_string();

        if !cleaned.is_empty() && (cleaned.contains('\\') || cleaned.contains('{')) {
            cleaned
        } else {
            text.to_string()
        }
    };

    // Post-process: fix common LLM syntax errors
    fix_lilypond_syntax(&code)
}

/// Fix common syntax errors that LLMs produce in LilyPond code.
/// This runs AFTER extraction and BEFORE compilation.
fn fix_lilypond_syntax(code: &str) -> String {
    log::debug!("[fix_lilypond_syntax] Input length: {} chars", code.len());
    let mut fixed = code.to_string();

    // 1. Fix spaces between note/rest names and durations: "r 8" → "r8", "c 4" → "c4"
    fixed = re_note_duration().replace_all(&fixed, "$1$2").to_string();

    // 2. Fix spaces between note and accidental: "c is" → "cis", "e es" → "ees"
    fixed = re_accidental().replace_all(&fixed, "$1$2").to_string();

    // 3. Fix orphaned trailing rests or notes at end of file (incomplete expressions)
    fixed = re_trailing().replace_all(&fixed, "").to_string();

    // 4. Fix double spaces in music expressions
    fixed = re_double_space().replace_all(&fixed, "$1 $2").to_string();

    // 5. Fix unmatched braces: add missing closing braces
    let open_count = fixed.matches('{').count();
    let close_count = fixed.matches('}').count();
    if open_count > close_count {
        let missing = open_count - close_count;
        for _ in 0..missing {
            fixed.push_str("\n}");
        }
    }

    // 6. Fix unmatched << >>: add missing >>
    let open_angle = fixed.matches("<<").count();
    let close_angle = fixed.matches(">>").count();
    if open_angle > close_angle {
        let missing = open_angle - close_angle;
        for _ in 0..missing {
            fixed.push_str("\n>>");
        }
    }

    // 7. Strip template placeholders
    fixed = crate::lilypond::compiler::strip_template_placeholders(&fixed);

    // 8. Structural safety: ensure the code is compilable
    fixed = ensure_lilypond_structure(&fixed);

    fixed
}

/// Ensure the LilyPond code has the minimum required structure for compilation.
/// Wraps bare music expressions and adds missing boilerplate.
fn ensure_lilypond_structure(code: &str) -> String {
    let trimmed = code.trim();

    // If already has \score, just ensure it has \layout
    if trimmed.contains("\\score") {
        let mut result = trimmed.to_string();
        // Add \layout {} inside \score if missing
        if !result.contains("\\layout") {
            // Insert \layout {} before the last } of \score
            if let Some(pos) = result.rfind('}') {
                result.insert_str(pos, "  \\layout {}\n");
            }
        }
        // Ensure \version exists
        if !result.contains("\\version") {
            result.insert_str(0, "\\version \"2.24.0\"\n");
        }
        return result;
    }

    // No \score found — wrap bare music content in a \score structure
    log::warn!("[ensure_lilypond_structure] No \\score found, wrapping bare content");

    // Determine clef and octave from content heuristics
    let uses_bass = trimmed.to_lowercase().contains("\\clef bass")
        || trimmed.to_lowercase().contains("bass");
    let clef = if uses_bass { "bass" } else { "treble" };
    let relative_octave = if uses_bass { "c" } else { "c''" };

    format!(
        "\\version \"2.24.0\"\n\n\\score {{\n  <<\n    \\new Staff {{\n      \\clef {}\n      \\relative {} {{\n{}\n      }}\n    }}\n  >>\n  \\layout {{}}\n}}",
        clef, relative_octave, trimmed
    )
}

// ─── LLM Call Helper ─────────────────────────────────────────────────────────

/// Unified LLM call helper. Uses **streaming** to ensure content is captured
/// even for thinking models where non-streaming `content` is often empty.
async fn call_llm(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_message: &str,
    temperature: f64,
    max_tokens: u32,
) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_message }
        ],
        "temperature": temperature,
        "max_tokens": max_tokens,
        "stream": true
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(std::time::Duration::from_secs(300))
        .send()
        .await
        .map_err(|e| format!("AI API 请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "无法读取错误信息".to_string());
        return Err(format!("AI API 返回错误 ({}): {}", status, error_body));
    }

    // Parse SSE stream: collect content from deltas
    let mut content_text = String::new();

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                log::warn!("[call_llm] stream chunk error (continuing): {}", e);
                continue;
            }
        };
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        // Process complete lines from buffer
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();

            if !line.starts_with("data: ") {
                continue;
            }
            let json_str = line[6..].trim();
            if json_str == "[DONE]" {
                continue;
            }

            if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(delta) = data
                    .get("choices")
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|choice| choice.get("delta"))
                {
                    if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                        content_text.push_str(content);
                    }
                }
            }
        }
    }

    log::debug!(
        "[call_llm] stream complete. content: {} chars",
        content_text.len(),
    );

    // Strip thinking tags as a defensive measure
    let content_text = strip_thinking_tags(&content_text);

    if !content_text.trim().is_empty() {
        log::debug!("[call_llm] using content from stream ({} chars)", content_text.len());
        Ok(content_text)
    } else {
        Err("AI 响应为空。建议检查 API 配置或简化请求。".to_string())
    }
}

// ─── Commands ────────────────────────────────────────────────────────────────

/// Generate LilyPond code from a natural language prompt via single AI call.
///
/// v0.5.0: Single-stage pipeline — the model handles music planning internally
/// and outputs LilyPond code directly. This replaces the old two-stage pipeline.
#[tauri::command]
pub async fn generate_lilypond_code(
    app: tauri::AppHandle,
    request: AiGenerateRequest,
) -> Result<AiGenerateResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法获取应用数据目录".to_string())?;

    let config = match load_ai_config(Some(&app_data)) {
        Some(c) => c,
        None => {
            return Ok(AiGenerateResult {
                success: false,
                lilypond_code: String::new(),
                error_message: Some(
                    "未配置 AI API Key。请设置环境变量 DEEPSEEK_API_KEY 或在设置中配置。".to_string(),
                ),
            });
        }
    };

    let url = format!(
        "{}/chat/completions",
        config.base_url.trim_end_matches('/')
    );

    // ── Single-stage generation ──
    log::info!("[AI] Generating LilyPond code (single-stage)...");

    let system_prompt = build_unified_system_prompt(request.knowledge_sections.as_deref());
    let user_message = build_unified_user_message(&request);

    let code_content = match call_llm(
        http_client(),
        &url,
        &config.api_key,
        &config.model,
        &system_prompt,
        &user_message,
        0.3,
        16384,
    )
    .await
    {
        Ok(content) => content,
        Err(e) => {
            return Ok(AiGenerateResult {
                success: false,
                lilypond_code: String::new(),
                error_message: Some(format!("代码生成失败: {}", e)),
            });
        }
    };

    let lilypond_code = extract_lilypond_code(&code_content);

    log::info!(
        "[AI] Generation complete. Code length: {} chars",
        lilypond_code.len()
    );

    Ok(AiGenerateResult {
        success: true,
        lilypond_code,
        error_message: None,
    })
}

/// Review generated LilyPond code against music theory rules.
/// Returns structured violation list with severity and locations.
///
/// v0.5.0: Advisory only — frontend shows warnings but does NOT auto-regenerate.
#[tauri::command]
pub async fn review_lilypond_code(
    app: tauri::AppHandle,
    request: ReviewRequest,
) -> Result<ReviewResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法获取应用数据目录".to_string())?;

    let config = match load_ai_config(Some(&app_data)) {
        Some(c) => c,
        None => {
            return Ok(ReviewResult {
                passed: true, // can't review without config — pass through
                score: 100,
                violations: vec![],
                summary: "未配置 AI — 跳过审查".to_string(),
            });
        }
    };

    let system_prompt = build_review_system_prompt(request.knowledge_sections.as_deref());
    let user_message = format!(
        "请审查以下 LilyPond 代码是否符合作曲规则：\n\n```lilypond\n{}\n```",
        request.lilypond_code
    );

    let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));

    let content = match call_llm(
        http_client(),
        &url,
        &config.api_key,
        &config.model,
        &system_prompt,
        &user_message,
        0.1,
        8192,
    )
    .await
    {
        Ok(c) => c,
        Err(_) => {
            return Ok(ReviewResult {
                passed: true, // API error — pass through to avoid blocking
                score: 100,
                violations: vec![],
                summary: "审查 API 不可用 — 跳过".to_string(),
            });
        }
    };

    parse_review_response(&content)
}

fn build_review_system_prompt(knowledge_sections: Option<&str>) -> String {
    let mut prompt = String::from(
        "你是乐谱审查专家，专精古典对位法和四部和声规则。\n\
         请审查用户提供的 LilyPond 代码，找出所有违反作曲规则的错误。\n\n\
         检查项：\n\
         1. 平行五度/八度（任何两声部之间）\n\
         2. 隐伏五度/八度（外声部同向进入纯五/八度且高音跳进）\n\
         3. 声部音域超出范围（S=c'-g'', A=g-d'', T=c-g', B=e,-c'）\n\
         4. 声部交错/超越\n\
         5. 重复音规则（导音不可重复、七音不可重复）\n\
         6. 倾向音解决（导音→上行级进、七音→下行级进）\n\
         7. 声部间距过大（S-A和A-T超过八度）\n\
         8. 终止式正确性\n\n\
         返回 JSON 格式（只用JSON，不要其他文字）：\n\
         {\n\
           \"passed\": true/false,\n\
           \"score\": 0-100,\n\
           \"violations\": [\n\
             {\"severity\": \"error|warning\", \"rule\": \"规则名\", \"location\": \"位置\", \"description\": \"详细描述\"}\n\
           ],\n\
           \"summary\": \"一句话总结\"\n\
         }",
    );

    if let Some(knowledge) = knowledge_sections {
        if !knowledge.is_empty() {
            prompt.push_str(&format!(
                "\n\n## 参考规则\n{}\n请在上述规则的约束下审查代码。",
                knowledge
            ));
        }
    }

    prompt
}

fn parse_review_response(content: &str) -> Result<ReviewResult, String> {
    // Extract JSON from response (may be wrapped in ``` fences)
    let json_str = if let Some(start) = content.find('{') {
        let end = content.rfind('}').unwrap_or(content.len());
        &content[start..=end]
    } else {
        content
    };

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
        return Ok(ReviewResult {
            passed: json["passed"].as_bool().unwrap_or(true),
            score: json["score"].as_u64().unwrap_or(100) as u32,
            violations: json["violations"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| ReviewViolation {
                            severity: v["severity"].as_str().unwrap_or("warning").to_string(),
                            rule: v["rule"].as_str().unwrap_or("").to_string(),
                            location: v["location"].as_str().unwrap_or("").to_string(),
                            description: v["description"].as_str().unwrap_or("").to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            summary: json["summary"].as_str().unwrap_or("").to_string(),
        });
    }

    // Fallback: JSON parse failed — return failed review with raw content
    log::warn!(
        "[parse_review_response] JSON parse failed for: {}",
        &content[..content.len().min(200)]
    );
    Ok(ReviewResult {
        passed: false,
        score: 0,
        violations: vec![],
        summary: format!("⚠️ 审查解析失败: {}", content),
    })
}

/// Check whether an AI API key is configured (does not test the connection).
#[tauri::command]
pub async fn check_ai_available(app: tauri::AppHandle) -> Result<bool, String> {
    let app_data = app.path().app_data_dir().ok();
    Ok(load_ai_config(app_data.as_deref()).is_some())
}

/// AI config for frontend display (API key masked).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfigDisplay {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub source: String, // "env" | "file" | "none"
}

/// Load current AI config for display in settings UI.
/// Always returns a masked key for security. The full key is kept internally for API calls.
#[tauri::command]
pub async fn get_ai_config(app: tauri::AppHandle) -> Result<AiConfigDisplay, String> {
    // Check env vars first
    let env_key = std::env::var("DEEPSEEK_API_KEY")
        .ok()
        .or_else(|| std::env::var("OPENAI_API_KEY").ok());
    if let Some(key) = env_key {
        if !key.is_empty() {
            return Ok(AiConfigDisplay {
                api_key: mask_key(&key),
                base_url: std::env::var("DEEPSEEK_BASE_URL")
                    .or_else(|_| std::env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
                model: std::env::var("DEEPSEEK_MODEL")
                    .or_else(|_| std::env::var("OPENAI_MODEL"))
                    .unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
                source: "env".to_string(),
            });
        }
    }

    // Check config file — return masked key for security
    let app_data = app.path().app_data_dir().ok();
    if let Some(dir) = app_data {
        let config_path = dir.join("ai-config.json");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let api_key = json["apiKey"]
                        .as_str()
                        .or_else(|| json["api_key"].as_str())
                        .unwrap_or("");
                    if !api_key.is_empty() {
                        return Ok(AiConfigDisplay {
                            api_key: mask_key(api_key),
                            base_url: json["baseUrl"]
                                .as_str()
                                .or_else(|| json["base_url"].as_str())
                                .unwrap_or(DEFAULT_BASE_URL)
                                .to_string(),
                            model: json["model"]
                                .as_str()
                                .unwrap_or(DEFAULT_MODEL)
                                .to_string(),
                            source: "file".to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(AiConfigDisplay {
        api_key: String::new(),
        base_url: DEFAULT_BASE_URL.to_string(),
        model: DEFAULT_MODEL.to_string(),
        source: "none".to_string(),
    })
}

/// Save AI configuration to the app data directory.
#[tauri::command]
pub async fn save_ai_config(
    app: tauri::AppHandle,
    api_key: String,
    base_url: String,
    model: String,
) -> Result<(), String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法获取应用数据目录".to_string())?;

    // Ensure the directory exists
    std::fs::create_dir_all(&app_data)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let config_path = app_data.join("ai-config.json");
    let json = serde_json::json!({
        "apiKey": api_key,
        "baseUrl": base_url,
        "model": model,
    });

    std::fs::write(
        &config_path,
        serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("写入配置文件失败: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600)).ok();
    }

    Ok(())
}

/// Test the AI API connection with current or provided config.
#[tauri::command]
pub async fn test_ai_connection(app: tauri::AppHandle) -> Result<String, String> {
    let app_data = app.path().app_data_dir().ok();
    let config = load_ai_config(app_data.as_deref())
        .ok_or_else(|| "未配置 API Key".to_string())?;

    let body = serde_json::json!({
        "model": config.model,
        "messages": [
            { "role": "user", "content": "Hello, respond with just 'ok'." }
        ],
        "max_tokens": 5
    });

    let url = format!(
        "{}/chat/completions",
        config.base_url.trim_end_matches('/')
    );

    let response = http_client()
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("连接失败: {}", e))?;

    if response.status().is_success() {
        Ok("连接成功".to_string())
    } else {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "未知错误".to_string());
        Err(format!("API 返回错误 ({}): {}", status, text))
    }
}

/// Mask an API key for display: show first 3 and last 4 characters.
fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "*".repeat(key.len());
    }
    let prefix = &key[..3];
    let suffix = &key[key.len() - 4..];
    format!("{}{}{}", prefix, "*".repeat(key.len() - 7), suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_lilypond_code_fenced() {
        let response = "Here is the code:\n```lilypond\n\\version \"2.24.0\"\n{ c4 d e f }\n```\nDone!";
        let code = extract_lilypond_code(response);
        assert!(code.contains("\\version"));
        assert!(code.contains("c4 d e f"));
        assert!(!code.contains("```"));
    }

    #[test]
    fn test_extract_lilypond_code_generic_fence() {
        let response = "```\n\\version \"2.24.0\"\n{ c4 d e f }\n```";
        let code = extract_lilypond_code(response);
        assert!(code.contains("\\version"));
    }

    #[test]
    fn test_extract_lilypond_code_raw() {
        let response = "\\version \"2.24.0\"\n{ c4 d e f }";
        let code = extract_lilypond_code(response);
        assert!(code.contains("\\version"));
    }

    #[test]
    fn test_build_unified_prompt_no_knowledge() {
        let prompt = build_unified_system_prompt(None);
        assert!(prompt.contains("LilyPond 乐谱代码生成器"));
        assert!(!prompt.contains("作曲知识参考"));
    }

    #[test]
    fn test_build_unified_prompt_with_knowledge() {
        let prompt = build_unified_system_prompt(Some("【声部进行】禁止平行五度"));
        assert!(prompt.contains("作曲知识参考"));
        assert!(prompt.contains("平行五度"));
    }

    #[test]
    fn test_build_unified_user_message_retry() {
        let req = AiGenerateRequest {
            knowledge_sections: None,
            prompt: "a melody".to_string(),
            context: None,
            previous_code: Some("bad code".to_string()),
            previous_errors: Some("error on line 3".to_string()),
            retry_count: Some(1),
        };
        let msg = build_unified_user_message(&req);
        assert!(msg.contains("编译失败"));
        assert!(msg.contains("bad code"));
        assert!(msg.contains("error on line 3"));
    }

    #[test]
    fn test_build_unified_user_message_first_try() {
        let req = AiGenerateRequest {
            knowledge_sections: None,
            prompt: "a simple melody".to_string(),
            context: None,
            previous_code: None,
            previous_errors: None,
            retry_count: None,
        };
        let msg = build_unified_user_message(&req);
        assert!(msg.contains("a simple melody"));
        assert!(!msg.contains("编译失败"));
    }

    #[test]
    fn test_mask_key() {
        // "sk-abcdef1234567890" has 19 chars: prefix "sk-" (3) + 12 stars + suffix "7890" (4)
        assert_eq!(mask_key("sk-abcdef1234567890"), "sk-************7890");
        // Short key
        assert_eq!(mask_key("short"), "*****");
        // Exactly 8 chars
        assert_eq!(mask_key("12345678"), "********");
        // 9 chars (just over threshold): prefix "123" + 2 stars + suffix "6789"
        assert_eq!(mask_key("123456789"), "123**6789");
    }

    #[test]
    fn test_fix_unmatched_braces() {
        // Missing closing brace
        let code = r#"\score { \new Staff { c4 d e f }"#;
        let fixed = fix_lilypond_syntax(code);
        let open = fixed.matches('{').count();
        let close = fixed.matches('}').count();
        assert_eq!(open, close, "Braces should be balanced: open={}, close={}", open, close);
    }

    #[test]
    fn test_fix_unmatched_angle_brackets() {
        // Missing >>
        let code = r#"\score { << { c4 d } { e4 f } }"#;
        let fixed = fix_lilypond_syntax(code);
        let open = fixed.matches("<<").count();
        let close = fixed.matches(">>").count();
        assert_eq!(open, close, "Angle brackets should be balanced: open={}, close={}", open, close);
    }

    #[test]
    fn test_no_false_positive_on_commands() {
        // "\relative c'" should NOT be modified — the space after \relative is intentional
        let code = r#"\relative c' { c4 d e f }"#;
        let fixed = fix_lilypond_syntax(code);
        assert!(fixed.contains("\\relative c'"), "Should preserve \\relative c': {}", fixed);
    }

    #[test]
    fn test_fix_strip_template_placeholders() {
        // Lines containing {{...}} placeholders should be entirely removed
        let code = r#"\version "2.24.0"
\header {
  title = {{TITLE}}
}
\score {
  \new Staff {
    \clef treble
    \time {{TIME}}
    {{TEMPO}}
    \relative c'' {
      c4 d e f
      {{UPPER}}
    }
  }
}"#;
        let fixed = fix_lilypond_syntax(code);
        assert!(!fixed.contains("{{TITLE}}"), "Should strip {{TITLE}} line: {}", fixed);
        assert!(!fixed.contains("{{TIME}}"), "Should strip {{TIME}} line: {}", fixed);
        assert!(!fixed.contains("{{TEMPO}}"), "Should strip {{TEMPO}} line: {}", fixed);
        assert!(!fixed.contains("{{UPPER}}"), "Should strip {{UPPER}} line: {}", fixed);
        // Valid content should remain
        assert!(fixed.contains("\\version"), "Should preserve \\version: {}", fixed);
        assert!(fixed.contains("c4 d e f"), "Should preserve music: {}", fixed);
    }

    #[test]
    fn test_fix_strip_placeholders_with_spaces() {
        // Regression test: {{ TIME}} (with space before closing braces) must also be stripped
        let code = "  \\time {{ TIME}}\n  c4 d e f";
        let fixed = fix_lilypond_syntax(code);
        assert!(!fixed.contains("{{ TIME}}"), "Should strip '{{ TIME}}': {}", fixed);
        assert!(fixed.contains("c4 d e f"), "Should preserve music: {}", fixed);
    }
}
