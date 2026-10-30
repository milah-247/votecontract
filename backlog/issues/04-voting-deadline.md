# Optional voting deadline enforced by ledger time

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `area: contract`, `complexity: medium`
**Dependencies:** None

## Problem
Voting only closes when the chairperson calls `end_voting`. If the chairperson goes missing, voting stays open indefinitely, and voters have no on-chain guarantee of when it closes.

## Scope
- Chairperson-only `set_deadline(timestamp: u64)`, allowed only in `NotStarted`. It must be in the future relative to `env.ledger().timestamp()`.
- `vote` returns `VotingEnded` when `ledger.timestamp() >= deadline`.
- After the deadline, **anyone** may call `end_voting` (no chairperson auth needed), so the state can be finalised.
- New read function `deadline() -> Option<u64>`.
- Emit a `deadline_set` event.

## Acceptance criteria
- [ ] Without a deadline, behaviour is unchanged.
- [ ] With a deadline, votes at `deadline - 1` succeed and votes at `deadline` fail.
- [ ] Non-chairperson `end_voting` succeeds only after the deadline.
- [ ] New errors (if any) appended; README updated.

## Testing requirements
- Use `env.ledger().set_timestamp` to test the boundaries.
- Auth tests: before the deadline, a non-chairperson calling `end_voting` fails, checked with `mock_auths` and the wrong signer.
