# Two-step chairperson transfer

**Complexity:** Medium (150 points)
**Labels:** `enhancement`, `security`, `area: auth`, `complexity: medium`
**Dependencies:** None

## Problem
The chairperson is fixed at deployment. If their key is lost or needs rotating, the election can't be administered any more. A one-step transfer to a mistyped address would lose control permanently.

## Scope
- `propose_chairperson(new: Address)` (current chairperson auth) stores a pending chairperson.
- `accept_chairperson()` (pending chairperson auth) completes the transfer.
- `cancel_chairperson_transfer()` (current chairperson auth).
- Events for proposed and accepted.
- Decide and document whether the new chairperson is automatically registered as a voter. The recommendation is **no**.

## Acceptance criteria
- [ ] Only the current chairperson can propose or cancel; only the pending address can accept.
- [ ] After acceptance, the old chairperson can no longer call chairperson functions.
- [ ] SECURITY.md security model updated.

## Testing requirements
- Positive and negative auth tests for each function, using `sign_as` / `mock_auths` with explicit signers.
- Test that the old chairperson's `start_voting` fails after the transfer.
