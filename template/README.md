# customFolio template

Next.js (App Router) developer portfolio template driven by a single JSON config. Designed to be scaffolded and customized by the **customFolio** Rust CLI (phases 2–3), with **domain presets** for different software-engineering roles.

## Quick start

```bash
cd template
pnpm install   # or npm install / yarn
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000).

## Customize content

Edit **[`content/portfolio.json`](./content/portfolio.json)** — name, bio, skills, experience, projects, theme color, section toggles, and `domain`.

Types live in [`lib/types.ts`](./lib/types.ts). Loader helpers in [`lib/portfolio.ts`](./lib/portfolio.ts).

### Preview another domain

Copy an example over the active config (keep a backup if you edited it):

```bash
cp content/examples/ml.json content/portfolio.json
# frontend | backend | fullstack (default in portfolio.json) | ml | mobile
# devops | data | security | game | general
pnpm dev
```

Domain metadata (labels, default section hints for the future CLI) is in [`lib/domains.ts`](./lib/domains.ts).

## Domains

| `domain` id | Focus |
|-------------|--------|
| `frontend` | UI, design systems, a11y |
| `backend` | APIs, data, services |
| `fullstack` | End-to-end product |
| `ml` | Models, MLOps, research links |
| `mobile` | Apps, store links |
| `devops` | Platform, IaC, SRE |
| `data` | Pipelines, warehouses |
| `security` | AppSec, detection |
| `game` | Engines, shipped titles |
| `general` | Balanced defaults |

Section visibility is controlled by `sections` in JSON (and empty arrays are not rendered). Extra social URLs go under `social.extra` (e.g. Hugging Face, App Store).

## Theme

- `theme.primary` — CSS color applied as `--primary`
- `theme.mode` — `system` | `light` | `dark` (initial preference; toggle overrides via `localStorage`)

## Project layout

```text
app/                 # App Router entry (layout + home composition)
components/
  sections/          # Hero, Skills, Experience, Projects, ...
  ui/                # Header, Footer, theme, primitives
content/
  portfolio.json     # Active portfolio (CLI will generate this)
  examples/          # Domain samples for local preview
lib/
  types.ts           # PortfolioConfig schema
  domains.ts         # Domain profiles
  portfolio.ts       # Load + helpers
  utils.ts
public/images/       # Optional avatars / project shots
```

## Scripts

| Command | Description |
|---------|-------------|
| `pnpm dev` | Dev server |
| `pnpm build` | Production build |
| `pnpm start` | Serve production build |
| `pnpm typecheck` | `tsc --noEmit` |

## Deploy

Any Next host works (Vercel, Node, Docker). Set `meta.siteUrl` for metadata base URL.

## CLI integration (upcoming)

The Rust CLI will:

1. Embed or copy this `template/` tree
2. Ask domain + identity prompts
3. Merge [`DOMAIN_PROFILES`](./lib/domains.ts) defaults into `sections` / skill hints
4. Write `content/portfolio.json` (and theme) for the user

Until then, hand-edit JSON or swap `content/examples/*.json`.
