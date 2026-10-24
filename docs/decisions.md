# Decision Records

Decision records are append-only. Reversals are new records referencing the
decision they replace.

## D001 — Mirror pi-norm-spec D018: extract the shared engine, parameterize only identity

**Decision.** This repository hosts the family SDK decided by pi-norm-spec
D018 (2026-10-23). The `norm-adapter-engine` crate owns the host-agnostic
engine-side merge/projection extracted from dsh-norm-spec `0.2.1`: the
norm-spec protocol mirrors (`norm_protocol.rs`), single-target prompt context
(`prompt_context.rs`), multi-target prompt context (`multi_prompt_context.rs`,
dsh-norm-spec D016 semantics), and the layout index (`layout_index.rs`,
dsh-norm-spec D014/D015). Every machine API identifier, error-code prefix,
and prompt marker comes from the adapter-supplied `AdapterIdentity`; nothing
else may vary between hosts. Bridge process protocols, sealed-payload and
distribution verification, upstream pins, host injection idioms, and release
cadences stay in each adapter repository. Adapters pin this crate exactly
(`=x.y.z`); this crate carries an independent SemVer line from `0.1.0`.

**Context.** dsh-norm-spec D001 forked pi-norm-spec's engine at its
E2-complete state under a fork-now-converge-later agreement: the shared Host
Adapter SDK waits for the second adapter. Both adapters are now published
(pi-norm-spec `v0.1.0-beta.1`, 2026-09-05; dsh-norm-spec `0.2.1`, 2026-10-01),
and both sides approved convergence during the norm-spec soak window. The
2026-10-23 audit found the shared modules identical modulo identity strings,
with dsh-norm-spec `0.2.1` additionally carrying the operating multi-target
and layout-index work — so it is the extraction source, cross-checked against
pi-norm-spec's copies.

**Rationale.** Convergence removes the defect class where a fix lands in one
fork only. Parameterizing exactly the identity points documented by
dsh-norm-spec D001 keeps each host's wire output byte-stable, so adoption is
verifiable by golden-output gates rather than review trust. A standalone
repository preserves both adapters' release independence; neither adapter
repo hosting the SDK recreates the asymmetry that decision rejected. The
phase plan and gates live in pi-norm-spec
`docs/planning/wsd-sdk-convergence-plan.md`; this repository's status doc
tracks its own execution.
