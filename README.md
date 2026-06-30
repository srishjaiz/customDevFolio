# customFolio

CLI-first toolkit for generating **domain-aware developer portfolios** on **Next.js**, inspired by the content-centric idea behind [developerFolio](https://github.com/srishjaiz/developerFolio).

Two ways to use it (depending on **branch**):

| Branch | What you get |
|--------|----------------|
| **`main`** | Classic **single-portfolio** flow: `customfolio create` → one Next app → edit `content/portfolio.json` |
| **`feature/multi-portfolio-builder`** | **Multi-portfolio builder**: sign up, own an **account**, upload a large **CSV** (streamed to NDJSON on disk, stored in **Postgres**), browse many portfolios in the browser |

> You are reading docs from whatever branch is checked out. Multi-portfolio steps below apply on **`feature/multi-portfolio-builder`**. To go back to the previous product line: `git checkout main`.

| | |
|---|---|
| **CLI** | Rust binary `customfolio` — scaffold single sites **or** CSV→NDJSON / import-db |
| **API** | `customfolio-api` — auth, accounts, portfolios, CSV upload (this feature branch) |
| **Template** | Next.js 15 App Router + Tailwind — static portfolio **and** `/login` / `/dashboard` UI |
| **DB** | PostgreSQL (free, self-hosted) — source of truth for multi-portfolio data |
| **Stack policy** | Free / OSS only — [docs/adr/0001-free-stack.md](./docs/adr/0001-free-stack.md) |

## How to use — multi-portfolio builder

**Full walkthrough:** [docs/multi-portfolio.md](./docs/multi-portfolio.md) · **Ops / env:** [docs/ops.md](./docs/ops.md) · **Docs index:** [docs/README.md](./docs/README.md)

### Quick path (browser)

```bash
git checkout feature/multi-portfolio-builder

# 1) Database
docker compose up -d postgres
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio

# 2) API (terminal A)
cargo run -p customfolio-server --bin customfolio-api
# → http://localhost:8080/health

# 3) UI (terminal B)
cd template && pnpm install
NEXT_PUBLIC_API_URL=http://localhost:8080 pnpm dev
# → http://localhost:3000
```

Then in the browser:

1. Open **[/signup](http://localhost:3000/signup)** → create a user (password ≥ 8 characters).
2. Open **[/dashboard](http://localhost:3000/dashboard)** → **Create** an account (e.g. “Cohort 2026”).
3. Open the account → **Upload CSV** (try [`examples/import/people.csv`](./examples/import/people.csv)).
4. Click a person to open their portfolio under that account.

Pipeline for every upload: **CSV file → NDJSON on disk (streaming, low memory) → upsert rows in Postgres**. Nothing requires paid SaaS.

### Quick path (CLI import)

With API up at least once so you have `user_id` / `account_id` (or create them via `curl` — see [docs/multi-portfolio.md](./docs/multi-portfolio.md)):

```bash
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio

# Optional: only convert (no DB)
cargo run -p customfolio -- csv-to-ndjson examples/import/people.csv -o /tmp/people.ndjson

# Full import: CSV → NDJSON under data/imports/<job>/ → Postgres
cargo run -p customfolio -- import-db \
  --csv examples/import/people.csv \
  --account-id <ACCOUNT_UUID> \
  --user-id <USER_UUID> \
  --continue-on-error
```

### How to use — classic single portfolio (`main` or this branch)

No database required:

```bash
git checkout main   # optional: stable single-portfolio line only
# or stay on feature/multi-portfolio-builder — create still works

cargo install --path cli   # from repo root
customfolio domains
customfolio create my-portfolio --domain frontend --full-name "Ada Lovelace" --yes

cd my-portfolio && pnpm install && pnpm dev
# edit content/portfolio.json
```

## Status

| Area | Status | Location |
|------|--------|----------|
| Next portfolio template | Done | [`template/`](./template/) |
| Rust CLI (`create`, `domains`) | Done | [`cli/`](./cli/) |
| Free-stack ADR | Done | [`docs/adr/0001-free-stack.md`](./docs/adr/0001-free-stack.md) |
| Postgres schema + repos | Done | [`server/`](./server/) |
| CSV → NDJSON (streaming) | Done | `customfolio csv-to-ndjson` |
| NDJSON → DB import | Done | `customfolio import-db`, `server::import_ndjson_file` |
| Auth + accounts API | Done | `customfolio-api` |
| Authenticated CSV upload | Done | `POST /accounts/{id}/imports` |
| Browser dashboard | Done | `template/app/login`, `signup`, `dashboard` |
| Ops / Docker API | Done | [`Dockerfile.api`](./Dockerfile.api), [`docs/ops.md`](./docs/ops.md) |
| Contributor workflow | Done | PR template, hooks, changelog CI |
| Branch protection on **`main`** | Active | Required CI; multi-portfolio lives on **feature branch** until you choose to merge |

## CLI commands

| Command | Description |
|---------|-------------|
| `customfolio create [name]` | Scaffold a **single** Next portfolio app |
| `customfolio domains` | List SWE domain presets |
| `customfolio csv-to-ndjson <csv> -o <ndjson>` | Stream large CSV → NDJSON on disk |
| `customfolio import-db --csv … --account-id … --user-id …` | CSV→NDJSON on disk, then upsert into Postgres |
| `customfolio --help` | Full flag reference |

### `csv-to-ndjson` flags

| Flag | Purpose |
|------|---------|
| `-o, --output` | NDJSON output path (required) |
| `--sample` | Include domain sample experience/projects |
| `--continue-on-error` | Skip bad rows instead of failing the run |
| `--errors <path>` | Write `{"line","error"}` NDJSON for failed rows |

### `import-db` flags

| Flag | Purpose |
|------|---------|
| `--csv` | Input CSV (always converted to NDJSON on disk first) |
| `--account-id` | Account UUID that owns portfolios |
| `--user-id` | User UUID recorded on the import job |
| `--database-url` | Postgres URL (or env `DATABASE_URL`) |
| `--work-dir` | Override job dir (default `data/imports/<job-id>`) |
| `--sample` | Sample content when converting CSV |
| `--continue-on-error` | Keep going on bad rows |

CSV columns: [`examples/import/README.md`](./examples/import/README.md).

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

### What `create` generates

1. Copies the embedded **Next.js template** (no `node_modules` / `.next`).
2. Sets **`package.json` `name`** from the project name.
3. Writes **`content/portfolio.json`** from answers + domain defaults.
4. Prints next steps (`pnpm install`, `pnpm dev`).

## API (multi-portfolio)

Binary: **`customfolio-api`** (`cargo run -p customfolio-server --bin customfolio-api`).

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| `GET` | `/health` | — | Health check |
| `POST` | `/auth/signup` | — | Register (argon2 password) |
| `POST` | `/auth/login` | — | Login → httpOnly cookie |
| `POST` | `/auth/logout` | cookie | Clear session |
| `GET` | `/auth/me` | cookie | Current user |
| `POST` / `GET` | `/accounts` | cookie | Create / list accounts |
| `GET` | `/accounts/{id}/portfolios` | cookie | List portfolios |
| `GET` | `/accounts/{id}/portfolios/{slug}` | cookie | One portfolio |
| `POST` | `/accounts/{id}/imports` | cookie | Multipart CSV upload (`file` or `csv`) |
| `GET` | `/imports/{job_id}` | cookie | Import job status |

Details and `curl` examples: [docs/multi-portfolio.md](./docs/multi-portfolio.md).

## Domains

Presets control default **section visibility**, **skill group hints**, **role title**, **CTA label**, and **primary color**. Override in `portfolio.json` or CSV fields.

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

```bash
cp template/content/examples/ml.json template/content/portfolio.json
# also: frontend, backend, devops, mobile, data, security, game, general
```

Domain metadata: [`template/lib/domains.ts`](./template/lib/domains.ts); CLI: [`cli/src/domain.rs`](./cli/src/domain.rs).

## Template (manual / development)

```bash
cd template
pnpm install
pnpm dev          # http://localhost:3000 — static portfolio.json by default on /
pnpm typecheck
pnpm build
```

| Area | Path |
|------|------|
| Active single-site content | [`template/content/portfolio.json`](./template/content/portfolio.json) |
| Multi-portfolio UI | `template/app/login`, `signup`, `dashboard` |
| API client | [`template/lib/api.ts`](./template/lib/api.ts) |
| Types / schema | [`template/lib/types.ts`](./template/lib/types.ts) |
| Sections | [`template/components/sections/`](./template/components/sections/) |

Sections render when **`sections.*` is true** and data is non-empty. Theme: `theme.primary`, `theme.mode`.

## Multi-account architecture (this feature branch)

Free / self-hosted only — [docs/adr/0001-free-stack.md](./docs/adr/0001-free-stack.md).

| Piece | Role |
|-------|------|
| **PostgreSQL** | Users, sessions, accounts, portfolios (`config` JSONB), import jobs |
| **`server/`** | Repos, NDJSON import, Axum API (`customfolio-api`) |
| **`cli/`** | `create`, `csv-to-ndjson`, `import-db` |
| **`template/`** | Portfolio sections + auth/dashboard pages |
| **`data/imports/`** | Per-job CSV / NDJSON / errors (gitignored) |
| **`docker-compose.yml`** | Postgres (+ optional `api` profile) |

```bash
docker compose up -d postgres
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio
cargo test --workspace   # server integration tests need DATABASE_URL
```

## Repository layout

```text
customFolio/
├── Cargo.toml                 # workspace: cli + server
├── Makefile
├── CHANGELOG.md
├── README.md
├── docker-compose.yml         # Postgres (+ optional api profile)
├── Dockerfile.api             # optional image for customfolio-api
├── .env.example
├── docs/
│   ├── README.md              # docs index
│   ├── multi-portfolio.md     # end-to-end how-to
│   ├── ops.md                 # env, cleanup, compose
│   └── adr/0001-free-stack.md
├── data/imports/              # runtime uploads (gitignored)
├── examples/import/           # sample CSV + column docs
├── cli/                       # customfolio binary
├── server/                    # customfolio-server lib + customfolio-api
├── scripts/
└── template/                  # Next.js portfolio + dashboard UI
```

## Develop (contributors)

```bash
git checkout feature/multi-portfolio-builder   # or branch from it / from main
make check                 # fmt + clippy + tests (+ template if pnpm available)
```

```bash
cargo run -p customfolio -- domains
cargo run -p customfolio-server --bin customfolio-api
cargo build --release -p customfolio
```

Prefer **[Conventional Commits](https://www.conventionalcommits.org/)**. For user-facing `feat` / `fix` / `perf`, update **`## [Unreleased]`** in [`CHANGELOG.md`](./CHANGELOG.md). See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Continuous integration

[`.github/workflows/ci.yml`](./.github/workflows/ci.yml) on PRs / pushes to `main` (Rust job uses a Postgres service when present on the branch).

| Job | Check name | What it does |
|-----|------------|--------------|
| `changelog` | **Changelog** | Require changelog for `feat` / `fix` / `perf` on PRs |
| `rust` | **Rust (cli)** | fmt, clippy, llvm-cov (≥80% lines), release smoke |
| `template` | **Next template** | typecheck, vitest coverage, build |

`main` stays protected and free of multi-portfolio merges until you intentionally open a PR from `feature/multi-portfolio-builder` → `main`.

## Versioning and changelog

See [`CHANGELOG.md`](./CHANGELOG.md). Multi-portfolio work is under **`[Unreleased]`** on this branch.

## License

MIT (workspace `Cargo.toml`).
