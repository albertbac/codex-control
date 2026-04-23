# Troubleshooting

## Sessions do not appear

- run `codex-control-hook doctor`
- confirm `codex_hooks = true`
- confirm your `hooks.json` points to `codex-control-hook ingest`
- verify the session is producing local hook events

## Hooks do not fire

- validate the matcher values in `hooks.json`
- ensure the hook binary is on `PATH`
- rerun the session after updating hook config

## Permission errors

- ensure the desktop app can read the working directory and transcript path
- confirm the local data directory is writable
- on Linux, ensure the terminal/editor command exists on the system

## Database or spool unavailable

- inspect the settings page for the selected path
- confirm the parent directory exists and is writable
- if SQLite fails, Codex Control falls back to JSONL spool

## Stale sessions remain visible

- stale sessions are downgraded only after process discovery confirms they are gone and no recent hook events were seen
- use the clear-local-data action if you want to remove old history entirely

## Transcript is missing

- the app tolerates a missing `transcript_path`
- missing transcripts reduce preview fidelity but do not block the timeline or dashboard
