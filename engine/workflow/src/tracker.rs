//! Workflow tracker for the OrqaStudio workflow engine.
//!
//! `WorkflowTracker` accumulates session-level events for process gate evaluation.
//! Each session gets a fresh tracker. Events accumulate over the session lifetime
//! and are used by the process gate evaluator to decide which thinking prompts
//! to inject into the agent's context.
//!
//! Path pattern classification (docs, planning, lessons) is driven by `TrackerConfig`.
//! No paths are hardcoded in the tracker itself.

use std::collections::HashSet;

/// Category a file path can be classified into for gate evaluation purposes.
///
/// Each category is tracked separately so that gates can fire based on which
/// types of context have been consulted during a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathCategory {
    /// Documentation artifacts — satisfies the docs-before-code gate.
    Docs,
    /// Planning/implementation artifacts — satisfies the plan-before-build gate.
    Planning,
    /// Lesson artifacts — satisfies the learn-after-doing gate.
    Lessons,
}

/// A single path classification rule mapping a path substring to a category.
///
/// When a file path contains `pattern` as a substring, the path is classified
/// into `category`.
#[derive(Debug, Clone)]
pub struct PathRule {
    /// Substring to match anywhere in the file path.
    pub pattern: String,
    /// Category to apply when the pattern matches.
    pub category: PathCategory,
}

impl PathRule {
    /// Create a new path classification rule.
    pub fn new(pattern: impl Into<String>, category: PathCategory) -> Self {
        Self {
            pattern: pattern.into(),
            category,
        }
    }
}

/// Configuration for the `WorkflowTracker`.
///
/// Holds the path classification rules that determine how file reads are
/// categorised for gate evaluation. This config is provided by the caller
/// (loaded from the resolved workflow or plugin config) so the tracker
/// contains no hardcoded path knowledge.
#[derive(Debug, Clone, Default)]
pub struct TrackerConfig {
    /// Rules applied in order to each file path recorded via `record_read`.
    /// A path may match multiple rules and be classified into multiple categories.
    pub path_rules: Vec<PathRule>,
}

impl TrackerConfig {
    /// Create a config with the given path classification rules.
    pub fn new(path_rules: Vec<PathRule>) -> Self {
        Self { path_rules }
    }

    /// Classify a file path against all rules and return the matching categories.
    ///
    /// A path may match multiple rules. The returned set contains every category
    /// whose rule's pattern appears anywhere in the path.
    pub fn classify(&self, path: &str) -> Vec<PathCategory> {
        self.path_rules
            .iter()
            .filter(|r| path.contains(r.pattern.as_str()))
            .map(|r| r.category.clone())
            .collect()
    }
}

/// Tracks session-level events for process gate evaluation.
///
/// Each session gets a fresh tracker. Events accumulate over the session lifetime.
/// The tracker is used by `process_gates` to decide which thinking prompts to inject.
/// Path classification is driven by the `TrackerConfig` provided at construction time.
#[derive(Debug, Default)]
pub struct WorkflowTracker {
    /// Configuration holding the path classification rules.
    config: TrackerConfig,
    /// All files read during this session (raw paths).
    files_read: Vec<String>,
    /// All files written during this session (raw paths).
    files_written: Vec<String>,
    /// Number of search tool calls made (search_regex / search_semantic / code_research).
    searches_performed: u32,
    /// Files classified as documentation this session.
    docs_consulted: Vec<String>,
    /// Files classified as planning artifacts this session.
    planning_consulted: Vec<String>,
    /// Knowledge loaded via `load_knowledge` during this session.
    knowledge_loaded: HashSet<String>,
    /// Bash commands run during this session.
    commands_run: Vec<String>,
    /// True after any verification command is detected.
    verification_run: bool,
    /// True after any lesson artifact has been read this session.
    lessons_checked: bool,
    /// Deduplication set for knowledge injection — prevents injecting the same knowledge twice.
    injected_knowledge: HashSet<String>,
    /// True after the first code-write gate fires, so it only fires once per session.
    pub first_code_write_gated: bool,
}

