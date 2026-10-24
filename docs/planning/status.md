# Development Status

## Resume here

- Stage: Phase 1 complete (2026-10-24). `norm-adapter-engine` `0.1.0`
  is published on crates.io from the exact tagged candidate `34ef0f5`
  (signed tag `v0.1.0`; draft GitHub Release awaiting maintainer
  publication). The maintainer confirmed the crate name
  `norm-adapter-engine` over the single-name alternative (2026-10-24).
  Phase 1 skeleton had landed 2026-10-23: The repository exists under
  `CyanoOrg/norm-adapter-sdk` (public since 2026-10-24: free-plan private
  repositories cannot carry rulesets, so the maintainer approved flipping
  public and arming governance in one motion),
  and `norm-adapter-engine` `0.1.0` builds green with the four modules
  extracted from dsh-norm-spec `0.2.1`, identity-parameterized per D001:
  `norm_protocol.rs` (copied verbatim — already host-neutral),
  `prompt_context.rs`, `multi_prompt_context.rs`, `layout_index.rs`
  (constructors take `&AdapterIdentity`; error codes carry the adapter's
  product prefix at runtime). Sixteen unit tests preserve the dsh `0.2.1`
  golden output through a test-only DSH identity, and `cargo fmt`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test`, and
  `cargo doc --workspace --no-deps` all pass on the initial extraction.
- Phase 1 closure evidence: the family CI gate (`rust-quality`) is green
  on every push; cross-identity golden fixtures lock byte-for-byte parity
  with both adapters' current implementations (19 tests); organization-team
  access and the D014-style
  rulesets are live (2026-10-24): `main-integrity`, `main-quality`,
  `main-review` (review-layer bypass for `norm-release-managers` only), and
  `release-tag-immutable`, all active with no other bypass; maintainer naming review of the
  repository and crate labels before the first crates.io publication.
- Phase 2 (pi-norm-spec swap) and Phase 3 (multi-target and layout-index
  adoption) stay sequenced by pi-norm-spec
  `docs/planning/wsd-sdk-convergence-plan.md`.

## Verification

- 2026-10-23 initial extraction: fmt/clippy/test/doc green, 16/16 tests.
