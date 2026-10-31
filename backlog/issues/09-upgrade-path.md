# Chairperson-controlled contract upgrade with version tracking

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `security`, `area: contract`, `complexity: medium`
**Dependencies:** None. Coordinate with backlog item 05 if both land.

## Problem
The deployed contract can't be patched. If a bug is found mid-election, the only option is to redeploy and lose all state.

## Scope
- `upgrade(new_wasm_hash: BytesN<32>)`, chairperson-only, calling `env.deployer().update_current_contract_wasm`.
- Decide and document whether upgrades are allowed while voting is `Active`. The recommendation is to block them with an error.
- `version() -> u32` constant.
- `upgraded` event.
- SECURITY.md: document that upgrades make the chairperson even more trusted.

## Acceptance criteria
- [ ] Non-chairperson upgrade fails.
- [ ] An upgrade test using a second WASM (e.g. via `contractimport!`) shows state survives the upgrade.

## Testing requirements
- Auth negative test; state-preservation test; test that upgrades are blocked while voting is active (if that policy is chosen).
