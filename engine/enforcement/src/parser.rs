//! Enforcement rule parser for the orqa-engine crate.
//!
//! Parses YAML frontmatter from enforcement rule `.md` files into typed
//! `EnforcementRule` values. This is a pure module — no filesystem I/O.
//! Callers provide the file content as a string; see `store::load_rules` for
//! the filesystem-level loader that drives this parser.
//!
//! ## Mechanism model
//!
//! Every enforcement entry in the YAML frontmatter carries a `mechanism:` tag
//! that selects which execution path (if any) the daemon uses:
//!
//! - `hook` — Claude Code hook matched against tool calls.  These are the
//!   only entries the daemon actively evaluates, and they are the only ones
//!   lifted into [`EnforcementEntry`].
//! - `behavioral`, `pre-commit`, `lint`, `tsc`, `cargo-test`, `vitest`,
//!   `svelte-check`, `json-schema`, `tool`, `svelte-check`, … — declarative
//!   documentation.  They describe how a rule is enforced *outside* the
//!   daemon (by a linter, a git hook, a test runner, the system prompt, …)
//!   and produce no runtime entries.
//!
//! Using an internally-tagged enum on `mechanism` makes the split explicit
//! and means new declarative mechanisms can be added to rule files without
//! the parser rejecting them — unknown mechanisms are captured by the
//! `Declarative` catch-all.
//!
//! Hook entries whose `event` or `action` don't match the known enums are
//! skipped at `debug!` level so the log doesn't drown in noise for
//! documented-only hooks like `PostToolUse / team_create / check`.

use serde::Deserialize;

use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::enforcement::{
    Condition, EnforcementEntry, EnforcementRule, EventType, RuleAction,
};

/// Raw YAML shape for a single enforcement entry.
///
/// Discriminated by the `mechanism` field.  Only the `Hook` variant produces
/// a runtime [`EnforcementEntry`]; every other variant is declarative and
/// silently dropped by the parser.
#[derive(Debug, Deserialize)]
#[serde(tag = "mechanism", rename_all = "kebab-case")]
enum RawEntry {
    /// Claude Code hook — evaluated by the daemon against tool calls.
    Hook(RawHook),
    /// Any other mechanism (behavioral, pre-commit, lint, tsc, cargo-test,
    /// vitest, svelte-check, json-schema, tool, …).  These are declarative
    /// only and carry no runtime payload the daemon needs.
    #[serde(other)]
    Declarative,
}

