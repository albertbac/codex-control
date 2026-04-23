use anyhow::{Context, Result};
use codex_core::{
  evaluate_policy,
  normalize_hook_payload,
  parse_hook_stdin,
  permission_request_deny_response,
  pre_tool_use_deny_response,
  redact_event_payload,
  LocalStore,
  PolicyVerdict,
};
use serde_json::json;

const EXAMPLE_CONFIG: &str = include_str!("../../../examples/hooks/config.toml");
const EXAMPLE_HOOKS: &str = include_str!("../../../examples/hooks/hooks.json");

#[derive(Debug, Clone, PartialEq)]
pub struct CommandOutput {
  pub stdout: String,
  pub stderr: String,
  pub exit_code: i32,
}

pub fn run_ingest(input: &str, emit_json_response: bool) -> CommandOutput {
  match ingest_impl(input, emit_json_response) {
    Ok(stdout) => CommandOutput {
      stdout,
      stderr: String::new(),
      exit_code: 0,
    },
    Err(error) => CommandOutput {
      stdout: String::new(),
      stderr: format!("{error:#}"),
      exit_code: 1,
    },
  }
}

pub fn run_policy(input: &str) -> CommandOutput {
  match policy_impl(input) {
    Ok(stdout) => CommandOutput {
      stdout,
      stderr: String::new(),
      exit_code: 0,
    },
    Err(error) => CommandOutput {
      stdout: String::new(),
      stderr: format!("{error:#}"),
      exit_code: 1,
    },
  }
}

pub fn run_doctor() -> CommandOutput {
  match doctor_impl() {
    Ok(stdout) => CommandOutput {
      stdout,
      stderr: String::new(),
      exit_code: 0,
    },
    Err(error) => CommandOutput {
      stdout: String::new(),
      stderr: format!("{error:#}"),
      exit_code: 1,
    },
  }
}

fn ingest_impl(input: &str, emit_json_response: bool) -> Result<String> {
  let raw = parse_hook_stdin(input)?;
  let event = redact_event_payload(&normalize_hook_payload(raw)?);
  let store = LocalStore::open()?;
  let _ = store.persist_event(&event)?;

  if emit_json_response {
    Ok(json!({"continue": true, "suppressOutput": false}).to_string())
  } else {
    Ok(String::new())
  }
}

fn policy_impl(input: &str) -> Result<String> {
  let raw = parse_hook_stdin(input)?;
  let event = normalize_hook_payload(raw)?;
  let output = match evaluate_policy(&event) {
    PolicyVerdict::AllowNoOutput => String::new(),
    PolicyVerdict::DenyPreToolUse(_) => pre_tool_use_deny_response().to_string(),
    PolicyVerdict::DenyPermissionRequest(_) => permission_request_deny_response().to_string(),
  };
  Ok(output)
}

fn doctor_impl() -> Result<String> {
  let store = LocalStore::open()?;
  let _config: toml::Value = toml::from_str(EXAMPLE_CONFIG).context("embedded config.toml is invalid")?;
  let _hooks: serde_json::Value =
    serde_json::from_str(EXAMPLE_HOOKS).context("embedded hooks.json is invalid")?;

  Ok(format!(
    "Codex Control hook doctor\n- store mode: {}\n- database path: {}\n- spool path: {}\n- embedded examples: valid\n- latest event: {}\n",
    store.mode().as_str(),
    store.paths().database_path,
    store.paths().spool_path,
    store.latest_event_at()?.unwrap_or_else(|| "none".to_string())
  ))
}
