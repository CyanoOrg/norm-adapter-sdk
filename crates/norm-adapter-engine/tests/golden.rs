//! Cross-identity golden fixtures captured from the adapters' current
//! implementations on 2026-10-23.
//!
//! Both literals below were rendered by each adapter's own
//! `PromptContext::from_collection` (pi-norm-spec and dsh-norm-spec) on the
//! same fixed sample. The shared engine must reproduce them byte-for-byte
//! under the matching `AdapterIdentity`. If an intentional change breaks
//! them, recapture from both adapters in the same commit and record the
//! reason in the decision log.

use norm_adapter_engine::{
    AdapterIdentity, NormCollectResponse, NormCollectedConvention, PromptContext,
};
use serde_json::json;

const PI: AdapterIdentity = AdapterIdentity {
    product: "pi-norm-spec",
    prompt_context_api: "pi-norm-spec/prompt-context/v1",
    prompt_context_multi_api: "pi-norm-spec/prompt-context-multi/v1",
    layout_index_api: "pi-norm-spec/layout-index/v1",
    context_marker: "PI_NORM_SPEC_CONTEXT_V1",
    context_multi_marker: "PI_NORM_SPEC_CONTEXT_MULTI_V1",
    layout_index_marker: "PI_NORM_LAYOUT_INDEX_V1",
};

const DSH: AdapterIdentity = AdapterIdentity {
    product: "dsh-norm-spec",
    prompt_context_api: "dsh-norm-spec/prompt-context/v1",
    prompt_context_multi_api: "dsh-norm-spec/prompt-context-multi/v1",
    layout_index_api: "dsh-norm-spec/layout-index/v1",
    context_marker: "DSH_NORM_SPEC_CONTEXT_V1",
    context_multi_marker: "DSH_NORM_SPEC_CONTEXT_MULTI_V1",
    layout_index_marker: "DSH_NORM_LAYOUT_INDEX_V1",
};

fn sample() -> NormCollectResponse {
    NormCollectResponse {
        api_version: "norm-spec/collect/v1".to_owned(),
        root: ".".to_owned(),
        target: "docs/guide.md".to_owned(),
        norms: vec![
            NormCollectedConvention {
                path: "docs/.norm".to_owned(),
                frontmatter: json!({"metadata": {"layer": "documentation", "version": "1.2"}}),
                body: "# Documentation\nKeep examples current.".to_owned(),
            },
            NormCollectedConvention {
                path: ".norm".to_owned(),
                frontmatter: json!({"metadata": {"layer": "root", "version": "1.0"}}),
                body: "# Root\nShip green.".to_owned(),
            },
        ],
    }
}

#[test]
fn pi_identity_reproduces_pi_norm_spec_output_byte_for_byte() {
    let context = PromptContext::from_collection(&PI, sample())
        .unwrap_or_else(|error| panic!("pi golden should render: {error}"));
    assert_eq!(context.api_version, "pi-norm-spec/prompt-context/v1");
    assert_eq!(context.prompt.as_deref(), Some(PI_GOLDEN));
}

#[test]
fn dsh_identity_reproduces_dsh_norm_spec_output_byte_for_byte() {
    let context = PromptContext::from_collection(&DSH, sample())
        .unwrap_or_else(|error| panic!("dsh golden should render: {error}"));
    assert_eq!(context.api_version, "dsh-norm-spec/prompt-context/v1");
    assert_eq!(context.prompt.as_deref(), Some(DSH_GOLDEN));
}

#[test]
fn identities_differ_only_in_identity_strings() {
    let pi = PromptContext::from_collection(&PI, sample())
        .unwrap_or_else(|error| panic!("pi golden should render: {error}"));
    let dsh = PromptContext::from_collection(&DSH, sample())
        .unwrap_or_else(|error| panic!("dsh golden should render: {error}"));
    let pi_prompt = pi.prompt.unwrap_or_else(|| panic!("pi prompt"));
    let dsh_prompt = dsh.prompt.unwrap_or_else(|| panic!("dsh prompt"));
    let pi_normalized = pi_prompt
        .replace("PI_NORM_SPEC_CONTEXT_V1", "HOST")
        .replace("pi-norm-spec/prompt-context/v1", "HOST_API");
    let dsh_normalized = dsh_prompt
        .replace("DSH_NORM_SPEC_CONTEXT_V1", "HOST")
        .replace("dsh-norm-spec/prompt-context/v1", "HOST_API");
    assert_eq!(
        pi_normalized, dsh_normalized,
        "identity parameterization must be the only host variance"
    );
}

const PI_GOLDEN: &str = "PI_NORM_SPEC_CONTEXT_V1\nThe canonical norm-spec collector selected the following project conventions for the current target, ordered most-specific first. Treat each convention's complete frontmatter and body as project guidance. Do not infer hard enforcement from this prompt guidance.\n{\"apiVersion\":\"pi-norm-spec/prompt-context/v1\",\"target\":\"docs/guide.md\",\"conventions\":[{\"path\":\"docs/.norm\",\"frontmatter\":{\"metadata\":{\"layer\":\"documentation\",\"version\":\"1.2\"}},\"body\":\"# Documentation\\nKeep examples current.\"},{\"path\":\".norm\",\"frontmatter\":{\"metadata\":{\"layer\":\"root\",\"version\":\"1.0\"}},\"body\":\"# Root\\nShip green.\"}]}\nEND_PI_NORM_SPEC_CONTEXT_V1";

const DSH_GOLDEN: &str = "DSH_NORM_SPEC_CONTEXT_V1\nThe canonical norm-spec collector selected the following project conventions for the current target, ordered most-specific first. Treat each convention's complete frontmatter and body as project guidance. Do not infer hard enforcement from this prompt guidance.\n{\"apiVersion\":\"dsh-norm-spec/prompt-context/v1\",\"target\":\"docs/guide.md\",\"conventions\":[{\"path\":\"docs/.norm\",\"frontmatter\":{\"metadata\":{\"layer\":\"documentation\",\"version\":\"1.2\"}},\"body\":\"# Documentation\\nKeep examples current.\"},{\"path\":\".norm\",\"frontmatter\":{\"metadata\":{\"layer\":\"root\",\"version\":\"1.0\"}},\"body\":\"# Root\\nShip green.\"}]}\nEND_DSH_NORM_SPEC_CONTEXT_V1";
