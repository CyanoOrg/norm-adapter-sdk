# Contributing

Read `AGENTS.md` before opening a change.

A contribution must preserve the core boundary: host variance enters only
through `AdapterIdentity`; bridge protocols, sealed-payload verification,
upstream pins, and host injection idioms belong in the adapter repositories,
and format semantics belong upstream in norm-spec.

External contributors should fork the public repository, create one
purpose-focused branch, and open a pull request. Commits must be signed and
use Conventional Commit prefixes. The protected `main` branch requires
linear history, passing CI, one approving review, and resolved review
threads. Workflows from external contributors require explicit maintainer
approval.

Run before review:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

Output-affecting changes must update the golden fixtures in the same change
and recapture them from both adapters.
