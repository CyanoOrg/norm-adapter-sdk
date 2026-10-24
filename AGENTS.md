# AGENTS.md

Entry point for humans and agents working on norm-adapter-sdk.

## Purpose

norm-adapter-sdk is the family SDK decided by pi-norm-spec D018 (mirrored
here as D001): one host-agnostic implementation of the engine-side
merge/projection over normalized norm-spec machine output. Host adapters
(pi-norm-spec, dsh-norm-spec) consume this crate under an exact pin and own
their bridges, sealing, pins, injection idioms, and releases.

Read first:

- `docs/decisions.md` for the immutable extraction boundary.
- `docs/planning/status.md` for live state.
- pi-norm-spec `docs/planning/wsd-sdk-convergence-plan.md` for the phase
  plan and gates.

## Rules

- Rust owns all computation here; there is no TypeScript in this repository.
- Host variance enters only through `AdapterIdentity`; anything else that
  differs between hosts belongs in an adapter repository.
- `unsafe` is forbidden. Production code avoids `unwrap` and `expect`.
- Structured outputs stay versioned and deterministic; fail instead of
  truncating or summarizing.
- Decisions are append-only; non-trivial scope changes require a decision
  record before implementation.

## Common commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

## Sources of truth

| Concern | Source of truth |
|---|---|
| extraction boundary | `docs/decisions.md` D001 |
| in-flight state | `docs/planning/status.md` |
| phase plan and gates | pi-norm-spec `wsd-sdk-convergence-plan.md` |
| `.norm` format | upstream `norm-spec` |
