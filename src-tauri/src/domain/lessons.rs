use serde::{Deserialize, Serialize};

/// A single lesson captured from agent sessions.
///
/// Lessons are stored as individual markdown files in `.orqa/lessons/`
/// with YAML frontmatter. They are first-class governance artifacts that
/// feed the self-learning loop (Pillar 1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Unique identifier, e.g. "IMPL-001".
    pub id: String,
    /// Short title describing the lesson.
    pub title: String,
    /// Category: "process", "coding", or "architecture".
    pub category: String,
    /// Number of times this pattern has recurred.
    pub recurrence: i32,
    /// Status: "active", "promoted", or "resolved".
    pub status: String,
    /// Path to the rule or standard this lesson was promoted to, if any.
    pub promoted_to: Option<String>,
    /// ISO-8601 date string when the lesson was first created.
    pub created: String,
    /// ISO-8601 date string when the lesson was last updated.
    pub updated: String,
    /// Full markdown body (everything after the YAML frontmatter).
    pub body: String,
    /// Relative file path within the project, e.g. ".orqa/governance/lessons/IMPL-001.md".
    pub file_path: String,
}

/// Input for creating a new lesson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLesson {
    pub title: String,
    pub category: String,
    pub body: String,
}

/// Parse a lesson markdown file.
///
/// The file must begin with a YAML frontmatter block delimited by `---` lines.
/// Everything after the closing `---` is the lesson body.
pub fn parse_lesson(content: &str, file_path: &str) -> Result<Lesson, String> {
    let (frontmatter, body) = split_frontmatter(content)?;
    let lesson = parse_frontmatter_fields(&frontmatter, body.trim().to_string(), file_path)?;
    Ok(lesson)
}

/// Split the file content into frontmatter string and body string.
fn split_frontmatter(content: &str) -> Result<(String, &str), String> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Err("lesson file must begin with '---' YAML frontmatter".to_string());
    }
    let after_open = &trimmed[3..];
    let close_pos = after_open
        .find("\n---")
        .ok_or_else(|| "lesson file missing closing '---' for frontmatter".to_string())?;
    let frontmatter = after_open[..close_pos].to_string();
    let body = &after_open[close_pos + 4..]; // skip "\n---"
    Ok((frontmatter, body))
}

/// Parse YAML frontmatter fields into a `Lesson`.
fn parse_frontmatter_fields(
    frontmatter: &str,
    body: String,
    file_path: &str,
) -> Result<Lesson, String> {
    let id = extract_field(frontmatter, "id")
        .ok_or_else(|| "frontmatter missing required field: id".to_string())?;
    let title = extract_field(frontmatter, "title")
        .ok_or_else(|| "frontmatter missing required field: title".to_string())?;
    let category = extract_field(frontmatter, "category")
        .ok_or_else(|| "frontmatter missing required field: category".to_string())?;
    let recurrence_str = extract_field(frontmatter, "recurrence")
        .ok_or_else(|| "frontmatter missing required field: recurrence".to_string())?;
    let recurrence = recurrence_str.parse::<i32>().map_err(|_| {
        format!("frontmatter 'recurrence' is not a valid integer: {recurrence_str}")
    })?;
    let status = extract_field(frontmatter, "status")
        .ok_or_else(|| "frontmatter missing required field: status".to_string())?;
    let promoted_to = extract_nullable_field(frontmatter, "promoted-to");
    let created = extract_field(frontmatter, "created")
        .ok_or_else(|| "frontmatter missing required field: created".to_string())?;
    let updated = extract_field(frontmatter, "updated")
        .ok_or_else(|| "frontmatter missing required field: updated".to_string())?;

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
        file_path: file_path.to_string(),
    })
}

/// Extract a scalar YAML field value by key from frontmatter text.
///
/// Handles both unquoted and quoted values. Returns `None` if the field is absent.
fn extract_field(frontmatter: &str, key: &str) -> Option<String> {
    for line in frontmatter.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            let value = rest.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() && value != "null" {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Extract a nullable YAML field value — returns `None` for "null" or absent fields.
fn extract_nullable_field(frontmatter: &str, key: &str) -> Option<String> {
    for line in frontmatter.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            let value = rest.trim().trim_matches('"').trim_matches('\'');
            if value.is_empty() || value == "null" {
                return None;
            }
            return Some(value.to_string());
        }
    }
    None
}

/// Render a `Lesson` as a markdown file string (frontmatter + body).
pub fn render_lesson(lesson: &Lesson) -> String {
    let promoted_to = lesson
        .promoted_to
        .as_deref()
        .map(|v| format!("\"{v}\""))
        .unwrap_or_else(|| "null".to_string());

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
            parse_lesson(SAMPLE, ".orqa/governance/lessons/IMPL-001.md").expect("should parse");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.title, "Agent forgot to load skills");
        assert_eq!(lesson.category, "process");
        assert_eq!(lesson.recurrence, 2);
        assert_eq!(lesson.status, "active");
        assert!(lesson.promoted_to.is_none());
        assert_eq!(lesson.created, "2026-03-05");
        assert_eq!(lesson.updated, "2026-03-05");
        assert!(lesson.body.contains("## Description"));
        assert_eq!(lesson.file_path, ".orqa/governance/lessons/IMPL-001.md");
    }

    #[test]
    fn parse_missing_frontmatter_returns_error() {
        let result = parse_lesson("no frontmatter here", ".orqa/governance/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_unclosed_frontmatter_returns_error() {
        let result = parse_lesson("---\nid: IMPL-001\n", ".orqa/governance/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_required_field_returns_error() {
        let bad = "---\nid: IMPL-001\n---\nbody\n";
        let result = parse_lesson(bad, ".orqa/governance/lessons/x.md");
        assert!(result.is_err());
    }

    #[test]
    fn parse_promoted_to_value() {
        let content = "---\nid: IMPL-002\ntitle: \"Test\"\ncategory: coding\nrecurrence: 3\nstatus: promoted\npromoted-to: \"RULE-001\"\ncreated: 2026-01-01\nupdated: 2026-01-02\n---\nbody\n";
        let lesson =
            parse_lesson(content, ".orqa/governance/lessons/IMPL-002.md").expect("should parse");
        assert_eq!(lesson.promoted_to, Some("RULE-001".to_string()));
    }

    #[test]
    fn render_round_trip() {
        let lesson =
            parse_lesson(SAMPLE, ".orqa/governance/lessons/IMPL-001.md").expect("should parse");
        let rendered = render_lesson(&lesson);
        let reparsed = parse_lesson(&rendered, ".orqa/governance/lessons/IMPL-001.md")
            .expect("should re-parse");
        assert_eq!(reparsed.id, lesson.id);
        assert_eq!(reparsed.title, lesson.title);
        assert_eq!(reparsed.recurrence, lesson.recurrence);
        assert_eq!(reparsed.promoted_to, lesson.promoted_to);
    }

    #[test]
    fn extract_field_unquoted() {
        assert_eq!(
            extract_field("category: process\n", "category"),
            Some("process".to_string())
        );
    }

    #[test]
    fn extract_field_quoted() {
        assert_eq!(
            extract_field("title: \"My title\"\n", "title"),
            Some("My title".to_string())
        );
    }

    #[test]
    fn extract_nullable_field_null() {
        assert_eq!(
            extract_nullable_field("promoted-to: null\n", "promoted-to"),
            None
        );
    }
}
