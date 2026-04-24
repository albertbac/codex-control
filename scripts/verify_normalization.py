from pathlib import Path
import json

FORBIDDEN = [
    "\u2028", "\u2029", "\u200e", "\u200f",
    "\u202a", "\u202b", "\u202c", "\u202d",
    "\u202e", "\ufeff",
]

MINIMUMS = {
    "README.md": 120,
    "docs/install.md": 60,
    "docs/architecture.md": 40,
    "docs/hooks.md": 100,
    "docs/security.md": 50,
    "docs/troubleshooting.md": 50,
    "CODEX_CONTROL_REWRITE_REPORT.md": 25,
    ".github/workflows/ci.yml": 50,
    "package.json": 20,
    "Cargo.toml": 10,
    "biome.json": 20,
    "examples/hooks/hooks.json": 70,
}

failed = False

for file, minimum in MINIMUMS.items():
    path = Path(file)
    if not path.exists():
        print(f"missing: {file}")
        failed = True
        continue

    data = path.read_bytes()
    if b"\r" in data:
        print(f"{file}: contains CR")
        failed = True

    text = data.decode("utf-8")
    for ch in FORBIDDEN:
        if ch in text:
            print(f"{file}: contains forbidden char U+{ord(ch):04X}")
            failed = True

    count = len(text.splitlines())
    print(f"{file}: {count} lines")
    if count < minimum:
        print(f"{file}: expected at least {minimum} physical lines")
        failed = True

for file in ["package.json", "biome.json", "examples/hooks/hooks.json"]:
    try:
        json.loads(Path(file).read_text(encoding="utf-8"))
    except Exception as exc:
        print(f"{file}: JSON parse failed: {exc}")
        failed = True

if failed:
    raise SystemExit(1)

print("normalization_ok")
