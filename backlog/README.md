# Drips Wave issue backlog (draft)

These are draft GitHub issues for maintainers to review. **None of them have been published yet.** Each issue can be picked up on its own and has a problem statement, scope, acceptance criteria, testing requirements, dependencies, suggested labels and a Wave complexity.

| # | Title | Type | Complexity | Points |
|---|---|---|---|---|
| 01 | Keep election state alive during long elections (storage TTL) | bug / storage | Medium | 150 |
| 02 | Report ties instead of silently picking the lowest index | feature | Medium | 150 |
| 03 | `final_winner()` that only answers once voting has ended | feature | Medium | 150 |
| 04 | Optional voting deadline enforced by ledger time | feature | Medium | 150 |
| 05 | Two-step chairperson transfer | feature / security | Medium | 150 |
| 06 | Vote delegation | feature | High | 200 |
| 07 | Batch voter registration | feature | Medium | 150 |
| 08 | Track registered voter count and votes cast | feature | Medium | 150 |
| 09 | Chairperson-controlled contract upgrade | feature / security | Medium | 150 |
| 10 | Property-based tests for tally invariants | testing | Medium | 150 |
| 11 | Resource usage regression tests | testing | Medium | 150 |
| 12 | Integration tests against the compiled WASM | testing / CI | Medium | 150 |
| 13 | Failed calls emit no events and change no state | testing | Trivial | 100 |
| 14 | Dependency vulnerability scanning in CI | security / CI | Trivial | 100 |
| 15 | Dependabot for Cargo and GitHub Actions | DX | Trivial | 100 |
| 16 | Code coverage reporting in CI | testing / CI | Medium | 150 |
| 17 | Testnet deployment and interaction guide | docs | Trivial | 100 |
| 18 | Architecture and storage layout documentation | docs | Trivial | 100 |
| 19 | Generated TypeScript bindings and example client | DX | High | 200 |
| 20 | Reproducible deploy script | DX | Medium | 150 |
| 21 | CHANGELOG and tagged release workflow | CI / docs | Medium | 150 |
| 22 | Threat model document mapped to tests | security / docs | Medium | 150 |

**Total:** 22 issues (5 Trivial, 15 Medium, 2 High) = 3,150 points. Check this against your Wave points budget before publishing everything at once.

## Possible merge conflicts

These items touch the same code. They don't block each other, but whoever merges second will need to rebase:

- New error codes: 03, 04, 05, 06, 07, 09. Always append at merge time.
- `Voter` struct / `vote` path: 01, 06, 08.
- `.github/workflows/`: 12, 14, 16, 19, 20, 21.

## Publishing

1. Review and edit the files in `issues/`.
2. Make sure the repo has been approved for the Wave program in the [Drips Wave app](https://www.drips.network/wave).
3. Run `scripts/create-backlog-issues.sh` to preview, then `scripts/create-backlog-issues.sh --publish` to create the labels and issues with `gh`.
4. Add each issue to the Wave and set its complexity in the Drips app (or apply the program label, e.g. `Stellar Wave`, if your repo is approved).
