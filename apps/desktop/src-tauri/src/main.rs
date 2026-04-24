mod commands;
mod git_inspector;
mod hook_server;
mod process_watcher;
mod redaction;
mod session_store;
mod transcript_parser;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::dashboard_snapshot,
            commands::session_timeline,
            commands::settings_info,
            commands::inspect_git_diff,
            commands::inspect_transcript,
            commands::open_terminal,
            commands::open_editor,
            commands::terminate_process,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Codex Control desktop runtime");
}
