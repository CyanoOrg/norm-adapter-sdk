//! Deterministic multi-scope prompt context over normalized norm-spec output.

use std::fmt;

use serde::Serialize;

use crate::{AdapterIdentity, NormCollectResponse};

/// Maximum UTF-8 size of one rendered multi-target prompt.
pub const MAX_MULTI_PROMPT_BYTES: usize = 256 * 1024;

/// Stable failure while constructing the merged prompt context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiPromptContextError {
    code: String,
    message: String,
}

impl MultiPromptContextError {
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

impl fmt::Display for MultiPromptContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for MultiPromptContextError {}

/// One convention inside a scope: full content on first appearance, a
/// shared marker in every later scope that also collects it (dsh-norm-spec
/// D016 semantics).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiScopeConvention {
    /// Root-relative portable path.
    pub path: String,
    /// Parsed YAML frontmatter as JSON data, absent for shared markers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<serde_json::Value>,
    /// Trimmed Markdown body, absent for shared markers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Set when the content was already rendered in an earlier scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

/// One requested scope: the target and its convention list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiScopeContext {
    /// Root-relative target echoed from the collection.
    pub target: String,
    /// Convention paths most-specific-first (including shared markers).
    pub convention_paths: Vec<String>,
    /// Conventions with full content once per path across the request.
    pub conventions: Vec<MultiScopeConvention>,
}

/// Versioned merged prompt context across a bounded set of targets.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPromptContext {
    /// Multi prompt-context protocol identifier.
    #[serde(rename = "apiVersion")]
    pub api_version: &'static str,
    /// Root marker used by upstream portable paths.
    pub root: String,
    /// One scope per requested collection, in request order.
    pub scopes: Vec<MultiScopeContext>,
    /// Complete deterministic prompt, or `None` when every scope is empty.
    pub prompt: Option<String>,
}

