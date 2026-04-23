# Install

## Desktop app

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
npm run tauri:dev
```

## Hook CLI

Install the hook CLI into your cargo bin directory:

```bash
cargo install --path packages/hook-cli
```

Confirm the binary is available:

```bash
codex-control-hook doctor
```

## Enable `codex_hooks`

Copy `examples/hooks/config.toml` into the Codex configuration location you use locally, then make sure `codex_hooks = true` is present.

## Place `hooks.json`

Copy `examples/hooks/hooks.json` to the path referenced by your Codex configuration.

## Local data paths

Typical paths:

- macOS database: `~/Library/Application Support/CodexControl/codex-control.db`
- macOS spool: `~/Library/Application Support/CodexControl/spool/events.jsonl`
- Linux database: `~/.local/share/CodexControl/codex-control.db`
- Linux spool: `~/.local/share/CodexControl/spool/events.jsonl`

The settings page also shows the active paths used by the current machine.
