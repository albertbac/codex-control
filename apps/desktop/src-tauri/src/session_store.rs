use codex_core::{
  reconcile_stale_status, DashboardSession, LocalStore, SettingsInfo, TimelineItem,
};
use serde_json::Value;

use crate::git_inspector::{inspect_diff_preview, inspect_git};
use crate::hook_server::hook_runtime_state;
use crate::process_watcher::collect_codex_processes;
use crate::transcript_parser::read_transcript_summary;

pub fn dashboard_snapshot() -> anyhow::Result<Vec<DashboardSession>> {
  let store = LocalStore::open()?;
  let sessions = store.list_sessions()?;
  let processes = collect_codex_processes();

  let mut dashboard = Vec::new();
  for session in sessions {
    let process = processes
      .iter()
      .find(|candidate| {
        (!candidate.cwd.is_empty() && candidate.cwd == session.cwd)
          || candidate.command.contains(&session.id)
      })
      .cloned();

    let git = inspect_git(session.repo_root.as_deref().unwrap_or(&session.cwd));
    let transcript = read_transcript_summary(session.transcript_path.as_deref());
    let status = reconcile_stale_status(&session, process.is_some());
    let is_stale = matches!(status, codex_core::SessionStatus::Unknown | codex_core::SessionStatus::Finished);

    dashboard.push(DashboardSession {
      id: session.id,
      cwd: session.cwd,
      repo_root: session.repo_root,
      repo_name: session.repo_name,
      branch: session.branch,
      model: session.model,
      transcript_path: session.transcript_path,
      status: status.clone(),
      last_prompt: session.last_prompt,
      last_command: session.last_command.or_else(|| transcript.as_ref().and_then(|value| value.last_command.clone())),
      last_assistant_message: session
        .last_assistant_message
        .or_else(|| transcript.as_ref().and_then(|value| value.last_assistant_message.clone())),
      started_at: session.started_at,
      updated_at: session.updated_at,
      approval_state: approval_state_label(&session.status),
      changed_files_count: git.changed_files_count,
      staged_count: git.staged_count,
      unstaged_count: git.unstaged_count,
      diff_stat: git.diff_stat,
      transcript_preview: transcript.and_then(|value| value.preview),
      process,
      source: if store.mode().as_str() == "sqlite" { "sqlite".to_string() } else { "spool".to_string() },
      is_stale,
    });
  }

  dashboard.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
  Ok(dashboard)
}

pub fn session_timeline(session_id: &str) -> anyhow::Result<Vec<TimelineItem>> {
  let store = LocalStore::open()?;
  let events = store.list_events(session_id)?;
  Ok(events
    .into_iter()
    .map(|event| TimelineItem {
      id: event.id,
      session_id: event.session_id,
      event_name: event.event_name,
      created_at: event.created_at,
      command: extract_command(&event.payload),
      approval_request: extract_approval_request(&event.payload),
      result_summary: extract_result_summary(&event.payload),
      transcript_path: extract_string(&event.payload, "transcript_path"),
      git_state: extract_git_state(&event.payload),
      payload: event.payload,
    })
    .collect())
}

pub fn settings_info() -> anyhow::Result<SettingsInfo> {
  let store = LocalStore::open()?;
  let hook = hook_runtime_state()?;
  Ok(SettingsInfo {
    paths: store.paths().clone(),
    hook_cli_available: hook.hook_cli_available,
    hook_cli_path: hook.hook_cli_path,
    store_mode: hook.store_mode,
    last_ingest_at: hook.last_ingest_at,
    hook_install_snippet: "cargo install --path packages/hook-cli".to_string(),
    notes: vec![
      "The desktop app stores data only on the local machine.".to_string(),
      "Hooks are guardrails and session telemetry, not a universal interception layer.".to_string(),
      "Windows hook workflows are intentionally not documented as production-ready.".to_string(),
    ],
  })
}

pub fn git_diff_preview(cwd: &str) -> String {
  inspect_diff_preview(cwd)
}

fn approval_state_label(status: &codex_core::SessionStatus) -> Option<String> {
  match status {
    codex_core::SessionStatus::WaitingApproval => Some("pending".to_string()),
    codex_core::SessionStatus::Errored => Some("attention".to_string()),
    _ => None,
  }
}

fn extract_string(payload: &Value, key: &str) -> Option<String> {
  payload.get(key).and_then(Value::as_str).map(str::to_string)
}

fn extract_command(payload: &Value) -> Option<String> {
  payload
    .get("tool_input")
    .and_then(|value| value.get("command"))
    .and_then(Value::as_str)
    .map(str::to_string)
}

fn extract_approval_request(payload: &Value) -> Option<String> {
  payload
    .get("tool_input")
    .and_then(|value| value.get("description"))
    .and_then(Value::as_str)
    .map(str::to_string)
    .or_else(|| extract_command(payload))
}

fn extract_result_summary(payload: &Value) -> Option<String> {
  payload
    .get("tool_response")
    .map(Value::to_string)
    .filter(|value| !value.is_empty())
}

fn extract_git_state(payload: &Value) -> Option<String> {
  payload
    .get("git_state")
    .and_then(Value::as_str)
    .map(str::to_string)
}
