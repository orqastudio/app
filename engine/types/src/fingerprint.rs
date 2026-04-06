//! Fingerprinting utilities for log event deduplication.
//!
//! Provides two public functions:
//! - `extract_template` strips dynamic tokens from a log message to produce a
//!   stable template. Two events with different artifact IDs but the same logical
//!   error yield the same template.
//! - `compute_fingerprint` hashes (source, level, template, stack_top) into a
//!   16-character hex string used as the canonical IssueGroup key.
//!
//! Parameterization rules are applied in specificity order (most specific first)
//! to avoid false positives. Plugin names, tool names, error class text, and
//! component prefixes are intentionally NOT parameterized — different values
//! there represent genuinely different issues.

use sha2::{Digest, Sha256};
use std::sync::LazyLock;

use regex::Regex;

/// Ordered list of (pattern, replacement) pairs applied by `extract_template`.
///
/// Rules are ordered most-specific first. Each rule is compiled once at first
/// use via `LazyLock`. The replacement strings use `{placeholder}` notation
/// so callers can read the template and understand what was stripped.
static RULES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    vec![
        // 1. Artifact IDs: e.g. RULE-00700241, EPIC-a3b4c5d6
        (
            Regex::new(r"[A-Z]{2,}-[a-f0-9]{6,10}").expect("rule 1 regex"),
            "{id}",
        ),
        // 2. UUIDs: e.g. 550e8400-e29b-41d4-a716-446655440000
        (
            Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                .expect("rule 2 regex"),
            "{uuid}",
        ),
        // 3. ISO timestamps: e.g. 2024-01-15T10:30:00Z or 2024-01-15T10:30:00.123Z
        (
            Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[\.\d]*Z?").expect("rule 3 regex"),
            "{timestamp}",
        ),
        // 4. Absolute Windows paths: e.g. C:\Users\foo\bar
        (
            Regex::new(r#"[A-Z]:\\[^\s"',]+"#).expect("rule 4 regex"),
            "{path}",
        ),
        // 5. Absolute Unix paths: e.g. /home/user/project/file.rs.
        //    Must be preceded by whitespace or start-of-string so that package
        //    name segments like "@orqastudio/plugin-rust" are not matched.
        (
            Regex::new(r#"(?:^|\s)/[^\s"',]+"#).expect("rule 5 regex"),
            " {path}",
        ),
        // 6. Localhost with port: e.g. localhost:3000 or 127.0.0.1:8080
        (
            Regex::new(r"(?:127\.0\.0\.1|localhost):\d{4,5}").expect("rule 6 regex"),
            "{addr}",
        ),
        // 7. Port numbers after colon: e.g. ":3000" when not already matched
        //    as part of a localhost address (rule 6 covers localhost:port).
        //    Matches a bare colon followed by 4–5 digits at a word boundary.
        //    Note: the regex crate does not support lookbehind; we match the
        //    leading colon and replace the whole token.
        (Regex::new(r":\d{4,5}\b").expect("rule 7 regex"), ":{port}"),
        // 8. PIDs (contextual): e.g. pid=12345 or pid 999
        (
            Regex::new(r"(?i)pid[= ]\d{3,6}").expect("rule 8 regex"),
            "pid {pid}",
        ),
        // 9. Session IDs: e.g. session_id=42 or session id 7
        (
            Regex::new(r"(?i)session[_ ](?:id[= ])?\d+").expect("rule 9 regex"),
            "session {sid}",
        ),
        // 10. Duration with parens: e.g. (12.5ms) or (300 µs)
        (
            Regex::new(r"\(\d+\.?\d*\s*(?:ms|s|µs)\)").expect("rule 10 regex"),
            "({dur})",
        ),
        // 11. Elapsed inline: e.g. elapsed_ms=1234
        (
            Regex::new(r"elapsed_ms=\d+").expect("rule 11 regex"),
            "elapsed_ms={n}",
        ),
        // 12. Artifact counts: e.g. "3 artifacts" or "1 artifact"
        (
            Regex::new(r"\d+ artifact(?:s)?").expect("rule 12 regex"),
            "{n} artifact(s)",
        ),
        // 13. Plugin counts: e.g. "5 plugins" or "1 plugin"
        (
            Regex::new(r"\d+ plugin(?:s)?").expect("rule 13 regex"),
            "{n} plugin(s)",
        ),
        // 14. Node counts: e.g. "12 nodes" or "1 node"
        (
            Regex::new(r"\d+ node(?:s)?").expect("rule 14 regex"),
            "{n} node(s)",
        ),
        // 15. Token counts: e.g. input_tokens=2048 or output_tokens=512.
        //     Both input and output are collapsed to the same template token
        //     so events differing only in direction merge into one issue group.
        (
            Regex::new(r"(?:input|output)_tokens=\d+").expect("rule 15 regex"),
            "{tok}_tokens={n}",
        ),
        // 16. Generic large numbers (4+ digits, standalone word boundary)
        (Regex::new(r"\b\d{4,}\b").expect("rule 16 regex"), "{n}"),
        // 17. Hex hashes (8+ chars, standalone word boundary): e.g. deadbeef12345678
        (
            Regex::new(r"\b[0-9a-f]{8,}\b").expect("rule 17 regex"),
            "{hash}",
        ),
        // 18. YAML error positions: e.g. "at line 42 column 7"
        (
            Regex::new(r"at line \d+ column \d+").expect("rule 18 regex"),
            "at line {n} column {n}",
        ),
    ]
});

