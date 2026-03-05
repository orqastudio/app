use std::path::Path;

use regex::Regex;

use crate::domain::enforcement::{
    Condition, EnforcementEntry, EnforcementRule, EventType, RuleAction, Verdict,
};
use crate::domain::enforcement_parser::load_rules;
use crate::error::OrqaError;

/// A compiled enforcement entry with pre-built regex objects for fast matching.
struct CompiledEntry {
    /// Index into `EnforcementEngine::rules` for the owning rule.
    rule_index: usize,
    action: RuleAction,
    event: EventType,
    /// Compiled condition regexes for file events: (field_name, regex).
    compiled_conditions: Vec<(String, Regex)>,
    /// Compiled bash pattern regex.
    compiled_bash_pattern: Option<Regex>,
}

/// Compile an `EnforcementEntry` into a `CompiledEntry`.
///
/// Returns `None` if any regex fails to compile (invalid pattern). The caller
/// logs the failure and skips the entry rather than failing the whole load.
fn compile_entry(
    entry: &EnforcementEntry,
    rule_index: usize,
    rule_name: &str,
) -> Option<CompiledEntry> {
    let compiled_conditions = entry
        .conditions
        .iter()
        .filter_map(|c: &Condition| match Regex::new(&c.pattern) {
            Ok(re) => Some((c.field.clone(), re)),
            Err(e) => {
                tracing::warn!(
                    "[enforcement] invalid regex '{}' in rule '{rule_name}': {e}",
                    c.pattern
                );
                None
            }
        })
        .collect::<Vec<_>>();

    // If we lost conditions due to compile errors, the entry cannot enforce
    // correctly — skip it entirely rather than producing false positives.
    if compiled_conditions.len() != entry.conditions.len() {
        return None;
    }

    let compiled_bash_pattern = match &entry.pattern {
        Some(p) => match Regex::new(p) {
            Ok(re) => Some(re),
            Err(e) => {
                tracing::warn!(
                    "[enforcement] invalid bash pattern '{}' in rule '{rule_name}': {e}",
                    p
                );
                return None;
            }
        },
        None => None,
    };

    Some(CompiledEntry {
        rule_index,
        action: entry.action.clone(),
        event: entry.event.clone(),
        compiled_conditions,
        compiled_bash_pattern,
    })
}

/// Build a short excerpt from the rule prose for use in verdict messages.
fn prose_excerpt(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.len() <= 200 {
        trimmed.to_string()
    } else {
        format!("{}…", &trimmed[..200])
    }
}

/// The enforcement engine. Holds parsed rules and pre-compiled regexes.
///
/// Load once when a project is opened, then call `evaluate_file` or
/// `evaluate_bash` for each tool execution.
pub struct EnforcementEngine {
    rules: Vec<EnforcementRule>,
    compiled: Vec<CompiledEntry>,
}

impl EnforcementEngine {
    /// Load enforcement rules from all `*.md` files in `rules_dir`.
    ///
    /// Files without YAML frontmatter load as documentation-only (no entries).
    /// Invalid regex patterns are skipped with a warning.
    pub fn load(rules_dir: &Path) -> Result<Self, OrqaError> {
        let rules = load_rules(rules_dir)?;
        let mut compiled = Vec::new();

        for (idx, rule) in rules.iter().enumerate() {
            for entry in &rule.entries {
                if let Some(ce) = compile_entry(entry, idx, &rule.name) {
                    compiled.push(ce);
                }
            }
        }

        tracing::debug!(
            "[enforcement] loaded {} rules, {} compiled entries",
            rules.len(),
            compiled.len()
        );

        Ok(Self { rules, compiled })
    }

    /// Evaluate a file write or edit tool call.
    ///
    /// Checks all entries with `event: file`. All conditions in an entry must
    /// match (AND logic) for the entry to produce a verdict.
    pub fn evaluate_file(&self, file_path: &str, new_text: &str) -> Vec<Verdict> {
        let mut verdicts = Vec::new();

        for ce in &self.compiled {
            if ce.event != EventType::File {
                continue;
            }

            let all_match = ce.compiled_conditions.iter().all(|(field, re)| {
                let value = match field.as_str() {
                    "file_path" => file_path,
                    "new_text" => new_text,
                    other => {
                        tracing::warn!("[enforcement] unknown condition field: '{other}'");
                        return false;
                    }
                };
                re.is_match(value)
            });

            if all_match {
                let rule = &self.rules[ce.rule_index];
                verdicts.push(Verdict {
                    rule_name: rule.name.clone(),
                    action: ce.action.clone(),
                    message: prose_excerpt(&rule.prose),
                });
            }
        }

        verdicts
    }

