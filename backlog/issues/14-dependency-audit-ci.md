# Add dependency vulnerability scanning to CI

**Complexity:** Trivial (100 points)
**Labels:** `security`, `ci`, `good first issue`, `complexity: trivial`
**Dependencies:** None

## Problem
CI doesn't check `Cargo.lock` for crates with known security advisories (RustSec).

## Scope
- New workflow `.github/workflows/audit.yml` running `cargo audit` (e.g. via `rustsec/audit-check`) on PRs that touch `Cargo.toml` or `Cargo.lock`, and weekly on a schedule.
- Set least-privilege `permissions:`.

## Acceptance criteria
- [ ] The workflow runs and passes on `main`, or any advisories it finds are written up as new issues.
- [ ] CONTRIBUTING.md mentions how to run `cargo audit` locally.

## Testing requirements
- Link a successful workflow run in the PR.
