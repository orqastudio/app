//! Enforcement engine for the orqa-engine crate.
//!
//! Provides `EnforcementEngine`: loads compiled-regex enforcement rules and
//! evaluates them against file writes, bash commands, and on-demand project scans.
//! Load once when a project is opened; call evaluate_file, evaluate_bash, or scan
//! for each relevant tool execution.

use std::path::Path;

use regex::Regex;

use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::enforcement::{
    Condition, EnforcementEntry, EnforcementRule, EventType, RuleAction, ScanFinding, Verdict,
};

/// A compiled enforcement entry with pre-built regex objects for fast matching.
///
/// Produced from `EnforcementEntry` at engine construction time; invalid regex
/// patterns are logged and the entry is dropped rather than failing the whole load.
struct CompiledEntry {
    /// Index into `EnforcementEngine::rules` for the owning rule.
    rule_index: usize,
    action: RuleAction,
    event: EventType,
    /// Compiled condition regexes for file and scan events: (field_name, regex).
    compiled_conditions: Vec<(String, Regex)>,
    /// Compiled bash pattern regex.
    compiled_bash_pattern: Option<Regex>,
    /// Raw scope glob pattern for scan entries.
    scope: Option<String>,
    /// Knowledge artifacts to inject when action is `inject`.
    knowledge: Vec<String>,
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
        scope: entry.scope.clone(),
        knowledge: entry.knowledge.clone(),
    })
}

/// Build a short excerpt from the rule prose for use in verdict messages.
///
/// Truncates to 200 characters and appends an ellipsis when the prose is longer,
/// so enforcement messages stay concise in agent context windows.
fn prose_excerpt(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.len() <= 200 {
        trimmed.to_owned()
    } else {
        format!("{}…", &trimmed[..200])
    }
}

/// Expand a glob pattern and return all matching file paths as strings.
///
/// Returns an error if the pattern itself is syntactically invalid; glob I/O
/// errors on individual entries are logged as warnings and skipped.
fn collect_glob_paths(pattern: &str) -> Result<Vec<String>, EngineError> {
    let entries = glob::glob(pattern)
        .map_err(|e| EngineError::Scan(format!("invalid glob pattern '{pattern}': {e}")))?;

    // I/O errors on individual entries are logged and skipped; a bad pattern
    // returns Err above, so we only filter_map here for per-entry glob errors.
    let paths = entries
        .filter_map(|entry| match entry {
            Ok(path) => path.to_str().map(str::to_owned),
            Err(e) => {
                tracing::warn!("[enforcement] glob entry error for '{pattern}': {e}");
                None
            }
        })
        .collect();

    Ok(paths)
}

