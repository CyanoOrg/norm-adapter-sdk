## Summary

Describe the problem and the resulting adapter behavior.

## Boundary and security impact

- [ ] Host variance enters only through `AdapterIdentity`; no adapter-specific
      logic is duplicated here.
- [ ] No YAML parsing, filesystem walking, schema validation, process
      lifecycle, or network I/O was added.
- [ ] Output-affecting changes update the golden fixtures in the same change
      and recapture them from both adapters.

## Verification

List the commands you ran.

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps`
- [ ] Documentation and changelog are updated when needed.
