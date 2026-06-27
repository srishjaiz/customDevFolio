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
| `customfolio --help` | Full flag reference |

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

## Repository layout

```text
customFolio/
├── Cargo.toml                 # Rust workspace
├── CHANGELOG.md               # Keep a Changelog (required for feat/fix/perf PRs)
├── CONTRIBUTING.md            # Commits, PRs, hooks, CI expectations
├── README.md
├── .gitmessage                # Commit message template (Conventional Commits)
├── .githooks/                 # commit-msg, prepare-commit-msg, pre-push
├── .github/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/ci.yml       # Rust + Next + Changelog jobs
├── cli/                       # customfolio binary (embeds template/)
├── scripts/
│   ├── setup-git.sh           # enable commit template + hooks (run once per clone)
│   └── check-changelog.sh     # local + CI changelog gate
└── template/                  # Next.js portfolio source of truth
```

The CLI embeds `template/` at **compile time** via `rust-embed` (excludes `node_modules`, `.next`, and the stock `content/portfolio.json`, which is generated per user).

## Develop (contributors)

```bash
# one-time per clone — commit template + enforcement hooks
./scripts/setup-git.sh

# Rust CLI
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# optional local coverage (same gate as CI; needs llvm-tools + cargo-llvm-cov)
# cargo install cargo-llvm-cov && rustup component add llvm-tools-preview
# cargo llvm-cov --workspace --fail-under-lines 80
cargo run -p customfolio -- domains
cargo run -p customfolio -- create /tmp/demo --domain devops --yes

# Release binary (LTO, stripped — see workspace Cargo.toml)
cargo build --release
./target/release/customfolio --help

# Template (if you touch UI or content schema)
cd template && pnpm install && pnpm typecheck && pnpm test && pnpm test:coverage && pnpm build
```

### Commit messages

Use **[Conventional Commits](https://www.conventionalcommits.org/)** (enforced by `.githooks/commit-msg` after setup):

```text
feat(cli): add --minimal scaffold mode
fix(template): hide blog when posts empty
docs: expand README with CI and domains
ci: require Changelog status check on main
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`. Prefer `git commit` (loads `.gitmessage`) over ad-hoc `-m` until the format is familiar.

### Changelog

For **`feat` / `fix` / `perf`** (and breaking changes), update [`CHANGELOG.md`](./CHANGELOG.md) in the same PR—typically under **`## [Unreleased]`** (Added / Changed / Fixed / Removed).

- **Local push hook:** `.githooks/pre-push` blocks pushes that add those commit types without a `CHANGELOG.md` change on the branch (`SKIP_CHANGELOG_CHECK=1` to override, discouraged).
- **CI:** PR job **Changelog** runs [`scripts/check-changelog.sh`](./scripts/check-changelog.sh). Preview locally:

```bash
./scripts/check-changelog.sh origin/main HEAD
```

Optional escape in a commit subject (discouraged): `[skip changelog]`.

### Pull requests

New PRs use [`.github/PULL_REQUEST_TEMPLATE.md`](./.github/PULL_REQUEST_TEMPLATE.md): summary, changelog checkbox, change type, and test plan. Target **`main`** from a feature branch.

More detail: [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Continuous integration

Workflow: [`.github/workflows/ci.yml`](./.github/workflows/ci.yml) — runs on **pull requests** and **pushes** to `main`.

| Job | Name (status check) | What it does |
|-----|---------------------|--------------|
| `changelog` | **Changelog** | PRs only: require `CHANGELOG.md` if commits/title are `feat` / `fix` / `perf` |
| `rust` | **Rust (cli)** | `cargo fmt`, `clippy -D warnings`, `cargo llvm-cov` (fail under **80%** lines), release build, smoke `customfolio create --yes` |
| `template` | **Next template** | `pnpm install --frozen-lockfile`, `typecheck`, `vitest` coverage (thresholds in `vitest.config.ts`), `build` |

## Branch protection

`main` is protected (classic branch protection **and** a repository ruleset):

| Rule | Setting |
|------|---------|
| Required status checks | **Rust (cli)**, **Next template**, **Changelog** (branch must be up to date with `main`) |
| Enforce for admins | Yes |
| Force push / delete `main` | Disabled |
| Pull requests | Encouraged (review count may be `0`; still prefer PR workflow) |
| Ruleset | [Protect main — require CI](https://github.com/srishjaiz/customDevFolio/rules/18207562) |

Merge only when all required checks are **green**. Do not push directly to `main` for feature work.

## Versioning and changelog history

See [`CHANGELOG.md`](./CHANGELOG.md) ([Keep a Changelog](https://keepachangelog.com/)). Current work is tracked under **`[Unreleased]`** and **`[0.1.0]`** as applicable.

## License

MIT (workspace `Cargo.toml`). Add a root `LICENSE` file before publishing to crates.io if you distribute the crate publicly.
