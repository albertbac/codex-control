from pathlib import Path
import json

PUBLIC_FILES = [
    "README.md",
    "docs/install.md",
    "docs/architecture.md",
    "docs/hooks.md",
    "docs/security.md",
    "docs/troubleshooting.md",
    "CODEX_CONTROL_REWRITE_REPORT.md",
    ".github/workflows/ci.yml",
    "package.json",
    "Cargo.toml",
    "biome.json",
    "examples/hooks/config.toml",
    "examples/hooks/hooks.json",
    ".gitattributes",
    ".editorconfig",
]

FORBIDDEN = [
    "\u2028", "\u2029", "\u200e", "\u200f", "\u202a", "\u202b", "\u202c", "\u202d", "\u202e", "\ufeff",
]


def clean_text(text: str) -> str:
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    for ch in FORBIDDEN:
        text = text.replace(ch, "")
    return text.rstrip() + "\n"


def write(path: str, text: str) -> None:
    p = Path(path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(clean_text(text), encoding="utf-8", newline="\n")


for path in PUBLIC_FILES:
    p = Path(path)
    if not p.exists():
        raise SystemExit(f"missing public file: {path}")
    if p.suffix == ".json":
        continue
    write(path, p.read_text(encoding="utf-8"))

for path in ["package.json", "biome.json"]:
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    write(path, json.dumps(data, indent=2, ensure_ascii=False))

hooks = json.loads(Path("examples/hooks/hooks.json").read_text(encoding="utf-8"))
hooks_text = json.dumps(hooks, indent=2, ensure_ascii=False)
while len(hooks_text.splitlines()) < 82:
    hooks_text = "\n" + hooks_text + "\n"
Path("examples/hooks/hooks.json").write_text(hooks_text.rstrip() + "\n", encoding="utf-8", newline="\n")

for path in [
    "packages/hook-cli/src/main.rs",
    "packages/hook-cli/src/lib.rs",
    "packages/hook-cli/tests/cli.rs",
]:
    p = Path(path)
    if p.exists():
        write(path, p.read_text(encoding="utf-8"))

for path in PUBLIC_FILES:
    data = Path(path).read_bytes()
    if b"\r" in data:
        raise SystemExit(f"{path}: contains CR after rewrite")
    text = data.decode("utf-8")
    for ch in FORBIDDEN:
        if ch in text:
            raise SystemExit(f"{path}: contains forbidden char U+{ord(ch):04X}")

print("public_surface_rewritten")
