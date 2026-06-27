#!/usr/bin/env bash
# Configure local git to use repo commit template and hooks (required for contributors).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

git config commit.template "$ROOT/.gitmessage"
git config core.hooksPath "$ROOT/.githooks"

echo "Configured:"
echo "  commit.template = $ROOT/.gitmessage"
echo "  core.hooksPath  = $ROOT/.githooks"
echo ""
echo "Commit messages must follow Conventional Commits."
echo "Pushes with feat/fix/perf require CHANGELOG.md updates on the branch."
echo "PR description starts from .github/PULL_REQUEST_TEMPLATE.md"
