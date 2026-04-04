use std::path::{Path, PathBuf};

use duckdb::{params, Connection};
use regex::Regex;

use crate::types::{ChunkInfo, IndexStatus, SearchResult};

/// A raw chunk row: (file_path, start_line, end_line, content, language).
type ChunkRow = (String, i32, i32, String, Option<String>);

/// An embedded chunk row: (id, file_path, start_line, end_line, content, language, embedding).
type EmbeddedChunkRow = (i32, String, i32, i32, String, Option<String>, Vec<f32>);

/// Error type for search store operations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// DuckDB database operation failed.
    #[error("DuckDB error: {0}")]
    DuckDb(#[from] duckdb::Error),

    /// The provided regex pattern is invalid.
    #[error("invalid regex pattern: {0}")]
    InvalidRegex(String),

    /// Underlying I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Manages the DuckDB database for storing and querying code chunks.
pub struct SearchStore {
    conn: Connection,
    index_path: PathBuf,
}

impl SearchStore {
    /// Open or create a DuckDB database at the given path and ensure the schema exists.
    pub fn new(path: &Path) -> Result<Self, StoreError> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn,
            index_path: path.to_path_buf(),
        };
        store.ensure_schema()?;
        Ok(store)
    }

    /// Create the chunks table and index if they do not already exist.
    ///
    /// Embeddings are stored as BLOBs (raw f32 bytes) for compatibility with
    /// the DuckDB Rust crate, which does not implement `ToSql`/`FromSql` for
    /// typed arrays. The application handles serialization.
    fn ensure_schema(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE SEQUENCE IF NOT EXISTS seq_chunk_id START 1;
             CREATE TABLE IF NOT EXISTS chunks (
                 id INTEGER DEFAULT nextval('seq_chunk_id') PRIMARY KEY,
                 file_path TEXT NOT NULL,
                 start_line INTEGER NOT NULL,
                 end_line INTEGER NOT NULL,
                 content TEXT NOT NULL,
                 language TEXT,
                 embedding BLOB
             );
             CREATE INDEX IF NOT EXISTS idx_chunks_file ON chunks(file_path);",
        )?;
        Ok(())
    }

    /// Drop and recreate the chunks table for a fresh index.
    pub fn clear(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "DROP TABLE IF EXISTS chunks;
             DROP SEQUENCE IF EXISTS seq_chunk_id;",
        )?;
        self.ensure_schema()?;
        Ok(())
    }

    /// Batch-insert chunks into the database using prepared statements.
    pub fn insert_chunks(&self, chunks: &[ChunkInfo]) -> Result<(), StoreError> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO chunks (file_path, start_line, end_line, content, language) \
             VALUES (?, ?, ?, ?, ?)",
        )?;

        for chunk in chunks {
            stmt.execute(params![
                chunk.file_path.as_str(),
                chunk.start_line as i32,
                chunk.end_line as i32,
                chunk.content.as_str(),
                chunk.language.as_deref(),
            ])?;
        }

        Ok(())
    }

    /// Search chunks using a Rust regex pattern.
    ///
    /// Fetches chunks from DuckDB (optionally filtered by file path prefix),
    /// then applies the regex in Rust for portable, predictable matching.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, StoreError> {
        let re = Regex::new(pattern).map_err(|e| StoreError::InvalidRegex(e.to_string()))?;
        let rows = self.fetch_regex_rows(path_filter)?;
        let results = build_regex_results(rows, &re, max_results);
        Ok(results)
    }

    /// Fetch raw chunk rows for regex search, optionally filtered by path prefix.
    fn fetch_regex_rows(&self, path_filter: Option<&str>) -> Result<Vec<ChunkRow>, StoreError> {
        let sql = if path_filter.is_some() {
            "SELECT file_path, start_line, end_line, content, language \
             FROM chunks WHERE file_path LIKE ? || '%' \
             ORDER BY file_path, start_line"
        } else {
            "SELECT file_path, start_line, end_line, content, language \
             FROM chunks ORDER BY file_path, start_line"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = if path_filter.is_some() {
            stmt.query_map(params![path_filter], map_chunk_row)?
        } else {
            stmt.query_map([], map_chunk_row)?
        };
        rows.collect::<Result<Vec<_>, _>>().map_err(StoreError::from)
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, StoreError> {
        let chunk_count: u32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))
            .map(|c: i64| c as u32)?;

        let embedding_count: u32 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM chunks WHERE embedding IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .map(|c: i64| c as u32)?;

        Ok(IndexStatus {
            is_indexed: chunk_count > 0,
            chunk_count,
            has_embeddings: embedding_count > 0,
            last_indexed: None,
            index_path: self.index_path.to_string_lossy().to_string(),
        })
    }

    /// Update the embedding vector for a specific chunk.
    ///
    /// Serializes the `f32` slice to raw bytes for BLOB storage.
    pub fn update_embedding(&self, chunk_id: i32, embedding: &[f32]) -> Result<(), StoreError> {
        let bytes = floats_to_bytes(embedding);
        self.conn.execute(
            "UPDATE chunks SET embedding = ? WHERE id = ?",
            params![bytes, chunk_id],
        )?;
        Ok(())
    }

    /// Batch update embeddings for multiple chunks.
    pub fn update_embeddings(&self, updates: &[(i32, Vec<f32>)]) -> Result<(), StoreError> {
        let mut stmt = self
            .conn
            .prepare("UPDATE chunks SET embedding = ? WHERE id = ?")?;
        for (chunk_id, embedding) in updates {
            let bytes = floats_to_bytes(embedding);
            stmt.execute(params![bytes, *chunk_id])?;
        }
        Ok(())
    }

    /// Get chunks that do not have embeddings yet, returning their IDs and content.
    pub fn get_unembedded_chunks(&self) -> Result<Vec<(i32, String)>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, content FROM chunks WHERE embedding IS NULL ORDER BY id")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(StoreError::from)
    }

    /// Semantic search by computing cosine similarity against stored embeddings.
    ///
    /// Fetches all embedded chunks, computes cosine similarity in Rust,
    /// and returns the top `max_results` matches sorted by score descending.
    pub fn search_semantic(
        &self,
        query_embedding: &[f32],
        max_results: u32,
    ) -> Result<Vec<SearchResult>, StoreError> {
        let rows = self.fetch_embedded_chunks()?;
        let results = build_semantic_results(rows, query_embedding, max_results);
        Ok(results)
    }

    /// Fetch all embedded chunks from DuckDB for semantic scoring.
    fn fetch_embedded_chunks(&self) -> Result<Vec<EmbeddedChunkRow>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, start_line, end_line, content, language, embedding \
             FROM chunks WHERE embedding IS NOT NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let file_path: String = row.get(1)?;
            let start_line: i32 = row.get(2)?;
            let end_line: i32 = row.get(3)?;
            let content: String = row.get(4)?;
            let language: Option<String> = row.get(5)?;
            let embedding_bytes: Vec<u8> = row.get(6)?;
            let embedding = bytes_to_floats(&embedding_bytes);
            Ok((
                id, file_path, start_line, end_line, content, language, embedding,
            ))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(StoreError::from)
    }
}

