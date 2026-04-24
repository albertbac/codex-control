# Codex Control

Codex Control is a local desktop dashboard for people who keep several Codex CLI sessions running at once and want one place to see what is actually happening.

When you have Codex working across multiple repos, branches, and terminals, the problem is not starting another session. The problem is remembering which one is waiting for approval, which one just failed a shell command, which one is still active, and which workspace has drifted since the last prompt. Codex Control is built for that specific situation.

## Screenshot

Screenshot coming after first verified desktop build.

## What it does

Codex Control keeps a local record of Codex hook events, enriches that record with repository and process information, and presents the result as a desktop dashboard.

In practice, that means:

- session cards grouped by repository
- a timeline per session with prompts, shell activity, approval requests, and tool results
- quick local actions such as opening the workspace, checking the transcript, and reviewing git state
- local persistence with SQLite first and JSONL fallback if the database is unavailable
- secret redaction before data is written to disk

The app is local-first. It does not depend on a server to be useful.

## Repository layout

- `apps/desktop`: Tauri desktop app, React UI, and local commands
- `packages/codex-core`: shared Rust domain logic for normalization, storage, redaction, repo context, and transcript parsing
- `packages/hook-cli`: the `codex-control-hook` binary used by Codex hooks
- `examples/hooks`: example `config.toml` and `hooks.json`
- `docs`: architecture, install notes, hook contract, security, and troubleshooting

## Install from source

### Prerequisites

- Rust stable toolchain
- Node.js 20+
- npm 10+
- Codex CLI on the same machine

### Clone and run

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
npm run build
npm run tauri:dev
```

## Hook setup

Install the hook binary locally:

```bash
cargo install --path packages/hook-cli
```

Then copy these examples into the Codex configuration you actually use:

- `examples/hooks/config.toml`
- `examples/hooks/hooks.json`

The hook contract is documented in [docs/hooks.md](docs/hooks.md). The CLI behavior matters here: successful ingest writes nothing to stdout unless `--emit-json-response` is explicitly requested.

## Development

```bash
npm install
npm run test
npm run build
cargo test --workspace
npm run tauri:dev
```

## Security model

Codex Control stores data locally and treats hooks as telemetry plus guardrails, not as a universal enforcement layer.

- data stays on the local machine
- destructive shell commands can be denied by policy output
- approval requests are never auto-approved by default
- transcript parsing is best-effort and does not mutate the source file

More detail lives in [docs/security.md](docs/security.md).

## Current limits

- macOS and Linux are the intended targets
- Windows hooks are not documented as production-ready
- session resume is intentionally not exposed as a real action until there is a safe local handoff path
- the current desktop UI uses polling instead of a push channel

If you run Codex in one terminal, you probably do not need this. If you run Codex across several repos and lose track of state, this is the tool.
