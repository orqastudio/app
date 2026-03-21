//! OrqaStudio LSP server library.
//!
//! Provides real-time diagnostics for `.orqa/` markdown files:
//! - Frontmatter schema validation (required fields, valid types)
//! - Relationship type validation
//! - Relationship target existence checking
//! - Bidirectional relationship enforcement
//! - Status validation (12 canonical statuses)
//!
//! # Usage
//!
//! ## Stdio (standard LSP transport)
//!
//! ```no_run
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() {
//!     let project_root = Path::new("/path/to/project");
//!     orqa_lsp_server::run_stdio(project_root).await.expect("LSP server failed");
//! }
//! ```
//!
//! ## TCP (debugging)
//!
//! ```no_run
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() {
//!     let project_root = Path::new("/path/to/project");
//!     orqa_lsp_server::run_tcp(project_root, 9257).await.expect("LSP server failed");
//! }
//! ```

pub mod error;
pub mod graph;
pub mod platform;
pub mod server;
pub mod types;
pub mod validation;

pub use error::LspError;
pub use graph::{
    build_artifact_graph, collect_body_refs, collect_relationship_refs, extract_frontmatter,
};
pub use server::{run_stdio, run_tcp};
pub use types::{ArtifactGraph, ArtifactNode};
pub use validation::{is_hex_artifact_id, is_valid_artifact_id, validate_file, VALID_STATUSES};
