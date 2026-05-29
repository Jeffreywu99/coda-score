use base64::Engine;
use regex::Regex;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use uuid::Uuid;

/// Timeout for LilyPond compilation (seconds)
const COMPILE_TIMEOUT_SECS: u64 = 60;

/// Cache for LilyPond version (fetched once, reused across templates)
static LILYPOND_VERSION: OnceLock<String> = OnceLock::new();

/// RAII guard for temporary compilation directories.
/// Automatically removes the directory on drop.
struct TempDir {
    path: PathBuf,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

// ─── Public Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: String, // "error" | "warning"
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LilyPondCompileResult {
    pub success: bool,
    pub output_format: String,
    pub output_base64: String,
    pub output_path: String,
    pub errors: Vec<CompileError>,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LilyPondInfo {
    pub available: bool,
    pub version: String,
    pub path: String,
}

// ─── LilyPond Binary Discovery ───────────────────────────────────────────────

/// Find the LilyPond executable path.
/// Priority: LILYPOND_PATH env var > platform default.
pub fn find_lilypond() -> PathBuf {
    // 1. Environment variable override
    if let Ok(path) = std::env::var("LILYPOND_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return p;
        }
    }

    // 2. Platform-specific defaults
    #[cfg(target_os = "windows")]
    {
        let candidates = [
            PathBuf::from(r"C:\Program Files\LilyPond\bin\lilypond.exe"),
            PathBuf::from(r"C:\Program Files\LilyPond\lilypond-2.26.0\bin\lilypond.exe"),
            PathBuf::from(r"C:\Program Files (x86)\LilyPond\bin\lilypond.exe"),
        ];

        for c in &candidates {
            if c.exists() {
                return c.clone();
            }
        }

        // Scan for any lilypond-X.Y.Z directory
        let base = PathBuf::from(r"C:\Program Files\LilyPond");
        if base.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("lilypond-") && entry.path().is_dir() {
                        let exe = entry.path().join("bin").join("lilypond.exe");
                        if exe.exists() {
                            return exe;
                        }
                    }
                }
            }
        }

        // Fallback: hope it's in PATH
        return PathBuf::from("lilypond");
    }

    #[cfg(target_os = "macos")]
    {
        // Homebrew (Apple Silicon)
        let brew_arm = PathBuf::from("/opt/homebrew/bin/lilypond");
        if brew_arm.exists() {
            return brew_arm;
        }
        // Homebrew (Intel)
        let brew_intel = PathBuf::from("/usr/local/bin/lilypond");
        if brew_intel.exists() {
            return brew_intel;
        }
        // Official .dmg installer
        let app_bundle = PathBuf::from("/Applications/LilyPond.app/Contents/Resources/bin/lilypond");
        if app_bundle.exists() {
            return app_bundle;
        }
    }

    #[cfg(target_os = "linux")]
    {
        return PathBuf::from("/usr/bin/lilypond");
    }

    // Fallback: hope it's in PATH (reachable on macOS when Homebrew is absent)
    #[cfg(target_os = "macos")]
    PathBuf::from("lilypond")
}

// ─── Safety Checks ──────────────────────────────────────────────────────────

/// Validate LilyPond code for safety.
/// Currently forbids `\include` directives to prevent filesystem access.
pub fn check_safety(code: &str) -> Result<(), String> {
    let re = Regex::new(r"(?m)^\s*\\include\s").map_err(|e| e.to_string())?;
    if re.is_match(code) {
        return Err(
            "安全检查失败：禁止使用 \\include 命令。仅允许纯内联 LilyPond 代码。".to_string(),
        );
    }
    Ok(())
}

// ─── Compilation ─────────────────────────────────────────────────────────────

