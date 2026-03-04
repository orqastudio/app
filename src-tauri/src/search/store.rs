use std::path::{Path, PathBuf};

use duckdb::{params, Connection};
use regex::Regex;

use super::types::{ChunkInfo, IndexStatus, SearchResult};

/// Error type for search store operations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("DuckDB error: {0}")]
    DuckDb(#[from] duckdb::Error),

    #[error("invalid regex pattern: {0}")]
    InvalidRegex(String),

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

        let (sql, has_path_filter) = if path_filter.is_some() {
            (
                "SELECT file_path, start_line, end_line, content, language \
                 FROM chunks WHERE file_path LIKE ? || '%' \
                 ORDER BY file_path, start_line",
                true,
            )
        } else {
            (
                "SELECT file_path, start_line, end_line, content, language \
                 FROM chunks ORDER BY file_path, start_line",
                false,
            )
        };

        let mut stmt = self.conn.prepare(sql)?;

        let rows = if has_path_filter {
            stmt.query_map(params![path_filter], map_chunk_row)?
        } else {
            stmt.query_map([], map_chunk_row)?
        };

        let mut results = Vec::new();

        for row_result in rows {
            let (file_path, start_line, end_line, content, language): (
                String,
                i32,
                i32,
                String,
                Option<String>,
            ) = row_result?;

            if let Some(mat) = re.find(&content) {
                // Extract a context line around the first match
                let match_context = extract_match_context(&content, mat.start(), mat.end());

                // Score: count of matches in the chunk
                let match_count = re.find_iter(&content).count();

                results.push(SearchResult {
                    file_path,
                    start_line: start_line as u32,
                    end_line: end_line as u32,
                    content: content.clone(),
                    language,
                    score: match_count as f64,
                    match_context,
                });

                if results.len() >= max_results as usize {
                    break;
                }
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
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
        let mut stmt = self.conn.prepare(
            "SELECT id, content FROM chunks WHERE embedding IS NULL ORDER BY id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
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
            Ok((id, file_path, start_line, end_line, content, language, embedding))
        })?;

        let mut scored: Vec<(f64, SearchResult)> = Vec::new();

        for row_result in rows {
            let (_id, file_path, start_line, end_line, content, language, embedding) =
                row_result?;

            let score = cosine_similarity(query_embedding, &embedding);

            let match_context = content
                .lines()
                .take(3)
                .collect::<Vec<_>>()
                .join("\n");

            scored.push((
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
            ));
        }

        // Sort by score descending
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Take top results
        let results = scored
            .into_iter()
            .take(max_results as usize)
            .map(|(_, result)| result)
            .collect();

        Ok(results)
    }
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
