# Batch voter registration

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `area: contract`, `complexity: medium`
**Dependencies:** None

## Problem
Registering N voters takes N transactions, each signed by the chairperson. That's slow and expensive for real elections.

## Scope
- `give_right_to_vote_batch(voters: Vec<(Address, u32)>)`, chairperson-only, with at most `MAX_BATCH` (e.g. 50) entries.
- Atomic: if any entry is invalid, the whole call fails, using the same validation and errors as `give_right_to_vote`.
- Reject duplicates inside the batch.
- Emit one `voter_registered` event per voter.
- Share the validation logic with `give_right_to_vote` rather than copying it.

## Acceptance criteria
- [ ] Empty batch, oversized batch, duplicate entries, underage voters and already-registered voters are all handled and tested.
- [ ] Failure leaves no partial registrations.

## Testing requirements
- Unit tests for each case above, plus a resource check (`env.cost_estimate()`) for a full batch to confirm it fits within limits.