/// Compile LilyPond code into the specified output format.
///
/// Creates a temporary directory under `{app_data}/lilypond-tmp/{uuid}/`,
/// writes the `.ly` source, invokes the LilyPond CLI, reads the output,
/// and cleans up (RAII).
pub async fn compile(
    code: &str,
    output_format: &str,
    app_data_dir: &std::path::Path,
) -> Result<LilyPondCompileResult, String> {
    let start = Instant::now();

    // 1. Safety check
    check_safety(code)?;

    // 2. Create temp directory with RAII guard
    let session_id = Uuid::new_v4().to_string();
    let tmp_dir = app_data_dir.join("lilypond-tmp").join(&session_id);
    tokio::fs::create_dir_all(&tmp_dir)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;
    let _temp_guard = TempDir {
        path: tmp_dir.clone(),
    };

    // 3. Write .ly source file
    // Preprocess: strip any {{...}} template placeholders (entire lines containing them)
    let code = strip_template_placeholders(code);

    let source_path = tmp_dir.join("score.ly");
    let mut file = tokio::fs::File::create(&source_path)
        .await
        .map_err(|e| format!("创建临时文件失败: {}", e))?;
    file.write_all(code.as_bytes())
        .await
        .map_err(|e| format!("写入临时文件失败: {}", e))?;
    file.flush().await.map_err(|e| e.to_string())?;
    drop(file);

    // 4. Build LilyPond command
    let lilypond_path = find_lilypond();
    let format_lower = output_format.to_lowercase();

    let mut cmd = Command::new(&lilypond_path);
    match format_lower.as_str() {
        "png" => {
            cmd.arg("--png");
        }
        "svg" => {
            cmd.arg("-dbackend=svg");
        }
        "pdf" => { /* PDF is the default output */ }
        "midi" => {
            cmd.arg("--pdf"); // Also generate PDF alongside MIDI
        }
        _ => return Err(format!("不支持的输出格式: {}", output_format)),
    }
    cmd.arg(format!("--output={}", tmp_dir.to_string_lossy()));
    cmd.arg(source_path.to_string_lossy().to_string());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // 5. Spawn process with timeout
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("无法启动 LilyPond 进程: {}。请确认已安装 LilyPond。", e))?;

    let timeout = Duration::from_secs(COMPILE_TIMEOUT_SECS);

    // Race the child process against a timeout.
    // If the timeout wins, we kill the child before reading its output.
    tokio::select! {
        _ = child.wait() => {}
        _ = tokio::time::sleep(timeout) => {
            let _ = child.kill().await;
            return Err(format!(
                "编译超时（{}秒），请简化代码或检查是否有无限循环",
                COMPILE_TIMEOUT_SECS
            ));
        }
    }

    // Process finished (or was killed) — collect output
    let output = child
        .wait_with_output()
        .await
        .map_err(|e| format!("读取进程输出失败: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let duration_ms = start.elapsed().as_millis() as u64;

    // 6. Handle compilation failure
    if !output.status.success() {
        let errors = parse_errors(&stderr);
        return Ok(LilyPondCompileResult {
            success: false,
            output_format: format_lower,
            output_base64: String::new(),
            output_path: String::new(),
            errors,
            stderr,
            duration_ms,
        });
    }

    // 7. Read compiled output file
    // LilyPond 2.24+ names PNG/SVG files as score-page1.png (not score.png)
    let ext = match format_lower.as_str() {
        "png" => "png",
        "svg" => "svg",
        "pdf" | "midi" => "pdf",
        _ => "pdf",
    };

    let page1_file = tmp_dir.join(format!("score-page1.{}", ext));
    let bare_file = tmp_dir.join(format!("score.{}", ext));
    let output_file = if page1_file.exists() { &page1_file } else { &bare_file };

    let (output_base64, output_path) = if output_file.exists() {
        let bytes = tokio::fs::read(&output_file)
            .await
            .map_err(|e| format!("读取编译输出文件失败: {}", e))?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        (encoded, output_file.to_string_lossy().to_string())
    } else {
        return Ok(LilyPondCompileResult {
            success: false,
            output_format: format_lower,
            output_base64: String::new(),
            output_path: String::new(),
            errors: vec![],
            stderr: format!(
                "编译产物未找到: 期望 {} 或 {}",
                page1_file.display(),
                bare_file.display()
            ),
            duration_ms: start.elapsed().as_millis() as u64,
        });
    };

    // Clean up source file, keep output for potential export
    let _ = tokio::fs::remove_file(&source_path).await;

    Ok(LilyPondCompileResult {
        success: true,
        output_format: format_lower,
        output_base64,
        output_path,
        errors: vec![],
        stderr,
        duration_ms,
    })
}

// ─── Error Parsing ───────────────────────────────────────────────────────────

/// Parse LilyPond stderr output into structured CompileError entries.
///
/// Matches patterns like:
///   `score.ly:12:3: error: ...`
///   `score.ly:5: warning: ...`
fn parse_errors(stderr: &str) -> Vec<CompileError> {
    let mut errors = Vec::new();

    let error_re =
        Regex::new(r"(?m)(?:^|\n)[^\n]*?:(\d+):(?:(\d+):)?\s*(error|warning|fatal error):\s*(.+?)(?:\n|$)")
            .unwrap();

    for cap in error_re.captures_iter(stderr) {
        let line: usize = cap[1].parse().unwrap_or(0);
        let column: usize = cap.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
        let severity_raw = cap[3].to_string();
        let message = cap[4].trim().to_string();
        let severity = if severity_raw == "warning" {
            "warning"
        } else {
            "error"
        };

        errors.push(CompileError {
            line,
            column,
            message,
            severity: severity.to_string(),
        });
    }

    // Fallback: if regex didn't match but there is stderr content, surface it
    if errors.is_empty() && !stderr.trim().is_empty() {
        // Extract meaningful lines (skip empty or purely decorative lines)
        let meaningful: Vec<&str> = stderr
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty()
                    && !t.starts_with("GNU LilyPond")
                    && !t.starts_with("Processing")
                    && !t.starts_with("Interpreting")
                    && !t.starts_with("Finding")
            })
            .collect();

        if !meaningful.is_empty() {
            errors.push(CompileError {
                line: 0,
                column: 0,
                message: meaningful.join("\n"),
                severity: "error".to_string(),
            });
        }
    }

    errors
}

