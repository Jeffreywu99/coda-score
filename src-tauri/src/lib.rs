mod commands;
mod lilypond;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::lilypond::compile_lilypond,
            commands::lilypond::check_lilypond_installation,
            commands::lilypond::get_lilypond_templates,
            commands::lilypond::search_reference,
            commands::lilypond::search_knowledge,
            commands::lilypond::export_lilypond_file,
            commands::ai_proxy::generate_lilypond_code,
            commands::ai_proxy::review_lilypond_code,
            commands::ai_proxy::check_ai_available,
            commands::ai_proxy::get_ai_config,
            commands::ai_proxy::save_ai_config,
            commands::ai_proxy::test_ai_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
