# Troubleshooting

## Sessions do not appear

- run `codex-control-hook doctor`
- confirm `codex_hooks = true`
- confirm your `hooks.json` points to `codex-control-hook ingest`
- verify the session is actually producing local hook events

## Hooks do not fire

- validate the matcher values in `hooks.json`
- ensure the hook binary is on `PATH`
- restart the Codex session after updating hook config

## Permission errors

- ensure the desktop app can read the working directory and transcript path
- confirm the local data directory is writable
- on Linux, ensure the terminal and editor commands exist on the system

## Database or spool unavailable

- inspect the settings view for the selected path
- confirm the parent directory exists and is writable
- if SQLite fails, Codex Control should fall back to the JSONL spool

## Stale sessions remain visible

- stale sessions are only downgraded after process discovery confirms they are gone and no recent hook events were seen
- use the clear-local-data action if you want to remove old history entirely

## Transcript is missing

- a missing `transcript_path` is tolerated
- missing transcripts reduce preview fidelity
- the dashboard and timeline should still load without the transcript file
