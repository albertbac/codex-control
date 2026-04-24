# Install

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

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
```

Install JavaScript dependencies:

```bash
npm install
```

Build the frontend and Rust workspace:

```bash
npm run build
npm run test
npm run clippy
```

Run the desktop app in development mode:

```bash
npm run tauri:dev
```

Build a desktop bundle:

```bash
npm run tauri:build
```

## Hook CLI

Install the hook CLI from the workspace:

```bash
cargo install --path packages/hook-cli
```

Verify the binary:

```bash
codex-control-hook doctor
```

## Codex hook configuration

The repo includes example files:

* `examples/hooks/config.toml`
* `examples/hooks/hooks.json`

The `config.toml` file enables Codex hooks:

```toml
[features]
codex_hooks = true
```

Place the hook files in the Codex configuration location you use. Common locations are documented by Codex, including the user-level config directory and repo-local config directory.

## Local verification

After setup, run:

```bash
codex-control-hook doctor
npm run test
npm run build
```

If the app opens but no sessions appear, check that hooks are enabled and that `codex-control-hook` is available in the shell path used by Codex.
