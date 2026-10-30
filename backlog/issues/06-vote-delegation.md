# Vote delegation (from the original Solidity design)

**Complexity:** High (200 points)
**Labels:** `enhancement`, `area: contract`, `complexity: high`
**Dependencies:** None. Coordinate with backlog item 08 (turnout counters) if both are in progress.

## Problem
The original Solidity `Voter` struct had a `delegate` field that was never used. It was dropped in the Soroban port. Delegation (as in the classic Solidity "Ballot" example) lets a voter hand their weight to someone else.

## Scope
- `delegate(from: Address, to: Address)`, requiring `from`'s auth and voting to be `Active`.
- Follow delegation chains with a **bounded** loop (e.g. at most 8 hops). Reject self-delegation and loops.
- If the final delegate has already voted, add the weight straight to their proposal's tally. Otherwise add it to the delegate's `weight`.
- Mark `from` as voted. Add a `delegate: Option<Address>` field to `Voter`. This is a storage layout change and must be documented.
- `delegated` event.

## Acceptance criteria
- [ ] Self-delegation, loops, chains that are too long, unregistered or invalid delegates, and already-voted delegators are all rejected with specific errors (appended).
- [ ] Total tally always equals the sum of the weights of voted voters.
- [ ] README and SECURITY.md updated.

## Testing requirements
- Unit tests for every error path, a direct delegate, a two-hop chain, and delegating to someone who has already voted.
- An invariant test that sum(tallies) == number of voters whose weight has been "spent".
