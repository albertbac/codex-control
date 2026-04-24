# Codex Control Rewrite Report

## Summary

This pass was a final public audit before wider disclosure.

The work focused on four areas:

- tightening public-facing copy so the repo reads like maintained software
- removing public wording that felt templated or provisional
- checking docs, source, fixtures, and runtime surfaces for accidental exposure
- recording only real command results from this environment

The hook contract was kept unchanged.

## Files changed

- `README.md`
- `docs/install.md`
- `docs/architecture.md`
- `docs/hooks.md`
- `docs/security.md`
- `docs/troubleshooting.md`
- `CODEX_CONTROL_REWRITE_REPORT.md`
- `.github/workflows/ci.yml`
- `package.json`
- `apps/desktop/src/app/App.tsx`
- `apps/desktop/src/features/settings/SettingsPanel.tsx`
- `apps/desktop/src/lib/tauri.ts`
- `apps/desktop/src/lib/fallbackData.ts`
- `apps/desktop/src-tauri/src/commands.rs`
- `apps/desktop/src-tauri/src/redaction.rs`
- `apps/desktop/src-tauri/src/session_store.rs`
- `apps/desktop/src-tauri/src/transcript_parser.rs`
- `packages/codex-core/src/lib.rs`
- `packages/codex-core/src/redaction.rs`
- `packages/hook-cli/src/lib.rs`
- removed the legacy screenshot asset file
- renamed the desktop fallback sample data module

## Text changed

The README now reads like project documentation written by a maintainer who uses the tool.

The docs were reformatted into readable Markdown and trimmed back where they were explaining more than the user needs to operate the project safely.

The screenshot note is now explicit and honest:

> No screenshot is published yet. I will add one after the first verified desktop build, not before.

## Public wording cleanup

This pass removed or replaced public wording that made the repo read like setup residue instead of maintained software.

Examples of cleanup in this pass:

- removed the old screenshot asset reference from the public docs
- replaced fallback UI copy that referred to temporary sample state in public-facing terms
- removed text that over-explained runtime internals where the docs only needed to explain behavior and limits
- kept the wording direct, technical, and local-first

## Sensitive exposure audit

Public docs, hook-facing stderr, desktop runtime error mapping, fallback sample data, and the doctor command were reviewed for unnecessary exposure.

Changes made:

- fallback sample data no longer uses personal-looking home-directory paths
- hook CLI stderr now passes through public-output sanitization before printing
- desktop runtime error messages now sanitize sensitive text before returning to the UI
- transcript inspection failures no longer echo the raw transcript path in user-facing errors
- hook doctor no longer prints full local storage paths
- timeline result summaries now sanitize tool-response previews before they reach the desktop UI

No real secrets, API keys, tokens, passwords, private keys, or private cookie values were found in the repository during this pass.

## Secret scan result

A repository-wide secret-oriented scan was reviewed manually.

Result:

- no real secret values found
- no real authorization headers found
- no real cookies found
- no real private keys found

Legitimate retained matches include:

- redaction code and tests under `packages/codex-core/src/redaction.rs`
- contract and model terms such as `session_id`, `token`, and `secret` in schemas, docs, and tests
- dependency metadata in `package-lock.json`

These retained matches are descriptive or defensive, not leaked credentials.

The final wording scan only retained dependency lock metadata from a test package name in `package-lock.json`. No public project copy still uses those terms.

## Legacy scan result

A full legacy-branding scan was rerun after the report cleanup.

Result:

- no relevant matches remained in the repository

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

## CI status observed before this pass

The latest published GitHub Actions run inspected during this pass was:

- workflow: `ci`
- run: `#2`
- commit: `235ad04`
- status: failure
- exit code: `100`

GitHub Actions currently fails with exit code 100 on the latest published run inspected during this pass.

The available evidence points to the Ubuntu package installation step in `.github/workflows/ci.yml`.

## CI fix attempted

The workflow was updated with these changes:

- `actions/checkout@v4` -> `actions/checkout@v5`
- `actions/setup-node@v4` -> `actions/setup-node@v5`
- added `cache: npm`
- changed `npm install` -> `npm ci`
- added `DEBIAN_FRONTEND: noninteractive` to the Ubuntu dependency step
- split the Linux package installation step across lines for readability and maintenance
- removed `libappindicator3-dev`
- kept the Tauri-relevant Ubuntu packages that match current Linux prerequisite guidance:
  - `libgtk-3-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libayatana-appindicator3-dev`
  - `librsvg2-dev`
  - `patchelf`

This is a real workflow fix attempt. It has not been verified by a completed GitHub Actions run inside this environment yet.

## CI/build/test commands run

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
added 63 packages in 378ms
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

The frontend workspace lint finished before the Rust step failed.

### `npm run test`

Result:

- frontend tests passed
- the root script then failed when it reached the Rust workspace test step

Exact failure tail:

```text
✓ src/lib/grouping.test.ts (1 test) 14ms
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
dist/index.html                   0.40 kB │ gzip:  0.27 kB
dist/assets/index-BXvXS5dW.css    4.88 kB │ gzip:  1.70 kB
dist/assets/core-DV6XEvTN.js      0.10 kB │ gzip:  0.11 kB
dist/assets/index-BtKxdfeU.js   164.38 kB │ gzip: 52.33 kB
✓ built in 1.01s
```

## GitHub repo description status

Attempted path:

- `gh repo edit albertbac/codex-control --description "Local visibility for Codex CLI sessions, approvals, hook events, and Git changes."`

Status:

- not executed here because `gh` is not available in this environment

Manual public description required:

> Local visibility for Codex CLI sessions, approvals, hook events, and Git changes.

## Remaining gaps

- Local Rust/Node verification is required before release.
- Local Node commands were exercised here, but full workspace verification still requires a machine with both Node and Rust available in the default PATH.
- The updated GitHub Actions workflow still needs a fresh run to confirm the Ubuntu package fix.
- No release artifact has been produced yet.
- No real desktop screenshot has been added yet.

## Release readiness

Release readiness: not ready until CI passes, Rust verification is complete, and a real desktop screenshot or release artifact exists.
