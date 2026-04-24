use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Map, Value};

static PRIVATE_KEY_BLOCK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)-----BEGIN [A-Z ]+PRIVATE KEY-----.*?-----END [A-Z ]+PRIVATE KEY-----")
        .expect("valid private key regex")
});
static BEARER_TOKEN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)Bearer\s+[A-Za-z0-9._\-+/=]{16,}").expect("valid bearer regex"));
static AUTHORIZATION_HEADER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(authorization\s*:\s*)([^\r\n]+)").expect("valid authorization header regex")
});
static COOKIE_HEADER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)((set-cookie|cookie)\s*:\s*)([^\r\n]+)").expect("valid cookie header regex")
});
static API_KEY_TOKEN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(sk|rk|pk|ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{16,}\b").expect("valid api key regex")
});
static AWS_ACCESS_KEY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\bAKIA[0-9A-Z]{16}\b").expect("valid aws key regex"));
static SENSITIVE_KEY_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(token|secret|password|passwd|api[_-]?key|private[_-]?key|authorization|cookie|session|access[_-]?token|refresh[_-]?token|client[_-]?secret)")
    .expect("valid sensitive key regex")
});
static ENV_ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][A-Z0-9_]{2,})=([^\s]{8,})").expect("valid env assignment regex")
});
static SESSION_ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(session(?:[_-]?token)?|access[_-]?token|refresh[_-]?token)\b\s*[:=]\s*([^\s,;]+)",
    )
    .expect("valid session assignment regex")
});
static COOKIE_ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(cookie)\b\s*=\s*([^\s,;]+)").expect("valid cookie assignment regex")
});
static PRIVATE_PATH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?x)
    (/Users/[^\s:\"']+)
    |
    (/home/[^\s:\"']+)
    |
    ([A-Za-z]:\\Users\\[^\s:\"']+)
  "#,
    )
    .expect("valid private path regex")
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
    map.iter()
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
    output = AUTHORIZATION_HEADER
        .replace_all(&output, "${1}[REDACTED_AUTH_HEADER]")
        .to_string();
    output = COOKIE_HEADER
        .replace_all(&output, "${1}[REDACTED_COOKIE]")
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
        .replace_all(&output, "${1}=[REDACTED_ENV_VALUE]")
        .to_string();
    output = SESSION_ASSIGNMENT
        .replace_all(&output, "${1}=[REDACTED_TOKEN]")
        .to_string();
    output = COOKIE_ASSIGNMENT
        .replace_all(&output, "${1}=[REDACTED_COOKIE]")
        .to_string();
    redact_entropy_like_tokens(&output)
}

pub fn sanitize_public_output(input: &str) -> String {
    let redacted = redact_text(input);
    PRIVATE_PATH
        .replace_all(&redacted, "[REDACTED_PATH]")
        .to_string()
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
    (has_symbol || has_upper) && has_digit && has_lower
}

#[cfg(test)]
mod tests {
    use super::{redact_text, redact_value, sanitize_public_output};
    use serde_json::json;

    #[test]
    fn redacts_bearer_and_env_assignments() {
        let redacted = redact_text(
            "Authorization: Bearer abcdefghijklmnopqrstuvwxyz123456\nOPENAI_API_KEY=sk-abcdefghijklmnopqrstuv",
        );
        assert!(redacted.contains("Authorization: [REDACTED_AUTH_HEADER]"));
        assert!(redacted.contains("OPENAI_API_KEY=[REDACTED_ENV_VALUE]"));
    }

    #[test]
    fn redacts_cookie_and_session_tokens() {
        let redacted =
            redact_text("Cookie: session=abcdef1234567890\nsession_token=abcdef1234567890");
        assert!(redacted.contains("Cookie: [REDACTED_COOKIE]"));
        assert!(redacted.contains("session_token=[REDACTED_TOKEN]"));
    }

    #[test]
    fn redacts_sensitive_object_keys() {
        let input = json!({"api_key": "super-secret-value", "nested": {"token": "abcdef"}});
        let redacted = redact_value(&input);
        assert_eq!(redacted["api_key"], "[REDACTED_SECRET]");
        assert_eq!(redacted["nested"]["token"], "[REDACTED_SECRET]");
    }

    #[test]
    fn redacts_private_paths_in_public_output() {
        let sanitized = sanitize_public_output(
            "unable to read transcript at /Users/example/private/transcript.jsonl",
        );
        assert!(sanitized.contains("[REDACTED_PATH]"));
    }
}
