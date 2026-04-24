# Architecture

Codex Control is a local desktop app. It does not require a hosted service.

The flow is:

~~~text
Codex hook event
  -> codex-control-hook
  -> local store
  -> desktop app
  -> session dashboard
~~~

## Hook ingestion

`codex-control-hook` reads one JSON object from `stdin` for each hook invocation.

The CLI normalizes the event, redacts sensitive values, stores the result locally, and exits using the Codex-compatible hook contract.

Unknown fields are preserved inside the event payload. Unknown event names are stored as `Unknown` rather than being dropped.

## Session store

The session store keeps normalized events, session status, workspace metadata, timestamps, last prompt, last command, and last assistant message when available.

SQLite is the preferred storage path. A JSONL spool is used when the database is not available.

## Process discovery

Process discovery enriches session state. It is not the only source of truth.

Hooks are the primary event source. Process discovery helps identify active local Codex processes, stale sessions, and local workspace context.

## Git inspection

Git inspection is local and best-effort.

The app collects repository root, branch name, changed file count, staged count, unstaged count, and diff summary.

Git inspection does not modify the repository.

## Transcript handling

When Codex provides a transcript path, Codex Control records the path and reads it on a best-effort basis.

Transcript parsing must tolerate missing files, changed formats, and permission errors. It must not mutate the transcript source.

## Desktop update flow

The desktop app reads from the local store and shows session state in the UI.

The current implementation uses polling. A local push update path is planned after the runtime transport is settled.

## Boundaries

Codex Control is visibility tooling. It is not a remote executor and not a complete security boundary.

Hooks are useful guardrails, but they do not intercept every possible shell or non-shell action.