/// Strips dynamic tokens from a log message to produce a stable template.
///
/// Two events that differ only in artifact IDs, timestamps, paths, or other
/// dynamic values will produce the same template. Plugin names, tool names,
/// error class text, and component prefixes are preserved because different
/// values there represent genuinely different issues.
///
/// Rules are applied in specificity order (most specific first) so that, for
/// example, ISO timestamps are matched before generic large numbers.
pub fn extract_template(message: &str) -> String {
    let mut result = message.to_owned();
    for (re, replacement) in RULES.iter() {
        result = re.replace_all(&result, *replacement).into_owned();
    }
    result
}

/// Hashes the canonical (source, level, template, stack_top) tuple into a
/// 16-character lowercase hex fingerprint.
///
/// The fingerprint is used as the IssueGroup key — two events with the same
/// source, level, message template, and top stack frame are considered the same
/// logical issue regardless of when they occurred or what the specific dynamic
/// values were.
///
/// `stack_top` should be the `file:line` of the innermost relevant stack frame,
/// or an empty string if no stack information is available.
pub fn compute_fingerprint(source: &str, level: &str, template: &str, stack_top: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    hasher.update(b"|");
    hasher.update(level.as_bytes());
    hasher.update(b"|");
    hasher.update(template.as_bytes());
    hasher.update(b"|");
    hasher.update(stack_top.as_bytes());
    let result = hasher.finalize();
    // Take first 8 bytes → 16 hex chars. This gives 2^64 collision resistance,
    // sufficient for deduplicating events within a session.
    hex::encode(&result[..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- extract_template tests ---

    #[test]
    fn different_artifact_ids_same_template() {
        // Artifact IDs with different suffixes must collapse to the same template.
        let a = extract_template("Failed to load RULE-00700241");
        let b = extract_template("Failed to load RULE-d4e7f3a1");
        assert_eq!(a, b);
        assert_eq!(a, "Failed to load {id}");
    }

    #[test]
    fn different_plugins_different_template() {
        // Plugin names are NOT parameterized — different plugins = different issues.
        let a = extract_template("[watcher] failed for @orqastudio/plugin-rust");
        let b = extract_template("[watcher] failed for @orqastudio/plugin-svelte");
        assert_ne!(a, b);
    }

    #[test]
    fn uuid_stripped() {
        let msg = "Session 550e8400-e29b-41d4-a716-446655440000 expired";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "Session {uuid} expired");
    }

    #[test]
    fn timestamp_stripped() {
        let msg = "Event recorded at 2024-01-15T10:30:00.123Z";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "Event recorded at {timestamp}");
    }

    #[test]
    fn path_stripped_unix() {
        let msg = "Failed to read /home/user/project/config.yaml";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "Failed to read {path}");
    }

    #[test]
    fn path_stripped_windows() {
        let msg = r"File not found: C:\Users\Bobbi\project\config.yaml";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "File not found: {path}");
    }

    #[test]
    fn port_stripped() {
        let msg = "Server listening on localhost:3000";
        let tmpl = extract_template(msg);
        // localhost:port → {addr}
        assert_eq!(tmpl, "Server listening on {addr}");
    }

    #[test]
    fn elapsed_stripped() {
        let msg = "Build complete elapsed_ms=4523";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "Build complete elapsed_ms={n}");
    }

    #[test]
    fn artifact_count_stripped() {
        let a = extract_template("Loaded 3 artifacts from registry");
        let b = extract_template("Loaded 17 artifacts from registry");
        assert_eq!(a, b);
        assert_eq!(a, "Loaded {n} artifact(s) from registry");
    }

    #[test]
    fn plugin_count_stripped() {
        let a = extract_template("Registered 2 plugins");
        let b = extract_template("Registered 10 plugins");
        assert_eq!(a, b);
        assert_eq!(a, "Registered {n} plugin(s)");
    }

    #[test]
    fn yaml_error_position_stripped() {
        let msg = "Parse error: at line 42 column 7";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "Parse error: at line {n} column {n}");
    }

    #[test]
    fn duration_with_parens_stripped() {
        let a = extract_template("Query complete (12.5ms)");
        let b = extract_template("Query complete (300ms)");
        assert_eq!(a, b);
        assert_eq!(a, "Query complete ({dur})");
    }

    #[test]
    fn token_counts_stripped() {
        // Both input_tokens and output_tokens are collapsed to {tok}_tokens={n}.
        let a = extract_template("Usage: input_tokens=2048");
        let b = extract_template("Usage: input_tokens=512");
        assert_eq!(a, b);
        assert_eq!(a, "Usage: {tok}_tokens={n}");
    }

    #[test]
    fn component_prefix_preserved() {
        // Component prefixes like [engine] are category identifiers and must stay.
        let msg = "[engine] initialization complete";
        let tmpl = extract_template(msg);
        assert_eq!(tmpl, "[engine] initialization complete");
    }

    // --- compute_fingerprint tests ---

    #[test]
    fn identical_messages_same_fingerprint() {
        let fp1 = compute_fingerprint("daemon", "ERROR", "Failed to load {id}", "");
        let fp2 = compute_fingerprint("daemon", "ERROR", "Failed to load {id}", "");
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn different_levels_different_fingerprint() {
        let fp1 = compute_fingerprint("daemon", "ERROR", "Connection refused", "");
        let fp2 = compute_fingerprint("daemon", "WARN", "Connection refused", "");
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn different_sources_different_fingerprint() {
        let fp1 = compute_fingerprint("daemon", "ERROR", "Startup failed", "");
        let fp2 = compute_fingerprint("app", "ERROR", "Startup failed", "");
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn empty_stack_top_handled() {
        // Must not panic and must produce a valid 16-char hex string.
        let fp = compute_fingerprint("worker", "WARN", "Queue timeout", "");
        assert_eq!(fp.len(), 16);
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn fingerprint_is_16_chars_hex() {
        let fp = compute_fingerprint("mcp", "INFO", "Request received", "src/main.rs:42");
        assert_eq!(fp.len(), 16);
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn stack_top_affects_fingerprint() {
        let fp1 = compute_fingerprint("daemon", "ERROR", "Crash", "src/a.rs:10");
        let fp2 = compute_fingerprint("daemon", "ERROR", "Crash", "src/b.rs:20");
        assert_ne!(fp1, fp2);
    }
}
