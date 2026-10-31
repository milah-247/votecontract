# Add `final_winner()` that only answers once voting has ended

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `area: contract`, `complexity: medium`
**Dependencies:** Soft dependency on backlog item 02 (tie reporting). Can be done independently.

## Problem
`winning_proposal()` can be called while voting is `Active`, so a front-end can show a "winner" mid-election. There's no on-chain way to ask for the **final** result only.

## Scope
- Append a new error `VotingNotEnded` at the end of `Error` (next free code at merge time).
- Add `final_winner() -> Result<u32, Error>`: returns `VotingNotEnded` unless the state is `Ended`, then behaves like `winning_proposal()`.
- Leave `winning_proposal()` unchanged as the "live leader" query.

## Acceptance criteria
- [ ] New error appended without renumbering existing codes; README error table updated.
- [ ] `final_winner` returns `VotingNotEnded` in `NotStarted` and `Active`, `NoVotesCast` when ended with no votes, and the winner otherwise.

## Testing requirements
- Unit tests for all three states, plus an ended election with no votes.