/// Raw YAML shape for a `mechanism: hook` entry.
///
/// Fields are intentionally permissive so rule files can grow new optional
/// keys (like `message` or `type`) without breaking the parser.  Unknown
/// keys are ignored by serde's default behavior.
#[derive(Debug, Deserialize)]
struct RawHook {
    /// Event subject — `file`, `bash`, `scan`, or `lint`.  Hooks whose
    /// event is anything else (e.g. `team_create`, `session_end`) are
    /// documented-only and skipped.
    event: String,
    /// Action to take on match — `block`, `warn`, or `inject`.  Unknown
    /// actions (e.g. `check`) are documented-only and skipped.
    action: String,
    /// Optional AND-composed conditions for file/scan events.
    #[serde(default)]
    conditions: Vec<RawCondition>,
    /// Optional regex pattern, used primarily by bash hooks.
    pattern: Option<String>,
    /// Optional glob scope for `scan` events.
    #[serde(default)]
    scope: Option<String>,
    /// Knowledge artifacts injected by `inject` hooks.
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
///
/// Only the fields the parser needs are modelled — every other top-level key
/// (id, title, status, created, relationships, …) is ignored by serde.
#[derive(Debug, Deserialize)]
struct RawFrontmatter {
    #[serde(default = "default_scope")]
    scope: String,
    #[serde(default)]
    enforcement: Vec<RawEntry>,
}

/// Returns the default scope value ("project") when the frontmatter omits it.
fn default_scope() -> String {
    "project".to_owned()
}

/// Split a markdown file into (frontmatter_yaml, prose_body).
///
/// Returns `None` for the frontmatter if the file does not start with `---`.
/// The frontmatter slice excludes the opening and closing `---` delimiters.
fn split_frontmatter(content: &str) -> (Option<&str>, &str) {
    if !content.starts_with("---") {
        return (None, content);
    }

    // Find the closing `---` (must be on its own line, after the opening).
    let after_open = &content[3..];
    if let Some(close_offset) = after_open.find("\n---") {
        let yaml = &after_open[..close_offset];
        // +4 skips "\n---"; +1 more for the newline after the closing delimiter.
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

/// Map a `mechanism: hook` YAML entry to an executable [`EnforcementEntry`].
///
/// Returns `Ok(Some(entry))` for hooks the daemon knows how to evaluate,
/// `Ok(None)` for hooks whose event or action isn't in the daemon's enum set
/// (treated as documentation-only), and `Err` only for truly malformed input.
fn parse_hook(raw: RawHook) -> Result<Option<EnforcementEntry>, EngineError> {
    let event = match raw.event.as_str() {
        "file" => EventType::File,
        "bash" => EventType::Bash,
        "scan" => EventType::Scan,
        "lint" => EventType::Lint,
        other => {
            // Documented-only: hooks targeting events the daemon doesn't
            // evaluate (team_create, session_end, prompt, …).
            tracing::debug!(
                "[enforcement] skipping hook with undaemoned event '{other}' \
                 — treated as declarative"
            );
            return Ok(None);
        }
    };

    let action = match raw.action.as_str() {
        "block" => RuleAction::Block,
        "warn" => RuleAction::Warn,
        "inject" => RuleAction::Inject,
        other => {
            // Documented-only: actions the daemon cannot dispatch.
            tracing::debug!(
                "[enforcement] skipping hook with undaemoned action '{other}' \
                 — treated as declarative"
            );
            return Ok(None);
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

    Ok(Some(EnforcementEntry {
        event,
        action,
        conditions,
        pattern: raw.pattern,
        scope: raw.scope,
        knowledge: raw.skills,
    }))
}

/// Parse rule content (file name stem + string content) into an `EnforcementRule`.
///
/// This is a pure function — no filesystem I/O. Callers are responsible for
/// reading the file content and providing the rule name (typically the file stem).
///
/// Files without YAML frontmatter or without an `enforcement:` key are
/// returned with empty `entries` — they are documentation-only rules.
pub fn parse_rule_content(name: &str, content: &str) -> Result<EnforcementRule, EngineError> {
    let (frontmatter_str, prose) = split_frontmatter(content);

    let (scope, entries) = match frontmatter_str {
        None => ("project".to_owned(), Vec::new()),
        Some(yaml) => {
            let raw: RawFrontmatter = serde_yaml::from_str(yaml).map_err(|e| {
                EngineError::Yaml(format!("invalid YAML frontmatter in '{name}': {e}"))
            })?;

            let mut parsed = Vec::new();
            for entry in raw.enforcement {
                match entry {
                    RawEntry::Hook(hook) => match parse_hook(hook) {
                        Ok(Some(e)) => parsed.push(e),
                        Ok(None) => {
                            // Already logged at debug level inside parse_hook.
                        }
                        Err(err) => {
                            tracing::warn!(
                                "[enforcement] skipping invalid hook in '{name}': {err}"
                            );
                        }
                    },
                    RawEntry::Declarative => {
                        // Behavioral / pre-commit / lint / tsc / … — not
                        // evaluated by the daemon.  No log line: these are
                        // the common case and would produce noise.
                    }
                }
            }

            (raw.scope, parsed)
        }
    };

    Ok(EnforcementRule {
        name: name.to_owned(),
        scope,
        entries,
        prose: prose.to_owned(),
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
    fn parse_hook_file_block() {
        let raw = RawHook {
            event: "file".to_owned(),
            action: "block".to_owned(),
            conditions: vec![
                RawCondition {
                    field: "file_path".to_owned(),
                    pattern: r"\.rs$".to_owned(),
                },
                RawCondition {
                    field: "new_text".to_owned(),
                    pattern: r"unwrap\(\)".to_owned(),
                },
            ],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        let entry = parse_hook(raw).expect("should parse").expect("should emit");
        assert_eq!(entry.event, EventType::File);
        assert_eq!(entry.action, RuleAction::Block);
        assert_eq!(entry.conditions.len(), 2);
    }

    #[test]
    fn parse_hook_bash_warn() {
        let raw = RawHook {
            event: "bash".to_owned(),
            action: "warn".to_owned(),
            conditions: vec![],
            pattern: Some("--no-verify".to_owned()),
            scope: None,
            skills: vec![],
        };
        let entry = parse_hook(raw).expect("should parse").expect("should emit");
        assert_eq!(entry.event, EventType::Bash);
        assert_eq!(entry.action, RuleAction::Warn);
        assert!(entry.pattern.is_some());
    }

    #[test]
    fn parse_hook_inject_with_knowledge() {
        let raw = RawHook {
            event: "file".to_owned(),
            action: "inject".to_owned(),
            conditions: vec![RawCondition {
                field: "file_path".to_owned(),
                pattern: r"src-tauri/.*\.rs$".to_owned(),
            }],
            pattern: None,
            scope: None,
            skills: vec!["rust-async-patterns".to_owned(), "tauri-v2".to_owned()],
        };
        let entry = parse_hook(raw).expect("should parse").expect("should emit");
        assert_eq!(entry.event, EventType::File);
        assert_eq!(entry.action, RuleAction::Inject);
        assert_eq!(entry.knowledge, vec!["rust-async-patterns", "tauri-v2"]);
    }

    #[test]
    fn parse_hook_undaemoned_event_is_none() {
        let raw = RawHook {
            event: "team_create".to_owned(),
            action: "block".to_owned(),
            conditions: vec![],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        assert!(parse_hook(raw).expect("should parse").is_none());
    }

    #[test]
    fn parse_hook_undaemoned_action_is_none() {
        let raw = RawHook {
            event: "file".to_owned(),
            action: "check".to_owned(),
            conditions: vec![],
            pattern: None,
            scope: None,
            skills: vec![],
        };
        assert!(parse_hook(raw).expect("should parse").is_none());
    }

    #[test]
    fn parse_hook_scan_with_scope() {
        let raw = RawHook {
            event: "scan".to_owned(),
            action: "warn".to_owned(),
            conditions: vec![RawCondition {
                field: "content".to_owned(),
                pattern: r"unwrap\(\)".to_owned(),
            }],
            pattern: None,
            scope: Some(".claude/agents/*.md".to_owned()),
            skills: vec![],
        };
        let entry = parse_hook(raw).expect("should parse").expect("should emit");
        assert_eq!(entry.event, EventType::Scan);
        assert_eq!(entry.action, RuleAction::Warn);
        assert_eq!(entry.conditions.len(), 1);
        assert_eq!(entry.scope.as_deref(), Some(".claude/agents/*.md"));
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
    fn parse_rule_content_mechanism_hook() {
        let content = r#"---
scope: project
enforcement:
  - mechanism: hook
    type: PreToolUse
    event: file
    action: block
    conditions:
      - field: file_path
        pattern: "src-tauri/src/.*\\.rs$"
      - field: new_text
        pattern: "unwrap\\(\\)"
  - mechanism: hook
    type: PreToolUse
    event: bash
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

    /// The real-world shape: a rule mixing behavioral guidance with actual
    /// hooks. Before the split-by-mechanism rewrite, this failed with
    /// `missing field 'event' at line 12 column 5` because the parser
    /// required `event` on every entry. Now the behavioral entry is
    /// silently dropped (it's declarative-only) and the hooks are lifted.
    #[test]
    fn parse_rule_content_mixed_mechanisms() {
        let content = r#"---
scope: project
enforcement:
  - mechanism: behavioral
    message: "Use worktree-based workflow with mandatory cleanup"
  - mechanism: hook
    type: PreToolUse
    event: bash
    action: block
    pattern: "git\\s+(commit|push)\\b[^|&;]*--no-verify"
  - mechanism: hook
    type: PreToolUse
    event: bash
    action: block
    pattern: "git\\s+reset\\b[^|&;]*--hard"
---
# Git Workflow
"#;

        let rule = parse_rule_content("git-workflow", content).expect("should parse");
        assert_eq!(
            rule.entries.len(),
            2,
            "behavioral entry should be dropped, 2 hooks should remain"
        );
        assert_eq!(rule.entries[0].event, EventType::Bash);
        assert_eq!(rule.entries[0].action, RuleAction::Block);
        assert_eq!(rule.entries[1].event, EventType::Bash);
        assert_eq!(rule.entries[1].action, RuleAction::Block);
    }

    /// Declarative mechanisms unknown to the parser (lint, pre-commit,
    /// json-schema, cargo-test, …) must not cause a top-level parse error.
    /// They are captured by the `Declarative` catch-all variant.
    #[test]
    fn parse_rule_content_tolerates_unknown_declarative_mechanisms() {
        let content = r#"---
scope: project
enforcement:
  - mechanism: behavioral
    message: "Use strict types"
  - mechanism: pre-commit
    description: "Runs tsc --strict"
  - mechanism: lint
    tool: eslint
    config: "./eslintrc.json"
  - mechanism: tsc
    args: ["--strict"]
  - mechanism: cargo-test
    scope: "**/*.rs"
  - mechanism: vitest
    scope: "**/*.test.ts"
  - mechanism: svelte-check
    scope: "**/*.svelte"
  - mechanism: json-schema
    schema: "./schema.json"
  - mechanism: tool
    name: "orqa validate"
  - mechanism: some-future-mechanism
    any: "shape"
---
# Rule
"#;

        let rule = parse_rule_content("tolerant", content).expect("should parse");
        assert!(
            rule.entries.is_empty(),
            "no declarative mechanism should produce runtime entries"
        );
    }

    /// An `action: check` hook — used by rules that run on tool events but
    /// only emit reminders — should deserialize cleanly and be skipped.
    #[test]
    fn parse_rule_content_check_action_is_declarative() {
        let content = r#"---
scope: project
enforcement:
  - mechanism: hook
    type: PostToolUse
    event: team_create
    action: check
    message: "Verify completion gate"
---
# Gate
"#;

        let rule = parse_rule_content("gate", content).expect("should parse");
        assert!(rule.entries.is_empty());
    }
}
