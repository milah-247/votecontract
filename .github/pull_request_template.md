## Summary

<!-- What does this change and why? -->

Closes #

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `stellar contract build` passes
- [ ] New/changed behaviour has success **and** failure tests
- [ ] Auth changes have negative tests (wrong signer), not just `mock_all_auths`
- [ ] No error codes renumbered; README updated if errors/events/interface changed
- [ ] No secrets, keys or `.env` files committed

## Breaking changes

<!-- Interface, error code, event or storage layout changes. "None" if none. -->
