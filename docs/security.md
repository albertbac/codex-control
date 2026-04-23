# Security

## Local-first

Codex Control is local-first.

- session data is stored locally
- no server sync is required
- no default outbound telemetry is enabled

## Storage

Persisted data lives in:

- SQLite database in the local application data directory
- JSONL spool fallback in the same local data directory

## Hook boundaries

Hooks are guardrails, not a perfect enforcement boundary.

- `PreToolUse` and `PermissionRequest` can deny destructive shell activity that matches configured hooks
- `PostToolUse` cannot undo work that already ran
- shell hooks do not imply universal interception of every tool
- the first release only treats Bash payloads as policy-aware events

## Approval safety

Approval requests are never auto-approved by default.

- destructive approval requests are explicitly denied by policy output
- non-destructive approval requests fall back to the normal Codex approval flow

## Secret redaction

Redaction runs before persistence.

Targets include:

- likely API keys
- Bearer tokens
- private keys
- sensitive `.env` values
- long high-entropy tokens

Redaction is heuristic. Review stored data before sharing the local database.

## Deleting local data

You can remove local state manually by deleting the application data directory described in `docs/install.md`, or by using the dashboard control that clears the local store.

## Auditing installed hooks

Audit installed hooks by reviewing the file you point Codex to in your config.

Minimum review points:

- `codex-control-hook ingest` is the only ingestion command
- `codex-control-hook policy` is present only where approval or shell policy is desired
- no unexpected shell command is executed before or after these entries
