use assert_cmd::Command;
use predicates::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn hook_command(test_name: &str) -> Command {
    let mut command = Command::cargo_bin("codex-control-hook").expect("binary available");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock")
        .as_nanos();
    command.env(
        "CODEX_CONTROL_DATA_DIR",
        std::env::temp_dir()
            .join("codex-control-tests")
            .join(format!("{test_name}-{nonce}")),
    );
    command
}

#[test]
fn ingest_accepts_valid_json_and_keeps_stdout_empty() {
    hook_command("ingest-valid")
        .arg("ingest")
        .write_stdin(std::fs::read_to_string(fixture("session_start.json")).expect("fixture"))
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn ingest_emit_json_response_returns_exact_json() {
    hook_command("ingest-json-response")
        .args(["ingest", "--emit-json-response"])
        .write_stdin(std::fs::read_to_string(fixture("user_prompt_submit.json")).expect("fixture"))
        .assert()
        .success()
        .stdout(predicate::eq(
            "{\"continue\":true,\"suppressOutput\":false}",
        ));
}

#[test]
fn policy_denies_destructive_pre_tool_use_with_exact_contract() {
    hook_command("policy-pre-tool-use")
        .arg("policy")
        .write_stdin(std::fs::read_to_string(fixture("pre_tool_use.json")).expect("fixture"))
        .assert()
        .success()
        .stdout(predicate::eq(
            "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"Destructive command blocked by Codex Control policy.\"}}",
        ));
}

#[test]
fn policy_denies_destructive_permission_request_with_exact_contract() {
    hook_command("policy-permission-request")
        .arg("policy")
        .write_stdin(std::fs::read_to_string(fixture("permission_request.json")).expect("fixture"))
        .assert()
        .success()
        .stdout(predicate::eq(
            "{\"hookSpecificOutput\":{\"hookEventName\":\"PermissionRequest\",\"decision\":{\"behavior\":\"deny\",\"message\":\"Blocked by Codex Control policy.\"}}}",
        ));
}

#[test]
fn ingest_accepts_unknown_events() {
    hook_command("ingest-unknown")
        .arg("ingest")
        .write_stdin(std::fs::read_to_string(fixture("unknown.json")).expect("fixture"))
        .assert()
        .success();
}

#[test]
fn doctor_prints_human_readable_status() {
    hook_command("doctor")
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Codex Control hook doctor"));
}

#[test]
fn ingest_rejects_invalid_json() {
    hook_command("ingest-invalid-json")
        .arg("ingest")
        .write_stdin("not-json")
        .assert()
        .failure();
}
