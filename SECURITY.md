# Security Policy

## Status

This contract **has not been independently audited.** Maintainers have reviewed it and it has unit tests (see `contracts/ballot/src/test.rs`), but that is not an audit. Don't use it to run elections with real-world consequences until it has had a professional review.

## Supported versions

| Version | Supported |
|---|---|
| `main` (Soroban port) | ✅ |
| Original Solidity contract (commit `693b8eb`) | ❌ superseded |

## Reporting a vulnerability

**Don't open a public issue, PR or discussion for a security problem.**

Report it privately through GitHub's [private vulnerability reporting](https://github.com/milah-247/votecontract/security/advisories/new) (Security tab → "Report a vulnerability").

Please include:

- the affected function(s) and commit
- a description of the impact
- a minimal reproduction, ideally a failing Soroban unit test

We'll aim to acknowledge reports within 5 business days and keep you updated until the issue is fixed. Once a fix is released we'll credit you in the advisory, unless you'd prefer to stay anonymous.

Please don't test against contracts deployed by other people, and don't access data that isn't yours.

## Scope

In scope: the contract code in `contracts/`, its storage and authorization model, and the build and CI configuration.

Out of scope: third-party dependencies (report those upstream; tell us if they affect this contract), and vulnerabilities in the Stellar network or Soroban host themselves.

## Security model

These properties are intended and enforced by tests. A report that breaks any of them is in scope:

1. Only the chairperson set at deployment can start or end voting, register voters, or invalidate voters (`require_auth` on the stored chairperson).
2. A vote for `voter` can only be cast with `voter`'s authorization.
3. Each registered, valid voter adds at most `weight` (currently 1) to exactly one proposal, once.
4. State only moves forward, `NotStarted → Active → Ended`, and `Ended` is final.
5. The constructor runs exactly once, at deployment, so the contract can't be re-initialised.
6. Tallies can't overflow (`checked_add`).

### Known trust assumptions and limitations

- **The chairperson is fully trusted.** They decide who may vote, can invalidate any voter who hasn't voted yet, and decide when voting ends. Nothing on-chain checks their decisions.
- **Age is self-reported by the chairperson.** The contract has no way to verify it.
- **Voter registration is public.** Voter addresses and their choices are visible on-chain; there is no ballot secrecy.
- **Live results:** `winning_proposal` can be read while voting is still open.
- **Storage TTL:** entries are extended to roughly 30 days on every write. If an election runs longer than that without activity, entries may be archived and need restoring before use.
- **Not upgradable:** there is currently no upgrade entry point.
