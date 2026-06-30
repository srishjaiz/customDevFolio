# Portfolio CSV import (Phase 2)

Stream a **CSV** of portfolio rows to **NDJSON on disk** without loading the full file into memory.

```bash
customfolio csv-to-ndjson examples/import/people.csv -o /tmp/people.ndjson
# optional: --sample  --continue-on-error  --errors /tmp/errors.ndjson
```

## Columns

Headers are case-insensitive. Aliases in parentheses are accepted.

| Column | Required | Notes |
|--------|----------|--------|
| `name` (`full_name`, `person_name`) | **yes** | Display name |
| `slug` | no | URL id; derived from `name` if omitted; must match `^[a-z0-9]+(?:-[a-z0-9]+)*$` |
| `domain` | no | Default `fullstack`. One of: frontend, backend, fullstack, ml, mobile, devops, data, security, game, general |
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
| `extra_json` | no | JSON object merged into the generated portfolio config |

Each NDJSON line is a full portfolio document (same fields as `content/portfolio.json`) plus **`slug`**.

Phase 3 will stream NDJSON into Postgres under an account.
