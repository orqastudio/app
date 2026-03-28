use std::path::Path;

use ignore::WalkBuilder;

use crate::types::ChunkInfo;

/// Target number of lines per chunk.
const TARGET_CHUNK_LINES: usize = 50;

/// Tolerance window for finding a blank-line split point near the target.
const SPLIT_TOLERANCE: usize = 10;

/// Maximum file size in bytes (1 MB). Files larger than this are skipped.
const MAX_FILE_SIZE: u64 = 1_048_576;

/// File extensions to skip (binary and non-text files).
const BINARY_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "woff", "woff2", "ttf", "eot", "otf", "mp3",
    "mp4", "avi", "mov", "zip", "tar", "gz", "rar", "7z", "exe", "dll", "so", "dylib", "wasm",
    "pdf", "lock",
];

/// Error type for chunking operations.
#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    /// Underlying I/O error reading a file.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Directory walk failed (e.g. permission denied).
    #[error("walk error: {0}")]
    Walk(String),
}

/// Compute the relative path string for a file entry under `root`.
fn rel_path_str(abs_path: &Path, root: &Path) -> String {
    abs_path
        .strip_prefix(root)
        .unwrap_or(abs_path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Read a single file's content, returning `None` if it should be skipped
/// (non-UTF-8, empty, too large, binary extension, or excluded prefix).
fn read_eligible_file(
    abs_path: &Path,
    rel_path: &str,
    excluded_paths: &[String],
) -> Result<Option<String>, ChunkError> {
    if excluded_paths
        .iter()
        .any(|excluded| rel_path.starts_with(excluded.as_str()))
    {
        return Ok(None);
    }

    if is_binary_extension(abs_path) {
        return Ok(None);
    }

    let metadata = std::fs::metadata(abs_path)?;
    if metadata.len() > MAX_FILE_SIZE {
        return Ok(None);
    }

    match std::fs::read_to_string(abs_path) {
        Ok(c) if c.is_empty() => Ok(None),
        Ok(c) => Ok(Some(c)),
        Err(e) if e.kind() == std::io::ErrorKind::InvalidData => Ok(None),
        Err(e) => Err(ChunkError::Io(e)),
    }
}

/// Walk a codebase rooted at `root`, respecting `.gitignore`, and split
/// every eligible text file into chunks of approximately `TARGET_CHUNK_LINES` lines.
///
/// `excluded_paths` contains path prefixes (relative to root) that should be skipped.
pub fn chunk_codebase(
    root: &Path,
    excluded_paths: &[String],
) -> Result<Vec<ChunkInfo>, ChunkError> {
    let mut chunks = Vec::new();

    let walker = WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry_result in walker {
        let entry = entry_result.map_err(|e| ChunkError::Walk(e.to_string()))?;

        if entry.file_type().is_none_or(|ft| !ft.is_file()) {
            continue;
        }

        let abs_path = entry.path();
        let rel_path = rel_path_str(abs_path, root);

        let Some(content) = read_eligible_file(abs_path, &rel_path, excluded_paths)? else {
            continue;
        };

        let language = detect_language(abs_path);
        let file_chunks = split_into_chunks(&content, &rel_path, language.as_deref());
        chunks.extend(file_chunks);
    }

    Ok(chunks)
}

/// Check whether a file's extension is in the binary skip list.
fn is_binary_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| BINARY_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
}

/// Detect the programming language from a file's extension.
fn detect_language(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" => Some("rust".to_owned()),
        "ts" | "tsx" => Some("typescript".to_owned()),
        "js" | "jsx" => Some("javascript".to_owned()),
        "svelte" => Some("svelte".to_owned()),
        "py" => Some("python".to_owned()),
        "go" => Some("go".to_owned()),
        "toml" => Some("toml".to_owned()),
        "json" => Some("json".to_owned()),
        "yaml" | "yml" => Some("yaml".to_owned()),
        "md" => Some("markdown".to_owned()),
        "html" => Some("html".to_owned()),
        "css" => Some("css".to_owned()),
        "sql" => Some("sql".to_owned()),
        "sh" | "bash" => Some("shell".to_owned()),
        _ => None,
    }
}

/// Split file content into chunks of approximately `TARGET_CHUNK_LINES` lines,
/// preferring to split at blank-line boundaries.
fn split_into_chunks(content: &str, file_path: &str, language: Option<&str>) -> Vec<ChunkInfo> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < lines.len() {
        let ideal_end = (start + TARGET_CHUNK_LINES).min(lines.len());

        let end = if ideal_end >= lines.len() {
            // Last chunk — take everything remaining
            lines.len()
        } else {
            // Try to find a blank line within the tolerance window
            find_blank_line_boundary(&lines, ideal_end)
        };

        let chunk_content = lines[start..end].join("\n");

        chunks.push(ChunkInfo {
            file_path: file_path.to_owned(),
            start_line: (start + 1) as u32, // 1-indexed
            end_line: end as u32,           // inclusive of last line
            content: chunk_content,
            language: language.map(String::from),
        });

        start = end;
    }

    chunks
}

