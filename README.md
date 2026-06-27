# customFolio

CLI-first toolkit for generating **domain-aware developer portfolios** on **Next.js**, inspired by the content-centric idea behind [developerFolio](https://github.com/srishjaiz/developerFolio).

Configure via prompts or flags → scaffold a ready-to-run app → customize through a single **`content/portfolio.json`** file.

## Quick start (CLI)

Requires [Rust](https://rustup.rs/) to build the CLI. Node 18+ is only needed to run the generated app.

```bash
cargo install --path cli

customfolio domains
customfolio create my-portfolio
# non-interactive:
customfolio create my-portfolio \
  --domain frontend \
  --full-name "Ada Lovelace" \
  --email ada@example.com \
  --github ada \
  --yes

cd my-portfolio && pnpm install && pnpm dev
```

Edit **`content/portfolio.json`** for ongoing customization.

### Commands

| Command | Description |
|---------|-------------|
| `customfolio create [name]` | Interactive wizard (TTY) or flags + `--yes` |
| `customfolio domains` | List SWE domain presets |

### Useful `create` flags

| Flag | Purpose |
|------|---------|
| `-d, --domain <id>` | `frontend`, `backend`, `fullstack`, `ml`, `mobile`, `devops`, `data`, `security`, `game`, `general` |
| `-o, --output <path>` | Output directory |
| `--full-name`, `--title`, `--bio`, `--location` | Identity |
| `--email`, `--github`, `--linkedin`, `--website`, `--resume-url` | Links |
| `--primary`, `--theme system\|light\|dark` | Branding |
| `-y, --yes` | No prompts (domain defaults) |
| `-f, --force` | Overwrite non-empty directory |
| `--git` | `git init` in the output |
| `--minimal` | Skip sample experience/projects |

## Domains

Presets set default sections, skill hints, titles, CTAs, and primary color (override in JSON anytime).

| id | Focus |
|----|--------|
| `frontend` | UI, design systems, a11y |
| `backend` | APIs, data, services |
| `fullstack` | End-to-end (CLI default) |
| `ml` | Models, MLOps, research links |
| `mobile` | Apps, store links |
| `devops` | Platform, IaC, SRE |
| `data` | Pipelines, warehouses |
| `security` | AppSec, detection |
| `game` | Engines, shipped titles |
| `general` | Balanced defaults |

## Template

```bash
cd template && pnpm install && pnpm dev
cp content/examples/ml.json content/portfolio.json   # preview another domain
```

See [`template/README.md`](./template/README.md). Content schema: [`template/lib/types.ts`](./template/lib/types.ts). Domain presets (TS): [`template/lib/domains.ts`](./template/lib/domains.ts) — mirrored in [`cli/src/domain.rs`](./cli/src/domain.rs).

## Repository layout

```text
customFolio/
  Cargo.toml     # workspace
  cli/           # Rust binary: customfolio (embeds template at build time)
  template/      # Next.js App Router portfolio
```

## Develop

```bash
cargo test
cargo run -p customfolio -- create /tmp/demo --domain devops --yes
cargo build --release
```

## License

MIT (workspace `Cargo.toml`).
