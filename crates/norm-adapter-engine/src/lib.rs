//! Framework-neutral shared engine for norm-spec host adapters.
//!
//! This crate owns the host-agnostic computation over normalized norm-spec
//! machine output: the typed protocol mirrors, the single- and multi-target
//! prompt-context projection, and the layout index. Every machine API
//! identifier, error-code prefix, and prompt marker is supplied by the
//! adapter's [`AdapterIdentity`]; nothing else may vary between hosts.
//!
//! It deliberately contains no YAML parser, filesystem inheritance walker,
//! schema validator, upstream pin, process lifecycle, or hidden network I/O.
//! Bridge protocols, sealed-payload verification, and host injection idioms
//! stay in each adapter repository (pi-norm-spec D018).

#![forbid(unsafe_code)]

mod identity;
mod layout_index;
mod multi_prompt_context;
mod norm_protocol;
mod prompt_context;

pub use identity::AdapterIdentity;
pub use layout_index::{LayoutIndex, LayoutIndexEntry, LayoutIndexError, MAX_LAYOUT_INDEX_BYTES};
pub use multi_prompt_context::{
    MAX_MULTI_PROMPT_BYTES, MultiPromptContext, MultiPromptContextError, MultiScopeContext,
    MultiScopeConvention,
};
pub use norm_protocol::{
    NormCollectResponse, NormCollectedConvention, NormCompatibility, NormConformanceCandidate,
    NormConformanceFailure, NormConformanceIdentity, NormConformanceIssue, NormConformanceReport,
    NormConformanceSuite, NormConformanceSummary, NormDiagnostic, NormErrorDetail,
    NormErrorResponse, NormProductIdentity, NormRustApiIdentity, NormScanCoverage, NormScanNaming,
    NormScanRecurringFilename, NormScanResponse, NormScanSymlink, NormScanSymlinkAction,
    NormScanSymlinkKind, NormScannedDirectory, NormValidateResponse, NormValidationResult,
    NormValidationStatus, NormValidationSummary,
};
pub use prompt_context::{MAX_PROMPT_CONTEXT_BYTES, PromptContext, PromptContextError};