impl MultiPromptContext {
    /// Merge the collections of a bounded target set, in request order.
    ///
    /// # Errors
    ///
    /// Returns a stable error if deterministic JSON serialization fails or
    /// the complete prompt would exceed [`MAX_MULTI_PROMPT_BYTES`]. Content
    /// is never truncated.
    pub fn from_collections(
        identity: &AdapterIdentity,
        root: impl Into<String>,
        collections: &[NormCollectResponse],
    ) -> Result<Self, MultiPromptContextError> {
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let mut scopes = Vec::with_capacity(collections.len());
        for collection in collections {
            let mut conventions = Vec::with_capacity(collection.norms.len());
            let mut convention_paths = Vec::with_capacity(collection.norms.len());
            for convention in &collection.norms {
                convention_paths.push(convention.path.clone());
                if !seen.insert(convention.path.as_str()) {
                    conventions.push(MultiScopeConvention {
                        path: convention.path.clone(),
                        frontmatter: None,
                        body: None,
                        shared: Some(true),
                    });
                    continue;
                }
                conventions.push(MultiScopeConvention {
                    path: convention.path.clone(),
                    frontmatter: Some(convention.frontmatter.clone()),
                    body: Some(convention.body.clone()),
                    shared: None,
                });
            }
            scopes.push(MultiScopeContext {
                target: collection.target.clone(),
                convention_paths,
                conventions,
            });
        }

        if scopes.iter().all(|scope| scope.conventions.is_empty()) {
            return Ok(Self {
                api_version: identity.prompt_context_multi_api,
                root: root.into(),
                scopes,
                prompt: None,
            });
        }

        let mut sections = Vec::with_capacity(scopes.len());
        for (index, scope) in scopes.iter().enumerate() {
            if scope.conventions.is_empty() {
                continue;
            }
            let projection = serde_json::to_string(&ScopeProjectionView {
                target: &scope.target,
                conventions: &scope.conventions,
            })
            .map_err(|error| MultiPromptContextError {
                code: format!("{}/context/serialization", identity.product),
                message: format!("multi prompt context could not be serialized: {error}"),
            })?;
            sections.push(format!(
                "scope {}/{} target={}:\n{projection}",
                index + 1,
                scopes.len(),
                scope.target
            ));
        }
        let prompt = format!(
            "{}{}\n{}",
            identity.context_multi_header(),
            sections.join("\n"),
            identity.context_multi_footer()
        );
        if prompt.len() > MAX_MULTI_PROMPT_BYTES {
            return Err(MultiPromptContextError {
                code: format!("{}/context/too-large", identity.product),
                message: format!(
                    "multi prompt context is {} bytes; maximum is {MAX_MULTI_PROMPT_BYTES} bytes",
                    prompt.len()
                ),
            });
        }

        Ok(Self {
            api_version: identity.prompt_context_multi_api,
            root: root.into(),
            scopes,
            prompt: Some(prompt),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScopeProjectionView<'a> {
    target: &'a str,
    conventions: &'a [MultiScopeConvention],
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{MAX_MULTI_PROMPT_BYTES, MultiPromptContext};
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

    fn collection(target: &str, norms: &[(&str, &str)]) -> NormCollectResponse {
        NormCollectResponse {
            api_version: "norm-spec/collect/v1".to_owned(),
            root: ".".to_owned(),
            target: target.to_owned(),
            norms: norms
                .iter()
                .map(|(path, body)| NormCollectedConvention {
                    path: (*path).to_owned(),
                    frontmatter: json!({"metadata": {"layer": "demo"}}),
                    body: (*body).to_owned(),
                })
                .collect(),
        }
    }

    #[test]
    fn scopes_stay_in_request_order_and_share_content_once() {
        let collections = vec![
            collection(
                "docs",
                &[("docs/.norm", "docs body"), (".norm", "root body")],
            ),
            collection(
                "crates",
                &[("crates/.norm", "crates body"), (".norm", "root body")],
            ),
            collection(".", &[(".norm", "root body")]),
        ];
        let context = MultiPromptContext::from_collections(&DSH, ".", &collections)
            .unwrap_or_else(|error| panic!("merge: {error}"));
        assert_eq!(context.api_version, "dsh-norm-spec/prompt-context-multi/v1");
        assert_eq!(context.scopes.len(), 3);
        assert_eq!(context.scopes[0].target, "docs");
        assert_eq!(context.scopes[2].conventions.len(), 1);
        assert_eq!(context.scopes[2].conventions[0].shared, Some(true));
        assert!(context.scopes[1].conventions[1].shared.unwrap_or(false));
        let prompt = context
            .prompt
            .as_deref()
            .unwrap_or_else(|| panic!("non-empty merge renders"));
        assert!(prompt.starts_with("DSH_NORM_SPEC_CONTEXT_MULTI_V1\n"));
        assert!(prompt.ends_with("\nEND_DSH_NORM_SPEC_CONTEXT_MULTI_V1"));
        assert!(prompt.contains("scope 1/3 target=docs:"));
        assert!(prompt.contains("docs body"));
        assert!(prompt.contains("\"crates/.norm\""));
        assert_eq!(
            prompt.matches("root body").count(),
            1,
            "root body renders once"
        );
    }

    #[test]
    fn all_empty_scopes_yield_a_typed_null_prompt() {
        let context = MultiPromptContext::from_collections(&DSH, ".", &[collection("docs", &[])])
            .unwrap_or_else(|error| panic!("empty merge: {error}"));
        assert!(context.scopes[0].conventions.is_empty());
        assert!(context.prompt.is_none());
    }

    #[test]
    fn empty_scopes_are_skipped_between_rendered_sections() {
        let collections = vec![
            collection("docs", &[("docs/.norm", "docs body")]),
            collection(".", &[]),
        ];
        let context = MultiPromptContext::from_collections(&DSH, ".", &collections)
            .unwrap_or_else(|error| panic!("one scope renders: {error}"));
        let prompt = context
            .prompt
            .as_deref()
            .unwrap_or_else(|| panic!("one scope renders"));
        assert!(prompt.contains("scope 1/2 target=docs:"));
        assert!(!prompt.contains("target=.:"));
    }

    #[test]
    fn oversized_merge_fails_instead_of_truncating() {
        let huge = "x".repeat(MAX_MULTI_PROMPT_BYTES);
        let collections = vec![collection("docs", &[("docs/.norm", &huge)])];
        let Err(error) = MultiPromptContext::from_collections(&DSH, ".", &collections) else {
            panic!("oversized merge must fail");
        };
        assert_eq!(error.code(), "dsh-norm-spec/context/too-large");
    }

    #[test]
    fn rendering_is_deterministic() {
        let collections = vec![
            collection(
                "docs",
                &[("docs/.norm", "docs body"), (".norm", "root body")],
            ),
            collection(
                "crates",
                &[("crates/.norm", "crates body"), (".norm", "root body")],
            ),
        ];
        let first = MultiPromptContext::from_collections(&DSH, ".", &collections)
            .unwrap_or_else(|error| panic!("first: {error}"));
        let second = MultiPromptContext::from_collections(&DSH, ".", &collections)
            .unwrap_or_else(|error| panic!("second: {error}"));
        assert_eq!(first, second);
    }
}
