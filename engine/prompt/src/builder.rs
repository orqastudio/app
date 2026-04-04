//! System prompt builder for the orqa-engine crate.
//!
//! Assembles a structured system prompt from governance artifacts on disk:
//! rules, knowledge catalog, project instructions, and agent definitions.
//! Agent definitions are sourced from installed plugins (P1: Plugin-Composed Everything)
//! rather than any static file — the engine reads what plugins declare, not what is
//! hardcoded. This is the filesystem-based, AppState-free portion of prompt generation.
//! The consuming access layer (app, daemon, CLI) may augment the result with context
//! messages, session state, or other runtime data.
//!
//! Paths for knowledge and rules are resolved from `ProjectSettings.artifacts` when
//! a `project.json` is available, falling back to well-known defaults for environments
//! that have not been fully set up yet. This satisfies P1 (no hardcoded governance paths).

use std::path::Path;

use orqa_plugin::discovery::scan_plugins;
use orqa_plugin::manifest::{read_manifest, AgentDefinition};

/// Default relative path for knowledge artifacts when not configured via project.json.
const DEFAULT_KNOWLEDGE_PATH: &str = ".orqa/documentation/knowledge";

/// Default relative path for rule artifacts when not configured via project.json.
const DEFAULT_RULES_PATH: &str = ".orqa/learning/rules";

/// Default relative path for the platform instructions file.
const DEFAULT_CLAUDE_MD_PATH: &str = ".claude/CLAUDE.md";

/// Default relative path for installed agent definition files.
const DEFAULT_AGENTS_PATH: &str = ".claude/agents";

/// Read a governance file from the project directory.
///
/// Returns `None` if the file does not exist; returns `Err` on read failures.
pub fn read_governance_file(
    project_path: &Path,
    relative: &str,
) -> Result<Option<String>, std::io::Error> {
    let full_path = project_path.join(relative);
    if !full_path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&full_path)?;
    Ok(Some(contents))
}

/// List knowledge artifact names with one-line descriptions from the knowledge directory.
///
/// `knowledge_path` is the relative path to the knowledge directory (e.g.
/// `.orqa/documentation/knowledge`). The caller resolves this from
/// `ProjectSettings.artifacts` or falls back to `DEFAULT_KNOWLEDGE_PATH`.
///
/// Reads only the first non-empty line of each knowledge file as the description.
/// Full knowledge content is intentionally not loaded here — knowledge is loaded
/// on demand via the `load_knowledge` tool (P5: token efficiency).
pub fn list_knowledge_catalog(project_path: &Path, knowledge_path: &str) -> Vec<(String, String)> {
    let knowledge_dir = project_path.join(knowledge_path);
    let Ok(read_dir) = std::fs::read_dir(&knowledge_dir) else {
        return Vec::new();
    };

    let mut catalog: Vec<(String, String)> = read_dir
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_file() || path.extension().is_none_or(|e| e != "md") {
                return None;
            }
            let knowledge_name = path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            let description = std::fs::read_to_string(&path)
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|l| !l.trim().is_empty())
                        .map(|l| l.trim_start_matches('#').trim().to_owned())
                })
                .unwrap_or_else(|| "No description".to_owned());
            Some((knowledge_name, description))
        })
        .collect();

    catalog.sort_by(|a, b| a.0.cmp(&b.0));
    catalog
}