impl WorkflowTracker {
    /// Create a fresh tracker with path classification driven by `config`.
    pub fn new(config: TrackerConfig) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// Record a file read and classify it using the tracker config.
    ///
    /// Each matching path rule category is applied independently:
    /// - `Docs` paths are added to `docs_consulted`.
    /// - `Planning` paths are added to `planning_consulted`.
    /// - `Lessons` paths set `lessons_checked`.
    pub fn record_read(&mut self, path: &str) {
        self.files_read.push(path.to_owned());

        for category in self.config.classify(path) {
            match category {
                PathCategory::Docs => {
                    self.docs_consulted.push(path.to_owned());
                }
                PathCategory::Planning => {
                    self.planning_consulted.push(path.to_owned());
                }
                PathCategory::Lessons => {
                    self.lessons_checked = true;
                }
            }
        }
    }

    /// Record a file write.
    pub fn record_write(&mut self, path: &str) {
        self.files_written.push(path.to_owned());
    }

    /// Record a search tool call (regex, semantic, or code research).
    pub fn record_search(&mut self) {
        self.searches_performed += 1;
    }

    /// Record a knowledge artifact being loaded via `load_knowledge`.
    pub fn record_knowledge_loaded(&mut self, name: &str) {
        self.knowledge_loaded.insert(name.to_owned());
    }

    /// Record a bash command.
    ///
    /// Detects verification commands to set `verification_run`.
    pub fn record_command(&mut self, cmd: &str) {
        self.commands_run.push(cmd.to_owned());

        let lower = cmd.to_lowercase();
        if lower.contains("make check")
            || lower.contains("make test")
            || lower.contains("cargo test")
            || lower.contains("cargo clippy")
            || lower.contains("npm run test")
            || lower.contains("npm run check")
        {
            self.verification_run = true;
        }
    }

    /// Mark a knowledge artifact as injected.
    ///
    /// Returns `true` if this is the first time this knowledge has been injected
    /// in this session (i.e. actually newly injected), `false` if already done.
    pub fn mark_knowledge_injected(&mut self, name: &str) -> bool {
        self.injected_knowledge.insert(name.to_owned())
    }

    /// True if any documentation file has been read this session.
    pub fn has_read_any_docs(&self) -> bool {
        !self.docs_consulted.is_empty()
    }

    /// True if any planning artifact has been read this session.
    pub fn has_read_any_planning(&self) -> bool {
        !self.planning_consulted.is_empty()
    }

    /// True if any search tool has been called this session.
    pub fn has_searched(&self) -> bool {
        self.searches_performed > 0
    }

    /// True if any research activity has been performed this session.
    ///
    /// Research counts as: reading docs, reading planning artifacts, or using search.
    pub fn has_done_any_research(&self) -> bool {
        self.has_read_any_docs() || self.has_searched() || self.has_read_any_planning()
    }

    /// True if a verification command was run this session.
    pub fn has_run_verification(&self) -> bool {
        self.verification_run
    }

    /// True if a lesson artifact was read this session.
    pub fn has_checked_lessons(&self) -> bool {
        self.lessons_checked
    }

    /// True if any non-`.orqa/` file has been written this session.
    ///
    /// Writes to `.orqa/` governance artifacts are not considered "code writes".
    pub fn has_written_code(&self) -> bool {
        self.files_written
            .iter()
            .any(|p| !p.starts_with(".orqa/") && !p.contains("/.orqa/"))
    }

