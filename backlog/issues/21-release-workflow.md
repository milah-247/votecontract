# CHANGELOG and tagged release workflow that publishes the WASM

**Complexity:** Medium (150 points)
**Labels:** `ci`, `documentation`, `complexity: medium`
**Dependencies:** None

## Problem
There are no releases. Anyone deploying has to build the contract themselves, and there's no record of interface changes between versions.

## Scope
- `CHANGELOG.md` in the [Keep a Changelog](https://keepachangelog.com) format, with an entry for the Soroban port.
- `.github/workflows/release.yml`, triggered on `v*` tags: builds the WASM, computes its SHA-256, and creates a GitHub release with the WASM and checksum attached.
- Least-privilege permissions (`contents: write` only on the release job).

## Acceptance criteria
- [ ] A test tag on a fork produces a release with the artifact and checksum (link in the PR).
- [ ] CONTRIBUTING.md describes the release steps.

## Testing requirements
- Link to the fork's release run.