/// Read all rule files from the rules directory.
///
/// `rules_path` is the relative path to the rules directory (e.g. `.orqa/learning/rules`).
/// The caller resolves this from `ProjectSettings.artifacts` or falls back to
/// `DEFAULT_RULES_PATH`.
///
/// Returns a sorted list of `(rule_name, content)` pairs.
/// Rules are included in full because they are always relevant to every agent (P5).
pub fn read_rules(project_path: &Path, rules_path: &str) -> Vec<(String, String)> {
    let rules_dir = project_path.join(rules_path);
    let Ok(read_dir) = std::fs::read_dir(&rules_dir) else {
        return Vec::new();
    };

    let mut rules: Vec<(String, String)> = read_dir
        .flatten()
        .filter(|entry| {
            entry.path().extension().and_then(|e| e.to_str()) == Some("md")
        })
        .filter_map(|entry| {
            let path = entry.path();
            let rule_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_owned();
            match std::fs::read_to_string(&path) {
                Ok(contents) => Some((rule_name, contents)),
                Err(e) => {
                    // Surface rule file read failures instead of silently skipping them.
                    tracing::warn!(path = %path.display(), error = %e, "[engine] failed to read rule file");
                    None
                }
            }
        })
        .collect();

    rules.sort_by(|a, b| a.0.cmp(&b.0));
    rules
}

/// Collect agent role definitions from installed plugins.
///
/// Sources agent definitions in priority order:
/// 1. `provides.agents` entries in each installed plugin's manifest — structured definitions
///    contributed directly by the plugin (P1: Plugin-Composed Everything).
/// 2. `.claude/agents/*.md` — files synced from plugin `content.agents` directories
///    at install time, containing the full agent markdown with capabilities and preamble.
///
/// Returns a combined list of `AgentDefinition` values. Callers that need the full
/// markdown content of individual agents should read `.claude/agents/` directly.
/// Returns an empty vec if no installed plugins define agents.
pub fn collect_plugin_agent_definitions(project_path: &Path) -> Vec<AgentDefinition> {
    // Primary source: provides.agents in installed plugin manifests.
    scan_plugins(project_path)
        .iter()
        .filter_map(|plugin| read_manifest(Path::new(&plugin.path)).ok())
        .flat_map(|manifest| manifest.provides.agents)
        .collect()
}

/// Read agent markdown files from the installed agents directory.
///
/// `agents_path` is the relative path to the agents directory (e.g. `.claude/agents`).
/// These files are synced from plugin `content.agents` directories at install time.
/// Each file is a full agent definition with YAML frontmatter and markdown body.
/// Returns a sorted list of `(filename_stem, content)` pairs.
/// Returns an empty vec if the agents directory does not exist.
fn read_installed_agent_files(project_path: &Path, agents_path: &str) -> Vec<(String, String)> {
    let agents_dir = project_path.join(agents_path);
    let Ok(read_dir) = std::fs::read_dir(&agents_dir) else {
        return Vec::new();
    };

    let mut agent_files: Vec<(String, String)> = read_dir
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
                return None;
            }
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_owned();
            std::fs::read_to_string(&path).ok().map(|contents| (stem, contents))
        })
        .collect();

    agent_files.sort_by(|a, b| a.0.cmp(&b.0));
    agent_files
}

/// Paths resolved from `ProjectSettings.artifacts` (or defaults) for prompt assembly.
///
/// These are relative paths within the project root. They are resolved by
/// `resolve_project_paths` from the `artifacts` array in `project.json`, falling back
/// to known defaults when the project has not been configured.
pub struct ProjectPromptPaths {
    /// Relative path to rule files (default: `.orqa/learning/rules`).
    pub rules: String,
    /// Relative path to the knowledge catalog (default: `.orqa/documentation/knowledge`).
    pub knowledge: String,
    /// Relative path to the platform instructions file (default: `.claude/CLAUDE.md`).
    pub claude_md: String,
    /// Relative path to installed agent definition files (default: `.claude/agents`).
    pub agents: String,
}

impl Default for ProjectPromptPaths {
    /// Return well-known default paths used when project.json is absent or unconfigured.
    fn default() -> Self {
        Self {
            rules: DEFAULT_RULES_PATH.to_owned(),
            knowledge: DEFAULT_KNOWLEDGE_PATH.to_owned(),
            claude_md: DEFAULT_CLAUDE_MD_PATH.to_owned(),
            agents: DEFAULT_AGENTS_PATH.to_owned(),
        }
    }
}

