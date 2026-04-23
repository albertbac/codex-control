# Hooks contract

## stdin contract

`codex-control-hook` reads exactly one JSON object from stdin.

Common fields expected from Codex:

- `session_id`
- `transcript_path`
- `cwd`
- `hook_event_name`
- `model`
- `turn_id` when the hook is turn-scoped

Unknown fields are preserved in `payload`.

## stdout behavior

### `codex-control-hook ingest`

- success: writes nothing to stdout
- diagnostics: stderr only

### `codex-control-hook ingest --emit-json-response`

Success emits exactly:

```json
{"continue":true,"suppressOutput":false}
```

### `codex-control-hook policy`

Safe commands:

- exit 0
- stdout empty

Denied `PreToolUse` output:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Destructive command blocked by Codex Control policy."
  }
}
```

Denied `PermissionRequest` output:

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

## Supported events

- `SessionStart`
- `UserPromptSubmit`
- `PreToolUse`
- `PermissionRequest`
- `PostToolUse`
- `Stop`
- unknown events normalize to `Unknown`

## Example tests with stdin piping

```bash
cat packages/hook-cli/tests/fixtures/session_start.json | cargo run -p codex-control-hook -- ingest
cat packages/hook-cli/tests/fixtures/pre_tool_use.json | cargo run -p codex-control-hook -- policy
cat packages/hook-cli/tests/fixtures/post_tool_use_failure.json | cargo run -p codex-control-hook -- ingest --emit-json-response
cargo run -p codex-control-hook -- doctor
```
