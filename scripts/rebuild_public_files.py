from pathlib import Path
import json

FORBIDDEN = [
    "\u2028", "\u2029", "\u200e", "\u200f",
    "\u202a", "\u202b", "\u202c", "\u202d",
    "\u202e", "\ufeff",
]

def clean_text(text: str) -> str:
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    for ch in FORBIDDEN:
        text = text.replace(ch, "")
    return text.rstrip() + "\n"

def write(path: str, text: str) -> None:
    p = Path(path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(clean_text(text), encoding="utf-8", newline="\n")

write("README.md", """# Codex Control

When you run more than one Codex CLI session, the terminal stops being enough.

Codex Control gives you a local desktop view of what each Codex session is doing: which repo it is in, what command it just ran, whether it is waiting for approval, what changed in Git, and where the transcript lives.

It is not a cloud service, not a remote controller, and not an analytics layer. It reads local Codex hook events, enriches them with local process and Git state, and keeps the data on your machine.

## Why this exists

When several Codex CLI sessions are open across different repos, it is hard to remember which one is editing code, which one is waiting for approval, and which one already changed files.

Codex Control keeps that state visible without sending it anywhere.

The target user is a developer who already works with Codex CLI and wants local visibility across sessions without adding a hosted service to the loop.

## What it shows

* Active and recent Codex sessions grouped by repository.
* Session status, model, current working directory, and transcript path.
* Last prompt, last shell command, and last assistant message when available.
* Approval state for shell actions that require a decision.
* Git context: branch, changed files, staged count, unstaged count, and diff summary.
* A per-session timeline of prompts, hook events, shell actions, approvals, failures, and stops.

## What it does not do

* It does not send session data to a server.
* It does not auto-approve Codex permission requests.
* It does not promise complete shell enforcement.
* It does not treat Windows hooks as a supported release target.
* It does not replace reviewing diffs before committing.

## Screenshots

No screenshot is published yet. I will add one after the first verified desktop build, not before.

## Install from source

Requirements:

* Rust stable toolchain.
* Node.js 20 or newer.
* npm 10 or newer.
* Codex CLI installed on the same machine.

Clone and run locally:

~~~bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
npm run build
npm run tauri:dev
~~~

Build a local desktop bundle:

~~~bash
npm run tauri:build
~~~

## Hook setup

Install the hook CLI from the workspace:

~~~bash
cargo install --path packages/hook-cli
~~~

Copy the example hook configuration into the Codex configuration location you use locally:

* `examples/hooks/config.toml`
* `examples/hooks/hooks.json`

The example `config.toml` enables Codex hooks. The example `hooks.json` wires Codex hook events into `codex-control-hook`.

Check the CLI after installation:

~~~bash
codex-control-hook doctor
~~~

## Hook CLI contract

The hook CLI reads one JSON object from `stdin` and preserves unknown fields inside `payload` after normalization.

`codex-control-hook ingest`:

* exits `0` on success
* writes nothing to `stdout` on success
* writes diagnostics to `stderr`
* does not print the raw hook input

`codex-control-hook ingest --emit-json-response` emits only:

~~~json
{"continue":true,"suppressOutput":false}
~~~

`codex-control-hook policy`:

* denies destructive `PreToolUse` and `PermissionRequest` events with Codex-compatible JSON output
* never auto-approves escalation
* does not return `updatedInput`, `updatedPermissions`, or `interrupt` for `PermissionRequest`
* does not print prose to `stdout` when called by a hook

The full hook contract and examples are in [docs/hooks.md](docs/hooks.md).

## Development

Common commands:

~~~bash
npm install
npm run lint
npm run test
npm run build
npm run clippy
npm run tauri:dev
~~~

Rust-only checks:

~~~bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
~~~

## Security model

Codex Control is local-first.

* Session data stays on disk on the local machine.
* Hooks are guardrails and telemetry, not universal interception.
* Destructive shell activity can be denied where the configured hooks see it.
* Transcript parsing is best-effort and never mutates the source transcript.
* Secret redaction runs before persistence.

More detail is in [docs/security.md](docs/security.md).

## Current limitations

* macOS and Linux are the intended targets.
* Windows hooks are not treated as a release target.
* The desktop UI currently uses polling instead of a push transport.
* Session resume is not exposed as an action until there is a safe local handoff path.
* No public release artifact is published yet.

## Roadmap

* Keep CI aligned with the local Rust, Node, and Tauri checks.
* Replace polling with a local push update path once the desktop runtime transport is settled.
* Add a real desktop screenshot after a verified desktop capture.
* Improve stale-session handling with better process correlation.

## License

MIT. See [LICENSE](LICENSE).
""")

write("docs/install.md", """# Install

Codex Control is currently source-first. There is no signed release artifact yet.

## Requirements

Install these before building:

* Rust stable toolchain.
* Node.js 20 or newer.
* npm 10 or newer.
* Codex CLI.
* Linux desktop dependencies required by Tauri, when building on Linux.

## Desktop app

Clone the repo:

~~~bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
~~~

Install JavaScript dependencies:

~~~bash
npm install
~~~

Build the frontend and Rust workspace:

~~~bash
npm run build
npm run test
npm run clippy
~~~

Run the desktop app in development mode:

~~~bash
npm run tauri:dev
~~~

Build a desktop bundle:

~~~bash
npm run tauri:build
~~~

## Hook CLI

Install the hook CLI from the workspace:

~~~bash
cargo install --path packages/hook-cli
~~~

Verify the binary:

~~~bash
codex-control-hook doctor
~~~

## Codex hook configuration

The repo includes example files:

* `examples/hooks/config.toml`
* `examples/hooks/hooks.json`

The `config.toml` file enables Codex hooks:

~~~toml
[features]
codex_hooks = true
~~~

Place the hook files in the Codex configuration location you use.

## Local verification

After setup, run:

~~~bash
codex-control-hook doctor
npm run test
npm run build
~~~

If the app opens but no sessions appear, check that hooks are enabled and that `codex-control-hook` is available in the shell path used by Codex.
""")

write("docs/architecture.md", """# Architecture

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
""")

write("docs/hooks.md", """# Hooks

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
printf '%s\\n' '{"session_id":"example","transcript_path":null,"cwd":"/tmp/project","hook_event_name":"SessionStart","model":"example-model","source":"startup"}' | codex-control-hook ingest
~~~

The command should exit `0` and print nothing to `stdout`.

For JSON response mode:

~~~bash
printf '%s\\n' '{"session_id":"example","transcript_path":null,"cwd":"/tmp/project","hook_event_name":"Stop","model":"example-model"}' | codex-control-hook ingest --emit-json-response
~~~

Expected output:

~~~json
{"continue":true,"suppressOutput":false}
~~~
""")

write("docs/security.md", """# Security

Codex Control is local-first visibility tooling for Codex CLI sessions.

It is not a security sandbox.

## Local storage

Session state is stored on the local machine.

Codex Control does not send session data to a hosted service. There is no telemetry enabled by default.

## Redaction

Sensitive values are redacted before persistence where they match known patterns.

Redaction covers common cases such as API key-like values, bearer credentials, authorization headers, private key blocks, environment-style secret values, long high-entropy tokens, cookies, and session-like values.

Redaction is a safety layer, not a guarantee. Review local data before sharing logs or reports.

## Hook limits

Hooks are useful guardrails. They are not complete enforcement.

`PreToolUse` and `PostToolUse` currently focus on Bash-shaped tool events. They do not cover every possible command path, generated script, non-shell tool, or external workflow.

`PostToolUse` runs after the action it observes. It cannot undo side effects.

## Approval behavior

Codex Control must not auto-approve permission requests by default.

The policy command denies known destructive requests and otherwise leaves Codex approval flow intact.

## Logs and errors

Public output should not contain raw hook payloads, full private paths, credentials, cookies, authorization headers, or unredacted command text.

Diagnostics should be short and sanitized.

## Data deletion

To delete local data, close the app and remove the local store or spool files configured for Codex Control.

If unsure, run:

~~~bash
codex-control-hook doctor
~~~

The doctor command should report status without printing sensitive values.

## Auditing hook configuration

Review the hook files you installed and confirm that they call the expected binary:

~~~bash
codex-control-hook ingest
codex-control-hook policy
~~~

Do not install hook files from an untrusted source.
""")

write("docs/troubleshooting.md", """# Troubleshooting

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
""")

write("CODEX_CONTROL_REWRITE_REPORT.md", """# Codex Control Rewrite Report

## Summary

Codex Control is a local desktop project for visibility into Codex CLI sessions, hook events, approvals, and Git changes.

This report tracks release readiness. It must stay factual and must not claim test, build, CI, or release status without evidence.

## Text normalization status

Public Markdown, YAML, JSON, and TOML files should be UTF-8 text with LF line endings and readable raw GitHub output.

## Hook contract status

The public hook contract is:

* `codex-control-hook ingest` exits `0` with empty `stdout` on success.
* `codex-control-hook ingest --emit-json-response` emits JSON compatible with Codex hook output.
* `codex-control-hook policy` denies destructive `PreToolUse` events with `hookSpecificOutput`.
* `codex-control-hook policy` denies destructive `PermissionRequest` events with `hookSpecificOutput`.
* `codex-control-hook policy` does not auto-approve escalation.
* `PermissionRequest` output must not include `updatedInput`, `updatedPermissions`, or `interrupt`.

## Release readiness

Not release-ready until:

* the latest GitHub Actions run passes
* local Rust checks pass
* local Node checks pass
* raw GitHub files render as readable multiline text
* no secret scan finding remains unreviewed
* a real desktop screenshot or release artifact exists
""")

write(".github/workflows/ci.yml", """name: ci

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

permissions:
  contents: read

jobs:
  validate:
    name: validate
    runs-on: ubuntu-latest
    timeout-minutes: 30

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Linux desktop dependencies
        env:
          DEBIAN_FRONTEND: noninteractive
        run: |
          sudo apt-get update
          sudo apt-get install -y \\
            libgtk-3-dev \\
            libwebkit2gtk-4.1-dev \\
            libayatana-appindicator3-dev \\
            librsvg2-dev \\
            patchelf

      - name: Set up Node.js
        uses: actions/setup-node@v4
        with:
          node-version: "20"
          cache: npm

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install JavaScript dependencies
        run: npm ci

      - name: Build frontend
        run: npm run build

      - name: Lint frontend
        run: npm run lint:frontend

      - name: Check Rust formatting
        run: cargo fmt --all --check

      - name: Test Rust workspace
        run: cargo test --workspace

      - name: Test frontend
        run: npm run test:frontend

      - name: Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings
""")

write("Cargo.toml", """[workspace]
members = [
    "apps/desktop/src-tauri",
    "packages/codex-core",
    "packages/hook-cli",
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT"
version = "0.1.0"
authors = ["Albert Bacelar"]
repository = "https://github.com/albertbac/codex-control"
""")

write("examples/hooks/config.toml", """[features]
codex_hooks = true
""")

package_json = {
    "name": "codex-control",
    "private": True,
    "version": "0.1.0",
    "description": "Local desktop visibility for Codex CLI sessions, hook events, approvals, and Git changes.",
    "workspaces": ["apps/desktop"],
    "scripts": {
        "dev": "npm --workspace apps/desktop run dev",
        "build": "npm --workspace apps/desktop run build",
        "lint": "npm run lint:frontend && npm run lint:rust",
        "lint:frontend": "npm --workspace apps/desktop run lint",
        "lint:rust": "cargo fmt --all --check",
        "format": "npm --workspace apps/desktop run format && cargo fmt --all",
        "test": "npm run test:frontend && npm run test:rust",
        "test:frontend": "npm --workspace apps/desktop run test",
        "test:rust": "cargo test --workspace",
        "clippy": "cargo clippy --workspace --all-targets -- -D warnings",
        "tauri:dev": "npm --workspace apps/desktop run tauri:dev",
        "tauri:build": "npm --workspace apps/desktop run tauri:build"
    }
}
write("package.json", json.dumps(package_json, indent=2))

biome_json = {
    "$schema": "https://biomejs.dev/schemas/2.0.5/schema.json",
    "formatter": {
        "enabled": True,
        "indentStyle": "space",
        "indentWidth": 2,
        "lineWidth": 100
    },
    "linter": {
        "enabled": True,
        "rules": {
            "recommended": True
        }
    },
    "files": {
        "includes": [
            "apps/desktop/src/**/*.ts",
            "apps/desktop/src/**/*.tsx",
            "apps/desktop/vite.config.ts"
        ]
    }
}
write("biome.json", json.dumps(biome_json, indent=2))

hooks_json = {
    "hooks": {
        "SessionStart": [
            {
                "matcher": "startup|resume",
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest",
                        "statusMessage": "Registering Codex session"
                    }
                ]
            }
        ],
        "UserPromptSubmit": [
            {
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest"
                    }
                ]
            }
        ],
        "PreToolUse": [
            {
                "matcher": "Bash",
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest"
                    },
                    {
                        "type": "command",
                        "command": "codex-control-hook policy"
                    }
                ]
            }
        ],
        "PermissionRequest": [
            {
                "matcher": "Bash",
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest"
                    },
                    {
                        "type": "command",
                        "command": "codex-control-hook policy"
                    }
                ]
            }
        ],
        "PostToolUse": [
            {
                "matcher": "Bash",
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest"
                    }
                ]
            }
        ],
        "Stop": [
            {
                "hooks": [
                    {
                        "type": "command",
                        "command": "codex-control-hook ingest",
                        "timeout": 10
                    }
                ]
            }
        ]
    }
}
write("examples/hooks/hooks.json", json.dumps(hooks_json, indent=2))

write(".gitattributes", """* text=auto eol=lf

*.md text eol=lf
*.yml text eol=lf
*.yaml text eol=lf
*.json text eol=lf
*.toml text eol=lf
*.ts text eol=lf
*.tsx text eol=lf
*.rs text eol=lf
*.css text eol=lf
*.html text eol=lf
*.sh text eol=lf
""")

write(".editorconfig", """root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 2
trim_trailing_whitespace = true

[*.rs]
indent_size = 4

[*.md]
trim_trailing_whitespace = false
""")
