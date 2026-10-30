# Track registered voter count and votes cast

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `area: contract`, `complexity: medium`
**Dependencies:** None

## Problem
There's no way to get turnout on-chain. Voters are stored as separate persistent entries and can't be enumerated, so clients can't tell how many people are registered or have voted.

## Scope
- Counters in instance storage: `registered_count` (includes the chairperson) and `votes_cast`.
- Read functions `registered_voter_count()` and `votes_cast()`.
- Update them in the constructor, `give_right_to_vote` and `vote`. Decide and document whether `invalidate_voter` decrements an "eligible" count.

## Acceptance criteria
- [ ] Counters are correct after any mix of registrations, invalidations and votes.
- [ ] `votes_cast()` always equals the sum of proposal tallies while weight is 1.

## Testing requirements
- Unit tests plus a multi-voter scenario test asserting both invariants.
