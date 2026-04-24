# Hooks

## stdin JSON

`codex-control-hook` reads exactly one JSON object from stdin.

Common fields accepted from Codex hook events:

- `session_id`
- `transcript_path`
- `cwd`
- `hook_event_name`
- `model`
- `turn_id` when present

Unknown fields are preserved under `payload` after normalization.

## `ingest` stdout behavior

`codex-control-hook ingest` succeeds quietly:

- success exit code: `0`
- stdout on success: empty
- diagnostics: stderr only
- raw input: not printed

This matters because hook stdout may be interpreted by Codex.

## `ingest --emit-json-response`

When JSON response mode is requested, stdout contains only this JSON value:

```json
{"continue":true,"suppressOutput":false}
```

No explanatory text is printed around it.

## `policy` behavior

`codex-control-hook policy` reads one JSON object from stdin and evaluates the event.

Safe commands:

- exit `0`
- stdout empty

Destructive `PreToolUse` events are denied with:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Destructive command blocked by Codex Control policy."
  }
}
```

Destructive `PermissionRequest` events are denied with:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "deny",
      "message": "Blocked by Codex Control policy."
    }
  }
}
```

`PermissionRequest` output does not include `updatedInput`, `updatedPermissions`, or `interrupt`.

The policy command does not auto-approve escalation.

## Supported events

- `SessionStart`
- `UserPromptSubmit`
- `PreToolUse`
- `PermissionRequest`
- `PostToolUse`
- `Stop`
- unknown event names normalize to `Unknown`

## Sanitized local examples

Run these from the repository root:

```bash
cat packages/hook-cli/tests/fixtures/session_start.json \
  | cargo run -p codex-control-hook -- ingest
```

```bash
cat packages/hook-cli/tests/fixtures/post_tool_use_failure.json \
  | cargo run -p codex-control-hook -- ingest --emit-json-response
```

```bash
cat packages/hook-cli/tests/fixtures/pre_tool_use.json \
  | cargo run -p codex-control-hook -- policy
```

## Testing with fixtures

The fixture files under `packages/hook-cli/tests/fixtures/` are intentionally small and sanitized. They are used to verify parsing, unknown-field preservation, status reduction, redaction, and the exact stdout contract.

Run the hook tests with:

```bash
cargo test -p codex-control-hook
```