// ─── Installation Check ─────────────────────────────────────────────────────

/// Check if LilyPond is installed and accessible.
pub async fn check_installation() -> Result<LilyPondInfo, String> {
    let lilypond_path = find_lilypond();
    let path_str = lilypond_path.to_string_lossy().to_string();

    match Command::new(&lilypond_path)
        .arg("--version")
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let version = parse_version(&stdout);
                Ok(LilyPondInfo {
                    available: true,
                    version,
                    path: path_str,
                })
            } else {
                Ok(LilyPondInfo {
                    available: false,
                    version: String::new(),
                    path: path_str,
                })
            }
        }
        Err(_) => Ok(LilyPondInfo {
            available: false,
            version: String::new(),
            path: path_str,
        }),
    }
}

/// Get the installed LilyPond version (cached after first call).
/// Returns the version string like "2.26.0" or an error if not installed.
pub async fn get_lilypond_version() -> Result<String, String> {
    // Check cache first
    if let Some(cached) = LILYPOND_VERSION.get() {
        return Ok(cached.clone());
    }

    // Fetch version from lilypond --version
    let info = check_installation().await?;
    if !info.available || info.version.is_empty() || info.version == "unknown" {
        return Err("LilyPond not installed or version not detected".to_string());
    }

    // Cache the version
    let _ = LILYPOND_VERSION.set(info.version.clone());
    Ok(info.version)
}

/// Extract version string from `lilypond --version` output.
/// Typical output: `GNU LilyPond 2.24.1`
fn parse_version(stdout: &str) -> String {
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    re.find(stdout)
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Strip lines containing {{...}} template placeholders from LilyPond code.
/// This prevents compilation errors when templates are compiled before AI fills the slots.
pub fn strip_template_placeholders(code: &str) -> String {
    let re = Regex::new(r"(?m)^.*\{\{.*?\}\}.*$\n?").unwrap();
    let stripped = re.replace_all(code, "").to_string();
    // Clean up multiple consecutive blank lines
    let blank_re = Regex::new(r"\n{3,}").unwrap();
    blank_re.replace_all(&stripped, "\n\n").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_safety_clean() {
        let code = r#"\version "2.24.0"
\relative c' { c4 d e f }"#;
        assert!(check_safety(code).is_ok());
    }

    #[test]
    fn test_check_safety_include() {
        let code = r#"\version "2.24.0"
\include "english.ly"
\relative c' { c4 d e f }"#;
        assert!(check_safety(code).is_err());
    }

    #[test]
    fn test_check_safety_indented_include() {
        let code = r#"  \include "something.ly""#;
        assert!(check_safety(code).is_err());
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("GNU LilyPond 2.24.1\n"), "2.24.1");
        assert_eq!(parse_version("GNU LilyPond 2.26.0"), "2.26.0");
        assert_eq!(parse_version("unknown output"), "unknown");
    }

    #[test]
    fn test_parse_errors_basic() {
        let stderr = "score.ly:12:3: error: syntax error, unexpected NOTENAME_PITCH";
        let errors = parse_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 12);
        assert_eq!(errors[0].column, 3);
        assert_eq!(errors[0].severity, "error");
    }

    #[test]
    fn test_parse_errors_warning() {
        let stderr = "score.ly:5: warning: Clashing note columns";
        let errors = parse_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].severity, "warning");
    }

    #[test]
    fn test_parse_errors_fallback() {
        let stderr = "something went terribly wrong";
        let errors = parse_errors(stderr);
        assert!(!errors.is_empty());
        assert_eq!(errors[0].line, 0);
    }
}
