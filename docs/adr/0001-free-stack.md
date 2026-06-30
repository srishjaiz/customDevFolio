# ADR 0001: Free / open-source stack for multi-portfolio accounts

**Status:** Accepted  
**Date:** 2026-06-30  
**Phases:** 0–1 (foundation); informs later auth, import, and UI phases

## Context

customFolio is evolving from a CLI that scaffolds a single Next.js portfolio (`content/portfolio.json`) to a multi-portfolio product where:

- Users **sign up / log in** and own **accounts**
- Large **CSV** datasets are uploaded **manually**, converted **on disk to NDJSON** (streaming), then stored in a **database**
- The browser lists many portfolios under one account

We require **only freely available dependencies**: permissively licensed OSS that can be **self-hosted** with no mandatory paid SaaS, proprietary SDKs, or commercial licenses.

## Decision

| Layer | Choice | License (typical) | Notes |
|--------|--------|-------------------|--------|
| Database | **PostgreSQL** | PostgreSQL License | Source of truth for users, accounts, portfolios, jobs, sessions |
| API / worker (future HTTP) | **Rust + Tokio + Axum** | MIT / Apache-2.0 | Same language as `cli/`; Phase 1 ships DB repos in `server/` |
| DB access | **SQLx** | MIT / Apache-2.0 | Async Postgres; embedded SQL migrations |
| Migrations | **SQLx `migrate!`** | (same) | Versioned SQL under `server/migrations/` |
| Portfolio document | **JSONB** (`portfolios.config`) | — | Matches existing `PortfolioConfig` camelCase JSON |
| Passwords (Phase 4) | **argon2** crate | MIT / Apache-2.0 | Not wired in Phase 1; column reserved on `users` |
| Sessions (Phase 4) | **Postgres `sessions` table** + httpOnly cookies | — | Avoid Redis as a requirement |
| CSV → NDJSON (Phase 2) | **`csv` + `serde_json`** crates | MIT / Apache-2.0 | Stream to disk; never load full CSV into RAM |
| File uploads (Phase 5) | **Local disk** `data/imports/<job_id>/` | — | Gitignored; no S3 required |
| Object storage (optional later) | **MinIO** (self-hosted) | AGPL for server | Only if disk is insufficient |
| Job queue (v1) | **`import_jobs` table** + Tokio task | — | No Redis required |
| Frontend | **Next.js** (`template/`) | MIT | Evolve to auth + account views; talks to API via `fetch` |
| Local orchestration | **Docker Compose** | Apache-2.0 | Postgres (+ later API/web) |
| TLS in production | **Caddy** or **nginx** + Let’s Encrypt | OSS | Free certificates |
| Logging | **`tracing`** | MIT | Stdout; optional Prometheus/Grafana later (OSS) |

### Explicit non-goals (not dependencies)

Clerk, Auth0, Firebase Auth, Supabase-as-required-backend, PlanetScale-as-requirement, AWS S3 as required path, paid email APIs for v1, proprietary ORMs.

## Repository layout

```text
customFolio/
├── Cargo.toml                 # workspace: cli, server
├── cli/                       # customfolio binary (scaffold create/domains)
├── server/                    # DB layer (Phase 1); Axum API later
│   ├── migrations/            # SQLx SQL migrations
│   └── src/                   # models, repos, db pool helpers
├── template/                  # Next.js portfolio UI (file-based today)
├── data/                      # gitignored runtime uploads / NDJSON
│   └── imports/
├── docs/adr/                  # architecture decision records
├── docker-compose.yml         # free Postgres for local/CI-adjacent dev
└── README.md
```

## Phase 1 data model (Postgres)

- **`users`** — identity; `password_hash` for Phase 4 auth
- **`accounts`** — multi-portfolio shell; `owner_user_id` → `users`
- **`portfolios`** — one row per person/site under an account; unique `(account_id, slug)`; full document in **`config` JSONB**
- **`import_jobs`** — CSV/NDJSON import status and paths (Phases 2–3/5)
- **`sessions`** — server-side sessions for cookie auth (Phase 4)

## Consequences

- Local and CI can run against **Docker Postgres** (`docker compose up -d postgres`) with `DATABASE_URL`.
- Integration tests that need a database are gated on `DATABASE_URL` (or Compose in CI).
- Offline **`customfolio create`** remains a free, file-based single-portfolio path; multi-account product path is **API + Postgres**.
- Later phases must not introduce paid-only dependencies without a new ADR.

## References

- Portfolio JSON shape: `template/lib/types.ts`, `cli/src/config.rs`
- Product plan: multi-portfolio accounts, CSV→NDJSON→DB, free stack only
