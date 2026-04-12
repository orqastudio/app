//! SurrealDB schema definitions for the artifact graph.
//!
//! Defines SCHEMAFULL tables for artifacts and typed relationship edges.
//! The schema mirrors the `ArtifactNode` / `ArtifactRef` types from
//! `engine/types` but is self-contained — this POC does not depend on
//! other engine crates.

use anyhow::Result;

use crate::GraphDb;

/// SurrealQL schema definition for the artifact graph.
///
/// Uses SCHEMAFULL tables so that field types are enforced at the DB level.
/// The `frontmatter` field is FLEXIBLE TYPE object to accept arbitrary YAML
/// key-value pairs without pre-declaring every possible field.
const SCHEMA: &str = "
-- Artifact node table
DEFINE TABLE IF NOT EXISTS artifact SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS artifact_type ON artifact TYPE string;
DEFINE FIELD IF NOT EXISTS title ON artifact TYPE string;
DEFINE FIELD IF NOT EXISTS description ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS status ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS priority ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS path ON artifact TYPE string;
DEFINE FIELD IF NOT EXISTS body ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS frontmatter ON artifact TYPE object FLEXIBLE;
DEFINE FIELD IF NOT EXISTS source_plugin ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS content_hash ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS updated ON artifact TYPE option<string>;
DEFINE FIELD IF NOT EXISTS updated_at ON artifact TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_artifact_type ON artifact FIELDS artifact_type;
DEFINE INDEX IF NOT EXISTS idx_artifact_status ON artifact FIELDS status;
DEFINE INDEX IF NOT EXISTS idx_artifact_path ON artifact FIELDS path UNIQUE;

-- Typed relationship edge table
DEFINE TABLE IF NOT EXISTS relates_to SCHEMAFULL TYPE RELATION IN artifact OUT artifact;
DEFINE FIELD IF NOT EXISTS relationship_type ON relates_to TYPE string;
DEFINE FIELD IF NOT EXISTS field ON relates_to TYPE string DEFAULT 'relationships';

DEFINE INDEX IF NOT EXISTS idx_relates_type ON relates_to FIELDS relationship_type;
";

/// Run all DEFINE TABLE / FIELD / INDEX statements against the database.
pub async fn initialize_schema(db: &GraphDb) -> Result<()> {
    db.db.query(SCHEMA).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn schema_initializes_without_error() {
        let db = GraphDb::open_memory().await.expect("open memory DB");
        initialize_schema(&db).await.expect("schema init");
    }

    #[tokio::test]
    async fn schema_is_idempotent() {
        let db = GraphDb::open_memory().await.expect("open memory DB");
        initialize_schema(&db).await.expect("first init");
        initialize_schema(&db).await.expect("second init");
    }
}
