# Keep election state alive during long elections (storage TTL)

**Complexity:** Medium (150 points)
**Labels:** `bug`, `area: storage`, `complexity: medium`
**Dependencies:** None

## Problem
`storage.rs` extends the TTL of instance storage and of each `Voter(Address)` persistent entry only when they are **written**, by about 30 days (`BUMP_AMOUNT`). If a voter is registered and then votes more than ~30 days later, or the contract sees no writes for ~30 days, entries can be archived. Read-only calls (`get_voter`, `winning_proposal`) never extend the TTL either, so the results of a finished election can also expire.

## Scope
- Add a permissionless `extend_ttl()` that extends the instance TTL, and an `extend_voter_ttl(voter)` that extends one voter entry (returns `VoterNotRegistered` if the voter doesn't exist).
- Extend the voter entry's TTL in `vote` before it is read/used. (It's already extended on write; check the read path.)
- Document the TTL behaviour and how to restore archived entries in the README "Storage" section.
- Out of scope: changing `BUMP_AMOUNT`.

## Acceptance criteria
- [ ] Both new functions exist, need no auth, and are documented with `///`.
- [ ] After advancing the ledger past the original TTL and calling the extend functions, the election is still usable in tests.
- [ ] README explains TTL, who should call the extend functions, and when.

## Testing requirements
- Use `env.ledger().with_mut(|l| l.sequence_number += …)` and `get_ttl()` (inside `env.as_contract`) to assert TTLs before and after.
- Negative test: `extend_voter_ttl` for an unknown address returns `VoterNotRegistered`.
- Existing tests keep passing.
