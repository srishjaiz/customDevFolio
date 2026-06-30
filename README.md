# customFolio

CLI-first toolkit for generating **domain-aware developer portfolios** on **Next.js**, inspired by the content-centric idea behind [developerFolio](https://github.com/srishjaiz/developerFolio).

Configure once via prompts or flags → scaffold a ready-to-run app → keep customizing through a single **`content/portfolio.json`** file.

| | |
|---|---|
| **CLI** | Rust binary `customfolio` — embeds the Next template and writes portfolio config |
| **Template** | Next.js 15 App Router + Tailwind CSS 4, section-driven UI |
| **Domains** | Presets for common SWE roles (frontend, ML, DevOps, …) |
| **CI** | GitHub Actions: Rust, Next template, and changelog gate on PRs |

## Status

| Phase | Status | Location |
|-------|--------|----------|
| **1 — Next template** | Done | [`template/`](./template/) |
| **2–3 — Rust CLI** | Done | [`cli/`](./cli/) (`customfolio`) |
| **Multi-account (free stack) Phase 0** | Done | [`docs/adr/0001-free-stack.md`](./docs/adr/0001-free-stack.md), [`docker-compose.yml`](./docker-compose.yml) |
| **Multi-account Phase 1 — Postgres + repos** | Done | [`server/`](./server/) (`customfolio-server`) |
| **Multi-account Phase 2 — CSV → NDJSON** | Done | `customfolio csv-to-ndjson` (streaming, on-disk intermediate) |
| **Multi-account Phase 3 — NDJSON → DB** | Done | `customfolio import-db` + `server::import_ndjson_file` |
| **Multi-account Phase 4 — Auth API** | Done | `customfolio-api` signup/login/sessions/accounts |
| **Multi-account Phase 5 — CSV upload API** | Done | `POST /accounts/{id}/imports` multipart upload |
| **Multi-account Phase 6 — Browser UI** | Done | template login/dashboard + upload from DB API |
| **Contributor workflow** | Done | PR template, commit hooks, changelog CI |
| **Branch protection** | Active on `main` | Required CI checks (see below) |

## Quick start (CLI)

**Build the CLI** (requires [Rust](https://rustup.rs/)). **Node 18+** is only needed to run the generated app.

```bash
# from repo root
cargo install --path cli

customfolio domains                 # list SWE domain presets
customfolio create my-portfolio     # interactive wizard (TTY)

# non-interactive / CI-friendly
customfolio create my-portfolio \
  --domain frontend \
  --full-name "Ada Lovelace" \
  --title "Frontend Engineer" \
  --email ada@example.com \
  --github ada \
  --yes --git

cd my-portfolio
pnpm install && pnpm dev            # or npm / yarn
```

Ongoing edits: **`content/portfolio.json`** (same schema as the in-repo template).

### CLI commands

| Command | Description |
|---------|-------------|
| `customfolio create [name]` | Scaffold app — interactive on TTY, or use flags + `--yes` |
| `customfolio domains` | Print domain ids, labels, and short descriptions |
| `customfolio csv-to-ndjson <csv> -o <ndjson>` | Stream large portfolio CSV → NDJSON on disk (Phase 2) |
| `customfolio --help` | Full flag reference |

### CSV → NDJSON (large datasets)

Converts a **CSV** of portfolio rows to **NDJSON** by streaming (row-by-row). Output is written to disk as an intermediate for later DB import (Phase 3)—it does **not** load the entire CSV into RAM as one list.

```bash
customfolio csv-to-ndjson examples/import/people.csv -o /tmp/people.ndjson
customfolio csv-to-ndjson people.csv -o people.ndjson --continue-on-error --errors errors.ndjson
```

| Flag | Purpose |
|------|---------|
| `-o, --output` | NDJSON output path (required) |
| `--sample` | Include domain sample experience/projects |
| `--continue-on-error` | Skip bad rows instead of failing the run |
| `--errors <path>` | Write `{"line","error"}` NDJSON for failed rows |

Column reference: [`examples/import/README.md`](./examples/import/README.md).

### Useful `create` flags

| Flag | Purpose |
|------|---------|
| `-d, --domain <id>` | Domain preset (see [Domains](#domains)) |
| `-o, --output <path>` | Output directory (default: `./<name>`) |
| `--full-name`, `--title`, `--bio`, `--location` | Identity |
| `--email`, `--github`, `--linkedin`, `--website`, `--resume-url` | Links |
| `--primary`, `--theme system\|light\|dark` | Brand color and initial color mode |
| `-y, --yes` | Skip prompts; use flags + domain defaults |
| `-f, --force` | Overwrite a non-empty output directory |
| `--git` | Run `git init` in the output |
| `--minimal` | Skip sample experience/projects (placeholders only) |

### What the CLI generates

1. Copies the embedded **Next.js template** (no `node_modules` / `.next`).
2. Sets **`package.json` `name`** from the project name.
3. Writes **`content/portfolio.json`** from your answers + domain defaults (sections, skill groups, theme, optional sample content).
4. Prints next steps (`pnpm install`, `pnpm dev`).

## Domains

Presets control default **section visibility**, **skill group hints**, **role title**, **CTA label**, and **primary color**. Override anything later in `portfolio.json`.

| `domain` id | Focus |
|-------------|--------|
| `frontend` | UI, design systems, accessibility |
| `backend` | APIs, data stores, services |
| `fullstack` | End-to-end product (CLI default) |
| `ml` | Models, MLOps, research-style links |
| `mobile` | Apps, store / platform links |
| `devops` | Platform, IaC, SRE / reliability |
| `data` | Pipelines, warehouses, orchestration |
| `security` | AppSec, detection, defensive tooling |
| `game` | Engines, gameplay, shipped titles |
| `general` | Balanced defaults for generalist SWE |

Rich examples to preview in the template (copy over active config):

```bash
cp template/content/examples/ml.json template/content/portfolio.json
# also: frontend, backend, devops, mobile, data, security, game, general
```

Domain metadata for the site lives in [`template/lib/domains.ts`](./template/lib/domains.ts); the CLI mirrors it in [`cli/src/domain.rs`](./cli/src/domain.rs).

## Template (manual / development)

Work on the portfolio UI without running the CLI:

```bash
cd template
pnpm install
pnpm dev          # http://localhost:3000
pnpm typecheck
pnpm build
```

| Area | Path |
|------|------|
| Active content | [`template/content/portfolio.json`](./template/content/portfolio.json) |
| Types / schema | [`template/lib/types.ts`](./template/lib/types.ts) |
| Domain presets (TS) | [`template/lib/domains.ts`](./template/lib/domains.ts) |
| Sections | [`template/components/sections/`](./template/components/sections/) |
| Domain samples | [`template/content/examples/`](./template/content/examples/) |

Sections render only when **`sections.*` is true** and the related data is non-empty. Theme uses `theme.primary` (CSS `--primary`) and `theme.mode` (`system` / `light` / `dark`), with a client toggle.

Full template notes: [`template/README.md`](./template/README.md).

## Multi-account foundation (Phases 0–1)

Free / self-hosted stack only (no paid SaaS). Decision record: [`docs/adr/0001-free-stack.md`](./docs/adr/0001-free-stack.md).

| Piece | Role |
|-------|------|
| **PostgreSQL** | Source of truth for users, accounts, portfolios (`config` JSONB), import jobs, sessions |
| **`server/`** (`customfolio-server`) | SQLx migrations + repositories (HTTP API comes in later phases) |
| **`docker-compose.yml`** | Local Postgres 16 |
| **`data/imports/`** | Runtime CSV / NDJSON paths (gitignored; used from Phase 2+) |

```bash
# Start free Postgres
docker compose up -d postgres

export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio
# or: cp .env.example .env

cargo test -p customfolio-server
# Integration tests exercise insert / list / get portfolio by (account_id, slug)
```

Offline **`customfolio create`** (single `portfolio.json` site) is unchanged. Multi-portfolio + auth + CSV import build on `server/` in later phases.

## Repository layout

```text
customFolio/
├── Cargo.toml                 # Rust workspace (cli + server)
├── Makefile                   # make check / setup / test / help (preferred DX)
├── CHANGELOG.md               # Keep a Changelog (required for feat/fix/perf PRs)
├── CONTRIBUTING.md            # Minimal contribute loop + CI expectations
├── README.md
├── docker-compose.yml         # Free Postgres for local / CI-adjacent dev
├── .env.example               # DATABASE_URL template
├── docs/adr/                  # Architecture decision records
├── data/imports/              # Gitignored upload / NDJSON intermediates
├── .gitmessage                # Commit template (auto-wired by make check)
├── .githooks/                 # Advisory commit-msg / pre-push tips (CI is the gate)
├── .github/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/ci.yml       # Rust + Next + Changelog jobs
├── cli/                       # customfolio binary (embeds template/)
├── server/                    # customfolio-server: migrations + repos (Phase 1)
├── scripts/
│   ├── check.sh               # Same as make check (no Make required)
│   ├── ensure-git-setup.sh    # Idempotent commit.template + hooksPath
│   ├── setup-git.sh           # Alias → ensure-git-setup.sh
│   └── check-changelog.sh     # CI changelog gate (optional local preview)
└── template/                  # Next.js portfolio source of truth
```

The CLI embeds `template/` at **compile time** via `rust-embed` (excludes `node_modules`, `.next`, and the stock `content/portfolio.json`, which is generated per user).

## Develop (contributors)

**Minimal loop** — no required one-time setup script:

```bash
git checkout -b my-change
# … edit …
make check                 # fmt + clippy + tests (+ template if pnpm is installed)
git commit -m "feat(cli): …" && git push -u origin HEAD && gh pr create
```

`make check` auto-wires the commit message template and git hooks for this clone. Hooks are **tips only**; **CI enforces** quality (coverage, changelog for `feat`/`fix`/`perf`). Details: [`CONTRIBUTING.md`](./CONTRIBUTING.md). Useful targets: `make help`, `make contribute`, `make test`, `make fmt`.

```bash
# Try the CLI while developing
cargo run -p customfolio -- domains
cargo run -p customfolio -- create /tmp/demo --domain devops --yes

# Release binary (LTO, stripped — see workspace Cargo.toml)
cargo build --release && ./target/release/customfolio --help
```

Prefer **[Conventional Commits](https://www.conventionalcommits.org/)** (`feat(cli): …`, `fix(template): …`). For user-facing `feat` / `fix` / `perf`, add a bullet under **`## [Unreleased]`** in [`CHANGELOG.md`](./CHANGELOG.md) in the same PR.

## Continuous integration

Workflow: [`.github/workflows/ci.yml`](./.github/workflows/ci.yml) — **pull requests** and **pushes** to `main`.

| Job | Name (status check) | What it does |
|-----|---------------------|--------------|
| `changelog` | **Changelog** | PRs only: require `CHANGELOG.md` if commits/title are `feat` / `fix` / `perf` |
| `rust` | **Rust (cli)** | `cargo fmt`, `clippy -D warnings`, `cargo llvm-cov` (fail under **80%** lines), release build, smoke create |
| `template` | **Next template** | `pnpm install`, typecheck, vitest coverage, build |

`main` is protected: required checks must be green; use a **feature branch + PR** (direct pushes to `main` are blocked). Ruleset: [Protect main — require CI](https://github.com/srishjaiz/customDevFolio/rules/18207562).

## Versioning and changelog history

See [`CHANGELOG.md`](./CHANGELOG.md) ([Keep a Changelog](https://keepachangelog.com/)). Current work is tracked under **`[Unreleased]`** and **`[0.1.0]`** as applicable.

## License

MIT (workspace `Cargo.toml`). Add a root `LICENSE` file before publishing to crates.io if you distribute the crate publicly.
