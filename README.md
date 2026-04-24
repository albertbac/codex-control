# Codex Control

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

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
npm run build
npm run tauri:dev
````

Build a local desktop bundle:

```bash
npm run tauri:build
```

## Hook setup

Install the hook CLI from the workspace:

```bash
cargo install --path packages/hook-cli
```

Copy the example hook configuration into the Codex configuration location you use locally:

* `examples/hooks/config.toml`
* `examples/hooks/hooks.json`

The example `config.toml` enables Codex hooks. The example `hooks.json` wires Codex hook events into `codex-control-hook`.

Check the CLI after installation:

```bash
codex-control-hook doctor
```

## Hook CLI contract

The hook CLI reads one JSON object from `stdin` and preserves unknown fields inside `payload` after normalization.

`codex-control-hook ingest`:

* exits `0` on success
* writes nothing to `stdout` on success
* writes diagnostics to `stderr`
* does not print the raw hook input

`codex-control-hook ingest --emit-json-response` emits only:

```json
{"continue":true,"suppressOutput":false}
```

`codex-control-hook policy`:

* denies destructive `PreToolUse` and `PermissionRequest` events with Codex-compatible JSON output
* never auto-approves escalation
* does not return `updatedInput`, `updatedPermissions`, or `interrupt` for `PermissionRequest`
* does not print prose to `stdout` when called by a hook

The full hook contract and examples are in [docs/hooks.md](docs/hooks.md).

## Development

Common commands:

```bash
npm install
npm run lint
npm run test
npm run build
npm run clippy
npm run tauri:dev
```

Rust-only checks:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

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
