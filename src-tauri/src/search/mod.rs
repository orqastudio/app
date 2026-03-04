pub mod chunker;
pub mod embedder;
pub mod store;
pub mod types;

use std::path::Path;

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
    pub fn new(db_path: &Path) -> Result<Self, String> {
        let store = SearchStore::new(db_path).map_err(|e| e.to_string())?;
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
    ) -> Result<IndexStatus, String> {
        self.project_root = Some(root.to_path_buf());
        let chunks =
            chunker::chunk_codebase(root, excluded_paths).map_err(|e| e.to_string())?;
        self.store.clear().map_err(|e| e.to_string())?;
        self.store
            .insert_chunks(&chunks)
            .map_err(|e| e.to_string())?;
        self.store.get_status().map_err(|e| e.to_string())
    }

    /// Initialize the embedder with a model from the given directory.
    ///
    /// Expects `model_dir/model.onnx` and `model_dir/tokenizer.json`.
    /// Once initialized, `embed_chunks` and `search_semantic` become available.
    pub fn init_embedder(&mut self, model_dir: &Path) -> Result<(), String> {
        embedder::ensure_model_exists(model_dir).map_err(|e| e.to_string())?;
        let emb = Embedder::new(model_dir).map_err(|e| e.to_string())?;
        self.embedder = Some(emb);
        Ok(())
    }

    /// Generate embeddings for all chunks that do not yet have them.
    ///
    /// Processes chunks in batches of 32. Returns the count of newly embedded chunks.
    pub fn embed_chunks(&mut self) -> Result<u32, String> {
        if self.embedder.is_none() {
            return Err("embedder not initialized".to_string());
        }

        let unembedded = self
            .store
            .get_unembedded_chunks()
            .map_err(|e| e.to_string())?;

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
                .ok_or_else(|| "embedder not initialized".to_string())?
                .embed(&texts)
                .map_err(|e| e.to_string())?;

            let updates: Vec<(i32, Vec<f32>)> = batch
                .iter()
                .zip(embeddings.into_iter())
                .map(|((id, _), emb_vec)| (*id, emb_vec))
                .collect();

            self.store
                .update_embeddings(&updates)
                .map_err(|e| e.to_string())?;
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
    ) -> Result<Vec<SearchResult>, String> {
        self.store
            .search_regex(pattern, path_filter, max_results)
            .map_err(|e| e.to_string())
    }

    /// Semantic search over embedded chunks using natural language.
    ///
    /// Embeds the query text and finds the most similar chunks by
    /// cosine similarity. Requires the embedder to be initialized.
    pub fn search_semantic(
        &mut self,
        query: &str,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, String> {
        let emb = self
            .embedder
            .as_mut()
            .ok_or_else(|| "embedder not initialized — model not loaded".to_string())?;

        let query_embeddings = emb.embed(&[query]).map_err(|e| e.to_string())?;
        let query_embedding = query_embeddings
            .into_iter()
            .next()
            .ok_or_else(|| "failed to embed query".to_string())?;

        self.store
            .search_semantic(&query_embedding, max_results)
            .map_err(|e| e.to_string())
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, String> {
        self.store.get_status().map_err(|e| e.to_string())
    }
}