/// Search for a blank-line split point near `target_line`.
///
/// Scans within `+/- SPLIT_TOLERANCE` lines of the target and returns the
/// position of the first blank line found (searching outward from the target).
/// If no blank line is found, returns the target.
fn find_blank_line_boundary(lines: &[&str], target: usize) -> usize {
    let lower = target.saturating_sub(SPLIT_TOLERANCE);
    let upper = (target + SPLIT_TOLERANCE).min(lines.len());

    // Search outward from the target for a blank line
    for offset in 0..=SPLIT_TOLERANCE {
        // Check after target first
        let after = target + offset;
        if after < upper && lines[after].trim().is_empty() {
            return after + 1; // split after the blank line
        }

        // Then check before target
        if offset <= target {
            let before = target - offset;
            if before >= lower && lines[before].trim().is_empty() {
                return before + 1; // split after the blank line
            }
        }
    }

    // No blank line found — split at the target
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_language_known_extensions() {
        let cases = vec![
            ("foo.rs", Some("rust")),
            ("bar.ts", Some("typescript")),
            ("baz.py", Some("python")),
            ("qux.go", Some("go")),
            ("file.svelte", Some("svelte")),
            ("unknown.xyz", None),
        ];

        for (filename, expected) in cases {
            let path = Path::new(filename);
            let result = detect_language(path);
            assert_eq!(result.as_deref(), expected, "detect_language({filename})");
        }
    }

    #[test]
    fn is_binary_extension_detects_binaries() {
        assert!(is_binary_extension(Path::new("image.png")));
        assert!(is_binary_extension(Path::new("font.woff2")));
        assert!(is_binary_extension(Path::new("archive.zip")));
        assert!(!is_binary_extension(Path::new("code.rs")));
        assert!(!is_binary_extension(Path::new("readme.md")));
    }

    #[test]
    fn split_short_file_produces_single_chunk() {
        let content = "line 1\nline 2\nline 3";
        let chunks = split_into_chunks(content, "test.rs", Some("rust"));
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start_line, 1);
        assert_eq!(chunks[0].end_line, 3);
        assert_eq!(chunks[0].language.as_deref(), Some("rust"));
    }

    #[test]
    fn split_long_file_produces_multiple_chunks() {
        let lines: Vec<String> = (1..=120).map(|i| format!("line {i}")).collect();
        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "big.rs", Some("rust"));
        assert!(
            chunks.len() >= 2,
            "expected at least 2 chunks, got {}",
            chunks.len()
        );
        // Verify no gaps — each chunk starts where the previous ended
        for pair in chunks.windows(2) {
            assert_eq!(pair[0].end_line + 1, pair[1].start_line);
        }
    }

    #[test]
    fn split_prefers_blank_line_boundary() {
        // Create content with a blank line at line 48 (0-indexed: index 47)
        let mut lines: Vec<String> = (1..=100).map(|i| format!("line {i}")).collect();
        lines[47] = String::new(); // blank line at position 48

        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "test.rs", None);

        // The first chunk should split at or near the blank line
        assert!(
            chunks[0].end_line >= 45 && chunks[0].end_line <= 55,
            "expected first chunk to end near line 48, got {}",
            chunks[0].end_line
        );
    }

    #[test]
    fn empty_content_produces_no_chunks() {
        let chunks = split_into_chunks("", "empty.rs", None);
        assert!(chunks.is_empty());
    }

    // ── rel_path_str tests ──────────────────────────────────────────────

    #[test]
    fn rel_path_str_strips_prefix() {
        let abs = Path::new("/home/user/project/src/main.rs");
        let root = Path::new("/home/user/project");
        assert_eq!(rel_path_str(abs, root), "src/main.rs");
    }

    #[test]
    fn rel_path_str_handles_windows_separators() {
        // Even if the path has backslashes, the output uses forward slashes
        let abs = Path::new("C:\\Users\\test\\project\\src\\lib.rs");
        let root = Path::new("C:\\Users\\test\\project");
        let result = rel_path_str(abs, root);
        assert!(
            !result.contains('\\'),
            "should not contain backslashes: {result}"
        );
        assert!(result.contains("src"));
    }

    #[test]
    fn rel_path_str_unrelated_root_returns_full_path() {
        let abs = Path::new("/other/path/file.rs");
        let root = Path::new("/home/user/project");
        // strip_prefix fails, so it returns the abs path (with slashes normalized)
        let result = rel_path_str(abs, root);
        assert!(result.contains("file.rs"));
    }

    // ── find_blank_line_boundary tests ──────────────────────────────────

    #[test]
    fn find_blank_line_at_target_returns_immediately() {
        let lines = vec!["code", "code", "", "more code", "more code"];
        // Target is line 2 (0-indexed), which is blank
        let boundary = find_blank_line_boundary(&lines, 2);
        assert_eq!(boundary, 3); // split after the blank line
    }

    #[test]
    fn find_blank_line_within_tolerance_forward() {
        let mut lines: Vec<&str> = vec!["code"; 60];
        lines[52] = ""; // blank line 2 positions after target=50
        let boundary = find_blank_line_boundary(&lines, 50);
        assert_eq!(boundary, 53); // split after blank line at index 52
    }

    #[test]
    fn find_blank_line_within_tolerance_backward() {
        let mut lines: Vec<&str> = vec!["code"; 60];
        lines[48] = ""; // blank line 2 positions before target=50
        let boundary = find_blank_line_boundary(&lines, 50);
        // Forward search happens first, so if no blank line is found forward,
        // it checks backward. At offset=2, forward=52 (not blank), backward=48 (blank)
        assert_eq!(boundary, 49);
    }

    #[test]
    fn no_blank_line_returns_target() {
        let lines = vec!["code"; 100];
        let boundary = find_blank_line_boundary(&lines, 50);
        assert_eq!(boundary, 50);
    }

    // ── is_binary_extension edge cases ──────────────────────────────────

    #[test]
    fn is_binary_extension_case_insensitive() {
        assert!(is_binary_extension(Path::new("image.PNG")));
        assert!(is_binary_extension(Path::new("archive.ZIP")));
    }

    #[test]
    fn is_binary_extension_no_extension() {
        assert!(!is_binary_extension(Path::new("Makefile")));
        assert!(!is_binary_extension(Path::new("LICENSE")));
    }

    // ── detect_language additional cases ─────────────────────────────────

    #[test]
    fn detect_language_all_known_extensions() {
        let cases = vec![
            ("file.rs", "rust"),
            ("file.ts", "typescript"),
            ("file.tsx", "typescript"),
            ("file.js", "javascript"),
            ("file.jsx", "javascript"),
            ("file.svelte", "svelte"),
            ("file.py", "python"),
            ("file.go", "go"),
            ("file.toml", "toml"),
            ("file.json", "json"),
            ("file.yaml", "yaml"),
            ("file.yml", "yaml"),
            ("file.md", "markdown"),
            ("file.html", "html"),
            ("file.css", "css"),
            ("file.sql", "sql"),
            ("file.sh", "shell"),
            ("file.bash", "shell"),
        ];
        for (filename, expected) in cases {
            let result = detect_language(Path::new(filename));
            assert_eq!(
                result.as_deref(),
                Some(expected),
                "detect_language({filename})"
            );
        }
    }

    #[test]
    fn detect_language_no_extension() {
        assert_eq!(detect_language(Path::new("Makefile")), None);
    }

    // ── split_into_chunks edge cases ────────────────────────────────────

    #[test]
    fn split_single_line_file() {
        let chunks = split_into_chunks("only one line", "single.rs", Some("rust"));
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start_line, 1);
        assert_eq!(chunks[0].end_line, 1);
        assert_eq!(chunks[0].content, "only one line");
    }

    #[test]
    fn split_exactly_target_lines() {
        let lines: Vec<String> = (1..=TARGET_CHUNK_LINES)
            .map(|i| format!("line {i}"))
            .collect();
        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "exact.rs", None);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start_line, 1);
        assert_eq!(chunks[0].end_line, TARGET_CHUNK_LINES as u32);
    }

    #[test]
    fn split_preserves_file_path_and_language() {
        let chunks = split_into_chunks("content", "my/path.ts", Some("typescript"));
        assert_eq!(chunks[0].file_path, "my/path.ts");
        assert_eq!(chunks[0].language.as_deref(), Some("typescript"));
    }

    #[test]
    fn split_no_language_is_none() {
        let chunks = split_into_chunks("content", "file.txt", None);
        assert_eq!(chunks[0].language, None);
    }

    #[test]
    fn chunks_cover_entire_file_without_gaps_or_overlaps() {
        // Generate a file with 237 lines (odd number, not multiple of TARGET_CHUNK_LINES)
        let lines: Vec<String> = (1..=237).map(|i| format!("line {i}")).collect();
        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "test.rs", None);

        // First chunk starts at line 1
        assert_eq!(chunks[0].start_line, 1);
        // Last chunk ends at line 237
        assert_eq!(chunks.last().unwrap().end_line, 237);
        // No gaps between chunks
        for pair in chunks.windows(2) {
            assert_eq!(
                pair[0].end_line + 1,
                pair[1].start_line,
                "gap between chunks ending at {} and starting at {}",
                pair[0].end_line,
                pair[1].start_line
            );
        }
    }

    // ── ChunkError tests ────────────────────────────────────────────────

    #[test]
    fn chunk_error_display_messages() {
        let err = ChunkError::Walk("bad entry".to_string());
        assert_eq!(err.to_string(), "walk error: bad entry");

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err = ChunkError::Io(io_err);
        assert!(err.to_string().contains("IO error"));
    }
}
