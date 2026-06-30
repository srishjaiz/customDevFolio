# Multi-portfolio builder (how to use)

This guide covers the **account + many portfolios** product on branch
**`feature/multi-portfolio-builder`**. It uses only free / self-hosted pieces
(Postgres, Rust API, Next.js, local disk for CSV/NDJSON).

> **`main` is separate.** Checkout `main` for the original CLI-only single-portfolio
> toolkit (no server/API/multi-account). Use this branch for multi-portfolio work.

Architecture decision: [adr/0001-free-stack.md](./adr/0001-free-stack.md) · Ops: [ops.md](./ops.md) · CSV columns: [../examples/import/README.md](../examples/import/README.md)

## What you get

1. **Sign up / log in** (session cookie, argon2 passwords in Postgres).
2. **Create an account** (team / cohort shell that owns many portfolios).
3. **Upload a large CSV** (or import via CLI): always **CSV → NDJSON on disk** (streaming), then **upsert into Postgres** under that account.
4. **Browse** all portfolios for the account in the browser and open one by slug.

```text
Browser (template :3000)  --cookie-->  customfolio-api (:8080)  -->  PostgreSQL
                                         ^
User uploads CSV -----------------------+
                                         |
                              data/imports/<job_id>/
                                source.csv
                                data.ndjson   (intermediate, low memory)
                                errors.ndjson
```

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Docker](https://docs.docker.com/get-docker/) (for Postgres)
- Node 18+ and [pnpm](https://pnpm.io/) (for the Next UI)
- Git checkout of **`feature/multi-portfolio-builder`**

```bash
git fetch origin
git checkout feature/multi-portfolio-builder
```

## Path A — Browser (recommended)

### 1. Start Postgres

```bash
docker compose up -d postgres
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio
# optional: cp .env.example .env
```

### 2. Start the API

```bash
cargo run -p customfolio-server --bin customfolio-api
# listens on http://localhost:8080
# health: curl http://localhost:8080/health
```

Env (see [ops.md](./ops.md)): `DATABASE_URL`, `CORS_ORIGIN=http://localhost:3000`, `DATA_DIR=data/imports`, `MAX_UPLOAD_BYTES`.

### 3. Start the web UI

```bash
cd template
pnpm install
NEXT_PUBLIC_API_URL=http://localhost:8080 pnpm dev
# http://localhost:3000
```

### 4. Use the product

| Step | URL / action |
|------|----------------|
| Sign up | [http://localhost:3000/signup](http://localhost:3000/signup) — email + password (min 8 chars) |
| Log in | [http://localhost:3000/login](http://localhost:3000/login) |
| Dashboard | [http://localhost:3000/dashboard](http://localhost:3000/dashboard) — create an **account** (e.g. “Cohort 2026”) |
| Account | Open the account → list portfolios |
| Upload | Choose a **CSV** file → **Upload CSV** (multipart to API; converts on disk then imports) |
| Portfolio | Click a person → view portfolio payload / details |

Sample CSV: [`examples/import/people.csv`](../examples/import/people.csv).

### 5. Log out

Use **Log out** on the dashboard (clears session cookie).

## Path B — CLI only (no browser)

Useful for automation / large files when you already have `user_id` and `account_id` (create them via API once, or insert via tests).

### Convert CSV → NDJSON (disk, streaming)

```bash
cargo run -p customfolio -- csv-to-ndjson examples/import/people.csv -o /tmp/people.ndjson
# flags: --sample  --continue-on-error  --errors /tmp/errors.ndjson
```

### Import into Postgres (CSV → NDJSON on disk → DB)

1. Sign up via API and create an account (or use dashboard), note UUIDs:

```bash
# signup (returns Set-Cookie + user JSON with id)
curl -s -c /tmp/cf.cookies -H 'Content-Type: application/json' \
  -d '{"email":"you@example.com","password":"password1","display_name":"You"}' \
  http://localhost:8080/auth/signup

# create account
curl -s -b /tmp/cf.cookies -H 'Content-Type: application/json' \
  -d '{"name":"Cohort 2026","slug":"cohort-2026"}' \
  http://localhost:8080/accounts
# → { "id": "<account-uuid>", ... }
```

2. Import:

```bash
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio

cargo run -p customfolio -- import-db \
  --csv examples/import/people.csv \
  --account-id <account-uuid> \
  --user-id <user-uuid> \
  --continue-on-error
```

Artifacts land under `data/imports/<job-id>/` (`source.csv`, `data.ndjson`, `errors.ndjson`).

## Path C — Single portfolio (classic, also on `main`)

No database required:

```bash
cargo install --path cli   # or: cargo run -p customfolio -- ...
customfolio create my-site --domain frontend --full-name "Ada" --yes
cd my-site && pnpm install && pnpm dev
# edit content/portfolio.json
```

## API reference (auth + data)

Base URL default: `http://localhost:8080`. Cookie name: `customfolio_session` (httpOnly).

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| `GET` | `/health` | no | Liveness |
| `POST` | `/auth/signup` | no | `{ email, password, display_name? }` → user + cookie |
| `POST` | `/auth/login` | no | `{ email, password }` → user + cookie |
| `POST` | `/auth/logout` | cookie | Clear cookie |
| `GET` | `/auth/me` | cookie | Current user |
| `POST` | `/accounts` | cookie | `{ name, slug?, description? }` |
| `GET` | `/accounts` | cookie | List owned accounts |
| `GET` | `/accounts/{id}/portfolios` | cookie | List portfolios |
| `GET` | `/accounts/{id}/portfolios/{slug}` | cookie | One portfolio (`config` JSONB) |
| `POST` | `/accounts/{id}/imports` | cookie | Multipart field `file` or `csv` |
| `GET` | `/imports/{job_id}` | cookie | Import job status / counts |

## Portfolio document shape

Each portfolio in the DB is a **`PortfolioConfig`** (same fields as `content/portfolio.json`) plus a **`slug`** for URLs. Sections: meta, domain, person, social, greeting, skills, experience, education, projects, achievements, blog, contact, theme, sections.

Types: [`template/lib/types.ts`](../template/lib/types.ts).

## Troubleshooting

| Issue | Check |
|-------|--------|
| Browser “not authenticated” / CORS errors | API running; `CORS_ORIGIN=http://localhost:3000`; UI uses `NEXT_PUBLIC_API_URL=http://localhost:8080`; cookies need same-site friendly localhost setup |
| Upload fails size | Raise `MAX_UPLOAD_BYTES` (default 50MiB) |
| Import row errors | See `data/imports/<job>/errors.ndjson` |
| Integration tests skip | Set `DATABASE_URL` and run Postgres |
| Want old product | `git checkout main` |

## Rollback to previous version

```bash
git checkout main
# CLI create + template only — no multi-portfolio server required
```
