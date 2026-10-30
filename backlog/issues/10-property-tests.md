# Property-based tests for tally invariants

**Complexity:** Medium (150 points)
**Labels:** `testing`, `complexity: medium`
**Dependencies:** None

## Problem
The unit tests cover specific scenarios. Nothing checks the core invariants across random sequences of registrations, invalidations and votes.

## Scope
- Add `proptest` as a dev-dependency.
- Generate random sequences of operations (register, invalidate, vote, start, end) with random voters and proposal indices. Apply them via `try_*` calls.
- After each sequence, assert:
  1. the sum of tallies equals the number of cards with `voted == true`;
  2. no card has `voted == true` and `is_valid == false` unless it was invalidated before voting (which should be impossible);
  3. state only ever moved forward.
- Keep the default case count fast enough for CI (under 30s).

## Acceptance criteria
- [ ] The property test lives in its own module and runs in `cargo test`.
- [ ] Shrinking produces readable failing cases (implement `Debug` on the op enum).

## Testing requirements
- Show in the PR that the test catches a deliberately introduced double-count bug. Revert the bug before merging.
