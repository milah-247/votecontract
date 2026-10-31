# Architecture and storage layout documentation

**Complexity:** Trivial (100 points)
**Labels:** `documentation`, `good first issue`, `complexity: trivial`
**Dependencies:** None

## Problem
New contributors have to read all of `lib.rs`, `storage.rs` and `types.rs` to understand the state machine and where data is kept.

## Scope
- `docs/architecture.md` covering: the state machine (Mermaid diagram), what each function does in each state, every `DataKey` with its storage type and TTL policy, and which errors each function can return.
- Link it from README and CONTRIBUTING.

## Acceptance criteria
- [ ] Matches the current code; the reviewer checks it against `lib.rs`.
- [ ] The Mermaid diagram renders on GitHub.

## Testing requirements
- None (docs only).
