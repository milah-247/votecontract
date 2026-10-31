# Code coverage reporting in CI

**Complexity:** Medium (150 points)
**Labels:** `testing`, `ci`, `complexity: medium`
**Dependencies:** None

## Problem
There's no measurement of which contract lines and branches the tests cover, so gaps aren't visible in review.

## Scope
- CI job using `cargo-llvm-cov` (e.g. `taiki-e/install-action`) that writes a coverage summary to the job summary and uploads an `lcov` artifact.
- Choose a minimum line-coverage threshold slightly below the current measured value, and fail the job below it.
- `make coverage` target.

## Acceptance criteria
- [ ] The job runs on PRs; the threshold and the current number are documented in CONTRIBUTING.md.

## Testing requirements
- Link a CI run showing the coverage summary.