/// Build `SearchResult` entries from raw regex-matched chunk rows.
fn build_regex_results(rows: Vec<ChunkRow>, re: &Regex, max_results: u32) -> Vec<SearchResult> {
    let mut results = Vec::new();
    for (file_path, start_line, end_line, content, language) in rows {
        if let Some(mat) = re.find(&content) {
            let match_context = extract_match_context(&content, mat.start(), mat.end());
            let match_count = re.find_iter(&content).count();
            results.push(SearchResult {
                file_path,
                start_line: start_line as u32,
                end_line: end_line as u32,
                content,
                language,
                score: match_count as f64,
                match_context,
            });
            if results.len() >= max_results as usize {
                break;
            }
        }
    }
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

/// Build `SearchResult` entries from embedded chunk rows, sorted by cosine similarity.
fn build_semantic_results(
    rows: Vec<EmbeddedChunkRow>,
    query_embedding: &[f32],
    max_results: u32,
) -> Vec<SearchResult> {
    let mut scored: Vec<(f64, SearchResult)> = rows
        .into_iter()
        .map(
            |(_id, file_path, start_line, end_line, content, language, embedding)| {
                let score = cosine_similarity(query_embedding, &embedding);
                let match_context = content.lines().take(3).collect::<Vec<_>>().join("\n");
                (
                    score,
                    SearchResult {
                        file_path,
                        start_line: start_line as u32,
                        end_line: end_line as u32,
                        content,
                        language,
                        score,
                        match_context,
                    },
                )
            },
        )
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored
        .into_iter()
        .take(max_results as usize)
        .map(|(_, r)| r)
        .collect()
}

/// Serialize a slice of f32 values to raw little-endian bytes.
fn floats_to_bytes(floats: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(floats.len() * 4);
    for f in floats {
        bytes.extend_from_slice(&f.to_le_bytes());
    }
    bytes
}

/// Deserialize raw little-endian bytes back to f32 values.
fn bytes_to_floats(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| {
            let arr: [u8; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];
            f32::from_le_bytes(arr)
        })
        .collect()
}

