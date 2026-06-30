# Operations notes (Phase 7 — free stack)

## Local

```bash
docker compose up -d postgres
export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio
cargo run -p customfolio-server --bin customfolio-api
# other terminal
cd template && NEXT_PUBLIC_API_URL=http://localhost:8080 pnpm dev
```

## Env

| Variable | Default | Purpose |
|----------|---------|---------|
| `DATABASE_URL` | compose URL | Postgres |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | API bind |
| `DATA_DIR` | `data/imports` | CSV / NDJSON job dirs |
| `MAX_UPLOAD_BYTES` | 50MiB | Upload limit |
| `CORS_ORIGIN` | `http://localhost:3000` | Browser origin (credentials) |
| `NEXT_PUBLIC_API_URL` | `http://localhost:8080` | Template → API |

## Cleanup

Job artifacts under `DATA_DIR/<job_id>/` (source.csv, data.ndjson, errors.ndjson). Delete old job directories periodically, e.g.:

```bash
find data/imports -mindepth 1 -mtime +7 -type d -exec rm -rf {} +
```

## Optional compose API

```bash
docker compose --profile full up -d --build
```

Uses free Debian/Rust images only — no paid registries required.
