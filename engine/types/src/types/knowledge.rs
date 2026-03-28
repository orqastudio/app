// Knowledge domain types for the OrqaStudio engine.
//
// Defines the KnowledgeMatch struct returned by the KnowledgeInjector in
// `orqa_engine::prompt::knowledge`. The injector embeds knowledge artifact
// descriptions and matches them against prompt embeddings via cosine similarity.

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
