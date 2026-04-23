pub fn sanitize_preview(input: &str) -> String {
  codex_core::redaction::redact_text(input)
}
