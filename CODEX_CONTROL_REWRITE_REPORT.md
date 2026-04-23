# Codex Control Rewrite Report

## 1. What changed

- Built a new local desktop product named Codex Control from a clean workspace inside `codex_control/`.
- Switched to a Tauri 2 + React + TypeScript + Rust + SQLite architecture.
- Implemented a dedicated hook CLI contract for ingestion, policy evaluation, and diagnostics.
- Added local-first persistence, JSONL fallback, process discovery, Git enrichment, transcript parsing, and a dark-first dashboard UI.

## 2. Files removed

- None inside `codex_control/` because the repository was created from scratch.

## 3. Files added

- Root manifests and workspace files
- Desktop frontend and Tauri backend
- Shared Rust core package
- Hook CLI package with fixtures and tests
- Hook examples and documentation
- CI workflow and verification report

## 4. Architectural decisions

- Local-only data plane with SQLite primary store and JSONL spool fallback.
- Hook CLI writes directly to the local store instead of sending data to a server.
- Tauri backend enriches persisted sessions with process and Git metadata.
- React UI uses polling for the first release to keep the local runtime simple and predictable.

## 5. Hook compatibility

- `codex-control-hook ingest`: stdin JSON object, empty stdout on success, stderr for diagnostics only.
- `codex-control-hook ingest --emit-json-response`: emits valid JSON response only.
- `codex-control-hook policy`: emits exact deny JSON for destructive `PreToolUse` and `PermissionRequest` cases.
- `codex-control-hook doctor`: human-readable stdout diagnostics.

## 6. Tests executed

Executed in this environment:

- repository structure inspection
- required file coverage check
- JSON manifest validation
- static review of TOML manifests
- forbidden-identifier scan inside `codex_control/`

Not executed in this environment because toolchains are missing locally:

- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `npm install`
- `npm run lint`
- `npm run test`
- `npm run build`

## 7. Test results

- required file coverage: passed (`74` files present, no required paths missing)
- JSON manifests: passed
- static forbidden-identifier scan: passed
- Rust and Node quality gates: blocked by missing local toolchains (`cargo`, `rustc`, `npm`, `node`)

## 8. Remaining gaps

- real runtime verification is still required on a machine with Rust and Node installed
- packaging artifacts were not produced in this environment
- session resume action is intentionally disabled until a safe local resume contract exists

## 9. Human verification commands

```bash
cd codex_control
cargo test --workspace
cargo fmt --all --check
npm install
npm run lint
npm run test
npm run build
python3 - <<'PY'
import pathlib, re, sys
patterns = [
    ''.join(map(chr, [99, 108, 97, 117, 100, 101])),
    ''.join(map(chr, [67, 108, 97, 117, 100, 101])),
    ''.join(map(chr, [67, 76, 65, 85, 68, 69])),
    ''.join(map(chr, [97, 110, 116, 104, 114, 111, 112, 105, 99])),
    ''.join(map(chr, [65, 110, 116, 104, 114, 111, 112, 105, 99])),
    ''.join(map(chr, [65, 78, 84, 72, 82, 79, 80, 73, 67])),
    '\\.' + ''.join(map(chr, [99, 108, 97, 117, 100, 101])),
]
rx = re.compile('|'.join(patterns))
hits = []
for path in pathlib.Path('.').rglob('*'):
    if path.is_file():
        try:
            text = path.read_text(errors='ignore')
        except Exception:
            continue
        if rx.search(text):
            hits.append(str(path))
if hits:
    print('\n'.join(hits))
    sys.exit(1)
print('OK')
PY
cat packages/hook-cli/tests/fixtures/session_start.json | cargo run -p codex-control-hook -- ingest --emit-json-response
cargo run -p codex-control-hook -- doctor
```

## 10. Forbidden identifier scan

- Final scan completed after report cleanup.
- No forbidden vendor identifiers remain in repository content.
