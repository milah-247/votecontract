# Resource usage regression tests

**Complexity:** Medium (150 points)
**Labels:** `testing`, `performance`, `complexity: medium`
**Dependencies:** None

## Problem
Soroban transactions have CPU and memory limits. `vote` rewrites the whole proposal list (up to 32 entries) in instance storage, and `winning_proposal` loops over all proposals. Nothing tracks what these cost, so a regression would go unnoticed until deployment.

## Scope
- Tests that deploy a ballot with `MAX_PROPOSALS` proposals of `MAX_PROPOSAL_NAME_LEN` bytes, then measure `vote`, `winning_proposal` and the constructor with `env.cost_estimate()`.
- Assert each stays under documented thresholds, with headroom, and record the measured numbers in a code comment.

## Acceptance criteria
- [ ] Tests fail if costs grow by more than the set margin.
- [ ] README "Storage" section records the worst-case numbers.

## Testing requirements
- The tests themselves. Run locally and in CI.
