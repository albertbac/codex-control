# Troubleshooting

## Hooks do not appear

- Run `codex-control-hook doctor`.
- Confirm `codex_hooks = true` in the Codex config you are actually using.
- Confirm `hooks.json` points to `codex-control-hook ingest`.
- Restart the Codex session after changing hook configuration.

## App starts but shows no sessions

- Start a Codex session that uses the configured hooks.
- Submit a prompt so at least one hook event is emitted.
- Check the settings view for the active database or spool path.
- Confirm the desktop app and hook CLI are using the same local data location.

## Database or spool unavailable

- Confirm the parent directory exists.
- Confirm the current user can write to that directory.
- Run `codex-control-hook doctor` for local path checks.
- If SQLite is unavailable, ingestion should fall back to the JSONL spool.

## Transcript missing

- Missing transcript paths are tolerated.
- The timeline should still show hook events.
- Prompt and assistant previews may be incomplete until a transcript is readable.

## Git not detected

- Confirm the session `cwd` is inside a Git repository.
- Confirm the `git` executable is on `PATH`.
- If a workspace was deleted or moved, Git context will be unavailable for that session.

## Local permissions

- The hook CLI needs write access to the local data directory.
- The desktop app needs read access to the same local data directory.
- Git and transcript inspection require read access to the working directory.
- Quick actions that open terminals or editors depend on the local operating system configuration.
