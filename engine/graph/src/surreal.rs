//! SurrealDB connection and schema module for orqa-graph.
//!
//! This module is the canonical home of the SurrealDB connection wrapper and
//! SCHEMAFULL schema definitions for the artifact graph. It is promoted from
//! the engine/graph-db proof-of-concept crate into the production engine crate.
//!
//! Provides:
//! - `GraphDb` — thin newtype wrapper around a SurrealDB connection
//! - `open_embedded` — opens a persistent SurrealKV database on disk
//! - `open_memory` — opens an in-memory database for testing
//! - `initialize_schema` — defines SCHEMAFULL tables and indexes, idempotent

use std::path::Path;

use anyhow::Result;
use surrealdb::engine::local::{Db, Mem, SurrealKv};
use surrealdb::Surreal;

/// Thin newtype wrapper around a SurrealDB connection.
///
/// Exposes the inner connection as a public field so callers can run raw
/// SurrealQL queries when the higher-level API does not cover their use case.
///
/// `Clone` is cheap — `Surreal<Db>` uses an `Arc` internally, so cloning
/// produces a second handle to the same underlying connection.
#[derive(Clone)]
pub struct GraphDb(pub Surreal<Db>);

/// SurrealQL schema for the artifact graph.
///
/// Defines SCHEMAFULL tables so field types are enforced at the DB level.
/// The `frontmatter` field is FLEXIBLE TYPE object to accept arbitrary YAML
/// key-value pairs without pre-declaring every possible key.
/// All DEFINE statements use IF NOT EXISTS so this block is safely idempotent.
const SCHEMA_SQL: &str = "
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
DEFINE FIELD IF NOT EXISTS version ON artifact TYPE int DEFAULT 1;
DEFINE FIELD IF NOT EXISTS updated_at ON artifact TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_artifact_type ON artifact FIELDS artifact_type;
DEFINE INDEX IF NOT EXISTS idx_artifact_status ON artifact FIELDS status;
DEFINE INDEX IF NOT EXISTS idx_artifact_path ON artifact FIELDS path UNIQUE;

-- Typed relationship edge table
DEFINE TABLE IF NOT EXISTS relates_to SCHEMAFULL TYPE RELATION IN artifact OUT artifact;
DEFINE FIELD IF NOT EXISTS relation_type ON relates_to TYPE string;
DEFINE FIELD IF NOT EXISTS field ON relates_to TYPE string DEFAULT 'relationships';

DEFINE INDEX IF NOT EXISTS idx_relates_type ON relates_to FIELDS relation_type;

-- Enforcement rule table — plugin-installed rule files tagged with source_plugin
DEFINE TABLE IF NOT EXISTS enforcement_rule SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS key ON enforcement_rule TYPE string;
DEFINE FIELD IF NOT EXISTS source_plugin ON enforcement_rule TYPE option<string>;
DEFINE FIELD IF NOT EXISTS content_hash ON enforcement_rule TYPE option<string>;
DEFINE FIELD IF NOT EXISTS content ON enforcement_rule TYPE option<string>;
DEFINE FIELD IF NOT EXISTS version ON enforcement_rule TYPE int DEFAULT 1;
DEFINE FIELD IF NOT EXISTS updated_at ON enforcement_rule TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_enforcement_rule_key ON enforcement_rule FIELDS key UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_enforcement_rule_plugin ON enforcement_rule FIELDS source_plugin;
";

/// Opens an embedded SurrealKV database at the given filesystem path.
///
/// Creates or re-opens a persistent on-disk database. After connecting,
/// switches to the `orqa` namespace and `artifacts` database. The caller
/// is responsible for running `initialize_schema` when a fresh schema is
/// needed — `open_embedded` does not run it automatically so that callers
/// opening an already-initialized store can skip the overhead.
pub async fn open_embedded(path: &Path) -> Result<GraphDb> {
    // SurrealKV requires forward slashes on all platforms.
    let path_str = path.to_string_lossy().replace('\\', "/");
    let db = Surreal::new::<SurrealKv>(path_str.as_str()).await?;
    db.use_ns("orqa").use_db("artifacts").await?;
    Ok(GraphDb(db))
}

/// Opens an in-memory SurrealDB instance.
///
/// Data is lost when the `GraphDb` is dropped. Intended for unit tests and
/// short-lived operations that do not require durability.
pub async fn open_memory() -> Result<GraphDb> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("orqa").use_db("artifacts").await?;
    Ok(GraphDb(db))
}

/// Applies the SCHEMAFULL schema to the connected database.
///
/// Uses `DEFINE … IF NOT EXISTS` throughout, making this call idempotent.
/// Safe to call on every startup or after opening an existing embedded store.
pub async fn initialize_schema(db: &GraphDb) -> Result<()> {
    db.0.query(SCHEMA_SQL).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_memory() {
        let db = open_memory().await.unwrap();
        // Should succeed
        let _ = db;
    }

    #[tokio::test]
    async fn test_schema_idempotent() {
        let db = open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        // Second call should not error
        initialize_schema(&db).await.unwrap();
    }

    #[tokio::test]
    async fn test_open_embedded() {
        let dir = tempfile::tempdir().unwrap();
        let db = open_embedded(dir.path()).await.unwrap();
        initialize_schema(&db).await.unwrap();
    }
}
