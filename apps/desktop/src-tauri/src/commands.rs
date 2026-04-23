use std::path::Path;
use std::process::Command;

use crate::session_store::{dashboard_snapshot as build_dashboard, git_diff_preview, session_timeline as build_timeline, settings_info as build_settings};
use crate::transcript_parser::inspect_transcript as inspect_transcript_file;

#[tauri::command]
pub fn dashboard_snapshot() -> Result<Vec<codex_core::DashboardSession>, String> {
  build_dashboard().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn session_timeline(session_id: String) -> Result<Vec<codex_core::TimelineItem>, String> {
  build_timeline(&session_id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn settings_info() -> Result<codex_core::SettingsInfo, String> {
  build_settings().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn inspect_git_diff(cwd: String) -> Result<String, String> {
  Ok(git_diff_preview(&cwd))
}

#[tauri::command]
pub fn inspect_transcript(path: String) -> Result<String, String> {
  inspect_transcript_file(&path).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_terminal(cwd: String) -> Result<(), String> {
  if !Path::new(&cwd).exists() {
    return Err("working directory does not exist".to_string());
  }

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .args(["-a", "Terminal", &cwd])
      .status()
      .map_err(|error| error.to_string())?;
    return Ok(());
  }

  #[cfg(target_os = "linux")]
  {
    let candidates = [
      ("x-terminal-emulator", vec!["--working-directory", &cwd]),
      ("gnome-terminal", vec!["--working-directory", &cwd]),
      ("konsole", vec!["--workdir", &cwd]),
      ("alacritty", vec!["--working-directory", &cwd]),
    ];
    for (binary, args) in candidates {
      if Command::new(binary).args(args.clone()).spawn().is_ok() {
        return Ok(());
      }
    }
    return Err("no supported terminal launcher found".to_string());
  }

  #[allow(unreachable_code)]
  Err("terminal integration is only implemented for macOS and Linux".to_string())
}

#[tauri::command]
pub fn open_editor(cwd: String) -> Result<(), String> {
  if !Path::new(&cwd).exists() {
    return Err("working directory does not exist".to_string());
  }

  let candidates = ["code", "cursor", "zed"];
  for binary in candidates {
    if Command::new(binary).arg(&cwd).spawn().is_ok() {
      return Ok(());
    }
  }

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .args(["-a", "Visual Studio Code", &cwd])
      .status()
      .map_err(|error| error.to_string())?;
    return Ok(());
  }

  #[cfg(target_os = "linux")]
  {
    Command::new("xdg-open")
      .arg(&cwd)
      .status()
      .map_err(|error| error.to_string())?;
    return Ok(());
  }

  #[allow(unreachable_code)]
  Err("editor integration is only implemented for macOS and Linux".to_string())
}

#[tauri::command]
pub fn terminate_process(pid: i64, confirm: bool) -> Result<(), String> {
  if !confirm {
    return Err("explicit confirmation is required before terminating a local Codex process".to_string());
  }
  Command::new("kill")
    .args(["-TERM", &pid.to_string()])
    .status()
    .map_err(|error| error.to_string())?;
  Ok(())
}
