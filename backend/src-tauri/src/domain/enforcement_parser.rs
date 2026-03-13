use serde::Deserialize;

use crate::domain::enforcement::{
    Condition, EnforcementEntry, EnforcementRule, EventType, RuleAction,
};
use crate::error::OrqaError;

/// Raw YAML frontmatter shape for an enforcement entry.
#[derive(Debug, Deserialize)]
struct RawEntry {
    event: String,
    action: String,
    #[serde(default)]
    conditions: Vec<RawCondition>,
    pattern: Option<String>,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    skills: Vec<String>,
}

/// Raw YAML frontmatter shape for a condition.
#[derive(Debug, Deserialize)]
struct RawCondition {
    field: String,
    pattern: String,
}

/// Raw YAML frontmatter for a rule file.
#[derive(Debug, Deserialize)]
struct RawFrontmatter {
    #[serde(default = "default_scope")]
    scope: String,
    #[serde(default)]
    enforcement: Vec<RawEntry>,
}

fn default_scope() -> String {
    "project".to_string()
}

/// Split a markdown file into (frontmatter_yaml, prose_body).
///
/// Returns `None` for the frontmatter if the file does not start with `---`.
fn split_frontmatter(content: &str) -> (Option<&str>, &str) {
    if !content.starts_with("---") {
        return (None, content);
    }

    // Find the closing `---` (must be on its own line, after the opening)
    let after_open = &content[3..];
    if let Some(close_offset) = after_open.find("\n---") {
        let yaml = &after_open[..close_offset];
        // +4 skips "\n---"; +1 more for the newline after the closing delimiter
        let rest_start = 3 + close_offset + 4;
        let prose = content
            .get(rest_start..)
            .unwrap_or("")
            .trim_start_matches('\n');
        (Some(yaml), prose)
    } else {
        (None, content)
    }
}

/// Parse a single `RawEntry` into an `EnforcementEntry`, validating field values.
fn parse_entry(raw: RawEntry) -> Result<EnforcementEntry, OrqaError> {
    let event = match raw.event.as_str() {
        "file" => EventType::File,
        "bash" => EventType::Bash,
        "scan" => EventType::Scan,
        "lint" => EventType::Lint,
        other => {
            return Err(OrqaError::Validation(format!(
                "unknown enforcement event type: '{other}'"
            )))
        }
    };

    let action = match raw.action.as_str() {
        "block" => RuleAction::Block,
        "warn" => RuleAction::Warn,
        "inject" => RuleAction::Inject,
        other => {
            return Err(OrqaError::Validation(format!(
                "unknown enforcement action: '{other}'"
            )))
        }
    };

    let conditions = raw
        .conditions
        .into_iter()
        .map(|c| Condition {
            field: c.field,
            pattern: c.pattern,
        })
        .collect();

    Ok(EnforcementEntry {
        event,
        action,
        conditions,
        pattern: raw.pattern,
        scope: raw.scope,
        skills: raw.skills,
    })
}

