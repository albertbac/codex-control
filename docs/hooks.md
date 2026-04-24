# Hooks

Codex Control integrates with Codex hooks through the `codex-control-hook` CLI.

Codex command hooks receive one JSON object on `stdin`. Codex treats exit code `0` with no output as success and continues. For `PreToolUse` and `PermissionRequest`, Codex supports hook-specific JSON output for deny decisions.

## Commands

### ingest

~~~bash
codex-control-hook ingest
~~~

Behavior:

* reads one JSON object from `stdin`
* normalizes the event
* preserves unknown fields
* redacts sensitive values before persistence
* stores the event locally
* exits `0` on success
* writes nothing to `stdout` on success
* writes diagnostics to `stderr`

### ingest with JSON response

~~~bash
codex-control-hook ingest --emit-json-response
~~~

On success, this emits only:

~~~json
{"continue":true,"suppressOutput":false}
~~~

### policy

~~~bash
codex-control-hook policy
~~~

Behavior:

* reads one JSON object from `stdin`
* denies destructive `PreToolUse` events with Codex-compatible JSON
* denies destructive `PermissionRequest` events with Codex-compatible JSON
* does not auto-approve escalation
* does not print prose to `stdout`

For a denied `PreToolUse` event, the output shape is:

~~~json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Destructive command blocked by Codex Control policy."
  }
}
~~~

For a denied `PermissionRequest` event, the output shape is:

~~~json
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "deny",
      "message": "Blocked by Codex Control policy."
    }
  }
}
~~~

`PermissionRequest` output must not include `updatedInput`, `updatedPermissions`, or `interrupt`.

## Common input fields

Codex Control accepts these common fields:

* `session_id`
* `transcript_path`
* `cwd`
* `hook_event_name`
* `model`
* `turn_id`

Unknown fields are preserved in the normalized payload.

## Supported events

Codex Control records these events:

* `SessionStart`
* `UserPromptSubmit`
* `PreToolUse`
* `PermissionRequest`
* `PostToolUse`
* `Stop`
* `Unknown`

Unknown events are not treated as errors.

## Example hook files

See:

* `examples/hooks/config.toml`
* `examples/hooks/hooks.json`

The examples use `codex-control-hook ingest` for event capture and `codex-control-hook policy` for deny decisions.

## Local testing

Use sanitized fixtures when testing locally.

~~~bash
printf '%s\n' '{"session_id":"example","transcript_path":null,"cwd":"/tmp/project","hook_event_name":"SessionStart","model":"example-model","source":"startup"}' | codex-control-hook ingest
~~~

The command should exit `0` and print nothing to `stdout`.

For JSON response mode:

~~~bash
printf '%s\n' '{"session_id":"example","transcript_path":null,"cwd":"/tmp/project","hook_event_name":"Stop","model":"example-model"}' | codex-control-hook ingest --emit-json-response
~~~

Expected output:

~~~json
{"continue":true,"suppressOutput":false}
~~~
