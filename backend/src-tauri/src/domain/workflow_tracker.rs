use std::collections::HashSet;

/// Tracks session-level events for process gate evaluation.
///
/// Each session gets a fresh tracker. Events accumulate over the session lifetime.
/// The tracker is used by `process_gates` to decide which thinking prompts to inject.
#[derive(Debug, Default)]
pub struct WorkflowTracker {
    /// All files read during this session (raw paths).
    files_read: Vec<String>,
    /// All files written during this session (raw paths).
    files_written: Vec<String>,
    /// Number of search tool calls made (search_regex / search_semantic / code_research).
    searches_performed: u32,
    /// Files read from `.orqa/documentation/`.
    docs_consulted: Vec<String>,
    /// Files read from `.orqa/planning/`.
    planning_consulted: Vec<String>,
    /// Skills loaded via `load_skill` during this session.
    skills_loaded: HashSet<String>,
    /// Bash commands run during this session.
    commands_run: Vec<String>,
    /// True after any `make check` or `make test` command is detected.
    verification_run: bool,
    /// True after any read of `.orqa/governance/lessons/`.
    lessons_checked: bool,
    /// Deduplication set for skill injection — prevents injecting the same skill twice.
    injected_skills: HashSet<String>,
    /// True after the first code-write gate fires, so it only fires once per session.
    pub first_code_write_gated: bool,
}

impl WorkflowTracker {
    /// Create a fresh tracker for a new session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a file read and auto-categorize it.
    ///
    /// - Paths containing `.orqa/documentation/` are added to `docs_consulted`.
    /// - Paths containing `.orqa/planning/` are added to `planning_consulted`.
    /// - Paths containing `.orqa/governance/lessons/` set `lessons_checked`.
    pub fn record_read(&mut self, path: &str) {
        self.files_read.push(path.to_string());

        if path.contains(".orqa/documentation/") {
            self.docs_consulted.push(path.to_string());
        }
        if path.contains(".orqa/planning/") {
            self.planning_consulted.push(path.to_string());
        }
        if path.contains(".orqa/governance/lessons/") {
            self.lessons_checked = true;
        }
    }

    /// Record a file write.
    pub fn record_write(&mut self, path: &str) {
        self.files_written.push(path.to_string());
    }

    /// Record a search tool call (regex, semantic, or code research).
    pub fn record_search(&mut self) {
        self.searches_performed += 1;
    }

    /// Record a skill being loaded via `load_skill`.
    pub fn record_skill_loaded(&mut self, name: &str) {
        self.skills_loaded.insert(name.to_string());
    }

    /// Record a bash command.
    ///
    /// Detects `make check` and `make test` variants to set `verification_run`.
    pub fn record_command(&mut self, cmd: &str) {
        self.commands_run.push(cmd.to_string());

        // Detect verification commands (make check, make test, make test-rust, etc.)
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

    /// Mark a skill as injected.
    ///
    /// Returns `true` if this is the first time this skill has been injected
    /// in this session (i.e. actually newly injected), `false` if already done.
    pub fn mark_skill_injected(&mut self, name: &str) -> bool {
        self.injected_skills.insert(name.to_string())
    }

    /// True if any file in `.orqa/documentation/` has been read this session.
    pub fn has_read_any_docs(&self) -> bool {
        !self.docs_consulted.is_empty()
    }

    /// True if any file in `.orqa/planning/` has been read this session.
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

    /// True if a verification command (`make check` / `make test`) was run this session.
    pub fn has_run_verification(&self) -> bool {
        self.verification_run
    }

    /// True if `.orqa/governance/lessons/` was read this session.
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

    // ── record_read ──

    #[test]
    fn record_read_non_orqa_file_does_not_affect_categories() {
        let mut t = WorkflowTracker::new();
        t.record_read("src-tauri/src/main.rs");
        assert!(!t.has_read_any_docs());
        assert!(!t.has_read_any_planning());
        assert!(!t.has_checked_lessons());
    }

    #[test]
    fn record_read_docs_path_sets_docs_consulted() {
        let mut t = WorkflowTracker::new();
        t.record_read(".orqa/documentation/architecture/overview.md");
        assert!(t.has_read_any_docs());
        assert!(!t.has_read_any_planning());
    }

    #[test]
    fn record_read_planning_path_sets_planning_consulted() {
        let mut t = WorkflowTracker::new();
        t.record_read(".orqa/planning/epics/EPIC-042.md");
        assert!(t.has_read_any_planning());
        assert!(!t.has_read_any_docs());
    }

    #[test]
    fn record_read_lessons_path_sets_lessons_checked() {
        let mut t = WorkflowTracker::new();
        t.record_read(".orqa/governance/lessons/IMPL-001.md");
        assert!(t.has_checked_lessons());
    }

    #[test]
    fn record_read_accumulates_all_file_reads() {
        let mut t = WorkflowTracker::new();
        t.record_read("file1.rs");
        t.record_read("file2.ts");
        assert_eq!(t.files_read.len(), 2);
    }

    // ── record_write ──

    #[test]
    fn record_write_orqa_file_is_not_code_write() {
        let mut t = WorkflowTracker::new();
        t.record_write(".orqa/governance/rules/RULE-042.md");
        assert!(!t.has_written_code());
        assert_eq!(t.code_write_count(), 0);
    }

    #[test]
    fn record_write_src_file_is_code_write() {
        let mut t = WorkflowTracker::new();
        t.record_write("src-tauri/src/domain/foo.rs");
        assert!(t.has_written_code());
        assert_eq!(t.code_write_count(), 1);
    }

    #[test]
    fn record_write_ui_file_is_code_write() {
        let mut t = WorkflowTracker::new();
        t.record_write("ui/lib/stores/navigation.svelte.ts");
        assert!(t.has_written_code());
        assert_eq!(t.code_write_count(), 1);
    }

    #[test]
    fn code_write_count_counts_only_non_orqa_writes() {
        let mut t = WorkflowTracker::new();
        t.record_write(".orqa/planning/tasks/TASK-001.md");
        t.record_write("src-tauri/src/main.rs");
        t.record_write("ui/App.svelte");
        assert_eq!(t.code_write_count(), 2);
    }

    // ── record_search ──

    #[test]
    fn has_searched_false_before_any_search() {
        let t = WorkflowTracker::new();
        assert!(!t.has_searched());
    }

    #[test]
    fn has_searched_true_after_record_search() {
        let mut t = WorkflowTracker::new();
        t.record_search();
        assert!(t.has_searched());
    }

    #[test]
    fn searches_accumulate() {
        let mut t = WorkflowTracker::new();
        t.record_search();
        t.record_search();
        t.record_search();
        assert_eq!(t.searches_performed, 3);
    }

    // ── record_skill_loaded ──

    #[test]
    fn record_skill_loaded_deduplicates() {
        let mut t = WorkflowTracker::new();
        t.record_skill_loaded("rust-async-patterns");
        t.record_skill_loaded("rust-async-patterns");
        assert_eq!(t.skills_loaded.len(), 1);
    }

    #[test]
    fn record_skill_loaded_tracks_multiple_skills() {
        let mut t = WorkflowTracker::new();
        t.record_skill_loaded("rust-async-patterns");
        t.record_skill_loaded("tauri-v2");
        assert_eq!(t.skills_loaded.len(), 2);
    }

    // ── record_command ──

    #[test]
    fn record_command_make_check_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("make check");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_make_test_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("make test");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_make_test_rust_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("make test-rust");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_cargo_test_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("cargo test --manifest-path src-tauri/Cargo.toml");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_cargo_clippy_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_npm_run_check_sets_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("npm run check");
        assert!(t.has_run_verification());
    }

