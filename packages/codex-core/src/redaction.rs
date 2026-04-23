use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Map, Value};

static PRIVATE_KEY_BLOCK: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?s)-----BEGIN [A-Z ]+PRIVATE KEY-----.*?-----END [A-Z ]+PRIVATE KEY-----")
    .expect("valid private key regex")
});
static BEARER_TOKEN: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?i)Bearer\s+[A-Za-z0-9._\-+/=]{16,}").expect("valid bearer regex")
});
static API_KEY_TOKEN: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"\b(sk|rk|pk|ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{16,}\b").expect("valid api key regex")
});
static AWS_ACCESS_KEY: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\bAKIA[0-9A-Z]{16}\b").expect("valid aws key regex"));
static SENSITIVE_KEY_NAME: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?i)(token|secret|password|passwd|api[_-]?key|private[_-]?key|authorization)")
    .expect("valid sensitive key regex")
});
static ENV_ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"\b([A-Z][A-Z0-9_]{2,})=([^\s]{8,})").expect("valid env assignment regex")
});

pub fn redact_value(value: &Value) -> Value {
  match value {
    Value::Object(map) => Value::Object(redact_map(map)),
    Value::Array(items) => Value::Array(items.iter().map(redact_value).collect()),
    Value::String(text) => Value::String(redact_text(text)),
    _ => value.clone(),
  }
}

fn redact_map(map: &Map<String, Value>) -> Map<String, Value> {
  map
    .iter()
    .map(|(key, value)| {
      let new_value = if is_sensitive_key(key) {
        redact_sensitive_value(value)
      } else {
        redact_value(value)
      };
      (key.clone(), new_value)
    })
    .collect()
}

fn redact_sensitive_value(value: &Value) -> Value {
  match value {
    Value::Null => Value::Null,
    Value::String(_) => Value::String("[REDACTED_SECRET]".to_string()),
    Value::Array(items) => Value::Array(items.iter().map(redact_sensitive_value).collect()),
    Value::Object(_) => Value::String("[REDACTED_SECRET]".to_string()),
    _ => value.clone(),
  }
}

fn is_sensitive_key(key: &str) -> bool {
  SENSITIVE_KEY_NAME.is_match(key)
}

pub fn redact_text(input: &str) -> String {
  let mut output = input.to_string();
  output = PRIVATE_KEY_BLOCK
    .replace_all(&output, "[REDACTED_PRIVATE_KEY]")
    .to_string();
  output = BEARER_TOKEN
    .replace_all(&output, "Bearer [REDACTED_TOKEN]")
    .to_string();
  output = API_KEY_TOKEN
    .replace_all(&output, "[REDACTED_API_KEY]")
    .to_string();
  output = AWS_ACCESS_KEY
    .replace_all(&output, "[REDACTED_AWS_ACCESS_KEY]")
    .to_string();
  output = ENV_ASSIGNMENT
    .replace_all(&output, "$1=[REDACTED_ENV_VALUE]")
    .to_string();
  redact_entropy_like_tokens(&output)
}

fn redact_entropy_like_tokens(input: &str) -> String {
  input
    .split_whitespace()
    .map(|segment| {
      if segment.len() >= 32 && looks_like_secret(segment) {
        "[REDACTED_TOKEN]".to_string()
      } else {
        segment.to_string()
      }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

fn looks_like_secret(segment: &str) -> bool {
  let has_lower = segment.chars().any(|ch| ch.is_ascii_lowercase());
  let has_upper = segment.chars().any(|ch| ch.is_ascii_uppercase());
  let has_digit = segment.chars().any(|ch| ch.is_ascii_digit());
  let has_symbol = segment.chars().any(|ch| !ch.is_ascii_alphanumeric());
  (has_lower && has_upper && has_digit) || (has_lower && has_digit && has_symbol)
}

#[cfg(test)]
mod tests {
  use super::{redact_text, redact_value};
  use serde_json::json;

  #[test]
  fn redacts_bearer_and_env_assignments() {
    let redacted = redact_text("Authorization: Bearer abcdefghijklmnopqrstuvwxyz123456 OPENAI_API_KEY=sk-abcdefghijklmnopqrstuv");
    assert!(redacted.contains("Bearer [REDACTED_TOKEN]"));
    assert!(redacted.contains("OPENAI_API_KEY=[REDACTED_ENV_VALUE]"));
  }

  #[test]
  fn redacts_sensitive_object_keys() {
    let input = json!({"api_key": "super-secret-value", "nested": {"token": "abcdef"}});
    let redacted = redact_value(&input);
    assert_eq!(redacted["api_key"], "[REDACTED_SECRET]");
    assert_eq!(redacted["nested"]["token"], "[REDACTED_SECRET]");
  }
}
