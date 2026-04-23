use serde::Serialize;
use which::which;

use codex_core::LocalStore;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookRuntimeState {
  pub hook_cli_available: bool,
  pub hook_cli_path: Option<String>,
  pub last_ingest_at: Option<String>,
  pub store_mode: String,
}

pub fn hook_runtime_state() -> anyhow::Result<HookRuntimeState> {
  let store = LocalStore::open()?;
  let hook_cli_path = which("codex-control-hook")
    .ok()
    .map(|path| path.to_string_lossy().to_string());

  Ok(HookRuntimeState {
    hook_cli_available: hook_cli_path.is_some(),
    hook_cli_path,
    last_ingest_at: store.latest_event_at()?,
    store_mode: store.mode().as_str().to_string(),
  })
}
