#!/usr/bin/env bash
set -euo pipefail

git show HEAD:README.md | wc -l
git show HEAD:.github/workflows/ci.yml | wc -l
git show HEAD:package.json | wc -l
git show HEAD:Cargo.toml | wc -l
git show HEAD:examples/hooks/hooks.json | wc -l
git show HEAD:packages/hook-cli/src/lib.rs | wc -l
