# Report ties instead of silently picking the lowest index

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `area: contract`, `complexity: medium`
**Dependencies:** None

## Problem
`winning_proposal()` returns the lowest-index proposal when two or more proposals share the top count (see the `tie_resolves_to_lowest_index` test). Callers can't tell a clear win from a tie, so a tied election shows up as a win for proposal 0.

## Scope
- Add `leading_proposals() -> Vec<u32>`: every index with the maximum count. Empty if no votes have been cast.
- Add `is_tie() -> bool`.
- Keep `winning_proposal()` behaviour unchanged for backward compatibility, and document the tie rule on it.

## Acceptance criteria
- [ ] New functions return correct results for no votes, a single winner, a two-way tie and an all-way tie.
- [ ] README interface table updated.

## Testing requirements
- Unit tests for each case above, including a 3+ proposal ballot.
- No change to existing test expectations.
