//! OrqaStudio LSP server library.
//!
//! Provides real-time diagnostics for `.orqa/` markdown files. Text-level
//! checks (frontmatter structure, duplicate keys, ID format) run locally on
//! the editor buffer. Graph-level checks (broken refs, missing inverses, type
//! constraints, cardinality, cycles) are delegated to the validation daemon
//! via its HTTP API.
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
//!     orqa_lsp_server::run_stdio(project_root, 9258).await.expect("LSP server failed");
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
//!     orqa_lsp_server::run_tcp(project_root, 9257, 9258).await.expect("LSP server failed");
//! }
//! ```

pub mod error;
pub mod graph;
pub mod server;
pub mod types;
pub mod validation;

pub use error::LspError;
pub use graph::{build_artifact_graph, ArtifactGraph, ArtifactNode};
pub use server::{run_stdio, run_tcp};
pub use validation::{is_hex_artifact_id, is_valid_artifact_id, validate_file};
