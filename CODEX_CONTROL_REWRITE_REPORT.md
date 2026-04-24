# Codex Control Rewrite Report

## Summary of this pass

This pass focused on the public-facing finish work before wider disclosure.

The goals were:

- rewrite the public docs in maintainer voice
- remove leftover filler language from the public-facing text
- investigate the failing GitHub Actions run
- fix the workflow where there was a concrete, mechanical issue
- record only commands that were actually run
- keep the hook contract unchanged

## Files changed

- `README.md`
- `docs/install.md`
- `docs/architecture.md`
- `docs/hooks.md`
- `docs/security.md`
- `docs/troubleshooting.md`
- rewrite report document
- `.github/workflows/ci.yml`
- `package.json`
- unused screenshot SVG removed

## Text changed

The README was rewritten to sound like a maintainer describing an actual tool, not a generated pitch.

Public docs were reformatted into readable Markdown with explicit headings, clearer shell examples, and shorter sections.

The screenshot image reference was removed. The repo now states only:

> Screenshot coming after the first verified desktop build.

Filler phrases and template-style wording were removed from the public docs in this pass.

## CI status observed before this pass

The latest published GitHub Actions run inspected during this pass was:

- workflow: `ci`
- run: `#2`
- commit: `235ad04`
- status: failure
- exit code: `100`

GitHub Actions currently fails with exit code 100 on the latest published run inspected during this pass.

The available evidence points to the Ubuntu package installation step in `.github/workflows/ci.yml`.

The previous workflow tried to install both `libappindicator3-dev` and `libayatana-appindicator3-dev` on `ubuntu-latest`. For current Ubuntu runners, `libappindicator3-dev` is the risky package in that pair and is the most likely cause of `apt-get install` exiting with code `100`.

## CI fix attempted

The workflow was updated with these changes:

- `actions/checkout@v4` -> `actions/checkout@v5`
- `actions/setup-node@v4` -> `actions/setup-node@v5`
- added `cache: npm`
- changed `npm install` -> `npm ci`
- added `DEBIAN_FRONTEND: noninteractive` to the Ubuntu dependency step
- split the `apt-get install` step across lines for readability and maintenance
- removed `libappindicator3-dev`
- kept the Tauri-relevant Ubuntu packages that match current Linux prerequisite guidance:
  - `libgtk-3-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libayatana-appindicator3-dev`
  - `librsvg2-dev`
  - `patchelf`

This is a real workflow fix attempt. It has not been verified by a completed GitHub Actions run inside this environment yet.

## Commands run locally in this environment

All commands below were run from the repo root.

For Node-based commands in this Codex session, the Homebrew toolchain had to be exposed explicitly:

```bash
export PATH="/opt/homebrew/bin:$PATH"
```

### `cargo fmt --all --check`

Result:

- failed immediately
- exact failure: `zsh: command not found: cargo`

### `cargo test --workspace`

Result:

- failed immediately
- exact failure: `zsh: command not found: cargo`

### `cargo clippy --workspace --all-targets -- -D warnings`

Result:

- failed immediately
- exact failure: `zsh: command not found: cargo`

### `npm install`

Result:

- succeeded

Exact output:

```text
added 63 packages in 481ms
```

### `npm run lint`

Result:

- failed overall
- the frontend lint step completed first
- the root script then failed because it calls Cargo

Exact failure:

```text
sh: cargo: command not found
```

### `npm run test`

Result:

- frontend tests passed
- the root script then failed when it reached the Rust workspace test step

Exact failure tail:

```text
✓ src/lib/grouping.test.ts (1 test)
Test Files  1 passed (1)
Tests  1 passed (1)
sh: cargo: command not found
```

### `npm run build`

Result:

- succeeded

Exact output tail:

```text
vite v5.4.21 building for production...
✓ 1636 modules transformed.
✓ built in 683ms
```

## Technical adjustments retained in this pass

These changes remain part of the repo because they fixed real local issues during this finish pass:

- updated frontend TypeScript config to unblock Vite type resolution
- fixed a nullable timeline load path in the desktop UI
- replaced `replaceAll` in one UI component to avoid avoidable compatibility friction
- changed the desktop build script so it does not emit tracked TypeScript and Vite artifacts
- added `package-lock.json` for reproducible npm installs
- updated `.gitignore` to keep generated build residue out of the repo

## Remaining gaps

- Local Rust/Node verification is required before release.
- Local Node commands were exercised here, but full workspace verification still requires a machine with both Node and Rust available in the default PATH.
- The updated GitHub Actions workflow still needs a fresh run to confirm the Ubuntu package fix.
- No release artifact has been produced yet.
- No real desktop screenshot has been added yet.

## Hook contract status

The hook contract was not changed in this pass.

The following behaviors remain required:

- `codex-control-hook ingest` exits `0` with empty stdout on success
- `codex-control-hook ingest --emit-json-response` emits only:

```json
{"continue":true,"suppressOutput":false}
```

- `codex-control-hook policy` denies destructive `PreToolUse` with:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Destructive command blocked by Codex Control policy."
  }
}
```

- `codex-control-hook policy` denies destructive `PermissionRequest` with:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "deny",
      "message": "Blocked by Codex Control policy."
    }
  }
}
```

## Release readiness

Release readiness: not ready until CI passes and a real screenshot and build artifact exist.
