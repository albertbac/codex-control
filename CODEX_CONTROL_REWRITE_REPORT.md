# Codex Control Release Audit

## Summary

This pass was a final technical, editorial, and exposure audit before public presentation.

The repository now has verified local Rust, Node, and Tauri checks. The public docs avoid broad claims, the hook contract is covered by tests, and the desktop build now produces a local macOS app bundle.

## Files Changed

- `.gitignore`
- `README.md`
- `.github/workflows/ci.yml`
- `package.json`
- `Cargo.lock`
- `apps/desktop/src/components/SessionCard.tsx`
- `apps/desktop/src/lib/fallbackData.ts`
- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/icons/icon.svg`
- `apps/desktop/src-tauri/icons/icon.png`
- Rust files under `apps/desktop/src-tauri/src/`
- Rust files under `packages/codex-core/src/`
- Rust files under `packages/hook-cli/src/`
- `packages/hook-cli/tests/cli.rs`
- `CODEX_CONTROL_REWRITE_REPORT.md`

## Technical Fixes

- Applied `cargo fmt` across the Rust workspace so CI format checks have the same baseline locally.
- Added `Cargo.lock`, which is appropriate for this application-style workspace.
- Added the missing Tauri icon asset required by `tauri::generate_context!()`.
- Fixed the Tauri `beforeBuildCommand` and `beforeDevCommand` so workspace builds run from the desktop package instead of resolving the wrong package path.
- Added a CI desktop build step so the workflow checks the Tauri app, not only the web bundle.
- Ignored Tauri-generated schema files under `apps/desktop/src-tauri/gen/`.
- Fixed `DashboardSession` construction to match the shared Rust model shape.
- Fixed missing imports in store tests.
- Added a `CODEX_CONTROL_DATA_DIR` override so CLI tests can write to an isolated temporary store instead of relying on an OS application directory.
- Stabilized hook policy JSON output with explicit contract strings for exact stdout tests.
- Strengthened redaction replacements for authorization headers, cookies, environment-style values, and session-style values.
- Fixed clippy findings without weakening tests.
- Kept the invalid nested interactive markup fix in the session card.
- Kept the safer Git inspection behavior: a missing workspace no longer falls back to inspecting the app repository itself.

## Editorial Cleanup

- `README.md` remains direct and maintainer-written.
- No public screenshot is claimed.
- The README now distinguishes local source builds from public release artifacts.
- Public docs avoid hype, broad production claims, fake badges, and old-product framing.

## Sanitization

Reviewed surfaces:

- README and docs
- hook CLI stdout and stderr behavior
- Tauri command errors
- fallback UI data
- hook fixtures
- report text
- public UI copy

Retained protections:

- hook CLI stderr passes through public-output sanitization
- desktop runtime errors sanitize sensitive text before returning to the UI
- transcript inspection errors do not echo raw transcript paths
- hook doctor does not print full local storage paths
- timeline result previews are sanitized before display
- fallback UI data avoids personal-looking paths and destructive commands

No real secrets, API keys, passwords, authorization headers, private keys, cookie values, or private paths were found in public-facing files.

## Hook Contract Status

The hook contract was preserved.

Verified behavior:

- `codex-control-hook ingest` exits `0` with empty stdout on success
- `codex-control-hook ingest` writes diagnostics only to stderr
- `codex-control-hook ingest --emit-json-response` emits only valid JSON equivalent to `{"continue":true,"suppressOutput":false}`
- `codex-control-hook policy` denies destructive `PreToolUse` with the required `hookSpecificOutput`
- `codex-control-hook policy` denies destructive `PermissionRequest` with the required `hookSpecificOutput`
- `PermissionRequest` output does not include `updatedInput`, `updatedPermissions`, or `interrupt`
- hook commands do not print prose to stdout when JSON output is required

## Scans

Legacy product scan:

- result: no relevant matches

Public wording scan:

- result: no public problematic matches
- retained matches are dependency lock metadata and Cargo lock metadata

Secret-oriented scan:

- result: no real secret values found
- retained matches are schema names, redaction code, tests, fixtures, documentation terms, and dependency metadata
- destructive command examples remain only in policy tests and hook fixtures because they verify deny behavior

Whitespace scan:

- result: `git diff --check` passed

## Commands Run

### `npm install`

Result: passed

```text
added 63 packages in 378ms
```

### `cargo fmt --all --check`

Result: passed

```text
no output
```

### `cargo test --workspace`

Result: passed

```text
hook CLI tests: 7 passed
codex-core tests: 11 passed
doc tests: 0 failed
```

### `cargo clippy --workspace --all-targets -- -D warnings`

Result: passed

```text
Finished dev profile
```

### `npm run lint`

Result: passed

```text
eslint .
cargo fmt --all --check
```

### `npm run test`

Result: passed

```text
Vitest: 1 test passed
Rust workspace: 18 tests passed
```

### `npm run build`

Result: passed

```text
vite build completed
1636 modules transformed
```

### `npm run clippy`

Result: passed

```text
cargo clippy --workspace --all-targets -- -D warnings
```

### `npm run tauri:build`

Initial result: failed

Cause: the Tauri `beforeBuildCommand` resolved to the wrong package directory.

Final result after fix: passed

```text
Built application
Finished 1 macOS app bundle
```

## GitHub Repository Description

Updated through GitHub CLI.

Current description:

```text
Local visibility for Codex CLI sessions, approvals, hook events, and Git changes.
```

## CI Status

The last remote CI run inspected before this commit failed on Rust formatting.

Local verification on this commit now passes:

- Rust format
- Rust tests
- Rust clippy
- frontend lint
- frontend tests
- frontend build
- Tauri desktop build

A fresh GitHub Actions run is still required after pushing this commit.

## Remaining Gaps

- No real desktop screenshot is published yet.
- No GitHub release artifact is published yet.
- Current-commit CI must complete successfully after push before public release is called verified.

## Release Readiness

Release readiness: source release candidate locally, pending current-commit CI, a real desktop screenshot, and a public release artifact.
