pub mod chunker;
pub mod embedder;
pub mod store;
pub mod types;

use std::path::Path;

use crate::error::OrqaError;
use embedder::Embedder;
use store::SearchStore;
use types::{IndexStatus, SearchResult};

/// The main search engine that coordinates indexing and searching.
///
/// The embedder is optional — regex search works without it.
/// Call `init_embedder` after construction to enable semantic search.
pub struct SearchEngine {
    store: SearchStore,
    embedder: Option<Embedder>,
    project_root: Option<std::path::PathBuf>,
}

impl SearchEngine {
    /// Create a new search engine backed by a DuckDB database at `db_path`.
    pub fn new(db_path: &Path) -> Result<Self, OrqaError> {
        let store = SearchStore::new(db_path)?;
        Ok(Self {
            store,
            embedder: None,
            project_root: None,
        })
    }

    /// Index a codebase rooted at `root`, storing chunks in DuckDB.
    ///
    /// This clears any existing index before re-indexing.
    pub fn index(
        &mut self,
        root: &Path,
        excluded_paths: &[String],
    ) -> Result<IndexStatus, OrqaError> {
        self.project_root = Some(root.to_path_buf());
        let chunks = chunker::chunk_codebase(root, excluded_paths)?;
        self.store.clear()?;
        self.store.insert_chunks(&chunks)?;
        let status = self.store.get_status()?;
        Ok(status)
    }

    /// Initialize the embedder with a model from the given directory.
    ///
    /// Downloads model files from Hugging Face if they don't exist locally.
    /// Once initialized, `embed_chunks` and `search_semantic` become available.
    pub async fn init_embedder<F>(
        &mut self,
        model_dir: &Path,
        progress_cb: F,
    ) -> Result<(), OrqaError>
    where
        F: Fn(&str, u64, Option<u64>),
    {
        embedder::ensure_model_exists(model_dir, progress_cb).await?;
        let emb = Embedder::new(model_dir)?;
        self.embedder = Some(emb);
        Ok(())
    }

    /// Load the embedder from an already-downloaded model directory.
    ///
    /// Does NOT download — call `embedder::ensure_model_exists` first.
    /// Use this when the download must happen outside the mutex lock.
    pub fn init_embedder_sync(&mut self, model_dir: &Path) -> Result<(), OrqaError> {
        let emb = Embedder::new(model_dir)?;
        self.embedder = Some(emb);
        Ok(())
    }

    /// Generate embeddings for all chunks that do not yet have them.
    ///
    /// Processes chunks in batches of 32. Returns the count of newly embedded chunks.
    pub fn embed_chunks(&mut self) -> Result<u32, OrqaError> {
        if self.embedder.is_none() {
            return Err(OrqaError::Search("embedder not initialized".to_string()));
        }

        let unembedded = self.store.get_unembedded_chunks()?;

        if unembedded.is_empty() {
            return Ok(0);
        }

        let batch_size = 32;
        let mut total_embedded = 0u32;

        for batch in unembedded.chunks(batch_size) {
            let texts: Vec<&str> = batch.iter().map(|(_, content)| content.as_str()).collect();

            // Borrow embedder mutably only for the embed call, then release.
            let embeddings = self
                .embedder
                .as_mut()
                .ok_or_else(|| OrqaError::Search("embedder not initialized".to_string()))?
                .embed(&texts)?;

            let updates: Vec<(i32, Vec<f32>)> = batch
                .iter()
                .zip(embeddings.into_iter())
                .map(|((id, _), emb_vec)| (*id, emb_vec))
                .collect();

            self.store.update_embeddings(&updates)?;
            total_embedded += updates.len() as u32;
        }

        Ok(total_embedded)
    }

    /// Search the indexed codebase with a regex pattern.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, OrqaError> {
        let results = self.store.search_regex(pattern, path_filter, max_results)?;
        Ok(results)
    }

    /// Semantic search over embedded chunks using natural language.
    ///
    /// Embeds the query text and finds the most similar chunks by
    /// cosine similarity. Requires the embedder to be initialized.
    pub fn search_semantic(
        &mut self,
        query: &str,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, OrqaError> {
        let emb = self.embedder.as_mut().ok_or_else(|| {
            OrqaError::Search("embedder not initialized — model not loaded".to_string())
        })?;

        let query_embeddings = emb.embed(&[query])?;
        let query_embedding = query_embeddings
            .into_iter()
            .next()
            .ok_or_else(|| OrqaError::Search("failed to embed query".to_string()))?;

        let results = self.store.search_semantic(&query_embedding, max_results)?;
        Ok(results)
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, OrqaError> {
        let status = self.store.get_status()?;
        Ok(status)
    }
}
