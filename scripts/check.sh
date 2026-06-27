#!/usr/bin/env bash
# One-shot local checks aligned with CI (coverage gates run in CI only).
# Usage: ./scripts/check.sh
# Env:
#   SKIP_TEMPLATE=1  — only Rust checks
#   SKIP_RUST=1      — only template checks (if present)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Auto-wire commit template + hooks so contributors skip a separate setup step
"$ROOT/scripts/ensure-git-setup.sh" || true

run_rust=1
run_template=1
[[ "${SKIP_RUST:-}" == "1" ]] && run_rust=0
[[ "${SKIP_TEMPLATE:-}" == "1" ]] && run_template=0

if [[ "$run_rust" -eq 1 ]]; then
  echo "==> cargo fmt"
  cargo fmt --all -- --check
  echo "==> cargo clippy"
  cargo clippy --workspace --all-targets -- -D warnings
  echo "==> cargo test"
  cargo test --workspace
fi

if [[ "$run_template" -eq 1 ]]; then
  if [[ ! -d "$ROOT/template" ]]; then
    echo "check: template/ missing; skipping frontend checks"
  elif ! command -v pnpm >/dev/null 2>&1; then
    echo "check: pnpm not found; skipping template checks (install pnpm or set SKIP_TEMPLATE=1)"
    echo "       CI still runs template typecheck/tests/build on PRs."
  else
    echo "==> template: pnpm install (if needed)"
    (
      cd "$ROOT/template"
      if [[ ! -d node_modules ]]; then
        pnpm install --frozen-lockfile
      fi
      echo "==> template: typecheck"
      pnpm typecheck
      echo "==> template: test"
      pnpm test
      echo "==> template: build"
      pnpm build
    )
  fi
fi

echo ""
echo "All local checks passed."
echo "Note: CI also runs coverage gates (cargo llvm-cov, vitest --coverage) and changelog rules."