    /// Evaluate a bash tool call.
    ///
    /// Checks all entries with `event: bash`. The entry's `pattern` must match
    /// the full command string for the entry to produce a verdict.
    pub fn evaluate_bash(&self, command: &str) -> Vec<Verdict> {
        let mut verdicts = Vec::new();

        for ce in &self.compiled {
            if ce.event != EventType::Bash {
                continue;
            }

            let matches = ce
                .compiled_bash_pattern
                .as_ref()
                .map(|re| re.is_match(command))
                .unwrap_or(false);

            if matches {
                let rule = &self.rules[ce.rule_index];
                verdicts.push(Verdict {
                    rule_name: rule.name.clone(),
                    action: ce.action.clone(),
                    message: prose_excerpt(&rule.prose),
                });
            }
        }

        verdicts
    }

    /// Return the loaded rules (for IPC listing).
    pub fn rules(&self) -> &[EnforcementRule] {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_rule_file(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(format!("{name}.md")), content).expect("write rule");
    }

    #[test]
    fn load_empty_rules_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let engine = EnforcementEngine::load(dir.path()).expect("should load");
        assert!(engine.rules().is_empty());
    }

    #[test]
    fn load_documentation_only_rule() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(dir.path(), "vision-alignment", "# Vision\n\nJust prose.");
        let engine = EnforcementEngine::load(dir.path()).expect("should load");
        assert_eq!(engine.rules().len(), 1);
        assert!(engine.rules()[0].entries.is_empty());
        assert!(engine.evaluate_file("anything.rs", "unwrap()").is_empty());
        assert!(engine.evaluate_bash("git commit --no-verify").is_empty());
    }

    #[test]
    fn file_rule_blocks_unwrap_in_rust() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "coding-standards",
            r#"---
scope: project
enforcement:
  - event: file
    action: block
    conditions:
      - field: file_path
        pattern: "src-tauri/src/.*\\.rs$"
      - field: new_text
        pattern: "unwrap\\(\\)"
---
# Coding Standards

Do not use unwrap() in production code.
"#,
        );

        let engine = EnforcementEngine::load(dir.path()).expect("should load");

        // Matching: Rust file path + unwrap in content
        let verdicts =
            engine.evaluate_file("src-tauri/src/domain/foo.rs", "let x = something.unwrap();");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].action, RuleAction::Block);
        assert_eq!(verdicts[0].rule_name, "coding-standards");

        // Non-matching: TypeScript file should not trigger Rust rule
        let verdicts = engine.evaluate_file("ui/src/foo.ts", "let x = something.unwrap();");
        assert!(verdicts.is_empty());

        // Non-matching: Rust file without unwrap
        let verdicts = engine.evaluate_file(
            "src-tauri/src/domain/foo.rs",
            "let x = something.map_err(|e| e.to_string())?;",
        );
        assert!(verdicts.is_empty());
    }

    #[test]
    fn bash_rule_blocks_no_verify() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "git-workflow",
            r#"---
scope: project
enforcement:
  - event: bash
    action: block
    pattern: "--no-verify"
---
# Git Workflow

Never use --no-verify on commits.
"#,
        );

        let engine = EnforcementEngine::load(dir.path()).expect("should load");

        let verdicts = engine.evaluate_bash("git commit --no-verify -m 'skip hooks'");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].action, RuleAction::Block);

        // Clean command should not trigger
        let verdicts = engine.evaluate_bash("git commit -m 'clean commit'");
        assert!(verdicts.is_empty());
    }

    #[test]
    fn warn_verdict_action() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "warn-rule",
            r#"---
scope: project
enforcement:
  - event: bash
    action: warn
    pattern: "git push --force"
---
# Warn on Force Push

Force pushing is risky.
"#,
        );

        let engine = EnforcementEngine::load(dir.path()).expect("should load");
        let verdicts = engine.evaluate_bash("git push --force origin main");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].action, RuleAction::Warn);
    }

    #[test]
    fn multiple_rules_can_trigger() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "rule-a",
            r#"---
scope: project
enforcement:
  - event: bash
    action: block
    pattern: "--no-verify"
---
# Rule A
"#,
        );
        write_rule_file(
            dir.path(),
            "rule-b",
            r#"---
scope: project
enforcement:
  - event: bash
    action: warn
    pattern: "--no-verify"
---
# Rule B
"#,
        );

        let engine = EnforcementEngine::load(dir.path()).expect("should load");
        let verdicts = engine.evaluate_bash("git commit --no-verify");
        assert_eq!(verdicts.len(), 2);
    }

    #[test]
    fn prose_excerpt_truncates_long_prose() {
        let long_prose = "a".repeat(300);
        let excerpt = prose_excerpt(&long_prose);
        assert!(excerpt.len() <= 204); // 200 chars + "…"
        assert!(excerpt.ends_with('…'));
    }

    #[test]
    fn prose_excerpt_keeps_short_prose() {
        let short_prose = "Short prose.";
        let excerpt = prose_excerpt(short_prose);
        assert_eq!(excerpt, "Short prose.");
    }
}