/// Compute cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0f64;
    let mut norm_a = 0.0f64;
    let mut norm_b = 0.0f64;

    for (x, y) in a.iter().zip(b.iter()) {
        let x = f64::from(*x);
        let y = f64::from(*y);
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom > 0.0 {
        dot / denom
    } else {
        0.0
    }
}

/// Map a DuckDB row to chunk field tuple.
fn map_chunk_row(
    row: &duckdb::Row<'_>,
) -> duckdb::Result<(String, i32, i32, String, Option<String>)> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}

/// Extract a short context string around a regex match within the chunk content.
fn extract_match_context(content: &str, match_start: usize, _match_end: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();

    // Find which line contains the match start
    let mut byte_offset = 0;
    let mut match_line_idx = 0;
    for (i, line) in lines.iter().enumerate() {
        let line_end = byte_offset + line.len() + 1; // +1 for newline
        if match_start < line_end {
            match_line_idx = i;
            break;
        }
        byte_offset = line_end;
    }

    // Return the matching line and one line of context on each side
    let start = if match_line_idx > 0 {
        match_line_idx - 1
    } else {
        0
    };
    let end = (match_line_idx + 2).min(lines.len());

    lines[start..end].join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Helper to create an in-memory SearchStore for testing.
    fn in_memory_store() -> SearchStore {
        let conn = Connection::open_in_memory().unwrap();
        let store = SearchStore {
            conn,
            index_path: PathBuf::from(":memory:"),
        };
        store.ensure_schema().unwrap();
        store
    }

    /// Helper to create a ChunkInfo for testing.
    fn test_chunk(
        path: &str,
        start: u32,
        end: u32,
        content: &str,
        lang: Option<&str>,
    ) -> ChunkInfo {
        ChunkInfo {
            file_path: path.to_string(),
            start_line: start,
            end_line: end,
            content: content.to_string(),
            language: lang.map(String::from),
        }
    }

    // ── Schema and lifecycle tests ──────────────────────────────────────

    #[test]
    fn new_store_has_empty_index() {
        let store = in_memory_store();
        let status = store.get_status().unwrap();
        assert!(!status.is_indexed);
        assert_eq!(status.chunk_count, 0);
        assert!(!status.has_embeddings);
    }

    #[test]
    fn insert_and_count_chunks() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("src/main.rs", 1, 10, "fn main() {}", Some("rust")),
            test_chunk("src/lib.rs", 1, 5, "pub mod foo;", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();

        let status = store.get_status().unwrap();
        assert!(status.is_indexed);
        assert_eq!(status.chunk_count, 2);
        assert!(!status.has_embeddings);
    }

    #[test]
    fn clear_removes_all_chunks() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("a.rs", 1, 5, "content a", Some("rust")),
            test_chunk("b.rs", 1, 5, "content b", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();
        assert_eq!(store.get_status().unwrap().chunk_count, 2);

        store.clear().unwrap();
        assert_eq!(store.get_status().unwrap().chunk_count, 0);
    }

    #[test]
    fn insert_chunk_without_language() {
        let store = in_memory_store();
        let chunks = vec![test_chunk("readme.txt", 1, 3, "hello world", None)];
        store.insert_chunks(&chunks).unwrap();
        assert_eq!(store.get_status().unwrap().chunk_count, 1);
    }

    // ── Regex search tests ──────────────────────────────────────────────

    #[test]
    fn regex_search_finds_matching_chunks() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk(
                "src/main.rs",
                1,
                10,
                "fn main() {\n    println!(\"hello\");\n}",
                Some("rust"),
            ),
            test_chunk("src/lib.rs", 1, 5, "pub mod utils;", Some("rust")),
            test_chunk(
                "src/utils.rs",
                1,
                8,
                "fn helper() {\n    println!(\"world\");\n}",
                Some("rust"),
            ),
        ];
        store.insert_chunks(&chunks).unwrap();

        let results = store.search_regex("println", None, 10).unwrap();
        assert_eq!(results.len(), 2);
        // Both matching chunks should contain "println"
        for r in &results {
            assert!(r.content.contains("println"));
        }
    }

    #[test]
    fn regex_search_with_path_filter() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("src/main.rs", 1, 5, "fn main() {}", Some("rust")),
            test_chunk("tests/test.rs", 1, 5, "fn test_main() {}", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();

        let results = store.search_regex("fn", Some("src/"), 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file_path, "src/main.rs");
    }

    #[test]
    fn regex_search_no_matches_returns_empty() {
        let store = in_memory_store();
        let chunks = vec![test_chunk("a.rs", 1, 5, "hello world", Some("rust"))];
        store.insert_chunks(&chunks).unwrap();

        let results = store.search_regex("nonexistent_pattern", None, 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn regex_search_invalid_pattern_returns_error() {
        let store = in_memory_store();
        let chunks = vec![test_chunk("a.rs", 1, 5, "content", Some("rust"))];
        store.insert_chunks(&chunks).unwrap();

        let result = store.search_regex("[invalid", None, 10);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, StoreError::InvalidRegex(_)));
    }

    #[test]
    fn regex_search_respects_max_results() {
        let store = in_memory_store();
        let chunks: Vec<ChunkInfo> = (0..10)
            .map(|i| test_chunk(&format!("file{i}.rs"), 1, 5, "fn foo() {}", Some("rust")))
            .collect();
        store.insert_chunks(&chunks).unwrap();

        let results = store.search_regex("fn foo", None, 3).unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn regex_search_scores_by_match_count() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("few.rs", 1, 3, "fn a() {}", Some("rust")),
            test_chunk(
                "many.rs",
                1,
                5,
                "fn a() {}\nfn b() {}\nfn c() {}",
                Some("rust"),
            ),
        ];
        store.insert_chunks(&chunks).unwrap();

        let results = store.search_regex("fn", None, 10).unwrap();
        assert_eq!(results.len(), 2);
        // The chunk with more matches should be first (sorted by score desc)
        assert!(results[0].score >= results[1].score);
        assert_eq!(results[0].file_path, "many.rs");
    }

    // ── Embedding tests ──────────────────────────────────────────────

    #[test]
    fn get_unembedded_chunks_returns_all_initially() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("a.rs", 1, 5, "content a", Some("rust")),
            test_chunk("b.rs", 1, 5, "content b", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();

        let unembedded = store.get_unembedded_chunks().unwrap();
        assert_eq!(unembedded.len(), 2);
    }

    #[test]
    fn update_embedding_marks_chunk_as_embedded() {
        let store = in_memory_store();
        let chunks = vec![test_chunk("a.rs", 1, 5, "content", Some("rust"))];
        store.insert_chunks(&chunks).unwrap();

        let unembedded = store.get_unembedded_chunks().unwrap();
        assert_eq!(unembedded.len(), 1);
        let chunk_id = unembedded[0].0;

        let embedding = vec![0.1f32, 0.2, 0.3];
        store.update_embedding(chunk_id, &embedding).unwrap();

        let unembedded_after = store.get_unembedded_chunks().unwrap();
        assert!(unembedded_after.is_empty());

        let status = store.get_status().unwrap();
        assert!(status.has_embeddings);
    }

    #[test]
    fn batch_update_embeddings() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("a.rs", 1, 5, "aaa", Some("rust")),
            test_chunk("b.rs", 1, 5, "bbb", Some("rust")),
            test_chunk("c.rs", 1, 5, "ccc", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();

        let unembedded = store.get_unembedded_chunks().unwrap();
        let updates: Vec<(i32, Vec<f32>)> = unembedded
            .iter()
            .map(|(id, _)| (*id, vec![0.1, 0.2, 0.3]))
            .collect();

        store.update_embeddings(&updates).unwrap();
        assert!(store.get_unembedded_chunks().unwrap().is_empty());
        assert_eq!(store.get_status().unwrap().chunk_count, 3);
        assert!(store.get_status().unwrap().has_embeddings);
    }

    // ── Semantic search tests ───────────────────────────────────────────

    #[test]
    fn semantic_search_returns_results_sorted_by_similarity() {
        let store = in_memory_store();
        let chunks = vec![
            test_chunk("close.rs", 1, 5, "very similar content", Some("rust")),
            test_chunk("far.rs", 1, 5, "completely different", Some("rust")),
        ];
        store.insert_chunks(&chunks).unwrap();

        let unembedded = store.get_unembedded_chunks().unwrap();
        // Give "close.rs" an embedding close to the query, "far.rs" an orthogonal one
        let close_embedding = vec![1.0f32, 0.0, 0.0];
        let far_embedding = vec![0.0f32, 1.0, 0.0];

        store
            .update_embedding(unembedded[0].0, &close_embedding)
            .unwrap();
        store
            .update_embedding(unembedded[1].0, &far_embedding)
            .unwrap();

        let query_embedding = vec![1.0f32, 0.0, 0.0]; // identical to close_embedding
        let results = store.search_semantic(&query_embedding, 10).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].file_path, "close.rs");
        assert!((results[0].score - 1.0).abs() < 0.001); // cosine sim = 1.0
        assert!(results[0].score > results[1].score);
    }

    #[test]
    fn semantic_search_respects_max_results() {
        let store = in_memory_store();
        let chunks: Vec<ChunkInfo> = (0..5)
            .map(|i| {
                test_chunk(
                    &format!("f{i}.rs"),
                    1,
                    5,
                    &format!("content {i}"),
                    Some("rust"),
                )
            })
            .collect();
        store.insert_chunks(&chunks).unwrap();

        let unembedded = store.get_unembedded_chunks().unwrap();
        for (id, _) in &unembedded {
            store.update_embedding(*id, &[0.5, 0.5, 0.5]).unwrap();
        }

        let results = store.search_semantic(&[0.5, 0.5, 0.5], 2).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn semantic_search_empty_index_returns_empty() {
        let store = in_memory_store();
        let results = store.search_semantic(&[1.0, 0.0, 0.0], 10).unwrap();
        assert!(results.is_empty());
    }

    // ── Pure function tests ─────────────────────────────────────────────

    #[test]
    fn floats_to_bytes_roundtrip() {
        let original = vec![1.0f32, -2.5, 3.14159, 0.0, f32::MAX, f32::MIN];
        let bytes = floats_to_bytes(&original);
        let recovered = bytes_to_floats(&bytes);
        assert_eq!(original, recovered);
    }

    #[test]
    fn floats_to_bytes_empty() {
        let bytes = floats_to_bytes(&[]);
        assert!(bytes.is_empty());
        let recovered = bytes_to_floats(&bytes);
        assert!(recovered.is_empty());
    }

    #[test]
    fn cosine_similarity_identical_vectors() {
        let v = vec![1.0f32, 2.0, 3.0];
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors() {
        let a = vec![1.0f32, 0.0, 0.0];
        let b = vec![0.0f32, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_opposite_vectors() {
        let a = vec![1.0f32, 0.0];
        let b = vec![-1.0f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_different_lengths_returns_zero() {
        let a = vec![1.0f32, 2.0];
        let b = vec![1.0f32, 2.0, 3.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn cosine_similarity_empty_returns_zero() {
        let empty: Vec<f32> = vec![];
        assert_eq!(cosine_similarity(&empty, &empty), 0.0);
    }

    #[test]
    fn cosine_similarity_zero_vectors_returns_zero() {
        let z = vec![0.0f32, 0.0, 0.0];
        assert_eq!(cosine_similarity(&z, &z), 0.0);
    }

    #[test]
    fn extract_match_context_first_line() {
        let content = "line one\nline two\nline three\nline four";
        let ctx = extract_match_context(content, 0, 4); // match at start of line 1
                                                        // Should include line 1 and line 2 (no line before line 1)
        assert!(ctx.contains("line one"));
        assert!(ctx.contains("line two"));
    }

    #[test]
    fn extract_match_context_middle_line() {
        let content = "line one\nline two\nline three\nline four";
        // "line two" starts at byte 9
        let ctx = extract_match_context(content, 9, 17);
        // Should include line 1 (before), line 2 (match), and line 3 (after)
        assert!(ctx.contains("line one"));
        assert!(ctx.contains("line two"));
        assert!(ctx.contains("line three"));
    }

    #[test]
    fn extract_match_context_last_line() {
        let content = "line one\nline two\nline three";
        // "line three" starts at byte 18
        let ctx = extract_match_context(content, 18, 28);
        assert!(ctx.contains("line two"));
        assert!(ctx.contains("line three"));
    }

    #[test]
    fn build_regex_results_empty_rows() {
        let re = Regex::new("test").unwrap();
        let results = build_regex_results(vec![], &re, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn build_semantic_results_empty_rows() {
        let results = build_semantic_results(vec![], &[1.0, 0.0, 0.0], 10);
        assert!(results.is_empty());
    }

    #[test]
    fn build_semantic_results_sorts_by_score_descending() {
        let rows: Vec<EmbeddedChunkRow> = vec![
            (
                1,
                "low.rs".into(),
                1,
                5,
                "low score".into(),
                Some("rust".into()),
                vec![0.0, 1.0, 0.0],
            ),
            (
                2,
                "high.rs".into(),
                1,
                5,
                "high score".into(),
                Some("rust".into()),
                vec![1.0, 0.0, 0.0],
            ),
            (
                3,
                "mid.rs".into(),
                1,
                5,
                "mid score".into(),
                Some("rust".into()),
                vec![0.7, 0.7, 0.0],
            ),
        ];
        let query = vec![1.0f32, 0.0, 0.0];
        let results = build_semantic_results(rows, &query, 10);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].file_path, "high.rs");
        // Scores should be descending
        for pair in results.windows(2) {
            assert!(pair[0].score >= pair[1].score);
        }
    }
}
