//! orqa-lesson: Lesson parse/render logic and file-backed store.
//!
//! Provides `parse_lesson` and `render_lesson` functions that read and write
//! lesson markdown files. Lesson files use YAML frontmatter followed by a
//! markdown body. The types themselves (`Lesson`, `NewLesson`) are defined in
//! `orqa_engine_types::types::lesson` and re-exported here for convenience.
//!
//! Frontmatter parsing uses a generic key-value map (`HashMap<String, String>`)
//! so that the parser is not coupled to any specific set of field names. Known
//! fields are accessed by string key after the map is built. This satisfies P1
//! (Plugin-Composed Everything) — if the lesson schema gains or loses fields via
//! a plugin, the parser does not need to change.
//!
//! The `store` submodule provides a file-backed `FileLessonStore` that implements
//! `orqa_engine_types::traits::storage::LessonStore`.

/// File-backed lesson store implementing `LessonStore`.
pub mod store;

pub use orqa_engine_types::types::lesson::*;

/// Parse a lesson markdown file.
///
/// The file must begin with a YAML frontmatter block delimited by `---` lines.
/// Everything after the closing `---` is the lesson body.
pub fn parse_lesson(content: &str, file_path: &str) -> Result<Lesson, String> {
    let (frontmatter, body) = split_frontmatter(content)?;
    let lesson = parse_frontmatter_fields(&frontmatter, body.trim().to_owned(), file_path)?;
    Ok(lesson)
}

/// Split the file content into frontmatter string and body string.
fn split_frontmatter(content: &str) -> Result<(String, &str), String> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Err("lesson file must begin with '---' YAML frontmatter".to_owned());
    }
    let after_open = &trimmed[3..];
    let close_pos = after_open
        .find("\n---")
        .ok_or_else(|| "lesson file missing closing '---' for frontmatter".to_owned())?;
    let frontmatter = after_open[..close_pos].to_owned();
    let body = &after_open[close_pos + 4..]; // skip "\n---"
    Ok((frontmatter, body))
}

/// Parse YAML frontmatter into a generic key-value map.
///
/// Each `key: value` line is stored as-is in the map. Values are unquoted
/// (both `"` and `'` delimiters are stripped). Lines that do not match the
/// `key: value` pattern are silently skipped.
///
/// Using a generic map here means the parser is not coupled to any specific
/// set of frontmatter field names (P1: Plugin-Composed Everything). Known
/// fields are extracted from the map after it is built.
fn parse_frontmatter_map(frontmatter: &str) -> std::collections::HashMap<String, Option<String>> {
    let mut map = std::collections::HashMap::new();
    for line in frontmatter.lines() {
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_owned();
            let raw = line[colon_pos + 1..].trim();
            let value = if raw.is_empty() || raw == "null" {
                None
            } else {
                Some(raw.trim_matches('"').trim_matches('\'').to_owned())
            };
            map.insert(key, value);
        }
    }
    map
}

/// Extract a required field from the frontmatter map, returning an error if absent or null.
fn required(
    map: &std::collections::HashMap<String, Option<String>>,
    key: &str,
) -> Result<String, String> {
    match map.get(key) {
        Some(Some(v)) => Ok(v.clone()),
        Some(None) => Err(format!("frontmatter field '{key}' is null (required)")),
        None => Err(format!("frontmatter missing required field: {key}")),
    }
}

/// Extract an optional nullable field from the frontmatter map.
fn optional(
    map: &std::collections::HashMap<String, Option<String>>,
    key: &str,
) -> Option<String> {
    map.get(key).and_then(Clone::clone)
}

/// Parse YAML frontmatter fields into a `Lesson`.
///
/// Builds a generic frontmatter map first, then extracts known fields by key.
/// Unknown fields in the frontmatter are ignored — adding new fields to the
/// lesson schema via a plugin does not require parser changes.
fn parse_frontmatter_fields(
    frontmatter: &str,
    body: String,
    file_path: &str,
) -> Result<Lesson, String> {
    let map = parse_frontmatter_map(frontmatter);

    let id = required(&map, "id")?;
    let title = required(&map, "title")?;
    let category = required(&map, "category")?;
    let recurrence_str = required(&map, "recurrence")?;
    let recurrence = recurrence_str.parse::<i32>().map_err(|_| {
        format!("frontmatter 'recurrence' is not a valid integer: {recurrence_str}")
    })?;
    let status = required(&map, "status")?;
    let promoted_to = optional(&map, "promoted-to");
    let created = required(&map, "created")?;
    let updated = required(&map, "updated")?;

    Ok(Lesson {
        id,
        title,
        category,
        recurrence,
        status,
        promoted_to,
        created,
        updated,
        body,
        file_path: file_path.to_owned(),
    })
}

