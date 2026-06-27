#!/usr/bin/env bash
# Idempotent: point this clone at repo commit template + hooks (no-op if already set).
# Safe to call from make / check scripts; does not require a separate "setup" step.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Only configure when inside a git work tree
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  exit 0
fi

want_template="$ROOT/.gitmessage"
want_hooks="$ROOT/.githooks"
cur_template="$(git config --local --get commit.template 2>/dev/null || true)"
cur_hooks="$(git config --local --get core.hooksPath 2>/dev/null || true)"

changed=0
if [[ "$cur_template" != "$want_template" ]]; then
  git config --local commit.template "$want_template"
  changed=1
fi
if [[ "$cur_hooks" != "$want_hooks" ]]; then
  git config --local core.hooksPath "$want_hooks"
  changed=1
fi

if [[ "${ENSURE_GIT_SETUP_VERBOSE:-}" == "1" && "$changed" -eq 1 ]]; then
  echo "ensure-git-setup: configured commit.template and core.hooksPath for this clone"
fi