    /// Returns the number of non-`.orqa/` file writes this session.
    pub fn code_write_count(&self) -> usize {
        self.files_written
            .iter()
            .filter(|p| !p.starts_with(".orqa/") && !p.contains("/.orqa/"))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a tracker with the standard path rules used in tests.
    fn tracker_with_standard_config() -> WorkflowTracker {
        WorkflowTracker::new(standard_config())
    }

    /// Standard path classification rules matching the original hardcoded paths.
    fn standard_config() -> TrackerConfig {
        TrackerConfig::new(vec![
            PathRule::new(".orqa/documentation/", PathCategory::Docs),
            PathRule::new(".orqa/implementation/", PathCategory::Planning),
            PathRule::new(".orqa/learning/lessons/", PathCategory::Lessons),
        ])
    }

    // ── record_read ──

    #[test]
    fn record_read_non_orqa_file_does_not_affect_categories() {
        let mut t = tracker_with_standard_config();
        t.record_read("src-tauri/src/main.rs");
        assert!(!t.has_read_any_docs());
        assert!(!t.has_read_any_planning());
        assert!(!t.has_checked_lessons());
    }

    #[test]
    fn record_read_docs_path_sets_docs_consulted() {
        let mut t = tracker_with_standard_config();
        t.record_read(".orqa/documentation/architecture/overview.md");
        assert!(t.has_read_any_docs());
        assert!(!t.has_read_any_planning());
    }

    #[test]
    fn record_read_planning_path_sets_planning_consulted() {
        let mut t = tracker_with_standard_config();
        t.record_read(".orqa/implementation/epics/EPIC-042.md");
        assert!(t.has_read_any_planning());
        assert!(!t.has_read_any_docs());
    }

    #[test]
    fn record_read_lessons_path_sets_lessons_checked() {
        let mut t = tracker_with_standard_config();
        t.record_read(".orqa/learning/lessons/IMPL-001.md");
        assert!(t.has_checked_lessons());
    }

    #[test]
    fn record_read_accumulates_all_file_reads() {
        let mut t = tracker_with_standard_config();
        t.record_read("file1.rs");
        t.record_read("file2.ts");
        assert_eq!(t.files_read.len(), 2);
    }

    // ── record_write ──

    #[test]
    fn record_write_orqa_file_is_not_code_write() {
        let mut t = tracker_with_standard_config();
        t.record_write(".orqa/learning/rules/RULE-042.md");
        assert!(!t.has_written_code());
        assert_eq!(t.code_write_count(), 0);
    }

    #[test]
    fn record_write_src_file_is_code_write() {
        let mut t = tracker_with_standard_config();
        t.record_write("src-tauri/src/domain/foo.rs");
        assert!(t.has_written_code());
        assert_eq!(t.code_write_count(), 1);
    }

    #[test]
    fn record_write_ui_file_is_code_write() {
        let mut t = tracker_with_standard_config();
        t.record_write("ui/lib/stores/navigation.svelte.ts");
        assert!(t.has_written_code());
        assert_eq!(t.code_write_count(), 1);
    }

    #[test]
    fn code_write_count_counts_only_non_orqa_writes() {
        let mut t = tracker_with_standard_config();
        t.record_write(".orqa/implementation/tasks/TASK-001.md");
        t.record_write("src-tauri/src/main.rs");
        t.record_write("ui/App.svelte");
        assert_eq!(t.code_write_count(), 2);
    }

    // ── record_search ──

    #[test]
    fn has_searched_false_before_any_search() {
        let t = tracker_with_standard_config();
        assert!(!t.has_searched());
    }

    #[test]
    fn has_searched_true_after_record_search() {
        let mut t = tracker_with_standard_config();
        t.record_search();
        assert!(t.has_searched());
    }

    #[test]
    fn searches_accumulate() {
        let mut t = tracker_with_standard_config();
        t.record_search();
        t.record_search();
        t.record_search();
        assert_eq!(t.searches_performed, 3);
    }

    // ── record_knowledge_loaded ──

    #[test]
    fn record_knowledge_loaded_deduplicates() {
        let mut t = tracker_with_standard_config();
        t.record_knowledge_loaded("rust-async-patterns");
        t.record_knowledge_loaded("rust-async-patterns");
        assert_eq!(t.knowledge_loaded.len(), 1);
    }

    #[test]
    fn record_knowledge_loaded_tracks_multiple_items() {
        let mut t = tracker_with_standard_config();
        t.record_knowledge_loaded("rust-async-patterns");
        t.record_knowledge_loaded("tauri-v2");
        assert_eq!(t.knowledge_loaded.len(), 2);
    }

    // ── record_command ──

    #[test]
    fn record_command_make_check_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("make check");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_make_test_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("make test");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_make_test_rust_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("make test-rust");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_cargo_test_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("cargo test --manifest-path src-tauri/Cargo.toml");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_cargo_clippy_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_npm_run_check_sets_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("npm run check");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_ls_does_not_set_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("ls -la");
        assert!(!t.has_run_verification());
    }