/// Parse rule content (file name stem + string content) into an `EnforcementRule`.
///
/// This is a pure function — no filesystem I/O. Callers are responsible for
/// reading the file content and providing the rule name (typically the file stem).
///
/// Files without YAML frontmatter or without an `enforcement:` key are
/// returned with empty `entries` — they are documentation-only rules.
pub fn parse_rule_content(name: &str, content: &str) -> Result<EnforcementRule, OrqaError> {
    let (frontmatter_str, prose) = split_frontmatter(content);

    let (scope, entries) = match frontmatter_str {
        None => ("project".to_string(), Vec::new()),
        Some(yaml) => {
            let raw: RawFrontmatter = serde_yaml::from_str(yaml).map_err(|e| {
                OrqaError::Serialization(format!("invalid YAML frontmatter in '{name}': {e}"))
            })?;

            let mut parsed = Vec::new();
            for entry in raw.enforcement {
                match parse_entry(entry) {
                    Ok(e) => parsed.push(e),
                    Err(err) => {
                        tracing::warn!("[enforcement] skipping invalid entry in '{name}': {err}");
                    }
                }
            }

            (raw.scope, parsed)
        }
    };

    Ok(EnforcementRule {
        name: name.to_string(),
        scope,
        entries,
        prose: prose.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_no_frontmatter() {
        let content = "# Just a heading\n\nSome prose.";
        let (fm, prose) = split_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(prose, content);
    }

    #[test]
    fn split_with_frontmatter() {
        let content = "---\nscope: project\n---\n# Heading\n\nProse here.";
        let (fm, prose) = split_frontmatter(content);
        // The closing `---` is found at "\n---", so the yaml slice ends just before
        // that newline — it does NOT include a trailing newline character.
        assert_eq!(fm, Some("\nscope: project"));
        assert_eq!(prose, "# Heading\n\nProse here.");
    }

    #[test]
    fn split_frontmatter_no_closing_delimiter() {
        let content = "---\nscope: project\n# Heading";
        let (fm, prose) = split_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(prose, content);
    }

    #[test]
    fn parse_entry_file_block() {
        let raw = RawEntry {
            event: "file".to_string(),
            action: "block".to_string(),
            conditions: vec![
                RawCondition {
                    field: "file_path".to_string(),
                    pattern: r"\.rs$".to_string(),
                },
                RawCondition {
                    field: "new_text".to_string(),
                    pattern: r"unwrap\(\)".to_string(),
                },
            ],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        let entry = parse_entry(raw).expect("should parse");
        assert_eq!(entry.event, EventType::File);
        assert_eq!(entry.action, RuleAction::Block);
        assert_eq!(entry.conditions.len(), 2);
    }

    #[test]
    fn parse_entry_bash_warn() {
        let raw = RawEntry {
            event: "bash".to_string(),
            action: "warn".to_string(),
            conditions: vec![],
            pattern: Some("--no-verify".to_string()),
            scope: None,
            skills: vec![],
        };
        let entry = parse_entry(raw).expect("should parse");
        assert_eq!(entry.event, EventType::Bash);
        assert_eq!(entry.action, RuleAction::Warn);
        assert!(entry.pattern.is_some());
    }

    #[test]
    fn parse_entry_inject_with_skills() {
        let raw = RawEntry {
            event: "file".to_string(),
            action: "inject".to_string(),
            conditions: vec![RawCondition {
                field: "file_path".to_string(),
                pattern: r"src-tauri/.*\.rs$".to_string(),
            }],
            pattern: None,
            scope: None,
            skills: vec!["rust-async-patterns".to_string(), "tauri-v2".to_string()],
        };
        let entry = parse_entry(raw).expect("should parse");
        assert_eq!(entry.event, EventType::File);
        assert_eq!(entry.action, RuleAction::Inject);
        assert_eq!(entry.skills, vec!["rust-async-patterns", "tauri-v2"]);
    }

    #[test]
    fn parse_entry_lint_event() {
        let raw = RawEntry {
            event: "lint".to_string(),
            action: "warn".to_string(),
            conditions: vec![],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        let entry = parse_entry(raw).expect("should parse");
        assert_eq!(entry.event, EventType::Lint);
    }

    #[test]
    fn parse_entry_unknown_event_errors() {
        let raw = RawEntry {
            event: "network".to_string(),
            action: "block".to_string(),
            conditions: vec![],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        assert!(parse_entry(raw).is_err());
    }

    #[test]
    fn parse_entry_unknown_action_errors() {
        let raw = RawEntry {
            event: "file".to_string(),
            action: "ignore".to_string(),
            conditions: vec![],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        assert!(parse_entry(raw).is_err());
    }

    #[test]
    fn parse_entry_scan_with_scope() {
        let raw = RawEntry {
            event: "scan".to_string(),
            action: "warn".to_string(),
            conditions: vec![RawCondition {
                field: "content".to_string(),
                pattern: r"unwrap\(\)".to_string(),
            }],
            pattern: None,
            scope: Some(".orqa/process/agents/*.md".to_string()),
            skills: vec![],
        };
        let entry = parse_entry(raw).expect("should parse");
        assert_eq!(entry.event, EventType::Scan);
        assert_eq!(entry.action, RuleAction::Warn);
        assert_eq!(entry.conditions.len(), 1);
        assert_eq!(entry.scope.as_deref(), Some(".orqa/process/agents/*.md"));
        assert!(entry.pattern.is_none());
    }

    #[test]
    fn parse_rule_content_no_frontmatter() {
        let rule = parse_rule_content("my-rule", "# My Rule\n\nSome prose.").expect("should parse");
        assert_eq!(rule.name, "my-rule");
        assert_eq!(rule.scope, "project");
        assert!(rule.entries.is_empty());
        assert!(rule.prose.contains("My Rule"));
    }

    #[test]
    fn parse_rule_content_with_enforcement() {
        let content = r#"---
scope: project
enforcement:
  - event: file
    action: block
    conditions:
      - field: file_path
        pattern: "src-tauri/src/.*\\.rs$"
      - field: new_text
        pattern: "unwrap\\(\\)"
  - event: bash
    action: block
    pattern: "--no-verify"
---
# Coding Standards

Do not use unwrap in production code.
"#;

        let rule = parse_rule_content("coding-standards", content).expect("should parse");
        assert_eq!(rule.name, "coding-standards");
        assert_eq!(rule.entries.len(), 2);
        assert_eq!(rule.entries[0].event, EventType::File);
        assert_eq!(rule.entries[0].conditions.len(), 2);
        assert_eq!(rule.entries[1].event, EventType::Bash);
        assert!(rule.entries[1].pattern.is_some());
        assert!(rule.prose.contains("Coding Standards"));
    }
}