/// Scan the lines of `content` for violations defined by a compiled scan entry.
///
/// This is a pure function — no filesystem I/O. The caller is responsible for
/// reading the file and providing its content as a string slice.
///
/// Checks every line against all `content` conditions; returns findings for
/// every line where all conditions match.
fn scan_content(
    file_path: &str,
    content: &str,
    ce: &CompiledEntry,
    rule: &EnforcementRule,
) -> Vec<ScanFinding> {
    content
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let all_match = ce
                .compiled_conditions
                .iter()
                .all(|(field, re)| match field.as_str() {
                    "content" => re.is_match(line),
                    other => {
                        tracing::warn!("[enforcement] unknown scan condition field: '{other}'");
                        false
                    }
                });

            if all_match {
                Some(ScanFinding {
                    rule_name: rule.name.clone(),
                    action: ce.action.clone(),
                    file_path: file_path.to_owned(),
                    line: idx + 1,
                    content: line.trim().to_owned(),
                    message: prose_excerpt(&rule.prose),
                })
            } else {
                None
            }
        })
        .collect()
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
    /// Build an `EnforcementEngine` from a pre-loaded set of rules.
    ///
    /// The caller is responsible for loading the rules from disk (via
    /// `enforcement::store::load_rules`). This constructor is pure — it only
    /// compiles regexes and builds internal indices. Invalid regex patterns are
    /// skipped with a warning.
    pub fn new(rules: Vec<EnforcementRule>) -> Self {
        let compiled: Vec<CompiledEntry> = rules
            .iter()
            .enumerate()
            .flat_map(|(idx, rule)| {
                rule.entries
                    .iter()
                    .filter_map(move |entry| compile_entry(entry, idx, &rule.name))
            })
            .collect();

        tracing::debug!(
            "[enforcement] loaded {} rules, {} compiled entries",
            rules.len(),
            compiled.len()
        );

        Self { rules, compiled }
    }

    /// Evaluate a file write or edit tool call.
    ///
    /// Checks all entries with `event: file`. All conditions in an entry must
    /// match (AND logic) for the entry to produce a verdict.
    ///
    /// Entries with `event: lint` are skipped — they are declarative only.
    pub fn evaluate_file(&self, file_path: &str, new_text: &str) -> Vec<Verdict> {
        self.compiled
            .iter()
            .filter(|ce| ce.event == EventType::File)
            .filter_map(|ce| {
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
                    Some(Verdict {
                        rule_name: rule.name.clone(),
                        action: ce.action.clone(),
                        message: prose_excerpt(&rule.prose),
                        knowledge: ce.knowledge.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Evaluate a bash tool call.
    ///
    /// Checks all entries with `event: bash`. The entry's `pattern` must match
    /// the full command string for the entry to produce a verdict.
    ///
    /// Entries with `event: lint` are skipped — they are declarative only.
    pub fn evaluate_bash(&self, command: &str) -> Vec<Verdict> {
        self.compiled
            .iter()
            .filter(|ce| ce.event == EventType::Bash)
            .filter_map(|ce| {
                let matches = ce
                    .compiled_bash_pattern
                    .as_ref()
                    .is_some_and(|re| re.is_match(command));

                if matches {
                    let rule = &self.rules[ce.rule_index];
                    Some(Verdict {
                        rule_name: rule.name.clone(),
                        action: ce.action.clone(),
                        message: prose_excerpt(&rule.prose),
                        knowledge: ce.knowledge.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Scan project files for governance violations defined by `event: scan` entries.
    ///
    /// For each scan entry, resolves the `scope` glob relative to `project_path`,
    /// reads each matching file, and checks every line against the entry's conditions.
    /// Returns a flat list of findings — all entries and all matching files combined.
    ///
    /// This method performs filesystem I/O (glob resolution + file reads).
    /// The domain logic for line matching is pure (see `scan_content`).
    pub fn scan(&self, project_path: &Path) -> Result<Vec<ScanFinding>, EngineError> {
        let mut findings = Vec::new();

        for ce in &self.compiled {
            if ce.event != EventType::Scan {
                continue;
            }

            let Some(scope) = &ce.scope else {
                tracing::warn!(
                    "[enforcement] scan entry in rule '{}' has no scope — skipping",
                    self.rules[ce.rule_index].name
                );
                continue;
            };

            let glob_pattern = project_path.join(scope);
            let glob_str = glob_pattern.to_string_lossy();

            let paths = collect_glob_paths(&glob_str)?;

            for file_path in paths {
                let content = match std::fs::read_to_string(&file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::warn!("[enforcement] cannot read file '{file_path}': {e}");
                        continue;
                    }
                };
                let file_findings =
                    scan_content(&file_path, &content, ce, &self.rules[ce.rule_index]);
                findings.extend(file_findings);
            }
        }

        Ok(findings)
    }

    /// Return the loaded rules (for IPC listing and diagnostics).
    pub fn rules(&self) -> &[EnforcementRule] {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store;
    use std::path::Path;

    fn write_rule_file(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(format!("{name}.md")), content).expect("write rule");
    }

    fn load_engine(rules_dir: &Path) -> EnforcementEngine {
        let rules = store::load_rules(rules_dir).expect("should load rules");
        EnforcementEngine::new(rules)
    }

    #[test]
    fn load_empty_rules_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let engine = load_engine(dir.path());
        assert!(engine.rules().is_empty());
    }

    #[test]
    fn load_documentation_only_rule() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(dir.path(), "vision-alignment", "# Vision\n\nJust prose.");
        let engine = load_engine(dir.path());
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
  - mechanism: hook
    event: file
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

        let engine = load_engine(dir.path());

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
  - mechanism: hook
    event: bash
    action: block
    pattern: "--no-verify"
---
# Git Workflow

Never use --no-verify on commits.
"#,
        );

        let engine = load_engine(dir.path());

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
  - mechanism: hook
    event: bash
    action: warn
    pattern: "git push --force"
---
# Warn on Force Push

Force pushing is risky.
"#,
        );

        let engine = load_engine(dir.path());
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
  - mechanism: hook
    event: bash
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
  - mechanism: hook
    event: bash
    action: warn
    pattern: "--no-verify"
---
# Rule B
"#,
        );

        let engine = load_engine(dir.path());
        let verdicts = engine.evaluate_bash("git commit --no-verify");
        assert_eq!(verdicts.len(), 2);
    }

    #[test]
    fn inject_verdict_is_non_blocking_and_carries_knowledge() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "knowledge-injector",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: file
    action: inject
    conditions:
      - field: file_path
        pattern: "src-tauri/.*\\.rs$"
    skills:
      - rust-async-patterns
      - tauri-v2
---
# Knowledge Injector

Load Rust knowledge when editing Rust files.
"#,
        );

        let engine = load_engine(dir.path());

        let verdicts = engine.evaluate_file("src-tauri/src/domain/foo.rs", "some content");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].action, RuleAction::Inject);
        assert_eq!(
            verdicts[0].knowledge,
            vec!["rust-async-patterns", "tauri-v2"]
        );

        // Non-matching path should produce no verdict
        let verdicts = engine.evaluate_file("ui/lib/foo.ts", "some content");
        assert!(verdicts.is_empty());
    }

    #[test]
    fn lint_event_entries_are_skipped_during_evaluation() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "lint-rule",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: lint
    action: warn
    pattern: "clippy"
---
# Lint Rule

Delegates to clippy.
"#,
        );

        let engine = load_engine(dir.path());

        // Lint entries must not produce verdicts from evaluate_file or evaluate_bash
        let verdicts = engine.evaluate_file("src-tauri/src/foo.rs", "clippy stuff");
        assert!(
            verdicts.is_empty(),
            "lint entries must not trigger evaluate_file"
        );

        let verdicts = engine.evaluate_bash("cargo clippy");
        assert!(
            verdicts.is_empty(),
            "lint entries must not trigger evaluate_bash"
        );
    }

    #[test]
    fn inject_verdict_knowledge_empty_by_default() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_rule_file(
            dir.path(),
            "inject-no-knowledge",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: bash
    action: inject
    pattern: "cargo build"
---
# Inject No Knowledge
"#,
        );

        let engine = load_engine(dir.path());
        let verdicts = engine.evaluate_bash("cargo build --release");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].action, RuleAction::Inject);
        assert!(verdicts[0].knowledge.is_empty());
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

    #[test]
    fn scan_finds_pattern_in_matching_files() {
        let rules_dir = tempfile::tempdir().expect("tempdir");
        let project_dir = tempfile::tempdir().expect("tempdir");

        // Create a subdirectory and agent file inside the project
        let agents_dir = project_dir.path().join("agents");
        std::fs::create_dir_all(&agents_dir).expect("create agents dir");

        std::fs::write(
            agents_dir.join("bad-agent.md"),
            "# Bad Agent\n\nDo not use unwrap() in production code.\n",
        )
        .expect("write agent file");

        std::fs::write(
            agents_dir.join("good-agent.md"),
            "# Good Agent\n\nThis agent is well-behaved.\n",
        )
        .expect("write agent file");

        write_rule_file(
            rules_dir.path(),
            "no-restating-rules",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: scan
    action: warn
    scope: "agents/*.md"
    conditions:
      - field: content
        pattern: "unwrap\\(\\)"
---
# No Restating Rules

Agent files should not restate rule content inline.
"#,
        );

        let engine = load_engine(rules_dir.path());
        let findings = engine
            .scan(project_dir.path())
            .expect("scan should succeed");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_name, "no-restating-rules");
        assert_eq!(findings[0].action, RuleAction::Warn);
        assert!(findings[0].file_path.contains("bad-agent.md"));
        assert_eq!(findings[0].line, 3);
        assert!(findings[0].content.contains("unwrap()"));
    }

    #[test]
    fn scan_returns_empty_when_no_scan_entries() {
        let rules_dir = tempfile::tempdir().expect("tempdir");
        let project_dir = tempfile::tempdir().expect("tempdir");

        write_rule_file(
            rules_dir.path(),
            "bash-only",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: bash
    action: block
    pattern: "--no-verify"
---
# Bash Only Rule
"#,
        );

        let engine = load_engine(rules_dir.path());
        let findings = engine
            .scan(project_dir.path())
            .expect("scan should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn scan_returns_empty_when_no_files_match_glob() {
        let rules_dir = tempfile::tempdir().expect("tempdir");
        let project_dir = tempfile::tempdir().expect("tempdir");

        write_rule_file(
            rules_dir.path(),
            "scan-rule",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: scan
    action: warn
    scope: "nonexistent-dir/*.md"
    conditions:
      - field: content
        pattern: "TODO"
---
# Scan Rule
"#,
        );

        let engine = load_engine(rules_dir.path());
        let findings = engine
            .scan(project_dir.path())
            .expect("scan should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn scan_reports_correct_line_numbers() {
        let rules_dir = tempfile::tempdir().expect("tempdir");
        let project_dir = tempfile::tempdir().expect("tempdir");

        std::fs::write(
            project_dir.path().join("file.md"),
            "line one\nline two\nTODO: fix this\nline four\n",
        )
        .expect("write file");

        write_rule_file(
            rules_dir.path(),
            "no-todo",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: scan
    action: warn
    scope: "*.md"
    conditions:
      - field: content
        pattern: "^TODO:"
---
# No TODO in Files
"#,
        );

        let engine = load_engine(rules_dir.path());
        let findings = engine
            .scan(project_dir.path())
            .expect("scan should succeed");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 3);
        assert_eq!(findings[0].content, "TODO: fix this");
    }

    #[test]
    fn scan_with_multiple_conditions_requires_all_to_match() {
        let rules_dir = tempfile::tempdir().expect("tempdir");
        let project_dir = tempfile::tempdir().expect("tempdir");

        std::fs::write(
            project_dir.path().join("agent.md"),
            "# Agent\nfoo bar\nbaz qux\nfoo qux\n",
        )
        .expect("write file");

        write_rule_file(
            rules_dir.path(),
            "multi-condition",
            r#"---
scope: project
enforcement:
  - mechanism: hook
    event: scan
    action: warn
    scope: "*.md"
    conditions:
      - field: content
        pattern: "foo"
      - field: content
        pattern: "qux"
---
# Multi Condition
"#,
        );

        let engine = load_engine(rules_dir.path());
        let findings = engine
            .scan(project_dir.path())
            .expect("scan should succeed");

        // Only "foo qux" (line 4) matches both conditions; "foo bar" and "baz qux" do not
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 4);
        assert_eq!(findings[0].content, "foo qux");
    }
}
