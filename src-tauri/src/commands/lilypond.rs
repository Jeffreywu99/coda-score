use serde::Serialize;
use tauri::Manager;

use crate::lilypond::compiler::{self, LilyPondCompileResult, LilyPondInfo};
use crate::lilypond::knowledge::{self, KnowledgeSection};
use crate::lilypond::reference::{self, ReferenceSection};

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSlot {
    pub name: String,
    #[serde(rename = "type")]
    pub slot_type: String,
    pub required: bool,
    pub description: String,
    pub default: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateEntry {
    pub id: String,
    pub category: String,
    pub name: String,
    pub name_en: String,
    pub description: String,
    pub slots: Vec<TemplateSlot>,
    pub lilypond_code: String,
}

// ─── Commands ────────────────────────────────────────────────────────────────

/// Compile LilyPond code and return the output (base64-encoded for png/svg).
#[tauri::command]
pub async fn compile_lilypond(
    app: tauri::AppHandle,
    code: String,
    output_format: String,
) -> Result<LilyPondCompileResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法获取应用数据目录".to_string())?;
    compiler::compile(&code, &output_format, &app_data).await
}

/// Check whether LilyPond is installed and return version info.
#[tauri::command]
pub async fn check_lilypond_installation() -> Result<LilyPondInfo, String> {
    compiler::check_installation().await
}

/// Return the list of built-in LilyPond code templates.
#[tauri::command]
pub async fn get_lilypond_templates() -> Result<Vec<TemplateEntry>, String> {
    let version = compiler::get_lilypond_version().await.unwrap_or_else(|_| "2.24.0".to_string());
    Ok(builtin_templates(&version))
}

/// Search the embedded LilyPond reference document by keywords.
#[tauri::command]
pub async fn search_reference(
    keywords: Vec<String>,
    max_sections: usize,
) -> Result<Vec<ReferenceSection>, String> {
    Ok(reference::search(&keywords, max_sections))
}

/// Search the embedded knowledge base (study/knowledge/) for sections matching the query.
/// Optionally filter by template ID to only return contextually relevant sections.
#[tauri::command]
pub async fn search_knowledge(
    query: String,
    max_results: usize,
    template_id: Option<String>,
) -> Result<Vec<KnowledgeSection>, String> {
    let context_filter: Option<Vec<&str>> = template_id
        .as_deref()
        .map(|tid| knowledge::contexts_for_template(tid));

    let filter_refs: Option<Vec<&str>> = context_filter.as_ref().map(|v| {
        v.iter().copied().collect()
    });

    Ok(knowledge::search_knowledge(
        &query,
        max_results,
        filter_refs.as_deref(),
    ))
}

/// Copy a compiled LilyPond output file from the temp directory to a user-chosen path.
#[tauri::command]
pub async fn export_lilypond_file(
    source_path: String,
    target_path: String,
) -> Result<(), String> {
    std::fs::copy(&source_path, &target_path)
        .map_err(|e| format!("导出文件失败: {}", e))?;
    Ok(())
}

// ─── Built-in Templates ─────────────────────────────────────────────────────

