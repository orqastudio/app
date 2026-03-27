// Knowledge domain types for the OrqaStudio engine.
//
// Defines the KnowledgeMatch struct returned by semantic knowledge injection.
// The KnowledgeInjector service itself remains in the app until the search/prompt
// crates are extracted in later migration phases.

/// A matched knowledge artifact with its similarity score.
///
/// Returned by the knowledge injector when matching a prompt against the
/// embedded descriptions of all loaded knowledge artifacts.
#[derive(Debug, Clone)]
pub struct KnowledgeMatch {
    /// Artifact name (filename stem, e.g. "rust-async-patterns").
    pub name: String,
    /// Short description extracted from the artifact's YAML frontmatter.
    pub description: String,
    /// Cosine similarity score between the prompt and this artifact's embedding.
    pub score: f32,
}
