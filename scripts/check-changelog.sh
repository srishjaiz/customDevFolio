#!/usr/bin/env bash
# Fail if feat/fix/perf commits (or PR title) exist in range without CHANGELOG.md changes.
# Usage: ./scripts/check-changelog.sh [base_ref] [head_ref]
# Defaults: origin/main..HEAD (or main..HEAD)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BASE_REF="${1:-}"
HEAD_REF="${2:-HEAD}"

if [[ -z "$BASE_REF" ]]; then
  if git rev-parse --verify origin/main >/dev/null 2>&1; then
    BASE_REF="origin/main"
  elif git rev-parse --verify main >/dev/null 2>&1; then
    BASE_REF="main"
  else
    echo "check-changelog: no main ref found; skipping"
    exit 0
  fi
fi

COMMITS_RANGE="${BASE_REF}..${HEAD_REF}"
echo "check-changelog: range ${COMMITS_RANGE}"

needs_changelog=0
reasons=()

PR_TITLE="${PR_TITLE:-}"
if [[ -n "$PR_TITLE" ]]; then
  if [[ "$PR_TITLE" =~ ^(feat|fix|perf)(\(|!|:|$) ]]; then
    needs_changelog=1
    reasons+=("PR title suggests user-facing change: ${PR_TITLE}")
  fi
fi

while IFS= read -r subject; do
  [[ -z "$subject" ]] && continue
  [[ "$subject" =~ ^Merge\  ]] && continue
  if [[ "$subject" =~ \[skip\ changelog\] ]] || [[ "$subject" =~ \[changelog\ skip\] ]]; then
    continue
  fi
  if [[ "$subject" =~ ^(feat|fix|perf)(\(|!|:|$) ]]; then
    needs_changelog=1
    reasons+=("commit: ${subject}")
  fi
done < <(git log --format='%s' "$COMMITS_RANGE" 2>/dev/null || true)

if [[ "$needs_changelog" -eq 0 ]]; then
  echo "check-changelog: no feat/fix/perf commits (or PR title); OK"
  exit 0
fi

if git diff --name-only "$COMMITS_RANGE" -- CHANGELOG.md | grep -q '^CHANGELOG\.md$'; then
  echo "check-changelog: CHANGELOG.md updated in range; OK"
  exit 0
fi

if git log --format='%H' "$COMMITS_RANGE" -- CHANGELOG.md | grep -q .; then
  echo "check-changelog: CHANGELOG.md touched by commits in range; OK"
  exit 0
fi

echo "check-changelog: FAIL — user-facing commits require an update to CHANGELOG.md" >&2
echo "" >&2
echo "Reasons:" >&2
for r in "${reasons[@]}"; do
  echo "  - $r" >&2
done
echo "" >&2
echo "Add an entry under ## [Unreleased] (or the next version) in CHANGELOG.md and commit it." >&2
echo "To skip intentionally, include [skip changelog] in the commit subject (discouraged)." >&2
exit 1