fn builtin_templates(version: &str) -> Vec<TemplateEntry> {
    let mut templates = vec![
        // ── Basic ──────────────────────────────────────────────────────────
        TemplateEntry {
            id: "blank".to_string(),
            category: "basic".to_string(),
            name: "自由草稿".to_string(),
            name_en: "Blank".to_string(),
            description: "空白五线谱，自由输入".to_string(),
            slots: vec![TemplateSlot {
                name: "MUSIC".to_string(), slot_type: "music".to_string(),
                required: false, description: "任意音乐内容".to_string(),
                default: "r1".to_string(),
            }],
            lilypond_code: r#"\version "2.24.0"

\score {
  \new Staff {
    \clef treble
    \time 4/4
    \relative c' {
      {{MUSIC}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "solo-melody".to_string(),
            category: "basic".to_string(),
            name: "独奏旋律".to_string(),
            name_en: "Solo Melody".to_string(),
            description: "带标题、速度的单声部旋律".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "COMPOSER".to_string(), slot_type: "text".to_string(), required: false, description: "作曲家".to_string(), default: "".to_string() },
                TemplateSlot { name: "TEMPO".to_string(), slot_type: "markup".to_string(), required: false, description: "速度标记".to_string(), default: "".to_string() },
                TemplateSlot { name: "TIME".to_string(), slot_type: "markup".to_string(), required: false, description: "拍号".to_string(), default: "\\time 4/4".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "旋律内容".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
  composer = "{{COMPOSER}}"
}

\score {
  \new Staff {
    \clef treble
    {{TIME}}
    {{TEMPO}}
    \relative c' {
      {{MUSIC}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "solo-bass".to_string(),
            category: "basic".to_string(),
            name: "低音独奏".to_string(),
            name_en: "Solo Bass".to_string(),
            description: "低音谱号独奏模板".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "TEMPO".to_string(), slot_type: "markup".to_string(), required: false, description: "速度标记".to_string(), default: "".to_string() },
                TemplateSlot { name: "TIME".to_string(), slot_type: "markup".to_string(), required: false, description: "拍号".to_string(), default: "\\time 4/4".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "旋律内容".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \clef bass
    {{TIME}}
    {{TEMPO}}
    \relative c {
      {{MUSIC}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        // ── Piano ──────────────────────────────────────────────────────────
        TemplateEntry {
            id: "piano-solo".to_string(),
            category: "piano".to_string(),
            name: "钢琴独奏".to_string(),
            name_en: "Piano Solo".to_string(),
            description: "双手钢琴大谱表".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "COMPOSER".to_string(), slot_type: "text".to_string(), required: false, description: "作曲家".to_string(), default: "".to_string() },
                TemplateSlot { name: "TEMPO".to_string(), slot_type: "markup".to_string(), required: false, description: "速度标记".to_string(), default: "".to_string() },
                TemplateSlot { name: "TIME".to_string(), slot_type: "markup".to_string(), required: false, description: "拍号".to_string(), default: "\\time 4/4".to_string() },
                TemplateSlot { name: "UPPER".to_string(), slot_type: "music".to_string(), required: true, description: "右手/高声部".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "LOWER".to_string(), slot_type: "music".to_string(), required: true, description: "左手/低声部".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
  composer = "{{COMPOSER}}"
}

\score {
  \new PianoStaff <<
    \new Staff = "upper" {
      \clef treble
      {{TIME}}
      {{TEMPO}}
      \relative c'' {
        {{UPPER}}
      }
    }
    \new Staff = "lower" {
      \clef bass
      \relative c {
        {{LOWER}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "piano-voice".to_string(),
            category: "piano".to_string(),
            name: "独唱+钢琴".to_string(),
            name_en: "Voice + Piano".to_string(),
            description: "独唱声部加钢琴伴奏".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "COMPOSER".to_string(), slot_type: "text".to_string(), required: false, description: "作曲家".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "声乐旋律".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "UPPER".to_string(), slot_type: "music".to_string(), required: true, description: "钢琴右手".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "LOWER".to_string(), slot_type: "music".to_string(), required: true, description: "钢琴左手".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
  composer = "{{COMPOSER}}"
}

\score {
  <<
    \new Staff = "voice" {
      \clef treble
      \time 4/4
      \relative c'' {
        {{MUSIC}}
      }
    }
    \new PianoStaff <<
      \new Staff = "upper" {
        \clef treble
        \relative c'' {
          {{UPPER}}
        }
      }
      \new Staff = "lower" {
        \clef bass
        \relative c {
          {{LOWER}}
        }
      }
    >>
  >>
  \layout { }
}"#.to_string(),
        },
        // ── Chamber ────────────────────────────────────────────────────────
        TemplateEntry {
            id: "string-quartet".to_string(),
            category: "chamber".to_string(),
            name: "弦乐四重奏".to_string(),
            name_en: "String Quartet".to_string(),
            description: "两把小提琴、中提琴、大提琴".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "VOICE1".to_string(), slot_type: "music".to_string(), required: true, description: "第一小提琴".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE2".to_string(), slot_type: "music".to_string(), required: true, description: "第二小提琴".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE3".to_string(), slot_type: "music".to_string(), required: true, description: "中提琴".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE4".to_string(), slot_type: "music".to_string(), required: true, description: "大提琴".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new StaffGroup <<
    \new Staff = "violinI" {
      \clef treble
      \time 4/4
      \relative c'' {
        {{VOICE1}}
      }
    }
    \new Staff = "violinII" {
      \clef treble
      \relative c'' {
        {{VOICE2}}
      }
    }
    \new Staff = "viola" {
      \clef alto
      \relative c' {
        {{VOICE3}}
      }
    }
    \new Staff = "cello" {
      \clef bass
      \relative c {
        {{VOICE4}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "duet".to_string(),
            category: "chamber".to_string(),
            name: "二重奏".to_string(),
            name_en: "Duet".to_string(),
            description: "两件乐器二重奏".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "第一声部".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "MUSIC2".to_string(), slot_type: "music".to_string(), required: true, description: "第二声部".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  <<
    \new Staff {
      \clef treble
      \time 4/4
      \relative c'' {
        {{MUSIC}}
      }
    }
    \new Staff {
      \clef treble
      \relative c'' {
        {{MUSIC2}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "woodwind-trio".to_string(),
            category: "chamber".to_string(),
            name: "木管三重奏".to_string(),
            name_en: "Woodwind Trio".to_string(),
            description: "长笛、双簧管、单簧管".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "VOICE1".to_string(), slot_type: "music".to_string(), required: true, description: "长笛".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE2".to_string(), slot_type: "music".to_string(), required: true, description: "双簧管".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE3".to_string(), slot_type: "music".to_string(), required: true, description: "单簧管 (降B)".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new StaffGroup <<
    \new Staff = "flute" {
      \clef treble
      \time 4/4
      \relative c'' {
        {{VOICE1}}
      }
    }
    \new Staff = "oboe" {
      \clef treble
      \relative c'' {
        {{VOICE2}}
      }
    }
    \new Staff = "clarinet" {
      \clef treble
      \relative c'' {
        {{VOICE3}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "brass-quintet".to_string(),
            category: "chamber".to_string(),
            name: "铜管五重奏".to_string(),
            name_en: "Brass Quintet".to_string(),
            description: "小号×2、圆号、长号、大号".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "VOICE1".to_string(), slot_type: "music".to_string(), required: true, description: "第一小号".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE2".to_string(), slot_type: "music".to_string(), required: true, description: "第二小号".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE3".to_string(), slot_type: "music".to_string(), required: true, description: "圆号".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE4".to_string(), slot_type: "music".to_string(), required: true, description: "长号".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE5".to_string(), slot_type: "music".to_string(), required: true, description: "大号".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new StaffGroup <<
    \new Staff = "trumpetI" {
      \clef treble
      \time 4/4
      \relative c'' {
        {{VOICE1}}
      }
    }
    \new Staff = "trumpetII" {
      \clef treble
      \relative c'' {
        {{VOICE2}}
      }
    }
    \new Staff = "horn" {
      \clef treble
      \relative c'' {
        {{VOICE3}}
      }
    }
    \new Staff = "trombone" {
      \clef bass
      \relative c {
        {{VOICE4}}
      }
    }
    \new Staff = "tuba" {
      \clef bass
      \relative c {
        {{VOICE5}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        // ── Vocal ──────────────────────────────────────────────────────────
        TemplateEntry {
            id: "satb-choir".to_string(),
            category: "vocal".to_string(),
            name: "SATB 合唱".to_string(),
            name_en: "SATB Choir".to_string(),
            description: "混声四部合唱（女高/女低/男高/男低）".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "TEMPO".to_string(), slot_type: "markup".to_string(), required: false, description: "速度标记".to_string(), default: "".to_string() },
                TemplateSlot { name: "VOICE1".to_string(), slot_type: "music".to_string(), required: true, description: "女高音 (Soprano)".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE2".to_string(), slot_type: "music".to_string(), required: true, description: "女低音 (Alto)".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE3".to_string(), slot_type: "music".to_string(), required: true, description: "男高音 (Tenor)".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE4".to_string(), slot_type: "music".to_string(), required: true, description: "男低音 (Bass)".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new ChoirStaff <<
    \new Staff = "soprano" {
      \clef treble
      \time 4/4
      {{TEMPO}}
      \relative c'' {
        {{VOICE1}}
      }
    }
    \new Staff = "alto" {
      \clef treble
      \relative c' {
        {{VOICE2}}
      }
    }
    \new Staff = "tenor" {
      \clef "treble_8"
      \relative c' {
        {{VOICE3}}
      }
    }
    \new Staff = "bass" {
      \clef bass
      \relative c {
        {{VOICE4}}
      }
    }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "satb-piano".to_string(),
            category: "vocal".to_string(),
            name: "SATB + 钢琴".to_string(),
            name_en: "SATB + Piano".to_string(),
            description: "混声合唱加钢琴伴奏".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "VOICE1".to_string(), slot_type: "music".to_string(), required: true, description: "女高音".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE2".to_string(), slot_type: "music".to_string(), required: true, description: "女低音".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE3".to_string(), slot_type: "music".to_string(), required: true, description: "男高音".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "VOICE4".to_string(), slot_type: "music".to_string(), required: true, description: "男低音".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "UPPER".to_string(), slot_type: "music".to_string(), required: false, description: "钢琴右手".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "LOWER".to_string(), slot_type: "music".to_string(), required: false, description: "钢琴左手".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  <<
    \new ChoirStaff <<
      \new Staff = "soprano" {
        \clef treble
        \time 4/4
        \relative c'' { {{VOICE1}} }
      }
      \new Staff = "alto" {
        \clef treble
        \relative c' { {{VOICE2}} }
      }
      \new Staff = "tenor" {
        \clef "treble_8"
        \relative c' { {{VOICE3}} }
      }
      \new Staff = "bass" {
        \clef bass
        \relative c { {{VOICE4}} }
      }
    >>
    \new PianoStaff <<
      \new Staff = "upper" {
        \clef treble
        \relative c'' { {{UPPER}} }
      }
      \new Staff = "lower" {
        \clef bass
        \relative c { {{LOWER}} }
      }
    >>
  >>
  \layout { }
}"#.to_string(),
        },
        // ── Contemporary ───────────────────────────────────────────────────
        TemplateEntry {
            id: "twelve-tone-row".to_string(),
            category: "contemporary".to_string(),
            name: "十二音序列".to_string(),
            name_en: "Twelve-Tone Row".to_string(),
            description: "十二音序列单行旋律（12/8拍）".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "序列标签（如 P0）".to_string(), default: "".to_string() },
                TemplateSlot { name: "ROW".to_string(), slot_type: "music".to_string(), required: true, description: "十二音序列（12个音级）".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
  subtitle = "Twelve-Tone Row"
}

\score {
  \new Staff {
    \clef treble
    \time 12/8
    \relative c' {
      {{ROW}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "twelve-tone-matrix".to_string(),
            category: "contemporary".to_string(),
            name: "矩阵总谱".to_string(),
            name_en: "Matrix Score".to_string(),
            description: "十二音矩阵完整总谱（12行×4形式）".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "总谱标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "ROW_P0".to_string(), slot_type: "music".to_string(), required: false, description: "P0 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P1".to_string(), slot_type: "music".to_string(), required: false, description: "P1 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P2".to_string(), slot_type: "music".to_string(), required: false, description: "P2 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P3".to_string(), slot_type: "music".to_string(), required: false, description: "P3 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P4".to_string(), slot_type: "music".to_string(), required: false, description: "P4 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P5".to_string(), slot_type: "music".to_string(), required: false, description: "P5 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P6".to_string(), slot_type: "music".to_string(), required: false, description: "P6 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P7".to_string(), slot_type: "music".to_string(), required: false, description: "P7 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P8".to_string(), slot_type: "music".to_string(), required: false, description: "P8 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P9".to_string(), slot_type: "music".to_string(), required: false, description: "P9 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P10".to_string(), slot_type: "music".to_string(), required: false, description: "P10 行".to_string(), default: "r1".to_string() },
                TemplateSlot { name: "ROW_P11".to_string(), slot_type: "music".to_string(), required: false, description: "P11 行".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
  subtitle = "Sequence Matrix"
}

\score {
  \new StaffGroup <<
    \new Staff = "P0"  { \clef treble \time 12/8 \relative c' { {{ROW_P0}}  } }
    \new Staff = "P1"  { \clef treble \relative c' { {{ROW_P1}}  } }
    \new Staff = "P2"  { \clef treble \relative c' { {{ROW_P2}}  } }
    \new Staff = "P3"  { \clef treble \relative c' { {{ROW_P3}}  } }
    \new Staff = "P4"  { \clef treble \relative c' { {{ROW_P4}}  } }
    \new Staff = "P5"  { \clef treble \relative c' { {{ROW_P5}}  } }
    \new Staff = "P6"  { \clef treble \relative c' { {{ROW_P6}}  } }
    \new Staff = "P7"  { \clef treble \relative c' { {{ROW_P7}}  } }
    \new Staff = "P8"  { \clef treble \relative c' { {{ROW_P8}}  } }
    \new Staff = "P9"  { \clef treble \relative c' { {{ROW_P9}}  } }
    \new Staff = "P10" { \clef treble \relative c' { {{ROW_P10}} } }
    \new Staff = "P11" { \clef treble \relative c' { {{ROW_P11}} } }
  >>
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "microtonal".to_string(),
            category: "contemporary".to_string(),
            name: "微音程".to_string(),
            name_en: "Microtonal".to_string(),
            description: "四分之一音等微分音记谱模板".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "微音程旋律".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

% 微音程记号说明：
% ih  = 半升 (quarter-tone sharp)
% eh  = 半降 (quarter-tone flat)
% iqs = 四分之三升
% iqf = 四分之三降

\score {
  \new Staff {
    \clef treble
    \time 4/4
    \relative c' {
      {{MUSIC}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "cluster".to_string(),
            category: "contemporary".to_string(),
            name: "音簇".to_string(),
            name_en: "Cluster".to_string(),
            description: "音簇记谱（\\makeClusters）".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "音簇内容".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \clef treble
    \time 4/4
    \makeClusters {
      {{MUSIC}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "proportional".to_string(),
            category: "contemporary".to_string(),
            name: "比例记谱".to_string(),
            name_en: "Proportional Notation".to_string(),
            description: "自由节奏/比例记谱（\\cadenzaOn）".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "自由节奏音乐".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \clef treble
    \cadenzaOn
    \relative c' {
      {{MUSIC}}
    }
    \cadenzaOff
  }
  \layout {
    \context {
      \Voice
      \remove "Forbid_line_break_engraver"
    }
  }
}"#.to_string(),
        },
        TemplateEntry {
            id: "graphic-staff".to_string(),
            category: "contemporary".to_string(),
            name: "扩展五线谱".to_string(),
            name_en: "Extended Staff".to_string(),
            description: "当代记谱：十二音临时记号样式、无小节线".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "MUSIC".to_string(), slot_type: "music".to_string(), required: true, description: "音乐内容".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \accidentalStyle dodecaphonic
    \clef treble
    \relative c' {
      {{MUSIC}}
    }
  }
  \layout {
    \context {
      \Staff
      \remove "Time_signature_engraver"
    }
  }
}"#.to_string(),
        },
        // ── Analysis ───────────────────────────────────────────────────────
        TemplateEntry {
            id: "pitch-class-chord".to_string(),
            category: "analysis".to_string(),
            name: "音级集合和弦".to_string(),
            name_en: "Pitch-Class Chord".to_string(),
            description: "音级集合导出的单个和弦输出".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "福特号/集合标签".to_string(), default: "".to_string() },
                TemplateSlot { name: "CHORDS".to_string(), slot_type: "music".to_string(), required: true, description: "和弦内容（如 <c e g>1）".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \clef treble
    \time 4/4
    \relative c' {
      {{CHORDS}}
    }
  }
  \layout { }
}"#.to_string(),
        },
        TemplateEntry {
            id: "chord-progression".to_string(),
            category: "analysis".to_string(),
            name: "和弦进行".to_string(),
            name_en: "Chord Progression".to_string(),
            description: "和弦序列/和声分析输出".to_string(),
            slots: vec![
                TemplateSlot { name: "TITLE".to_string(), slot_type: "text".to_string(), required: false, description: "乐曲标题".to_string(), default: "".to_string() },
                TemplateSlot { name: "CHORDS".to_string(), slot_type: "music".to_string(), required: true, description: "和弦序列（如 <c e g>1 | <f a c>1）".to_string(), default: "r1".to_string() },
            ],
            lilypond_code: r#"\version "2.24.0"

\header {
  title = "{{TITLE}}"
}

\score {
  \new Staff {
    \clef treble
    \time 4/4
    \relative c' {
      {{CHORDS}}
    }
  }
  \layout { }
}"#.to_string(),
        },
    ];

    // Post-process: replace hardcoded version with actual installed version
    if version != "2.24.0" {
        let old_version = "\\version \"2.24.0\"";
        let new_version = format!("\\version \"{}\"", version);
        for template in &mut templates {
            template.lilypond_code = template.lilypond_code.replace(old_version, &new_version);
        }
    }

    templates
}
