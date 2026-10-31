# Configure Dependabot for Cargo and GitHub Actions

**Complexity:** Trivial (100 points)
**Labels:** `dx`, `ci`, `good first issue`, `complexity: trivial`
**Dependencies:** None

## Problem
`soroban-sdk` and the CI actions get regular releases, and nothing prompts maintainers to update them.

## Scope
- Add `.github/dependabot.yml` covering `cargo` (root) and `github-actions`, with weekly checks and grouped minor/patch updates.

## Acceptance criteria
- [ ] Valid config. GitHub's "Dependency graph → Dependabot" page shows both ecosystems.

## Testing requirements
- None beyond config validation; include a screenshot or link in the PR.
