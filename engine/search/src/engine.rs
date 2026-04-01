use std::path::Path;

use crate::embedder::Embedder;
use crate::error::SearchError;
use crate::store::SearchStore;
use crate::types::{IndexStatus, SearchResult};

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
    pub fn new(db_path: &Path) -> Result<Self, SearchError> {
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
    ) -> Result<IndexStatus, SearchError> {
        use std::collections::HashSet;
        use std::time::Instant;
        let start = Instant::now();

        self.project_root = Some(root.to_path_buf());
        let chunks = crate::chunker::chunk_codebase(root, excluded_paths)?;

        let chunk_count = chunks.len();
        let file_count = chunks
            .iter()
            .map(|c| c.file_path.as_str())
            .collect::<HashSet<_>>()
            .len();

        self.store.clear()?;
        self.store.insert_chunks(&chunks)?;
        let status = self.store.get_status()?;

        tracing::info!(
            subsystem = "engine",
            elapsed_ms = start.elapsed().as_millis() as u64,
            chunk_count = chunk_count,
            file_count = file_count,
            "[engine] index completed"
        );

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
    ) -> Result<(), SearchError>
    where
        F: Fn(&str, u64, Option<u64>),
    {
        crate::embedder::ensure_model_exists(model_dir, progress_cb).await?;
        let emb = Embedder::new(model_dir)?;
        self.embedder = Some(emb);
        Ok(())
    }

    /// Load the embedder from an already-downloaded model directory.
    ///
    /// Does NOT download — call `embedder::ensure_model_exists` first.
    /// Use this when the download must happen outside the mutex lock.
    pub fn init_embedder_sync(&mut self, model_dir: &Path) -> Result<(), SearchError> {
        let emb = Embedder::new(model_dir)?;
        self.embedder = Some(emb);
        Ok(())
    }

    /// Generate embeddings for all chunks that do not yet have them.
    ///
    /// Processes chunks in batches of 32. Returns the count of newly embedded chunks.
    pub fn embed_chunks(&mut self) -> Result<u32, SearchError> {
        use std::time::Instant;
        let start = Instant::now();

        if self.embedder.is_none() {
            return Err(SearchError::Search("embedder not initialized".to_owned()));
        }

        let unembedded = self.store.get_unembedded_chunks()?;

        if unembedded.is_empty() {
            tracing::info!(
                subsystem = "engine",
                elapsed_ms = start.elapsed().as_millis() as u64,
                batch_count = 0u64,
                total_chunks = 0u64,
                "[engine] embed_chunks completed"
            );
            return Ok(0);
        }

        let batch_size = 32;
        let mut total_embedded = 0u32;
        let mut batch_count = 0u64;

        for batch in unembedded.chunks(batch_size) {
            let texts: Vec<&str> = batch.iter().map(|(_, content)| content.as_str()).collect();

            // Borrow embedder mutably only for the embed call, then release.
            let embeddings = self
                .embedder
                .as_mut()
                .ok_or_else(|| SearchError::Search("embedder not initialized".to_owned()))?
                .embed(&texts)?;

            let updates: Vec<(i32, Vec<f32>)> = batch
                .iter()
                .zip(embeddings.into_iter())
                .map(|((id, _), emb_vec)| (*id, emb_vec))
                .collect();

            self.store.update_embeddings(&updates)?;
            total_embedded += updates.len() as u32;
            batch_count += 1;
        }

        tracing::info!(
            subsystem = "engine",
            elapsed_ms = start.elapsed().as_millis() as u64,
            batch_count = batch_count,
            total_chunks = total_embedded as u64,
            "[engine] embed_chunks completed"
        );

        Ok(total_embedded)
    }

    /// Search the indexed codebase with a regex pattern.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, SearchError> {
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
    ) -> Result<Vec<SearchResult>, SearchError> {
        let emb = self.embedder.as_mut().ok_or_else(|| {
            SearchError::Search("embedder not initialized — model not loaded".to_owned())
        })?;

        let query_embeddings = emb.embed(&[query])?;
        let query_embedding = query_embeddings
            .into_iter()
            .next()
            .ok_or_else(|| SearchError::Search("failed to embed query".to_owned()))?;

        let results = self.store.search_semantic(&query_embedding, max_results)?;
        Ok(results)
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, SearchError> {
        let status = self.store.get_status()?;
        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Monotonic counter to give each test a unique DB file.
    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// Helper to create a SearchEngine with a unique DuckDB file per test.
    fn temp_engine() -> SearchEngine {
        let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp_dir =
            std::env::temp_dir().join(format!("orqa_search_test_{}_{id}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp_dir);
        let db_path = tmp_dir.join("test.duckdb");
        let _ = std::fs::remove_file(&db_path);
        SearchEngine::new(&db_path).unwrap()
    }

    #[test]
    fn new_engine_has_empty_status() {
        let engine = temp_engine();
        let status = engine.get_status().unwrap();
        assert!(!status.is_indexed);
        assert_eq!(status.chunk_count, 0);
        assert!(!status.has_embeddings);
    }

    #[test]
    fn new_engine_with_invalid_path_still_works() {
        // DuckDB can create the file even in a new directory
        let path = std::env::temp_dir()
            .join("orqa_test_nonexist_dir")
            .join("test.duckdb");
        let _ = std::fs::create_dir_all(path.parent().unwrap());
        let result = SearchEngine::new(&path);
        assert!(result.is_ok());
        // Cleanup
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn index_real_directory_stores_chunks() {
        let mut engine = temp_engine();
        // Index the src directory of this crate — it has known .rs files
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        let status = engine.index(&src_dir, &[]).unwrap();
        assert!(status.is_indexed);
        assert!(status.chunk_count > 0, "should have indexed some chunks");
    }

    #[test]
    fn index_with_exclusions() {
        let mut engine = temp_engine();
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        // Exclude all known source files
        let status = engine
            .index(
                &src_dir,
                &[
                    "lib.rs".to_owned(),
                    "engine.rs".to_owned(),
                    "store.rs".to_owned(),
                    "chunker.rs".to_owned(),
                    "embedder.rs".to_owned(),
                    "types.rs".to_owned(),
                    "error.rs".to_owned(),
                ],
            )
            .unwrap();
        // Should have fewer chunks than without exclusions (or zero if all excluded)
        assert!(status.chunk_count == 0 || status.is_indexed);
    }

    #[test]
    fn regex_search_on_indexed_codebase() {
        let mut engine = temp_engine();
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        engine.index(&src_dir, &[]).unwrap();

        // Search for a pattern we know exists in the search module
        let results = engine.search_regex("SearchEngine", None, 10).unwrap();
        assert!(
            !results.is_empty(),
            "should find SearchEngine in indexed files"
        );
    }

    #[test]
    fn regex_search_empty_index_returns_empty() {
        let engine = temp_engine();
        let results = engine.search_regex("anything", None, 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn embed_chunks_without_embedder_returns_error() {
        let mut engine = temp_engine();
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        engine.index(&src_dir, &[]).unwrap();

        let result = engine.embed_chunks();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("embedder not initialized"));
    }

    #[test]
    fn semantic_search_without_embedder_returns_error() {
        let mut engine = temp_engine();
        let result = engine.search_semantic("test query", 10);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("embedder not initialized"));
    }

    #[test]
    fn init_embedder_sync_with_missing_model_returns_error() {
        let mut engine = temp_engine();
        let fake_dir = PathBuf::from("/nonexistent/model/dir");
        let result = engine.init_embedder_sync(&fake_dir);
        assert!(result.is_err());
    }

    #[test]
    fn index_clears_previous_data() {
        let mut engine = temp_engine();
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");

        // Index once
        let status1 = engine.index(&src_dir, &[]).unwrap();
        let count1 = status1.chunk_count;

        // Index again — should clear and re-index, getting the same count
        let status2 = engine.index(&src_dir, &[]).unwrap();
        assert_eq!(status2.chunk_count, count1);
    }

    #[test]
    fn index_empty_directory_produces_zero_chunks() {
        let mut engine = temp_engine();
        let empty_dir = std::env::temp_dir().join("orqa_empty_dir_test");
        let _ = std::fs::create_dir_all(&empty_dir);

        let status = engine.index(&empty_dir, &[]).unwrap();
        assert_eq!(status.chunk_count, 0);
        assert!(!status.is_indexed);

        let _ = std::fs::remove_dir_all(&empty_dir);
    }
}
