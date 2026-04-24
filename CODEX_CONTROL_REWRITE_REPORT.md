# Codex Control Release Audit

## Summary

This pass was a final release audit for the public repository.

The work focused on:

- fixing real technical issues found during review
- removing public wording that read like generated setup material
- reducing accidental exposure in docs, UI copy, runtime errors, and test surfaces
- keeping the Codex hook contract unchanged
- recording only checks that were actually run

## Technical fixes

- Fixed invalid nested interactive markup in the session card. The card is now an `article` with keyboard selection, and the quick actions remain real buttons.
- Replaced the diff action and terminate action icons with less theatrical, more operational icons.
- Removed a destructive command from fallback UI data. The fallback data now uses non-destructive commands.
- Changed Git inspection so missing workspaces do not fall back to inspecting the app repository itself.
- Split root scripts into explicit frontend and Rust commands:
  - `lint:frontend`
  - `lint:rust`
  - `test:frontend`
  - `test:rust`
  - `clippy`
- Updated CI to call frontend lint/test scripts directly after the separate Rust steps.
- Updated Windows wording so the docs do not use release-claim language for unsupported hook workflows.

## Editorial cleanup

- `README.md` keeps a direct maintainer voice.
- The screenshot section is explicit:

```text
No screenshot is published yet. I will add one after the first verified desktop build, not before.
```

- Public docs avoid broad claims, hype language, fake screenshots, fake badges, and generated-sounding framing.
- Public wording avoids the previous product legacy.

## Sanitization

Reviewed surfaces:

- README and docs
- hook CLI stderr behavior
- Tauri command errors
- fallback UI data
- hook fixtures
- report text
- public UI copy

Changes retained:

- hook CLI stderr passes through public-output sanitization
- desktop runtime errors sanitize sensitive text before returning to the UI
- transcript inspection errors no longer echo the raw transcript path
- hook doctor does not print full local storage paths
- timeline result previews are sanitized before display
- fallback data avoids personal-looking paths

No real secrets, API keys, passwords, authorization headers, private keys, or cookie values were found.

## Hook Contract Status

The hook contract was not changed.

Required behavior remains:

- `codex-control-hook ingest` exits `0` with empty stdout on success
- `codex-control-hook ingest` writes diagnostics only to stderr
- `codex-control-hook ingest --emit-json-response` emits only:

```json
{"continue":true,"suppressOutput":false}
```

- `codex-control-hook policy` denies destructive `PreToolUse` with the required `hookSpecificOutput`
- `codex-control-hook policy` denies destructive `PermissionRequest` with the required `hookSpecificOutput`
- `PermissionRequest` output does not include `updatedInput`, `updatedPermissions`, or `interrupt`
- hook commands do not print non-JSON prose to stdout when JSON output is required

## Scans

Legacy product scan:

- result: no relevant matches

Public wording scan:

- result: no public problematic matches
- retained match: dependency lock metadata for a test package name in `package-lock.json`

Secret-oriented scan:

- result: no real secret values found
- retained matches are schema names, redaction code, tests, fixtures, documentation terms, and dependency metadata
- destructive command examples remain only in policy tests and hook fixtures because they verify the deny contract

## Commands Run

### `npm install`

Result: passed

```text
added 63 packages in 566ms
```

### `npm run lint:frontend`

Result: passed

```text
eslint .
```

### `npm run test:frontend`

Result: passed

```text
Test Files  1 passed (1)
Tests  1 passed (1)
```

### `npm run build`

Result: passed

```text
vite v5.4.21 building for production...
✓ 1636 modules transformed.
✓ built in 663ms
```

### `npm run lint`

Result: failed after frontend lint passed

```text
sh: cargo: command not found
```

Impact: the root lint command still requires the Rust toolchain, which is expected for the full workspace but unavailable in this environment.

### `npm run test`

Result: failed after frontend tests passed

```text
sh: cargo: command not found
```

Impact: the root test command still requires the Rust toolchain, which is expected for the full workspace but unavailable in this environment.

### `cargo fmt --all --check`

Result: not executed successfully

```text
zsh:1: command not found: cargo
```

### `cargo test --workspace`

Result: not executed successfully

```text
zsh:1: command not found: cargo
```

### `cargo clippy --workspace --all-targets -- -D warnings`

Result: not executed successfully

```text
zsh:1: command not found: cargo
```

## GitHub Repository Description

Updated through GitHub CLI.

Current description:

```text
Local visibility for Codex CLI sessions, approvals, hook events, and Git changes.
```

## CI Status

The last public GitHub Actions run inspected before these final changes still showed failure with exit code `100`.

The workflow has been adjusted, but this final commit still needs a fresh completed GitHub Actions run before release can be called verified.

## Remaining Gaps

- Rust toolchain is not available in this local environment.
- Rust format, Rust tests, and Clippy still need to run in CI or on a machine with Rust installed.
- No release artifact has been produced.
- No real desktop screenshot has been added.
- CI must pass on the current commit before this should be treated as release-ready.

## Release Readiness

Release readiness: not ready until CI passes, Rust verification is complete, and a real desktop screenshot or release artifact exists.