/// Render a `Lesson` as a markdown file string (frontmatter + body).
pub fn render_lesson(lesson: &Lesson) -> String {
    let promoted_to = lesson
        .promoted_to
        .as_deref()
        .map_or_else(|| "null".to_owned(), |v| format!("\"{v}\""));

    format!(
        "---\nid: {}\ntitle: \"{}\"\ncategory: {}\nrecurrence: {}\nstatus: {}\npromoted-to: {}\ncreated: {}\nupdated: {}\n---\n{}",
        lesson.id,
        lesson.title,
        lesson.category,
        lesson.recurrence,
        lesson.status,
        promoted_to,
        lesson.created,
        lesson.updated,
        lesson.body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"---
id: IMPL-001
title: "Agent forgot to load skills"
category: process
recurrence: 2
status: active
promoted-to: null
created: 2026-03-05
updated: 2026-03-05
---
## Description
Test body.
"#;

    #[test]
    fn parse_valid_lesson() {
        let lesson =
            parse_lesson(SAMPLE, ".orqa/learning/lessons/IMPL-001.md").expect("should parse");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.title, "Agent forgot to load skills");
        assert_eq!(lesson.category, "process");
        assert_eq!(lesson.recurrence, 2);
        assert_eq!(lesson.status, "active");
        assert!(lesson.promoted_to.is_none());
        assert_eq!(lesson.created, "2026-03-05");
        assert_eq!(lesson.updated, "2026-03-05");
        assert!(lesson.body.contains("## Description"));
        assert_eq!(lesson.file_path, ".orqa/learning/lessons/IMPL-001.md");
    }

    #[test]
    fn parse_missing_frontmatter_returns_error() {
        let result = parse_lesson("no frontmatter here", ".orqa/learning/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_unclosed_frontmatter_returns_error() {
        let result = parse_lesson("---\nid: IMPL-001\n", ".orqa/learning/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_required_field_returns_error() {
        let bad = "---\nid: IMPL-001\n---\nbody\n";
        let result = parse_lesson(bad, ".orqa/learning/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_promoted_to_value() {
        let content = "---\nid: IMPL-002\ntitle: \"Test\"\ncategory: coding\nrecurrence: 3\nstatus: promoted\npromoted-to: \"RULE-001\"\ncreated: 2026-01-01\nupdated: 2026-01-02\n---\nbody\n";
        let lesson =
            parse_lesson(content, ".orqa/learning/lessons/IMPL-002.md").expect("should parse");
        assert_eq!(lesson.promoted_to, Some("RULE-001".to_owned()));
    }

    #[test]
    fn render_round_trip() {
        let lesson =
            parse_lesson(SAMPLE, ".orqa/learning/lessons/IMPL-001.md").expect("should parse");
        let rendered = render_lesson(&lesson);
        let reparsed =
            parse_lesson(&rendered, ".orqa/learning/lessons/IMPL-001.md").expect("should re-parse");
        assert_eq!(reparsed.id, lesson.id);
        assert_eq!(reparsed.title, lesson.title);
        assert_eq!(reparsed.recurrence, lesson.recurrence);
        assert_eq!(reparsed.promoted_to, lesson.promoted_to);
    }

    #[test]
    fn parse_frontmatter_map_unquoted_value() {
        let map = parse_frontmatter_map("category: process\n");
        assert_eq!(map.get("category").cloned().flatten(), Some("process".to_owned()));
    }

    #[test]
    fn parse_frontmatter_map_quoted_value() {
        let map = parse_frontmatter_map("title: \"My title\"\n");
        assert_eq!(map.get("title").cloned().flatten(), Some("My title".to_owned()));
    }

    #[test]
    fn parse_frontmatter_map_null_value() {
        let map = parse_frontmatter_map("promoted-to: null\n");
        assert_eq!(map.get("promoted-to").cloned(), Some(None));
    }

    #[test]
    fn parse_frontmatter_map_extra_fields_are_preserved() {
        // Extra fields from future schema extensions are captured in the map,
        // not silently dropped. They are just not mapped to Lesson struct fields.
        let map = parse_frontmatter_map("id: X\nfuture-field: some-value\n");
        assert!(map.contains_key("future-field"));
    }

    #[test]
    fn required_returns_error_for_null_field() {
        let map = parse_frontmatter_map("promoted-to: null\n");
        assert!(required(&map, "promoted-to").is_err());
    }

    #[test]
    fn required_returns_error_for_absent_field() {
        let map = parse_frontmatter_map("id: X\n");
        assert!(required(&map, "missing-field").is_err());
    }

    #[test]
    fn optional_returns_none_for_null_or_absent() {
        let map = parse_frontmatter_map("promoted-to: null\n");
        assert!(optional(&map, "promoted-to").is_none());
        assert!(optional(&map, "not-present").is_none());
    }
}
