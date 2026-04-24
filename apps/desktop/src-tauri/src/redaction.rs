pub fn sanitize_preview(input: &str) -> String {
    codex_core::redaction::redact_text(input)
}

pub fn sanitize_runtime_message(input: &str) -> String {
    codex_core::sanitize_public_output(input)
}
