# Contributing

Minimal path — **no mandatory setup script**.

```bash
git clone <repo> && cd customFolio   # or your fork
git checkout -b my-change
# … edit …
make check                          # fmt + clippy + tests (+ template if pnpm is installed)
git add -A && git commit            # opens .gitmessage template after first `make` / `make setup`
git push -u origin HEAD
gh pr create                        # PR template is applied automatically
```

`make check` (and `make setup`) auto-configure this clone’s commit template and git hooks. Hooks are **advisory** (tips only). **CI is the source of truth.**

## What CI requires on PRs

| Check | When it matters |
|-------|-----------------|
| **Rust (cli)** | Always — `fmt`, `clippy`, tests + coverage (≥80% lines), release smoke |
| **Next template** | Always — typecheck, vitest coverage, build |
| **Changelog** | If commits or PR title are `feat` / `fix` / `perf` — update [`CHANGELOG.md`](./CHANGELOG.md) under `## [Unreleased]` |

You do **not** need `cargo-llvm-cov` or local coverage tools; CI runs those gates.

## Commit messages (preferred)

[Conventional Commits](https://www.conventionalcommits.org/):

```text
feat(cli): add --minimal scaffold mode
fix(template): hide blog when posts empty
docs: clarify contribute flow
```

Types: `feat` `fix` `docs` `style` `refactor` `perf` `test` `build` `ci` `chore` `revert`.

## Make targets

| Command | Purpose |
|---------|---------|
| `make` / `make check` | Local checks before a PR |
| `make setup` | Wire commit template + hooks (also done by `make check`) |
| `make test` | `cargo test --workspace` |
| `make fmt` | Apply `rustfmt` |
| `make template-test` | Template typecheck + unit tests + build |
| `make contribute` | Print this flow |
| `make help` | List targets |

Without Make: `./scripts/check.sh` (same as `make check`).

## Scope tips

- **CLI only** — `SKIP_TEMPLATE=1 make check` or just `cargo test --workspace` after `cargo fmt` / `clippy`.
- **Template only** — `SKIP_RUST=1 make check` (needs `pnpm`).
- Prefer a **feature branch + PR** into `main` (`main` is protected).

Questions? Open an issue or draft PR and iterate with CI.
