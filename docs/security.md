# Security

## Local-first behavior

Codex Control keeps session data on the local machine.

- no server sync is required
- no remote controller is involved
- no default outbound telemetry is enabled

## Storage

Persisted data lives in the local application data directory:

- SQLite database for the primary store
- JSONL spool file when the database is unavailable

## Hook boundaries

Hooks are guardrails, not a complete enforcement boundary.

- `PreToolUse` and `PermissionRequest` can deny destructive shell activity that matches configured hooks
- `PostToolUse` cannot undo work that already ran
- shell hooks do not imply universal interception of every tool
- the current policy logic is scoped to Bash-style payloads

## Approval safety

Approval requests are never auto-approved by default.

- destructive approval requests are denied by policy output
- non-destructive approval requests fall back to the standard Codex approval flow

## Secret redaction

Redaction runs before persistence.

Current targets include:

- likely API keys
- Bearer tokens
- private keys
- sensitive `.env` values
- long high-entropy tokens

Redaction is heuristic. If you plan to share local data, inspect it first.

## Deleting local data

You can remove local state by deleting the application data directory described in [install.md](install.md), or by using the dashboard control that clears the local store.

## Auditing installed hooks

Review the hook file you point Codex to in your local configuration.

Minimum checks:

- `codex-control-hook ingest` is the only ingestion command
- `codex-control-hook policy` is only present where approval or shell policy is intended
- no unexpected shell command runs before or after those entries
