# Test that failed calls emit no events and change no state

**Complexity:** Trivial (100 points)
**Labels:** `testing`, `good first issue`, `complexity: trivial`
**Dependencies:** None

## Problem
Indexers trust events. Tests check events on successful calls, but none confirm that a failing call (e.g. `vote` with `AlreadyVoted`) emits nothing and leaves storage unchanged.

## Scope
- For each state-changing function (`start_voting`, `end_voting`, `give_right_to_vote`, `invalidate_voter`, `vote`), add a test that triggers at least one error with `try_*`. Then assert `env.events().all().events().is_empty()` and that the relevant read functions return the same values as before.

## Acceptance criteria
- [ ] At least one failure-path test per state-changing function.

## Testing requirements
- New tests only. Don't change the contract.
