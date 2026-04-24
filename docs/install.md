# Install

## Requirements

- Rust stable toolchain
- Node.js 20 or newer
- npm 10 or newer
- Codex CLI installed on the same machine
- macOS or Linux for the intended desktop target

Linux desktop builds also need the native libraries used by Tauri and WebKitGTK. The CI workflow documents the Ubuntu package list used for that environment.

## Install the desktop app from source

Clone the repository and install workspace dependencies:

```bash
git clone https://github.com/albertbac/codex-control.git
cd codex-control
npm install
```

Run the development desktop app:

```bash
npm run tauri:dev
```

Build the web bundle and local desktop bundle:

```bash
npm run build
npm run tauri:build
```

No public release artifact is published yet. Until there is one, source builds are the intended path.

## Install the hook CLI

Install the CLI binary from the local workspace:

```bash
cargo install --path packages/hook-cli
```

Check that it is available:

```bash
codex-control-hook doctor
```

## Configure Codex hooks

Use the example files as a starting point:

- `examples/hooks/config.toml`
- `examples/hooks/hooks.json`

The config file must enable Codex hooks:

```toml
[features]
codex_hooks = true
```

The hooks file should call `codex-control-hook ingest` for session events and `codex-control-hook policy` where shell policy decisions are expected.

## Verify local operation

Run these checks from the repository root:

```bash
cargo test --workspace
npm run test
codex-control-hook doctor
```

Then start a Codex session that uses the configured hooks and open the desktop app. New hook events should appear in the dashboard after the session emits events.

## Short troubleshooting

- If sessions do not appear, run `codex-control-hook doctor` and check the hook path.
- If the desktop app starts but stays empty, confirm the Codex session is using the same hook configuration you edited.
- If the local store is not writable, fix permissions on the application data directory shown by the settings view.
- If transcripts are missing, the timeline still loads but preview detail is reduced.
