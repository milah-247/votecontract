# Generated TypeScript bindings and example client

**Complexity:** High (200 points)
**Labels:** `dx`, `enhancement`, `complexity: high`
**Dependencies:** Works best after backlog item 17 (testnet guide), but not blocked by it.

## Problem
Front-end developers have no typed client for the contract.

## Scope
- Script `scripts/bindings.sh` that runs `stellar contract bindings typescript` against the built WASM into `bindings/ts`.
- `examples/ts`: a minimal Node script that reads `proposals()`, `state()` and `get_voter()` from a testnet contract ID passed in through an environment variable, and shows how a voter would sign `vote`.
- CI job that regenerates the bindings and fails if they're out of date (or document why they're generated rather than committed).

## Acceptance criteria
- [ ] `npm ci && npm run build` succeeds in `examples/ts`.
- [ ] The README in `examples/ts` explains setup without any secrets.

## Testing requirements
- The example script type-checks in CI; a manual testnet run is described in the PR.