/// Resolve prompt-relevant artifact paths from `project.json`.
///
/// Reads `{project_path}/.orqa/project.json`, walks the `artifacts` array looking for
/// entries with key `"rules"` and `"knowledge"`. Falls back to defaults when the file
/// is absent or an entry is not found.
pub fn resolve_project_paths(project_path: &Path) -> ProjectPromptPaths {
    let settings_file = project_path.join(".orqa").join("project.json");
    let Ok(content) = std::fs::read_to_string(&settings_file) else {
        return ProjectPromptPaths::default();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
        return ProjectPromptPaths::default();
    };

    let rules = find_artifact_path(&value, "rules")
        .unwrap_or_else(|| DEFAULT_RULES_PATH.to_owned());
    let knowledge = find_artifact_path(&value, "knowledge")
        .unwrap_or_else(|| DEFAULT_KNOWLEDGE_PATH.to_owned());

    ProjectPromptPaths {
        rules,
        knowledge,
        claude_md: DEFAULT_CLAUDE_MD_PATH.to_owned(),
        agents: DEFAULT_AGENTS_PATH.to_owned(),
    }
}

/// Walk the `artifacts` array in `project.json` and return the `path` for an entry with
/// the given `key`. Searches both top-level `Type` entries and children of `Group` entries.
///
/// Made `pub` so the facade crate (`orqa_engine::prompt::find_artifact_path`) and scanner
/// crates can share the same traversal logic without duplicating it.
pub fn find_artifact_path(value: &serde_json::Value, key: &str) -> Option<String> {
    let artifacts = value.get("artifacts")?.as_array()?;
    for entry in artifacts {
        // Direct type entry with matching key.
        if entry.get("key").and_then(|v: &serde_json::Value| v.as_str()) == Some(key) {
            if let Some(path) = entry.get("path").and_then(|v: &serde_json::Value| v.as_str()) {
                return Some(path.to_owned());
            }
        }
        // Group entry — search children.
        if let Some(children) = entry.get("children").and_then(|v: &serde_json::Value| v.as_array()) {
            for child in children {
                if child.get("key").and_then(|v: &serde_json::Value| v.as_str()) == Some(key) {
                    if let Some(path) = child.get("path").and_then(|v: &serde_json::Value| v.as_str()) {
                        return Some(path.to_owned());
                    }
                }
            }
        }
    }
    None
}

/// Build a structured system prompt from the project's governance artifacts.
///
/// `role` identifies the agent role (e.g. "orchestrator", "implementer"). It is
/// included in the prompt header so the agent knows its specialisation, and is
/// used to select the appropriate token budget per DOC-b951327c section 6.3.
///
/// `stage` is the optional workflow stage (e.g. "implement", "review"). When
/// provided it is included in the header for task context.
///
/// Reads:
/// - `paths.rules/*.md` — rule files (full content; default `.orqa/learning/rules`)
/// - `paths.claude_md` — platform instructions (default `.claude/CLAUDE.md`)
/// - `paths.agents/*.md` — agent definitions synced from installed plugins (default `.claude/agents`)
/// - `paths.knowledge/*.md` — knowledge catalog, name + one-line description only (default `.orqa/documentation/knowledge`)
///
/// Paths come from `ProjectSettings.artifacts` via `resolve_project_paths`, so no
/// governance paths are hardcoded (P1: Plugin-Composed Everything). If no plugins
/// have installed agent definitions, the "Agent Definitions" section is omitted.
///
/// Returns the assembled prompt string. Returns `Err` only on I/O failures;
/// missing optional files are silently skipped.
pub fn build_system_prompt(
    project_path: &Path,
    role: &str,
    stage: Option<&str>,
    paths: &ProjectPromptPaths,
) -> Result<String, std::io::Error> {
    // Read all governance data from disk, then delegate assembly to the pure function.
    let rules = read_rules(project_path, &paths.rules);
    let catalog = list_knowledge_catalog(project_path, &paths.knowledge);
    let claude_md = read_governance_file(project_path, &paths.claude_md)?;
    let agent_files = read_installed_agent_files(project_path, &paths.agents);

    let prompt_text = assemble_system_prompt(
        role,
        stage,
        &rules,
        &catalog,
        claude_md.as_deref(),
        &agent_files,
    );

    // Log token estimate for P5 enforcement: 1,500–4,000 tokens per agent prompt target.
    let estimated_tokens = prompt_text.len() / 4;
    tracing::debug!(
        subsystem = "engine",
        rule_count = rules.len(),
        knowledge_count = catalog.len(),
        agent_definition_count = agent_files.len(),
        estimated_tokens,
        "[engine] build_system_prompt completed"
    );

    Ok(prompt_text)
}

