# Install

## Prerequisites

- Rust stable toolchain
- Node.js 20+
- npm 10+
- Codex CLI on the same machine

## Clone and run the desktop app

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
npm run build
npm run tauri:dev
```

## Install the hook CLI

```bash
cargo install --path packages/hook-cli
```

Confirm the binary is available:

```bash
codex-control-hook doctor
```

## Enable `codex_hooks`

Copy `examples/hooks/config.toml` into the Codex configuration location you use locally and make sure this flag is present:

```toml
[features]
codex_hooks = true
```

## Place `hooks.json`

Copy `examples/hooks/hooks.json` to the hooks path referenced by your Codex configuration.

## Local data paths

Typical paths are:

- macOS database: `~/Library/Application Support/CodexControl/codex-control.db`
- macOS spool: `~/Library/Application Support/CodexControl/spool/events.jsonl`
- Linux database: `~/.local/share/CodexControl/codex-control.db`
- Linux spool: `~/.local/share/CodexControl/spool/events.jsonl`

The desktop settings view should expose the active data paths used on the current machine.
