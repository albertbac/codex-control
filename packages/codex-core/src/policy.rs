use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};

use crate::models::{CodexEvent, EventName};

pub const PRE_TOOL_USE_DENY_RESPONSE_JSON: &str = "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"Destructive command blocked by Codex Control policy.\"}}";
pub const PERMISSION_REQUEST_DENY_RESPONSE_JSON: &str = "{\"hookSpecificOutput\":{\"hookEventName\":\"PermissionRequest\",\"decision\":{\"behavior\":\"deny\",\"message\":\"Blocked by Codex Control policy.\"}}}";

static DESTRUCTIVE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    [
        r"(^|\s)rm\s+-rf(\s|$)",
        r"git\s+reset\s+--hard",
        r"git\s+clean\s+-fd",
        r"(^|\s)mkfs(\.|\s)",
        r"(^|\s)dd\s+if=",
        r"(^|\s)(shutdown|reboot|halt)(\s|$)",
        r"curl\s+[^|]+\|\s*(sh|bash)",
        r"chmod\s+-R\s+777\s+/",
        r":\(\)\s*\{\s*:\|:\&\s*\};:",
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("valid destructive regex"))
    .collect()
});

#[derive(Debug, Clone, PartialEq)]
pub enum PolicyVerdict {
    AllowNoOutput,
    DenyPreToolUse(Value),
    DenyPermissionRequest(Value),
}

pub fn evaluate_policy(event: &CodexEvent) -> PolicyVerdict {
    let command = extract_command(&event.payload);
    match event.event_name {
        EventName::PreToolUse if command.as_deref().is_some_and(is_destructive_command) => {
            PolicyVerdict::DenyPreToolUse(pre_tool_use_deny_response())
        }
        EventName::PermissionRequest if command.as_deref().is_some_and(is_destructive_command) => {
            PolicyVerdict::DenyPermissionRequest(permission_request_deny_response())
        }
        _ => PolicyVerdict::AllowNoOutput,
    }
}

pub fn pre_tool_use_deny_response() -> Value {
    json!({
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by Codex Control policy."
      }
    })
}

pub fn permission_request_deny_response() -> Value {
    json!({
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "deny",
          "message": "Blocked by Codex Control policy."
        }
      }
    })
}

fn extract_command(payload: &Value) -> Option<String> {
    payload
        .get("tool_input")
        .and_then(|value| value.get("command"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn is_destructive_command(command: &str) -> bool {
    DESTRUCTIVE_PATTERNS
        .iter()
        .any(|pattern| pattern.is_match(command))
}

#[cfg(test)]
mod tests {
    use crate::{normalize_hook_payload, parse_hook_stdin};

    use super::{evaluate_policy, PolicyVerdict};

    #[test]
    fn denies_destructive_pre_tool_use() {
        let value = parse_hook_stdin(
      r#"{"session_id":"s-1","cwd":"/tmp","hook_event_name":"PreToolUse","tool_input":{"command":"rm -rf /tmp/demo"}}"#,
    )
    .expect("parses json");
        let event = normalize_hook_payload(value).expect("normalizes event");
        let verdict = evaluate_policy(&event);
        assert!(matches!(verdict, PolicyVerdict::DenyPreToolUse(_)));
    }

    #[test]
    fn denies_destructive_permission_request() {
        let value = parse_hook_stdin(
      r#"{"session_id":"s-1","cwd":"/tmp","hook_event_name":"PermissionRequest","tool_input":{"command":"git reset --hard"}}"#,
    )
    .expect("parses json");
        let event = normalize_hook_payload(value).expect("normalizes event");
        let verdict = evaluate_policy(&event);
        assert!(matches!(verdict, PolicyVerdict::DenyPermissionRequest(_)));
    }
}