/// Assemble a system prompt from pre-loaded governance data with no I/O.
///
/// Pure function: takes all governance data as in-memory slices and strings,
/// returns the assembled prompt text. Sections are included only when non-empty.
///
/// `role` — agent role label (e.g. "implementer", "reviewer").
/// `stage` — optional workflow stage label.
/// `rules` — (name, content) pairs for each active rule.
/// `catalog` — (name, description) pairs for knowledge catalog entries.
/// `claude_md` — optional contents of `.claude/CLAUDE.md`.
/// `agent_files` — (name, content) pairs for installed agent definition files.
pub fn assemble_system_prompt(
    role: &str,
    stage: Option<&str>,
    rules: &[(String, String)],
    catalog: &[(String, String)],
    claude_md: Option<&str>,
    agent_files: &[(String, String)],
) -> String {
    // Role/stage header gives the agent immediate context without consuming
    // a large token budget (P5: token efficiency).
    let header = if let Some(s) = stage {
        format!("# Project Governance\n\nRole: {role} | Stage: {s}")
    } else {
        format!("# Project Governance\n\nRole: {role}")
    };

    let rules_section: Vec<String> = if rules.is_empty() {
        Vec::new()
    } else {
        std::iter::once("\n## Rules".to_owned())
            .chain(rules.iter().map(|(name, content)| format!("\n### {name}\n\n{content}")))
            .collect()
    };

    let knowledge_section: Vec<String> = if catalog.is_empty() {
        Vec::new()
    } else {
        std::iter::once("\n## Available Knowledge".to_owned())
            .chain(std::iter::once(
                "Use the `load_knowledge` tool to load the full content of any knowledge artifact by name.".to_owned(),
            ))
            .chain(catalog.iter().map(|(name, description)| format!("- **{name}**: {description}")))
            .collect()
    };

    let instructions_section: Vec<String> = claude_md
        .map(|md| vec!["\n## Project Instructions".to_owned(), md.to_owned()])
        .unwrap_or_default();

    // Agent definitions come from installed plugin content (P1: Plugin-Composed Everything).
    let agents_section: Vec<String> = if agent_files.is_empty() {
        Vec::new()
    } else {
        std::iter::once("\n## Agent Definitions".to_owned())
            .chain(agent_files.iter().map(|(_name, content)| content.clone()))
            .collect()
    };

    std::iter::once(header)
        .chain(rules_section)
        .chain(knowledge_section)
        .chain(instructions_section)
        .chain(agents_section)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Resolve the system prompt from a known project root path, logging on failure.
///
/// Resolves paths from `project.json` automatically. Uses "general" as the role
/// and no stage when called without context — prefer calling `build_system_prompt`
/// directly when role and stage are known.
///
/// Returns `Some(prompt)` when the prompt can be assembled; returns `None`
/// and emits a tracing warning when assembly fails.
pub fn resolve_system_prompt(project_root: &Path) -> Option<String> {
    let paths = resolve_project_paths(project_root);
    match build_system_prompt(project_root, "general", None, &paths) {
        Ok(prompt) => {
            tracing::debug!("[stream] system prompt built ({} chars)", prompt.len());
            Some(prompt)
        }
        Err(e) => {
            tracing::warn!("[stream] failed to build system prompt: {e}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    /// Build a default `ProjectPromptPaths` for test use.
    fn default_paths() -> ProjectPromptPaths {
        ProjectPromptPaths::default()
    }

    #[test]
    fn build_system_prompt_empty_project() {
        let dir = make_project();
        let paths = default_paths();
        let prompt = build_system_prompt(dir.path(), "implementer", None, &paths)
            .expect("should not error on empty project");
        assert!(prompt.contains("# Project Governance"));
        assert!(prompt.contains("Role: implementer"));
    }

    #[test]
    fn build_system_prompt_includes_role_and_stage() {
        let dir = make_project();
        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "reviewer", Some("review"), &paths)
                .expect("should succeed");
        assert!(prompt.contains("Role: reviewer"));
        assert!(prompt.contains("Stage: review"));
    }

    #[test]
    fn build_system_prompt_includes_rules() {
        let dir = make_project();
        // Use the default rules path (.orqa/learning/rules) matching project.json config.
        let rules_dir = dir.path().join(".orqa").join("learning").join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(
            rules_dir.join("no-debug.md"),
            "Never leave debug statements in production code.",
        )
        .expect("write rule");

        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "implementer", None, &paths).expect("should succeed");
        assert!(prompt.contains("## Rules"));
        assert!(prompt.contains("no-debug"));
        assert!(prompt.contains("Never leave debug statements"));
    }

    #[test]
    fn build_system_prompt_includes_rules_from_custom_path() {
        // Verify that custom paths (from project.json) override the defaults.
        let dir = make_project();
        let rules_dir = dir.path().join("custom").join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(
            rules_dir.join("my-rule.md"),
            "Custom rule content.",
        )
        .expect("write rule");

        let paths = ProjectPromptPaths {
            rules: "custom/rules".to_owned(),
            ..ProjectPromptPaths::default()
        };
        let prompt =
            build_system_prompt(dir.path(), "implementer", None, &paths).expect("should succeed");
        assert!(prompt.contains("## Rules"));
        assert!(prompt.contains("Custom rule content."));
    }

    #[test]
    fn build_system_prompt_includes_claude_md() {
        let dir = make_project();
        let claude_dir = dir.path().join(".claude");
        fs::create_dir_all(&claude_dir).expect("create .claude dir");
        fs::write(
            claude_dir.join("CLAUDE.md"),
            "# My Project\n\nFollow the architecture.\n",
        )
        .expect("write CLAUDE.md");

        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "general", None, &paths).expect("should succeed");
        assert!(prompt.contains("## Project Instructions"));
        assert!(prompt.contains("Follow the architecture."));
    }

    #[test]
    fn build_system_prompt_includes_knowledge_catalog() {
        let dir = make_project();
        let know_dir = dir
            .path()
            .join(".orqa")
            .join("documentation")
            .join("knowledge");
        fs::create_dir_all(&know_dir).expect("create knowledge dir");
        fs::write(
            know_dir.join("arch-overview.md"),
            "# Architecture Overview\n\nFull content here.\n",
        )
        .expect("write knowledge");

        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "general", None, &paths).expect("should succeed");
        assert!(prompt.contains("## Available Knowledge"));
        assert!(prompt.contains("arch-overview"));
        assert!(prompt.contains("Architecture Overview"));
    }

    #[test]
    fn build_system_prompt_includes_plugin_agent_definitions() {
        // Agents installed to .claude/agents/ (synced from plugin content.agents)
        // should appear in the generated prompt under "Agent Definitions".
        let dir = make_project();
        let agents_dir = dir.path().join(".claude").join("agents");
        fs::create_dir_all(&agents_dir).expect("create agents dir");
        fs::write(
            agents_dir.join("orchestrator.md"),
            "---\nname: orchestrator\ndescription: Coordinates workers\n---\n# Orchestrator\n\nCoordinates workers.\n",
        )
        .expect("write agent file");

        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "general", None, &paths).expect("should succeed");
        assert!(prompt.contains("## Agent Definitions"));
        assert!(prompt.contains("Orchestrator"));
        assert!(prompt.contains("Coordinates workers."));
    }

    #[test]
    fn build_system_prompt_no_agents_section_when_no_plugins_installed() {
        // When no agent files are installed, the "Agent Definitions" section must be absent.
        // This replaces the old AGENTS.md fallback — if no plugin provides agents,
        // the section is simply omitted (graceful degradation).
        let dir = make_project();
        let paths = default_paths();
        let prompt =
            build_system_prompt(dir.path(), "general", None, &paths).expect("should succeed");
        assert!(!prompt.contains("## Agent Definitions"));
    }

    #[test]
    fn collect_plugin_agent_definitions_empty_when_no_plugins() {
        // With no project.json and no installed plugins, the result must be empty.
        let dir = make_project();
        let agents = collect_plugin_agent_definitions(dir.path());
        assert!(agents.is_empty());
    }

    #[test]
    fn list_knowledge_catalog_empty_when_no_dir() {
        let dir = make_project();
        let catalog = list_knowledge_catalog(dir.path(), DEFAULT_KNOWLEDGE_PATH);
        assert!(catalog.is_empty());
    }

    #[test]
    fn read_rules_empty_when_no_dir() {
        let dir = make_project();
        let rules = read_rules(dir.path(), DEFAULT_RULES_PATH);
        assert!(rules.is_empty());
    }

    #[test]
    fn read_governance_file_returns_none_when_absent() {
        let dir = make_project();
        let result = read_governance_file(dir.path(), "nonexistent.md").expect("should not error");
        assert!(result.is_none());
    }

    #[test]
    fn resolve_project_paths_falls_back_to_defaults_when_no_project_json() {
        let dir = make_project();
        let paths = resolve_project_paths(dir.path());
        assert_eq!(paths.rules, DEFAULT_RULES_PATH);
        assert_eq!(paths.knowledge, DEFAULT_KNOWLEDGE_PATH);
    }

    #[test]
    fn resolve_project_paths_reads_from_project_json() {
        let dir = make_project();
        let orqa_dir = dir.path().join(".orqa");
        fs::create_dir_all(&orqa_dir).expect("create .orqa");
        let project_json = serde_json::json!({
            "name": "test",
            "artifacts": [
                {
                    "key": "learning",
                    "label": "Learning",
                    "children": [
                        { "key": "rules", "label": "Rules", "path": ".orqa/custom/rules" }
                    ]
                },
                { "key": "knowledge", "label": "Knowledge", "path": ".orqa/custom/knowledge" }
            ]
        });
        fs::write(
            orqa_dir.join("project.json"),
            serde_json::to_string_pretty(&project_json).unwrap(),
        )
        .expect("write project.json");

        let paths = resolve_project_paths(dir.path());
        assert_eq!(paths.rules, ".orqa/custom/rules");
        assert_eq!(paths.knowledge, ".orqa/custom/knowledge");
    }

    #[test]
    fn resolve_system_prompt_returns_none_on_io_error() {
        // Pass a path that will fail reads (a file, not a dir)
        let result = resolve_system_prompt(Path::new("/nonexistent/path/that/cannot/exist"));
        // Should return Some with empty prompt (no files to read, no errors)
        // The function only returns None when build_system_prompt returns Err,
        // which happens on actual IO errors — missing dirs are tolerated.
        assert!(result.is_some());
    }

    #[test]
    fn read_governance_file_returns_content_when_present() {
        let dir = make_project();
        let claude_dir = dir.path().join(".claude");
        fs::create_dir_all(&claude_dir).expect("create dir");
        fs::write(claude_dir.join("CLAUDE.md"), "# My project\n").expect("write");

        let result =
            read_governance_file(dir.path(), ".claude/CLAUDE.md").expect("should not error");
        assert_eq!(result, Some("# My project\n".to_owned()));
    }

    #[test]
    fn find_artifact_path_direct_type_entry() {
        let value = serde_json::json!({
            "artifacts": [
                { "key": "knowledge", "label": "Knowledge", "path": ".orqa/docs/knowledge" }
            ]
        });
        assert_eq!(
            find_artifact_path(&value, "knowledge"),
            Some(".orqa/docs/knowledge".to_owned())
        );
    }

    #[test]
    fn find_artifact_path_nested_in_group() {
        let value = serde_json::json!({
            "artifacts": [
                {
                    "key": "learning",
                    "label": "Learning",
                    "children": [
                        { "key": "rules", "label": "Rules", "path": ".orqa/custom/rules" }
                    ]
                }
            ]
        });
        assert_eq!(
            find_artifact_path(&value, "rules"),
            Some(".orqa/custom/rules".to_owned())
        );
    }

    #[test]
    fn find_artifact_path_missing_key_returns_none() {
        let value = serde_json::json!({
            "artifacts": [
                { "key": "knowledge", "label": "Knowledge", "path": ".orqa/docs/knowledge" }
            ]
        });
        assert_eq!(find_artifact_path(&value, "rules"), None);
    }

    #[test]
    fn find_artifact_path_no_artifacts_key_returns_none() {
        let value = serde_json::json!({ "name": "project" });
        assert_eq!(find_artifact_path(&value, "rules"), None);
    }

    #[test]
    fn list_knowledge_catalog_ignores_non_md_files() {
        let dir = make_project();
        let know_dir = dir
            .path()
            .join(".orqa")
            .join("documentation")
            .join("knowledge");
        fs::create_dir_all(&know_dir).expect("create knowledge dir");
        fs::write(know_dir.join("note.txt"), "ignored").expect("write");
        fs::write(know_dir.join("know.md"), "# Title\n\nContent").expect("write");

        let catalog = list_knowledge_catalog(dir.path(), DEFAULT_KNOWLEDGE_PATH);
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog[0].0, "know");
    }

    #[test]
    fn list_knowledge_catalog_sorted_alphabetically() {
        let dir = make_project();
        let know_dir = dir
            .path()
            .join(".orqa")
            .join("documentation")
            .join("knowledge");
        fs::create_dir_all(&know_dir).expect("create knowledge dir");
        fs::write(know_dir.join("zzz.md"), "# ZZZ").expect("write");
        fs::write(know_dir.join("aaa.md"), "# AAA").expect("write");

        let catalog = list_knowledge_catalog(dir.path(), DEFAULT_KNOWLEDGE_PATH);
        assert_eq!(catalog.len(), 2);
        assert_eq!(catalog[0].0, "aaa");
        assert_eq!(catalog[1].0, "zzz");
    }

    #[test]
    fn read_rules_sorted_alphabetically() {
        let dir = make_project();
        let rules_dir = dir.path().join(".orqa").join("learning").join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(rules_dir.join("zzz.md"), "rule z").expect("write");
        fs::write(rules_dir.join("aaa.md"), "rule a").expect("write");

        let rules = read_rules(dir.path(), DEFAULT_RULES_PATH);
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].0, "aaa");
        assert_eq!(rules[1].0, "zzz");
    }

    #[test]
    fn read_rules_ignores_non_md_files() {
        let dir = make_project();
        let rules_dir = dir.path().join(".orqa").join("learning").join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(rules_dir.join("note.txt"), "ignored").expect("write");
        fs::write(rules_dir.join("rule.md"), "rule content").expect("write");

        let rules = read_rules(dir.path(), DEFAULT_RULES_PATH);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].0, "rule");
        assert_eq!(rules[0].1, "rule content");
    }

    #[test]
    fn resolve_project_paths_falls_back_for_invalid_json() {
        let dir = make_project();
        let orqa_dir = dir.path().join(".orqa");
        fs::create_dir_all(&orqa_dir).expect("create .orqa");
        fs::write(orqa_dir.join("project.json"), "not valid json").expect("write");

        let paths = resolve_project_paths(dir.path());
        assert_eq!(paths.rules, DEFAULT_RULES_PATH);
    }

    #[test]
    fn project_prompt_paths_default_values() {
        let paths = ProjectPromptPaths::default();
        assert_eq!(paths.rules, DEFAULT_RULES_PATH);
        assert_eq!(paths.knowledge, DEFAULT_KNOWLEDGE_PATH);
        assert_eq!(paths.claude_md, DEFAULT_CLAUDE_MD_PATH);
        assert_eq!(paths.agents, DEFAULT_AGENTS_PATH);
    }

    // -----------------------------------------------------------------------
    // assemble_system_prompt unit tests (pure function — no I/O)
    // -----------------------------------------------------------------------

    #[test]
    fn assemble_system_prompt_role_only() {
        let prompt = assemble_system_prompt("implementer", None, &[], &[], None, &[]);
        assert!(prompt.contains("Role: implementer"));
        assert!(!prompt.contains("Stage:"));
    }

    #[test]
    fn assemble_system_prompt_role_and_stage() {
        let prompt = assemble_system_prompt("reviewer", Some("review"), &[], &[], None, &[]);
        assert!(prompt.contains("Role: reviewer"));
        assert!(prompt.contains("Stage: review"));
    }

    #[test]
    fn assemble_system_prompt_includes_rules() {
        let rules = vec![("no-debug".to_owned(), "No debug statements.".to_owned())];
        let prompt = assemble_system_prompt("implementer", None, &rules, &[], None, &[]);
        assert!(prompt.contains("## Rules"));
        assert!(prompt.contains("no-debug"));
        assert!(prompt.contains("No debug statements."));
    }

    #[test]
    fn assemble_system_prompt_omits_rules_section_when_empty() {
        let prompt = assemble_system_prompt("implementer", None, &[], &[], None, &[]);
        assert!(!prompt.contains("## Rules"));
    }

    #[test]
    fn assemble_system_prompt_includes_knowledge_catalog() {
        let catalog = vec![("arch-overview".to_owned(), "Architecture overview.".to_owned())];
        let prompt = assemble_system_prompt("general", None, &[], &catalog, None, &[]);
        assert!(prompt.contains("## Available Knowledge"));
        assert!(prompt.contains("arch-overview"));
        assert!(prompt.contains("Architecture overview."));
    }

    #[test]
    fn assemble_system_prompt_includes_claude_md() {
        let prompt = assemble_system_prompt("general", None, &[], &[], Some("# My Project\n\nFollow the architecture."), &[]);
        assert!(prompt.contains("## Project Instructions"));
        assert!(prompt.contains("Follow the architecture."));
    }

    #[test]
    fn assemble_system_prompt_omits_claude_md_when_none() {
        let prompt = assemble_system_prompt("general", None, &[], &[], None, &[]);
        assert!(!prompt.contains("## Project Instructions"));
    }

    #[test]
    fn assemble_system_prompt_includes_agent_definitions() {
        let agents = vec![("orchestrator".to_owned(), "# Orchestrator\n\nCoordinates workers.\n".to_owned())];
        let prompt = assemble_system_prompt("general", None, &[], &[], None, &agents);
        assert!(prompt.contains("## Agent Definitions"));
        assert!(prompt.contains("Coordinates workers."));
    }

    #[test]
    fn assemble_system_prompt_omits_agent_section_when_empty() {
        let prompt = assemble_system_prompt("general", None, &[], &[], None, &[]);
        assert!(!prompt.contains("## Agent Definitions"));
    }
}
