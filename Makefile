# Contributor shortcuts — prefer these over memorizing long cargo/pnpm lines.
#   make          → same as make check
#   make check    → fmt + clippy + tests (+ template if pnpm available)
#   make setup    → wire commit template + git hooks (also auto-runs from check)
#   make test     → tests only
#   make fmt      → apply rustfmt
#   make help     → list targets

.PHONY: help setup check test fmt clippy cli-test template-test template-install contribute

help:
	@echo "customFolio contributor commands"
	@echo ""
	@echo "  make setup            One-time: commit template + git hooks (auto-run by make check)"
	@echo "  make check            What to run before a PR (fmt, clippy, tests, template)"
	@echo "  make test             cargo test --workspace"
	@echo "  make fmt              cargo fmt --all (apply)"
	@echo "  make clippy           cargo clippy -D warnings"
	@echo "  make template-test    pnpm test in template/ (installs deps if needed)"
	@echo "  make contribute       Print the minimal contribution flow"
	@echo ""
	@echo "CI enforces coverage + changelog; you do not need cargo-llvm-cov locally."

setup:
	@ENSURE_GIT_SETUP_VERBOSE=1 ./scripts/ensure-git-setup.sh
	@echo "Done. Hooks are advisory (warn only); CI is the source of truth."

check:
	@./scripts/check.sh

test:
	@./scripts/ensure-git-setup.sh || true
	cargo test --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

cli-test: test

template-install:
	cd template && pnpm install --frozen-lockfile

template-test:
	@./scripts/ensure-git-setup.sh || true
	@command -v pnpm >/dev/null || (echo "pnpm required for template tests" >&2; exit 1)
	cd template && (test -d node_modules || pnpm install --frozen-lockfile) && pnpm typecheck && pnpm test && pnpm build

contribute:
	@echo "Minimal contribution flow:"
	@echo "  1. git checkout -b your-branch"
	@echo "  2. make changes"
	@echo "  3. make check          # or rely on CI"
	@echo "  4. git commit -m 'type(scope): summary'   # Conventional Commits preferred"
	@echo "  5. git push -u origin HEAD && gh pr create"
	@echo ""
	@echo "For feat/fix/perf: add a bullet under ## [Unreleased] in CHANGELOG.md (CI checks this)."
	@echo "Full notes: CONTRIBUTING.md"

# Default target
.DEFAULT_GOAL := check
