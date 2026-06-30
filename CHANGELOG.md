# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) where applicable.

## [Unreleased]

### Changed

- **Contributor DX:** `make check` is the single local entrypoint (auto-configures commit template + hooks); git hooks are advisory only; CI remains the hard gate. Shorter `CONTRIBUTING.md` and PR template; no mandatory `./scripts/setup-git.sh` step.

### Added

- **Phase 5 — Manual CSV upload:** authenticated `POST /accounts/{id}/imports` (multipart) writes CSV to disk, streams to NDJSON, imports to Postgres; `GET /imports/{job_id}` for status.
- **Phase 4 — Auth API:** `customfolio-api` (Axum) with signup/login/logout/me (argon2 + httpOnly session cookies), create/list accounts, list/get portfolios for owners.
- **Phase 3 — NDJSON → Postgres:** `customfolio import-db` always converts CSV→NDJSON on disk then streams NDJSON into portfolios (upsert by account+slug); `server` `import_ndjson_file` + import job updates.
- **Phase 2 — CSV → NDJSON (streaming):** `customfolio csv-to-ndjson <csv> -o <ndjson>` streams large CSVs to on-disk NDJSON (one portfolio + `slug` per line) using free `csv` + `serde_json` crates—never loads the full file as one array. Flags: `--sample`, `--continue-on-error`, `--errors`. Example: [`examples/import/people.csv`](./examples/import/people.csv).
- **Phase 0 — free stack ADR & layout:** [`docs/adr/0001-free-stack.md`](./docs/adr/0001-free-stack.md) documents OSS-only choices (Postgres, SQLx, local disk imports, no paid auth/SaaS); [`docker-compose.yml`](./docker-compose.yml) for PostgreSQL 16; `data/imports/` for future CSV/NDJSON; `.env.example` with `DATABASE_URL`.
- **Phase 1 — Postgres storage (`server/` / `customfolio-server`):** SQLx migrations for `users`, `accounts`, `portfolios` (JSONB `config`), `import_jobs`, `sessions`; repositories for create/list/get/upsert by `(account_id, slug)`; integration tests when `DATABASE_URL` is set.
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
