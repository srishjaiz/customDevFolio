# Portfolio CSV import

Used by:

- `customfolio csv-to-ndjson` (CLI — convert only)
- `customfolio import-db` (CLI — convert + Postgres)
- `POST /accounts/{id}/imports` (API upload — same pipeline)

Always **streams** CSV → **NDJSON on disk** before DB writes (avoids loading the whole file into RAM).

```bash
customfolio csv-to-ndjson examples/import/people.csv -o /tmp/people.ndjson
# optional: --sample  --continue-on-error  --errors /tmp/errors.ndjson

customfolio import-db --csv examples/import/people.csv \
  --account-id <uuid> --user-id <uuid>
```

| File | Purpose |
|------|---------|
| [`people.csv`](./people.csv) | Small happy-path sample (3 rows) |
| [`edge_cases.csv`](./edge_cases.csv) | Breaking + edge cases (~39 rows) — use with `--continue-on-error` |
| [`generated/`](./generated/) | Local large CSVs (gitignored) via [`scripts/generate_portfolio_csv.py`](../../scripts/generate_portfolio_csv.py) |

End-to-end product guide: [`docs/multi-portfolio.md`](../../docs/multi-portfolio.md).

## Testing with edge cases and large data

```bash
# Edge / breaking rows (expect some failures)
cargo run -p customfolio -- csv-to-ndjson examples/import/edge_cases.csv \
  -o /tmp/edge.ndjson --continue-on-error --errors /tmp/edge.errors.ndjson

# Generate 10k mixed (≈2% invalid) — streaming writer
python3 scripts/generate_portfolio_csv.py -n 10000 --breaking 0.02 --edge-every 50 \
  -o examples/import/generated/large_10k_mixed.csv

cargo run -p customfolio -- csv-to-ndjson examples/import/generated/large_10k_mixed.csv \
  -o /tmp/large.ndjson --continue-on-error --errors /tmp/large.errors.ndjson

# Optional: import to Postgres (API or CLI) after signup + account
cargo run -p customfolio -- import-db \
  --csv examples/import/generated/large_10k_mixed.csv \
  --account-id <uuid> --user-id <uuid> --continue-on-error
```

### What `edge_cases.csv` covers

| Category | Examples |
|----------|----------|
| Happy path | All 10 domains, full columns |
| Defaults | Missing `domain` |
| Slugs | Duplicates, invalid explicit slug, long slug |
| Names | Unicode (José, 李雷, Cyrillic, emoji), commas/quotes/newlines |
| Links | GitHub username vs `http(s)` URL |
| Theme / color | light/dark/system, hex / oklch |
| Overlay | Valid `extra_json` merge; **invalid** `extra_json` (fails row) |
| Security-ish text | SQL-like strings, HTML/`<script>` in bio (stored as text) |
| Breaking | Missing/empty `name`, unknown domain, domain typo, bad theme |

Use **`--continue-on-error`** so valid rows still convert while errors go to `--errors`.

## Columns

Headers are case-insensitive. Aliases in parentheses are accepted.

| Column | Required | Notes |
|--------|----------|--------|
| `name` (`full_name`, `person_name`) | **yes** | Display name |
| `slug` | no | URL id; derived from `name` if omitted; `^[a-z0-9]+(?:-[a-z0-9]+)*$` |
| `domain` | no | Default `fullstack` (see root README domains table) |
| `title` | no | Role title (domain default if empty) |
| `bio` | no | |
| `location` | no | |
| `email` | no | |
| `github` | no | Username or URL |
| `linkedin` | no | |
| `website` (`site`, `site_url`) | no | |
| `resume_url` (`resumeUrl`) | no | |
| `primary` (`primary_color`) | no | Brand color, e.g. `#6366f1` |
| `theme` (`theme_mode`) | no | `system` \| `light` \| `dark` |
| `extra_json` | no | JSON object merged into generated portfolio config (CLI converter) |

Each NDJSON line is a portfolio document (same fields as `content/portfolio.json`) plus **`slug`**, then stored as `portfolios.config` JSONB (slug also a column).
