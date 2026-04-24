# Codex Control Rewrite Report

## Summary

Codex Control is a local desktop project for visibility into Codex CLI sessions, hook events, approvals, and Git changes.

This report tracks release readiness. It must stay factual and must not claim test, build, CI, or release status without evidence.

## Text normalization status

Public Markdown, YAML, JSON, and TOML files should be UTF-8 text with LF line endings and readable raw GitHub output.

## Hook contract status

The public hook contract is:

* `codex-control-hook ingest` exits `0` with empty `stdout` on success.
* `codex-control-hook ingest --emit-json-response` emits JSON compatible with Codex hook output.
* `codex-control-hook policy` denies destructive `PreToolUse` events with `hookSpecificOutput`.
* `codex-control-hook policy` denies destructive `PermissionRequest` events with `hookSpecificOutput`.
* `codex-control-hook policy` does not auto-approve escalation.
* `PermissionRequest` output must not include `updatedInput`, `updatedPermissions`, or `interrupt`.

## Release readiness

Not release-ready until:

* the latest GitHub Actions run passes
* local Rust checks pass
* local Node checks pass
* raw GitHub files render as readable multiline text
* no secret scan finding remains unreviewed
* a real desktop screenshot or release artifact exists
