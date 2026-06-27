# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) where applicable.

## [Unreleased]

### Changed

- **Contributor DX:** `make check` is the single local entrypoint (auto-configures commit template + hooks); git hooks are advisory only; CI remains the hard gate. Shorter `CONTRIBUTING.md` and PR template; no mandatory `./scripts/setup-git.sh` step.

### Added

- `Makefile` and `scripts/check.sh` / `scripts/ensure-git-setup.sh` for one-command local validation aligned with CI (without requiring `cargo-llvm-cov` locally).
- Expanded CLI unit and integration tests (`config`, `domain`, `scaffold`, `prompts`, `cli`, `cli_smoke`) for high coverage of domain presets, scaffolding, and flag parsing.
- Template unit tests with Vitest (`template/lib/*.test.ts`) covering `utils`, `domains`, and `portfolio` helpers; `pnpm test` / `pnpm test:coverage` scripts.
- CI: Rust job runs `cargo llvm-cov` with an 80% line-coverage gate; template job runs Vitest with coverage thresholds; coverage artifacts uploaded on PRs/pushes.
- Pull request template (`.github/PULL_REQUEST_TEMPLATE.md`) with summary, changelog, and test-plan sections.
- Commit message template (`.gitmessage`) and contributor setup script (`scripts/setup-git.sh`).
- Git hooks (`.githooks/`): Conventional Commits on `commit-msg`, changelog reminder on `pre-push` for `feat`/`fix`/`perf`.
- `CONTRIBUTING.md` describing required commit, changelog, and PR workflow.
- CI **Changelog** job + `scripts/check-changelog.sh`: PRs with `feat`/`fix`/`perf` commits (or matching PR title) must update `CHANGELOG.md`.

## [0.1.0] — 2026-06-27

### Added

- **Next.js portfolio template** (`template/`) with App Router, Tailwind CSS v4, and a single `content/portfolio.json` content model.
- **Ten SWE domain presets** (`frontend`, `backend`, `fullstack`, `ml`, `mobile`, `devops`, `data`, `security`, `game`, `general`) with section defaults, skill-group hints, and example JSON under `template/content/examples/`.
- **Section-driven UI**: Hero, Skills, Experience, Projects, Education, Achievements, Blog, Contact — rendered only when enabled and non-empty.
- **Theme support**: primary CSS color token, system/light/dark initial mode, client toggle.
- **Rust CLI** (`customfolio` in `cli/`): embeds the template, interactive create wizard (`dialoguer`), and non-interactive `--yes` flags.
- Commands: `customfolio create`, `customfolio domains`.
- Scaffold writes `package.json` name, generated `content/portfolio.json`, optional `--git` / `--force` / `--minimal`.
- **CI** (GitHub Actions): `cargo fmt` / `clippy` / `test` / release smoke create, plus template `pnpm typecheck` & `pnpm build`.
- Root docs: `README.md`, this changelog, workspace `Cargo.toml` / `.gitignore`.

### Notes

- Generated demo directories (e.g. local `my-fe-folio/`) are gitignored and not part of the release.
- Node is required only to run scaffolded apps; the CLI builds as a single binary.
