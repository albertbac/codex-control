# Codex Control Release Audit

## Summary

This pass focused on public presentation and structural validity only.

No hook behavior was changed. The README, public docs, report, and GitHub Actions workflow were reviewed for raw readability, Markdown/YAML structure, overbroad language, and exposure risk.

## Files Changed

- `README.md`
- `docs/install.md`
- `docs/architecture.md`
- `docs/hooks.md`
- `docs/security.md`
- `docs/troubleshooting.md`
- `.github/workflows/ci.yml`
- `CODEX_CONTROL_REWRITE_REPORT.md`

## Markdown/YAML Formatting Fixed

- Rewrote the public Markdown files as normal multiline documents with headings, paragraphs, lists, and fenced code blocks.
- Moved screenshot language into a dedicated `Screenshots` section in the README.
- Rewrote `.github/workflows/ci.yml` with explicit step names, clear indentation, and multiline shell commands.
- Validated the workflow YAML with a parser.

Line counts after formatting:

- `README.md`: 157 lines
- `docs/install.md`: 85 lines
- `docs/architecture.md`: 106 lines
- `docs/hooks.md`: 115 lines
- `docs/security.md`: 62 lines
- `docs/troubleshooting.md`: 41 lines
- `.github/workflows/ci.yml`: 61 lines

## Technical Fixes

- No product architecture was rewritten.
- No hook contract code was changed.
- The CI workflow still runs dependency installation, Rust formatting, Rust tests, Rust clippy, frontend lint, frontend tests, frontend build, and desktop build.
- No test was removed.
- No failure masking was added.
- No `|| true` was added.

## Exposure Audit

Reviewed surfaces:

- README
- public docs
- CI workflow
- report text
- hook contract documentation
- scan results for code, tests, fixtures, lockfiles, and docs

Result:

- No real secrets were found in public-facing files.
- No token, API key, password, private key, cookie value, authorization header value, personal path, complete hook payload, or sensitive command output was added to public docs or the report.
- Security terms that remain in docs are descriptive and relate to redaction behavior.
- Security terms that remain in code are part of redaction implementation and tests.
- Dependency lockfile matches are package metadata, not exposed credentials.

## Legacy Scan

Outcome:

- The requested legacy-product search was run.
- No matches remained after excluding third-party dependencies.

## Public Wording Scan

Outcome:

- The requested public-wording search was run.
- No public wording problem was found.
- Retained matches are dependency metadata in `package-lock.json` and Cargo lockfile metadata.

## Secret-Oriented Scan

Outcome:

- The requested secret-oriented search was run.
- No real secret values were found.
- Retained matches are documentation of redaction behavior, redaction code, redaction tests, and dependency lockfile metadata.

## Hook Contract Status

The hook contract was preserved.

Verified by the existing Rust tests:

- `codex-control-hook ingest` exits `0` and keeps stdout empty on success.
- `codex-control-hook ingest --emit-json-response` emits only JSON equivalent to `{"continue":true,"suppressOutput":false}`.
- `codex-control-hook policy` denies destructive `PreToolUse` with the required `hookSpecificOutput` shape.
- `codex-control-hook policy` denies destructive `PermissionRequest` with the required `hookSpecificOutput` shape.
- `PermissionRequest` output does not include `updatedInput`, `updatedPermissions`, or `interrupt`.

No hook behavior was edited in this pass.

## Commands Run

### `npm install`

Outcome: passed.

```text
added 63 packages in 384ms
```

### `npm run build`

Outcome: passed.

```text
vite build completed
1636 modules transformed
```

### `npm run lint`

Outcome: passed.

```text
eslint .
cargo fmt --all --check
```

### `npm run test`

Outcome: passed.

```text
Vitest: 1 test passed
Rust workspace: 18 tests passed
```

### `cargo fmt --all --check`

Outcome: passed.

```text
no output
```

### `cargo test --workspace`

Outcome: passed.

```text
hook CLI tests: 7 passed
codex-core tests: 11 passed
doc tests: 0 failed
```

### `cargo clippy --workspace --all-targets -- -D warnings`

Outcome: passed.

```text
Finished dev profile
```

### `.github/workflows/ci.yml` YAML parse

Outcome: passed.

```text
ci_yml_ok
```

### `git diff --check`

Outcome: passed.

```text
no output
```

### GitHub Actions status check

Command used:

```bash
gh run list -R albertbac/codex-control -L 3
```

Outcome:

- Latest remote workflow visible before this commit: success.
- This local formatting pass still requires a push before GitHub Actions can verify the new commit.

## Failures

No local verification command failed in this pass.

## Remaining Gaps

- No real desktop screenshot is published yet.
- No public release artifact is published yet.
- GitHub Actions must run on the commit containing this formatting pass before claiming remote CI for this exact revision.

## Release Readiness

Release readiness: source release candidate, pending a real screenshot, a published release artifact, and GitHub Actions confirmation for the final formatting commit.
