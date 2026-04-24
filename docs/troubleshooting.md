# Troubleshooting

## No sessions appear

Check that Codex hooks are enabled.

~~~toml
[features]
codex_hooks = true
~~~

Confirm that the hook file is installed in the Codex configuration location you use.

Run:

~~~bash
codex-control-hook doctor
~~~

## Hooks are not firing

Confirm that `codex-control-hook` is available in the shell path used by Codex.

Run:

~~~bash
which codex-control-hook
~~~

If the binary is missing, reinstall it:

~~~bash
cargo install --path packages/hook-cli
~~~

## The app opens but stays empty

Start a fresh Codex CLI session after installing the hook files.

Existing sessions may not emit all lifecycle events retroactively.

## The local store is unavailable

Run:

~~~bash
codex-control-hook doctor
~~~

If the database path is unavailable, Codex Control should fall back to the local event spool.

Check local filesystem permissions for the configured data directory.

## Transcript is missing

A transcript path is recorded only when Codex provides one.

Transcript files can also be moved, deleted, or restricted by local permissions. The app should tolerate a missing transcript and continue showing session metadata.

## Git data is missing

Confirm that the session working directory is inside a Git repository.

Run from the session directory:

~~~bash
git rev-parse --show-toplevel
~~~

If the command fails, the app cannot show branch or diff information for that session.

## CI fails locally

Run the checks separately:

~~~bash
npm run build
npm run lint
npm run test
npm run clippy
~~~

Fix the first failing command before rerunning the full set.
