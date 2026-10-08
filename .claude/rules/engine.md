---
paths:
  - "crates/ark/**"
---

# The `ark` simulation

- No dependencies outside dev-dependencies. Platform needs (time, input, storage) come
  in through the caller, never from `std` services.
- No panics on data from input or saves: use `get`, checked and saturating arithmetic,
  and clamp on the way in. Indexing is fine only when the bound is a type invariant.
- Determinism:
  - no clock
  - no thread-local or OS randomness; use the seeded generator in `sim/rng.rs`
  - no iteration over hash maps
  - no float operation whose result differs across platforms
- `tick` allocates nothing. The benchmark (`crates/ark/benches/tick.rs`) asserts zero
  allocations and fails the gate.
- Rule constants live in `tuning.rs` and authored sectors in `sectors.rs`, which is
  parsed at compile time.
- Snapshots: `crates/ark/tests/snapshots/`, reviewed with `cargo insta`. Update one only
  when the PR says why the behaviour changed.
