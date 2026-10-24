//! Deterministic prompt context constructed from normalized norm-spec output.

use std::{error::Error, fmt};

use serde::Serialize;

use crate::{AdapterIdentity, NormCollectResponse, NormCollectedConvention};

/// Maximum UTF-8 size of one injected prompt.
pub const MAX_PROMPT_CONTEXT_BYTES: usize = 256 * 1024;

/// Ephemeral context returned to the host adapter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptContext {
    /// Prompt-context protocol identifier.
    #[serde(rename = "apiVersion")]
    pub api_version: &'static str,
    /// Root-relative collection target returned by norm-spec.
    pub target: String,
    /// Convention paths in upstream most-specific-first order.
    pub convention_paths: Vec<String>,
    /// Complete deterministic prompt, or `None` for a valid empty collection.
    pub prompt: Option<String>,
}

/// Stable failure while constructing prompt context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptContextError {
    code: String,
    message: String,
}

impl PromptContextError {
    /// Stable machine-readable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Human-readable diagnostic.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PromptContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for PromptContextError {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PromptProjection<'a> {
    #[serde(rename = "apiVersion")]
    api_version: &'a str,
    target: &'a str,
    conventions: &'a [NormCollectedConvention],
}

impl PromptContext {
    /// Construct a complete prompt projection from one canonical collection.
    ///
    /// # Errors
    ///
    /// Returns a stable error if deterministic JSON serialization fails or the
    /// complete prompt would exceed [`MAX_PROMPT_CONTEXT_BYTES`]. Content is
    /// never truncated.
    pub fn from_collection(
        identity: &AdapterIdentity,
        collection: NormCollectResponse,
    ) -> Result<Self, PromptContextError> {
        let convention_paths = collection
            .norms
            .iter()
            .map(|convention| convention.path.clone())
            .collect();
        if collection.norms.is_empty() {
            return Ok(Self {
                api_version: identity.prompt_context_api,
                target: collection.target,
                convention_paths,
                prompt: None,
            });
        }

        let projection = PromptProjection {
            api_version: identity.prompt_context_api,
            target: &collection.target,
            conventions: &collection.norms,
        };
        let json = serde_json::to_string(&projection).map_err(|error| PromptContextError {
            code: format!("{}/context/serialization", identity.product),
            message: format!("prompt context could not be serialized: {error}"),
        })?;
        let prompt = format!(
            "{}{json}{}",
            identity.context_header(),
            identity.context_footer()
        );
        if prompt.len() > MAX_PROMPT_CONTEXT_BYTES {
            return Err(PromptContextError {
                code: format!("{}/context/too-large", identity.product),
                message: format!(
                    "prompt context is {} bytes; maximum is {MAX_PROMPT_CONTEXT_BYTES} bytes",
                    prompt.len()
                ),
            });
        }

        Ok(Self {
            api_version: identity.prompt_context_api,
            target: collection.target,
            convention_paths,
            prompt: Some(prompt),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{MAX_PROMPT_CONTEXT_BYTES, PromptContext};
    use crate::{AdapterIdentity, NormCollectResponse, NormCollectedConvention};

    /// Golden identity: preserves the exact dsh-norm-spec 0.2.1 output this
    /// module was extracted from.
    const DSH: AdapterIdentity = AdapterIdentity {
        product: "dsh-norm-spec",
        prompt_context_api: "dsh-norm-spec/prompt-context/v1",
        prompt_context_multi_api: "dsh-norm-spec/prompt-context-multi/v1",
        layout_index_api: "dsh-norm-spec/layout-index/v1",
        context_marker: "DSH_NORM_SPEC_CONTEXT_V1",
        context_multi_marker: "DSH_NORM_SPEC_CONTEXT_MULTI_V1",
        layout_index_marker: "DSH_NORM_LAYOUT_INDEX_V1",
    };

    fn collection(norms: Vec<NormCollectedConvention>) -> NormCollectResponse {
        NormCollectResponse {
            api_version: "norm-spec/collect/v1".to_owned(),
            root: ".".to_owned(),
            target: "docs/guide.md".to_owned(),
            norms,
        }
    }

    #[test]
    fn preserves_complete_conventions_and_specificity_order() {
        let context = PromptContext::from_collection(
            &DSH,
            collection(vec![
                NormCollectedConvention {
                    path: "docs/.norm".to_owned(),
                    frontmatter: json!({"agent_rules": {"update_order": ["docs first"]}}),
                    body: "# Documentation\nKeep examples current.".to_owned(),
                },
                NormCollectedConvention {
                    path: ".norm".to_owned(),
                    frontmatter: json!({"metadata": {"layer": "root"}}),
                    body: "# Root".to_owned(),
                },
            ]),
        )
        .unwrap_or_else(|error| panic!("prompt context should render: {error}"));

        assert_eq!(context.api_version, "dsh-norm-spec/prompt-context/v1");
        assert_eq!(context.target, "docs/guide.md");
        assert_eq!(context.convention_paths, ["docs/.norm", ".norm"]);
        let prompt = context
            .prompt
            .unwrap_or_else(|| panic!("non-empty collection should produce a prompt"));
        assert!(prompt.starts_with("DSH_NORM_SPEC_CONTEXT_V1\n"));
        assert!(prompt.ends_with("\nEND_DSH_NORM_SPEC_CONTEXT_V1"));
        let specific = prompt
            .find("docs/.norm")
            .unwrap_or_else(|| panic!("specific convention should be present"));
        let root = prompt
            .find(".norm\",\"frontmatter\":{\"metadata")
            .unwrap_or_else(|| panic!("root convention should be present"));
        assert!(specific < root);
        assert!(prompt.contains("docs first"));
        assert!(prompt.contains("Keep examples current."));
    }

    #[test]
    fn empty_collection_is_typed_without_a_synthetic_prompt() {
        let context = PromptContext::from_collection(&DSH, collection(Vec::new()))
            .unwrap_or_else(|error| panic!("empty collection should be valid: {error}"));

        assert!(context.convention_paths.is_empty());
        assert!(context.prompt.is_none());
    }

    #[test]
    fn rendering_is_deterministic() {
        let response = collection(vec![NormCollectedConvention {
            path: ".norm".to_owned(),
            frontmatter: json!({"metadata": {"version": "1.0", "layer": "root"}}),
            body: "# Root".to_owned(),
        }]);

        let first = PromptContext::from_collection(&DSH, response.clone())
            .unwrap_or_else(|error| panic!("first rendering should pass: {error}"));
        let second = PromptContext::from_collection(&DSH, response)
            .unwrap_or_else(|error| panic!("second rendering should pass: {error}"));
        assert_eq!(first, second);
    }

    #[test]
    fn oversized_context_fails_instead_of_truncating() {
        let result = PromptContext::from_collection(
            &DSH,
            collection(vec![NormCollectedConvention {
                path: ".norm".to_owned(),
                frontmatter: json!({"metadata": {"layer": "root"}}),
                body: "x".repeat(MAX_PROMPT_CONTEXT_BYTES),
            }]),
        );
        let Err(error) = result else {
            panic!("oversized context must fail");
        };

        assert_eq!(error.code(), "dsh-norm-spec/context/too-large");
        assert!(error.message().contains("maximum"));
    }
}
