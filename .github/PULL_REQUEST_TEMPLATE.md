## Summary

<!-- 1–3 bullets: what this PR does and why -->

-

## Changelog

<!-- Required for user-facing or behavior changes. Keep aligned with CHANGELOG.md -->

- [ ] Updated [`CHANGELOG.md`](../CHANGELOG.md) under the appropriate version / `Unreleased` section
- [ ] No user-facing change (docs-only, chore, CI-only) — changelog N/A

### Entry (paste or summarize)

```markdown
### Added | Changed | Fixed | Removed
- …
```

## Type of change

- [ ] `feat` — new feature
- [ ] `fix` — bug fix
- [ ] `docs` — documentation only
- [ ] `ci` — CI / tooling
- [ ] `chore` — maintenance
- [ ] `refactor` — no behavior change
- [ ] `test` — tests only
- [ ] `breaking` — breaking change (describe migration below)

## Test plan

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Template (if touched): `cd template && pnpm typecheck && pnpm build`
- [ ] Manual: …

## Commit style

Commits on this PR should follow [Conventional Commits](https://www.conventionalcommits.org/) (see `.gitmessage` and `CONTRIBUTING.md`). Example: `feat(cli): add --minimal flag`.

## Notes for reviewers

<!-- Optional: risks, follow-ups, screenshots -->
