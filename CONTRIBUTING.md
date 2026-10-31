# Contributing

Thanks for helping. This guide covers everything you need to go from a fresh clone to a merged PR.

## Drips Wave contributors

This repo takes part in [Drips Wave](https://docs.drips.network/wave/). Here's how it works for you:

1. Find an open issue with the Wave program label (for example `Stellar Wave`). Each issue has a complexity label:
   `complexity: trivial` (100 pts), `complexity: medium` (150 pts) or `complexity: high` (200 pts).
2. Apply through the [Drips Wave app](https://www.drips.network/wave). Don't start work until a maintainer assigns you. Unassigned PRs may be closed.
3. Open your PR before the Wave ends. Issues are only credited if they are resolved during an active Wave.
4. One issue per PR. Put `Closes #<issue>` in the PR description.

If an issue is unclear, ask in the issue before you start rather than guessing.

## Setup

```bash
git clone https://github.com/milah-247/votecontract.git
cd votecontract
make check     # fmt-check + clippy + tests + WASM build
```

`rust-toolchain.toml` pins stable Rust and installs `rustfmt`, `clippy` and the `wasm32v1-none` target, so `rustup` does the rest. You need the [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli) v25.2 or later to build the WASM (`make build`), because soroban-sdk 29 won't build contracts through plain `cargo`. Tests and lint don't need it.

## Before you open a PR

All of these must pass, and CI runs the same checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
stellar contract build
```

## Code guidelines

- **Every state change needs tests**, both the success path and each error it can return. Use `client.try_*` and assert the exact error, e.g. `assert_eq!(client.try_vote(&v, &0), Err(Ok(Error::AlreadyVoted)))`.
- **Authorization tests must not rely only on `mock_all_auths()`.** Assert `env.auths()` for the expected signer, and add a negative test that uses `mock_auths` with the wrong signer. The helpers `assert_auth` and `sign_as` in `test.rs` do this.
- **Error codes are append-only.** Never renumber or reuse a code in `types.rs`. Add new variants at the end and document them in the README table.
- **Events are a public API.** Changing an event's name or fields is a breaking change; say so in the PR.
- **Storage:** election-wide data goes in instance storage; per-address data goes in persistent storage, and its TTL must be extended when it's written (see `storage.rs`). Explain any new key in the PR.
- **Bound every loop** over user-supplied data (see `MAX_PROPOSALS`).
- No `unwrap()`/`expect()` on user-controlled input in contract code; return an `Error` instead.
- Keep public functions documented with `///` comments, including an `# Errors` section.
- Don't weaken or delete existing tests to make a change pass. If a behaviour change makes a test wrong, explain why in the PR.

## Commits and PRs

- Use [Conventional Commits](https://www.conventionalcommits.org/) prefixes: `feat:`, `fix:`, `test:`, `docs:`, `ci:`, `refactor:`, `chore:`.
- Fill in the PR template, including what you tested.
- Keep PRs focused. Unrelated formatting or refactoring makes review slower.

## Reporting bugs and security issues

- Bugs and feature ideas: open a GitHub issue using the templates.
- **Security vulnerabilities: don't open a public issue.** Follow [SECURITY.md](SECURITY.md).

## License

By contributing, you agree that your contributions are licensed under the MIT License.