    #[test]
    fn record_command_ls_does_not_set_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("ls -la");
        assert!(!t.has_run_verification());
    }

    #[test]
    fn record_command_git_commit_does_not_set_verification_run() {
        let mut t = WorkflowTracker::new();
        t.record_command("git commit -m 'add feature'");
        assert!(!t.has_run_verification());
    }

    // ── mark_skill_injected ──

    #[test]
    fn mark_skill_injected_returns_true_first_time() {
        let mut t = WorkflowTracker::new();
        assert!(t.mark_skill_injected("rust-async-patterns"));
    }

    #[test]
    fn mark_skill_injected_returns_false_second_time() {
        let mut t = WorkflowTracker::new();
        t.mark_skill_injected("rust-async-patterns");
        assert!(!t.mark_skill_injected("rust-async-patterns"));
    }

    #[test]
    fn mark_skill_injected_different_skills_both_true() {
        let mut t = WorkflowTracker::new();
        assert!(t.mark_skill_injected("tauri-v2"));
        assert!(t.mark_skill_injected("svelte5-best-practices"));
    }

    // ── has_done_any_research ──

    #[test]
    fn has_done_any_research_false_initially() {
        let t = WorkflowTracker::new();
        assert!(!t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_docs_read() {
        let mut t = WorkflowTracker::new();
        t.record_read(".orqa/documentation/product/vision.md");
        assert!(t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_searched() {
        let mut t = WorkflowTracker::new();
        t.record_search();
        assert!(t.has_done_any_research());
    }

    #[test]
    fn has_done_any_research_true_when_planning_read() {
        let mut t = WorkflowTracker::new();
        t.record_read(".orqa/planning/tasks/TASK-001.md");
        assert!(t.has_done_any_research());
    }

    // ── absolute path handling ──

    #[test]
    fn absolute_path_with_orqa_docs_detected_as_docs() {
        let mut t = WorkflowTracker::new();
        // Simulate an absolute path (e.g. on Windows)
        t.record_read("C:/Users/user/code/project/.orqa/documentation/dev/standards.md");
        assert!(t.has_read_any_docs());
    }

    #[test]
    fn absolute_path_with_orqa_planning_detected_as_planning() {
        let mut t = WorkflowTracker::new();
        t.record_read("/home/user/project/.orqa/planning/epics/EPIC-001.md");
        assert!(t.has_read_any_planning());
    }

    #[test]
    fn orqa_write_via_absolute_path_is_not_code() {
        let mut t = WorkflowTracker::new();
        t.record_write("/home/user/project/.orqa/governance/rules/RULE-042.md");
        assert!(!t.has_written_code());
    }
}