    #[test]
    fn record_command_git_commit_does_not_set_verification_run() {
        let mut t = tracker_with_standard_config();
        t.record_command("git commit -m 'add feature'");
        assert!(!t.has_run_verification());
    }

    // ── mark_knowledge_injected ──

    #[test]
    fn mark_knowledge_injected_returns_true_first_time() {
        let mut t = tracker_with_standard_config();
        assert!(t.mark_knowledge_injected("rust-async-patterns"));
    }

    #[test]
    fn mark_knowledge_injected_returns_false_second_time() {
        let mut t = tracker_with_standard_config();
        t.mark_knowledge_injected("rust-async-patterns");
        assert!(!t.mark_knowledge_injected("rust-async-patterns"));
    }

    #[test]
    fn mark_knowledge_injected_different_items_both_true() {
        let mut t = tracker_with_standard_config();
        assert!(t.mark_knowledge_injected("tauri-v2"));
        assert!(t.mark_knowledge_injected("svelte5-best-practices"));
    }

    // ── has_done_any_research ──

    #[test]
    fn has_done_any_research_false_initially() {
        let t = tracker_with_standard_config();
        assert!(!t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_docs_read() {
        let mut t = tracker_with_standard_config();
        t.record_read(".orqa/documentation/product/vision.md");
        assert!(t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_searched() {
        let mut t = tracker_with_standard_config();
        t.record_search();
        assert!(t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_planning_read() {
        let mut t = tracker_with_standard_config();
        t.record_read(".orqa/implementation/tasks/TASK-001.md");
        assert!(t.has_done_any_research());
    }

    // ── absolute path handling ──

    #[test]
    fn absolute_path_with_orqa_docs_detected_as_docs() {
        let mut t = tracker_with_standard_config();
        t.record_read("C:/Users/user/code/project/.orqa/documentation/dev/standards.md");
        assert!(t.has_read_any_docs());
    }

    #[test]
    fn absolute_path_with_orqa_planning_detected_as_planning() {
        let mut t = tracker_with_standard_config();
        t.record_read("/home/user/project/.orqa/implementation/epics/EPIC-001.md");
        assert!(t.has_read_any_planning());
    }

    #[test]
    fn orqa_write_via_absolute_path_is_not_code() {
        let mut t = tracker_with_standard_config();
        t.record_write("/home/user/project/.orqa/learning/rules/RULE-042.md");
        assert!(!t.has_written_code());
    }

    // ── custom config ──

    #[test]
    fn custom_config_classifies_custom_paths() {
        let config = TrackerConfig::new(vec![
            PathRule::new("custom/docs/", PathCategory::Docs),
            PathRule::new("custom/plans/", PathCategory::Planning),
            PathRule::new("custom/retrospectives/", PathCategory::Lessons),
        ]);
        let mut t = WorkflowTracker::new(config);

        t.record_read("custom/docs/architecture.md");
        assert!(t.has_read_any_docs());

        t.record_read("custom/plans/epic-001.md");
        assert!(t.has_read_any_planning());

        t.record_read("custom/retrospectives/retro-2026-01.md");
        assert!(t.has_checked_lessons());
    }

    #[test]
    fn empty_config_classifies_nothing() {
        let mut t = WorkflowTracker::new(TrackerConfig::default());
        t.record_read(".orqa/documentation/overview.md");
        t.record_read(".orqa/implementation/tasks/TASK-001.md");
        t.record_read(".orqa/learning/lessons/IMPL-001.md");

        assert!(!t.has_read_any_docs());
        assert!(!t.has_read_any_planning());
        assert!(!t.has_checked_lessons());
    }

    #[test]
    fn path_can_match_multiple_categories() {
        // A path that contains both patterns should be classified into both categories.
        let config = TrackerConfig::new(vec![
            PathRule::new("shared/", PathCategory::Docs),
            PathRule::new("shared/", PathCategory::Planning),
        ]);
        let mut t = WorkflowTracker::new(config);
        t.record_read("shared/notes.md");
        assert!(t.has_read_any_docs());
        assert!(t.has_read_any_planning());
    }
}
