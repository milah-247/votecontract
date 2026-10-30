# Reproducible deploy script

**Complexity:** Medium (150 points)
**Labels:** `dx`, `complexity: medium`
**Dependencies:** None

## Problem
Deployment is copy-paste from the README. It's easy to deploy the wrong WASM or to the wrong network.

## Scope
- `scripts/deploy.sh <network> <identity> <chairperson> <proposal names JSON>` that builds, prints the WASM hash, deploys with constructor args and prints the contract ID.
- Refuse to deploy to `mainnet` unless `CONFIRM_MAINNET=yes` is set.
- `set -euo pipefail`; never echo secrets.
- Add a `shellcheck` step to CI.

## Acceptance criteria
- [ ] Script works on testnet (contract ID in the PR).
- [ ] shellcheck passes in CI.

## Testing requirements
- Manual testnet run and CI shellcheck.
