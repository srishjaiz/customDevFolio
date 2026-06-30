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

Sample file: [`people.csv`](./people.csv). End-to-end guide: [`docs/multi-portfolio.md`](../../docs/multi-portfolio.md).

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
