#!/usr/bin/env bash
# Back-compat alias — prefer `make setup` or just `make check` (auto-configures).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENSURE_GIT_SETUP_VERBOSE=1 exec "$ROOT/scripts/ensure-git-setup.sh"
