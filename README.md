# VoteContract — Ballot on Stellar Soroban

[![CI](https://github.com/milah-247/votecontract/actions/workflows/ci.yml/badge.svg)](https://github.com/milah-247/votecontract/actions/workflows/ci.yml)

A chairperson-managed election contract for [Stellar Soroban](https://developers.stellar.org/docs/build/smart-contracts/overview), written in Rust. One registered voter, one vote.

> This repository started as a Solidity/Foundry `Ballot` contract (see commit `693b8eb`). It has been ported to Soroban. The behaviour is the same, with fixes listed under [Changes from the Solidity version](#changes-from-the-solidity-version).

> **Status:** not audited. Do not use it for elections that matter until it has had an independent security review. See [SECURITY.md](SECURITY.md).

## How it works

```
             start_voting            end_voting
NotStarted ───────────────▶ Active ───────────────▶ Ended (final)
  register / invalidate      register / invalidate     read-only
                             vote
```

| Role | Can do |
|---|---|
| **Chairperson** (set at deploy) | `start_voting`, `end_voting`, `give_right_to_vote`, `invalidate_voter`. Also registered as a voter. |
| **Registered voter** (age 18–150) | `vote` once while voting is `Active` |
| **Anyone** | All read-only queries |

Authorization uses Soroban's `require_auth()`. Chairperson functions need the stored chairperson's signature, and `vote(voter, …)` needs `voter`'s signature. Nobody can vote on someone else's behalf.

## Contract interface

| Function | Auth | Description |
|---|---|---|
| `__constructor(chairperson, proposal_names)` | — | Runs once at deploy. 1–32 unique, non-empty names, each up to 64 bytes. |
| `start_voting()` | chairperson | `NotStarted → Active`; records the ledger timestamp |
| `end_voting()` | chairperson | `Active → Ended` (final) |
| `give_right_to_vote(voter, age)` | chairperson | Register a voter with weight 1. Not allowed after the election ends. |
| `invalidate_voter(voter)` | chairperson | Block a registered voter who has not voted yet |
| `vote(voter, proposal)` | voter | Cast one vote for a proposal index |
| `chairperson()` / `state()` / `voting_started()` | — | Configuration and lifecycle |
| `voting_start_time()` / `voting_end_time()` | — | Ledger timestamps (0 if not set yet) |
| `get_voter(voter)` | — | `Option<Voter>` (replaces `getMyVoterCard`) |
| `proposal(index)` / `proposals()` / `proposal_count()` | — | Proposals and their tallies |
| `winning_proposal()` / `winner_name()` | — | Current leader. A tie goes to the lowest index. Errors if no votes have been cast. |

### Error codes

| Code | Error | Code | Error |
|---|---|---|---|
| 1 | `AlreadyVoted` | 9 | `InvalidProposal` |
| 2 | `NoRightToVote` | 10 | `NoProposals` |
| 3 | `VoterUnderage` | 11 | `TooManyProposals` |
| 4 | `InvalidVoterCard` | 12 | `InvalidProposalName` |
| 5 | `VotingNotStarted` | 13 | `InvalidAge` |
| 6 | `VotingAlreadyStarted` | 14 | `NoVotesCast` |
| 7 | `VotingEnded` | 15 | `VoterNotRegistered` |
| 8 | `VoterAlreadyRegistered` | 16 | `Overflow` |

Codes are part of the public API. They are never renumbered; new errors are added at the end.

### Events

| Event | Topics | Data |
|---|---|---|
| `voting_started` | — | `timestamp` |
| `voting_ended` | — | `timestamp` |
| `voter_registered` | `voter` | `age` |
| `voter_invalidated` | `voter` | — |
| `vote_cast` | `voter` | `proposal`, `weight` |

### Storage

| Data | Storage type | TTL handling |
|---|---|---|
| Chairperson, state, timestamps, proposals and tallies | Instance | Extended to ~30 days on every write |
| Voter cards (`Voter(Address)`) | Persistent, one entry per voter | Extended to ~30 days on every write |

Proposals are capped at 32 so that `winning_proposal` and the instance entry stay small.

## Repository layout

```
.
├── Cargo.toml                 # workspace; soroban-sdk version pinned here
├── rust-toolchain.toml        # stable Rust + wasm32v1-none target
├── contracts/ballot/
│   └── src/
│       ├── lib.rs             # contract entry points
│       ├── types.rs           # Error, ElectionState, Voter, Proposal, DataKey, limits
│       ├── storage.rs         # storage access + TTL extension
│       ├── events.rs          # #[contractevent] definitions
│       └── test.rs            # unit tests
├── backlog/                   # drafted GitHub issues for Drips Wave (not yet published)
└── .github/                   # CI, issue and PR templates
```

## Development

### Prerequisites

- Rust via [rustup](https://rustup.rs). `rust-toolchain.toml` installs the right toolchain, `rustfmt`, `clippy` and the `wasm32v1-none` target automatically.
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli) v25.2 or later (CI uses v28.0.0). soroban-sdk 29 refuses to build WASM through plain `cargo build`, so `make build` calls `stellar contract build`. You don't need the CLI for `make test` or `make lint`.

### Common commands

```bash
make test        # cargo test --workspace
make fmt         # cargo fmt --all
make lint        # cargo clippy --all-targets -- -D warnings
make build       # stellar contract build -> target/wasm32v1-none/release/ballot.wasm
make check       # everything CI runs
```

### Deploying to testnet (manual)

Nothing in this repo deploys automatically. To try it on testnet:

```bash
stellar keys generate --global alice --network testnet --fund
make build
stellar contract deploy \
  --wasm target/wasm32v1-none/release/ballot.wasm \
  --source alice --network testnet \
  -- --chairperson alice --proposal_names '["Alice","Bob"]'
```

Never commit secret keys. `.stellar/` and `.env` are git-ignored.

## Changes from the Solidity version

| Solidity behaviour | Soroban port |
|---|---|
| `msg.sender` checks | `require_auth()` on the stored chairperson or the voter |
| Constructor | `__constructor`; runs atomically at deploy and cannot be re-run |
| Out-of-range `vote(proposal)` panicked with a generic array error | Returns `InvalidProposal` with no side effects |
| `invalidateVoter` on an unknown address silently wrote an empty record | Returns `VoterNotRegistered` |
| `invalidateVoter` after the voter had voted left the vote counted | Returns `AlreadyVoted` |
| Re-registering a voter reported `NoRightToVote` | Reports `VoterAlreadyRegistered` |
| `winningProposal` returned 0 when nobody had voted | Returns `NoVotesCast` |
| No way to close voting | `end_voting` and a final `Ended` state |
| No input limits | 1–32 unique proposals, names up to 64 bytes, age 18–150 |
| Unused `delegate` field | Removed; delegation is tracked in the backlog |
| `uint` tallies | `u64` tallies with checked addition |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Open issues are in [`backlog/`](backlog/) and on GitHub. This project takes part in [Drips Wave](https://docs.drips.network/wave/).

## License

MIT. See [LICENSE](LICENSE).
