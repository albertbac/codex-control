use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{CodexEvent, EventName};
use crate::redaction::redact_value;

pub fn parse_hook_stdin(input: &str) -> Result<Value> {
    let parsed: Value = serde_json::from_str(input).context("invalid JSON received on stdin")?;
    if !parsed.is_object() {
        bail!("hook input must be a JSON object");
    }
    Ok(parsed)
}

pub fn normalize_hook_payload(raw: Value) -> Result<CodexEvent> {
    let object = raw
        .as_object()
        .ok_or_else(|| anyhow!("hook input must be a JSON object"))?;

    let session_id =
        string_field(&raw, "session_id")?.ok_or_else(|| anyhow!("missing session_id"))?;
    let cwd = string_field(&raw, "cwd")?.ok_or_else(|| anyhow!("missing cwd"))?;
    let hook_event_name =
        string_field(&raw, "hook_event_name")?.ok_or_else(|| anyhow!("missing hook_event_name"))?;
    let model = string_field(&raw, "model")?;
    let turn_id = string_field(&raw, "turn_id")?;
    let created_at = string_field(&raw, "created_at")?
        .unwrap_or_else(|| Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    let event_name = normalize_event_name(&hook_event_name);
    let id = string_field(&raw, "event_id")?.unwrap_or_else(|| Uuid::new_v4().to_string());

    let payload = Value::Object(object.clone());

    Ok(CodexEvent {
        id,
        session_id,
        turn_id,
        event_name,
        cwd,
        model,
        payload,
        created_at,
    })
}

pub fn redact_event_payload(event: &CodexEvent) -> CodexEvent {
    let mut clone = event.clone();
    clone.payload = redact_value(&clone.payload);
    clone
}

fn normalize_event_name(input: &str) -> EventName {
    match input {
        "SessionStart" => EventName::SessionStart,
        "UserPromptSubmit" => EventName::UserPromptSubmit,
        "PreToolUse" => EventName::PreToolUse,
        "PermissionRequest" => EventName::PermissionRequest,
        "PostToolUse" => EventName::PostToolUse,
        "Stop" => EventName::Stop,
        _ => EventName::Unknown,
    }
}

fn string_field(value: &Value, key: &str) -> Result<Option<String>> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(other) => Err(anyhow!("field {key} must be a string, got {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_hook_payload, parse_hook_stdin};
    use crate::models::EventName;
    use serde_json::json;

    #[test]
    fn preserves_unknown_fields() {
        let input = json!({
          "session_id": "s-1",
          "cwd": "/tmp/project",
          "hook_event_name": "UserPromptSubmit",
          "model": "gpt-5.4",
          "turn_id": "t-1",
          "prompt": "hello",
          "custom_field": {"nested": true}
        });

        let event = normalize_hook_payload(input).expect("normalizes payload");
        assert_eq!(event.event_name, EventName::UserPromptSubmit);
        assert_eq!(event.payload["custom_field"]["nested"], true);
    }

    #[test]
    fn rejects_invalid_json() {
        let result = parse_hook_stdin("not-json");
        assert!(result.is_err());
    }
}
