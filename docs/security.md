# Security

## Local-only model

Codex Control keeps session data on the local machine.

It does not require a hosted service, remote controller, or analytics endpoint to operate. The hook CLI writes to local storage, and the desktop app reads from local storage.

## Telemetry

No outbound telemetry is enabled by default.

If future integrations are added, they should be opt-in and documented separately.

## Redaction

Redaction runs before persistence and before local UI previews where applicable.

Current redaction targets include:

- likely API keys
- bearer-style tokens
- authorization header values
- cookie header values
- private key blocks
- sensitive `.env` assignments
- long high-entropy token-like strings

Redaction is heuristic. Do not treat it as permission to store or share sensitive data carelessly.

## Hook limits

Hooks are guardrails, not a complete security boundary.

- `PreToolUse` can deny matching shell activity before it runs.
- `PermissionRequest` can deny matching approval requests.
- `PostToolUse` records what happened but cannot undo effects.
- Hook coverage depends on the events Codex is configured to emit.
- Shell policy does not imply universal interception of every tool.

## Approval behavior

Codex Control does not auto-approve permission requests.

Destructive requests covered by the policy command are denied. Non-destructive requests fall back to the normal Codex approval flow.

## Deleting local data

Delete the local application data directory shown by the settings view, or use the dashboard control that clears the local store when available.

Typical locations are listed in [install.md](install.md).

## Auditing configuration

Before relying on a hook setup, inspect the hook file you pointed Codex to.

Minimum checks:

- `codex-control-hook ingest` is present for the events you want to track.
- `codex-control-hook policy` is present only where policy decisions are intended.
- No unexpected shell command runs before or after those entries.
- The hook binary resolves to the version you installed from this repository.
