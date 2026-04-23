use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSummary {
  pub last_assistant_message: Option<String>,
  pub last_user_prompt: Option<String>,
  pub last_command: Option<String>,
  pub preview: Option<String>,
}

pub fn parse_transcript(path: &Path) -> TranscriptSummary {
  let Ok(content) = fs::read_to_string(path) else {
    return TranscriptSummary::default();
  };

  let mut summary = TranscriptSummary::default();
  let mut preview_lines: Vec<String> = Vec::new();

  for line in content.lines().rev().take(200) {
    if preview_lines.len() < 8 {
      preview_lines.push(line.to_string());
    }

    if let Ok(json) = serde_json::from_str::<Value>(line) {
      if summary.last_assistant_message.is_none() {
        summary.last_assistant_message = extract_textual_message(&json, "assistant");
      }
      if summary.last_user_prompt.is_none() {
        summary.last_user_prompt = extract_textual_message(&json, "user");
      }
      if summary.last_command.is_none() {
        summary.last_command = json
          .get("tool_input")
          .and_then(|value| value.get("command"))
          .and_then(Value::as_str)
          .map(str::to_string);
      }
    }
  }

  preview_lines.reverse();
  let preview = preview_lines
    .into_iter()
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n");
  if !preview.trim().is_empty() {
    summary.preview = Some(preview);
  }

  summary
}

fn extract_textual_message(value: &Value, role: &str) -> Option<String> {
  if value.get("role").and_then(Value::as_str) != Some(role) {
    return None;
  }

  if let Some(content) = value.get("content") {
    if let Some(text) = content.as_str() {
      return Some(text.to_string());
    }
    if let Some(items) = content.as_array() {
      let collected = items
        .iter()
        .filter_map(|item| item.get("text").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join(" ");
      if !collected.is_empty() {
        return Some(collected);
      }
    }
  }

  None
}
