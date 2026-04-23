use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
  Working,
  Idle,
  WaitingApproval,
  Errored,
  Finished,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventName {
  #[serde(rename = "SessionStart")]
  SessionStart,
  #[serde(rename = "UserPromptSubmit")]
  UserPromptSubmit,
  #[serde(rename = "PreToolUse")]
  PreToolUse,
  #[serde(rename = "PermissionRequest")]
  PermissionRequest,
  #[serde(rename = "PostToolUse")]
  PostToolUse,
  #[serde(rename = "Stop")]
  Stop,
  #[serde(rename = "Unknown")]
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CodexSession {
  pub id: String,
  pub cwd: String,
  pub repo_root: Option<String>,
  pub repo_name: Option<String>,
  pub branch: Option<String>,
  pub model: Option<String>,
  pub transcript_path: Option<String>,
  pub status: SessionStatus,
  pub last_prompt: Option<String>,
  pub last_command: Option<String>,
  pub last_assistant_message: Option<String>,
  pub started_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CodexEvent {
  pub id: String,
  pub session_id: String,
  pub turn_id: Option<String>,
  pub event_name: EventName,
  pub cwd: String,
  pub model: Option<String>,
  pub payload: Value,
  pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSnapshot {
  pub pid: i64,
  pub parent_pid: Option<i64>,
  pub cwd: String,
  pub command: String,
  pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GitSnapshot {
  pub changed_files_count: usize,
  pub staged_count: usize,
  pub unstaged_count: usize,
  pub diff_stat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSession {
  #[serde(flatten)]
  pub session: CodexSession,
  pub approval_state: Option<String>,
  pub changed_files_count: usize,
  pub staged_count: usize,
  pub unstaged_count: usize,
  pub diff_stat: Option<String>,
  pub transcript_preview: Option<String>,
  pub process: Option<ProcessSnapshot>,
  pub source: String,
  pub is_stale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimelineItem {
  pub id: String,
  pub session_id: String,
  pub event_name: EventName,
  pub created_at: String,
  pub command: Option<String>,
  pub approval_request: Option<String>,
  pub result_summary: Option<String>,
  pub transcript_path: Option<String>,
  pub git_state: Option<String>,
  pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataPaths {
  pub data_dir: String,
  pub database_path: String,
  pub spool_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SettingsInfo {
  pub paths: DataPaths,
  pub hook_cli_available: bool,
  pub hook_cli_path: Option<String>,
  pub store_mode: String,
  pub last_ingest_at: Option<String>,
  pub hook_install_snippet: String,
  pub notes: Vec<String>,
}
