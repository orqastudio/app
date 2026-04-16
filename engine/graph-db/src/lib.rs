//! SurrealDB proof-of-concept for the OrqaStudio artifact graph.
//!
//! This is a standalone benchmark crate that validates SurrealDB 3.x as the
//! artifact graph storage engine. It does NOT integrate with the daemon or
//! any existing engine crates — all types and parsing logic are self-contained.
//!
//! # Architecture
//!
//! - `GraphDb` — thin wrapper around a SurrealDB connection
//! - `schema` — SCHEMAFULL table/field/index definitions
//! - `ingest` — walks `.orqa/` directories, parses YAML frontmatter, inserts data
//! - `queries` — graph queries expressed in SurrealQL

pub mod ingest;
pub mod queries;
pub mod schema;
pub mod writers;

#[cfg(test)]
mod tests;

use anyhow::Result;
use surrealdb::engine::local::{Mem, SurrealKv};
use surrealdb::Surreal;

/// Wrapper around a SurrealDB connection for the artifact graph.
///
/// Provides factory methods for embedded (SurrealKV) and in-memory storage.
/// All graph operations are expressed as SurrealQL queries via the inner `db` handle.
pub struct GraphDb {
    /// The underlying SurrealDB connection. Public for direct query access in
    /// benchmarks and advanced use cases.
    pub db: Surreal<surrealdb::engine::local::Db>,
}

impl GraphDb {
    /// Open an embedded SurrealKV database at the given path.
    ///
    /// Creates the directory if it does not exist. Enables fsync for durability
    /// via SurrealKV's default configuration. Initializes the namespace/database
    /// and runs the schema.
    pub async fn open_embedded(path: &std::path::Path) -> Result<Self> {
        let db = Surreal::new::<SurrealKv>(path.to_string_lossy().as_ref()).await?;
        db.use_ns("orqa").use_db("graph").await?;
        let graph_db = Self { db };
        schema::initialize_schema(&graph_db).await?;
        Ok(graph_db)
    }

    /// Open an in-memory SurrealDB instance for testing.
    ///
    /// Data is lost when the `GraphDb` is dropped. Initializes the namespace/database
    /// and runs the schema.
    pub async fn open_memory() -> Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("orqa").use_db("graph").await?;
        let graph_db = Self { db };
        schema::initialize_schema(&graph_db).await?;
        Ok(graph_db)
    }

    /// Re-initialize the schema (idempotent).
    ///
    /// Useful after opening a pre-existing embedded database to ensure the
    /// schema is up to date with the latest definitions.
    pub async fn initialize_schema(&self) -> Result<()> {
        schema::initialize_schema(self).await
    }
}
