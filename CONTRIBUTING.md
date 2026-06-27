# Contributing

## One-time setup (required)

From the repo root:

```bash
./scripts/setup-git.sh
```

This sets:

| Setting | Purpose |
|---------|---------|
| `commit.template` → `.gitmessage` | Editor opens with commit message guidance |
| `core.hooksPath` → `.githooks` | Enforces commit format and changelog on push |

Without this, commits may be rejected by hooks once configured, and teammates won’t share the same checks.

## Commit messages (required)

Use **[Conventional Commits](https://www.conventionalcommits.org/)**:

```text
<type>(<optional-scope>): <summary>

[optional body]

[optional footer]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`  
**Scopes (examples):** `cli`, `template`, `ci`, `docs`

The `commit-msg` hook rejects non-conforming subjects (except merge/revert).

Examples:

```text
feat(cli): add --minimal scaffold mode
fix(template): respect sections.blog when posts empty
docs: add PR and commit templates
ci: require status checks on main
```

Prefer `git commit` (opens template) over one-line `-m` until you’re fluent with the format. If you use `-m`, still follow the pattern above.

## Changelog (required for user-facing changes)

Update [`CHANGELOG.md`](./CHANGELOG.md) in the **same PR** (ideally same commit or a follow-up on the branch) for:

- `feat`, `fix`, `perf`
- Any `BREAKING CHANGE`

Add bullets under `## [Unreleased]` (or the next version section) using Keep a Changelog categories: **Added**, **Changed**, **Fixed**, **Removed**.

The `pre-push` hook blocks pushes that introduce `feat`/`fix`/`perf` commits without any `CHANGELOG.md` change on the branch. Escape hatch (discouraged): `SKIP_CHANGELOG_CHECK=1 git push`.

Docs-only, CI-only, or pure chore work may skip the changelog; say so in the PR template checkbox.

## Pull requests (required template)

Opening a PR loads [`.github/PULL_REQUEST_TEMPLATE.md`](./.github/PULL_REQUEST_TEMPLATE.md). Fill in:

1. **Summary**
2. **Changelog** (checkbox + entry or N/A)
3. **Type of change**
4. **Test plan**

`main` requires green CI (**Rust (cli)**, **Next template**) before merge. See README **Branch protection**.

## Local checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# CI also runs: cargo llvm-cov --workspace --fail-under-lines 80
# if you touched template/
cd template && pnpm typecheck && pnpm test && pnpm test:coverage && pnpm build
```

## CI changelog check

On pull requests, the **Changelog** job runs `./scripts/check-changelog.sh` against `origin/<base>..HEAD`.

It fails if any commit subject (or the PR title) starts with `feat` / `fix` / `perf` and `CHANGELOG.md` is not changed on the branch.

Local preview:

```bash
./scripts/check-changelog.sh origin/main HEAD
```

Escape hatch in a commit subject (discouraged): `[skip changelog]`.
