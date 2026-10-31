# Testnet deployment and interaction guide

**Complexity:** Trivial (100 points)
**Labels:** `documentation`, `good first issue`, `complexity: trivial`
**Dependencies:** None

## Problem
The README has a short deploy snippet but doesn't show how to call each function with the Stellar CLI or read the results.

## Scope
- `docs/testnet.md`: create and fund an identity, build, deploy (with constructor args), then `stellar contract invoke` examples for every function, including how to sign `vote` as a voter identity.
- Link it from the README.
- Use placeholder identities only. Never include secret keys.

## Acceptance criteria
- [ ] Every command was run against testnet by the contributor. Put the contract ID used in the PR description.
- [ ] Commands match the current interface.

## Testing requirements
- Manual testnet run, described in the PR.
