//! Host-adapter identity parameterization.
//!
//! The engine is host-agnostic: every machine API identifier, error-code
//! prefix, and prompt marker comes from the adapter's own compile-time
//! identity. This is exactly the extraction boundary documented by
//! dsh-norm-spec D001 — nothing else may vary between hosts.

/// Compile-time identity strings one host adapter supplies to the shared
/// engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterIdentity {
    /// Product namespace used as the error-code prefix, e.g. `pi-norm-spec`.
    pub product: &'static str,
    /// Single-target prompt-context machine API id, e.g.
    /// `pi-norm-spec/prompt-context/v1`.
    pub prompt_context_api: &'static str,
    /// Multi-target prompt-context machine API id, e.g.
    /// `pi-norm-spec/prompt-context-multi/v1`.
    pub prompt_context_multi_api: &'static str,
    /// Layout-index machine API id, e.g. `pi-norm-spec/layout-index/v1`.
    pub layout_index_api: &'static str,
    /// Uppercase single-target prompt marker, e.g. `PI_NORM_SPEC_CONTEXT_V1`.
    pub context_marker: &'static str,
    /// Uppercase multi-target prompt marker, e.g.
    /// `PI_NORM_SPEC_CONTEXT_MULTI_V1`.
    pub context_multi_marker: &'static str,
    /// Uppercase layout-index prompt marker, e.g. `PI_NORM_LAYOUT_INDEX_V1`.
    pub layout_index_marker: &'static str,
}

impl AdapterIdentity {
    /// Single-target prompt header: marker line plus the fixed guidance
    /// prose shared by every host adapter.
    #[must_use]
    pub fn context_header(&self) -> String {
        format!(
            "{}\n{}",
            self.context_marker,
            "The canonical norm-spec collector selected the following project conventions for the current target, ordered most-specific first. Treat each convention's complete frontmatter and body as project guidance. Do not infer hard enforcement from this prompt guidance.\n"
        )
    }

    /// Single-target prompt footer closing the marker block.
    #[must_use]
    pub fn context_footer(&self) -> String {
        format!("\nEND_{}", self.context_marker)
    }

    /// Multi-target prompt header: marker line plus the fixed guidance
    /// prose shared by every host adapter.
    #[must_use]
    pub fn context_multi_header(&self) -> String {
        format!(
            "{}\n{}",
            self.context_multi_marker,
            "The canonical norm-spec collector selected the project conventions for each scoped directory, ordered most recent first, most-specific first within a scope. Treat each convention's complete frontmatter and body as project guidance. Do not infer hard enforcement from this prompt guidance.\n"
        )
    }

    /// Multi-target prompt footer closing the marker block.
    #[must_use]
    pub fn context_multi_footer(&self) -> String {
        format!("\nEND_{}", self.context_multi_marker)
    }

    /// Layout-index prompt header: marker line plus the fixed guidance
    /// prose shared by every host adapter.
    #[must_use]
    pub fn layout_index_header(&self) -> String {
        format!(
            "{}\n{}",
            self.layout_index_marker,
            "The project directories below declare .norm conventions. Each directory's conventions apply to that directory and are inherited from its parents; consult them before working there.\n"
        )
    }

    /// Layout-index prompt footer closing the marker block.
    #[must_use]
    pub fn layout_index_footer(&self) -> String {
        format!("\nEND_{}", self.layout_index_marker)
    }
}

#[cfg(test)]
mod tests {
    use super::AdapterIdentity;

    const PI: AdapterIdentity = AdapterIdentity {
        product: "pi-norm-spec",
        prompt_context_api: "pi-norm-spec/prompt-context/v1",
        prompt_context_multi_api: "pi-norm-spec/prompt-context-multi/v1",
        layout_index_api: "pi-norm-spec/layout-index/v1",
        context_marker: "PI_NORM_SPEC_CONTEXT_V1",
        context_multi_marker: "PI_NORM_SPEC_CONTEXT_MULTI_V1",
        layout_index_marker: "PI_NORM_LAYOUT_INDEX_V1",
    };

    #[test]
    fn header_and_footer_wrap_the_marker() {
        assert!(PI.context_header().starts_with("PI_NORM_SPEC_CONTEXT_V1\n"));
        assert_eq!(PI.context_footer(), "\nEND_PI_NORM_SPEC_CONTEXT_V1");
        assert!(
            PI.context_multi_header()
                .starts_with("PI_NORM_SPEC_CONTEXT_MULTI_V1\n")
        );
        assert_eq!(
            PI.context_multi_footer(),
            "\nEND_PI_NORM_SPEC_CONTEXT_MULTI_V1"
        );
        assert!(
            PI.layout_index_header()
                .starts_with("PI_NORM_LAYOUT_INDEX_V1\n")
        );
        assert_eq!(PI.layout_index_footer(), "\nEND_PI_NORM_LAYOUT_INDEX_V1");
    }
}
