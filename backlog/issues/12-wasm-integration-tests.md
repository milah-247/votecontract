# Integration tests against the compiled WASM

**Complexity:** Medium (150 points)
**Labels:** `testing`, `ci`, `complexity: medium`
**Dependencies:** None

## Problem
All tests run the contract as native Rust. Differences that only show up in the `wasm32v1-none` build (e.g. `panic = "abort"`, `overflow-checks`, size limits) aren't tested.

## Scope
- Add an integration test crate (e.g. `tests/integration`) that loads `target/wasm32v1-none/release/ballot.wasm` with `soroban_sdk::contractimport!` and runs a full election: deploy, register, start, vote, end, winner.
- Update CI so the WASM is built before this test runs. Document the order in CONTRIBUTING.md.

## Acceptance criteria
- [ ] `make check` and CI run the integration test after building the WASM.
- [ ] The test covers at least one error path (e.g. duplicate vote) through the WASM.

## Testing requirements
- The integration test itself, passing in CI.
