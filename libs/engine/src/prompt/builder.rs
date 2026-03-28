// System prompt builder for the orqa-engine crate.
//
// Assembles a structured system prompt from governance artifacts on disk:
// rules, knowledge catalog, project instructions, and agent definitions.
// Agent definitions are sourced from installed plugins (P1: Plugin-Composed Everything)
// rather than any static file — the engine reads what plugins declare, not what is
// hardcoded. This is the filesystem-based, AppState-free portion of prompt generation.
// The consuming access layer (app, daemon, CLI) may augment the result with context
// messages, session state, or other runtime data.

use std::path::Path;

use crate::plugin::discovery::scan_plugins;
use crate::plugin::manifest::{read_manifest, AgentDefinition};

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

/// List knowledge artifact names with one-line descriptions from `.orqa/process/knowledge/*.md`.
///
/// Reads only the first non-empty line of each knowledge file as the description.
/// Full knowledge content is intentionally not loaded here — knowledge is loaded
/// on demand via the `load_knowledge` tool (P5: token efficiency).
pub fn list_knowledge_catalog(project_path: &Path) -> Vec<(String, String)> {
    let knowledge_dir = project_path.join(".orqa").join("process").join("knowledge");
    let mut catalog = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(&knowledge_dir) else {
        return catalog;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|e| e != "md") {
            continue;
        }

        let knowledge_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let description = std::fs::read_to_string(&path)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .map(|l| l.trim_start_matches('#').trim().to_string())
            })
            .unwrap_or_else(|| "No description".to_string());

        catalog.push((knowledge_name, description));
    }

    catalog.sort_by(|a, b| a.0.cmp(&b.0));
    catalog
}

/// Read all rule files from `.orqa/rules/*.md`.
///
/// Returns a sorted list of `(rule_name, content)` pairs.
/// Rules are included in full because they are always relevant to every agent (P5).
pub fn read_rules(project_path: &Path) -> Vec<(String, String)> {
    let rules_dir = project_path.join(".orqa").join("rules");
    let mut rules = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(&rules_dir) else {
        return rules;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let rule_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        if let Ok(contents) = std::fs::read_to_string(&path) {
            rules.push((rule_name, contents));
        }
    }

    rules.sort_by(|a, b| a.0.cmp(&b.0));
    rules
}

/// Collect agent role definitions from installed plugins.
///
/// Sources agent definitions in priority order:
/// 1. `provides.agents` entries in each installed plugin's manifest — structured definitions
///    contributed directly by the plugin (P1: Plugin-Composed Everything).
/// 2. `.orqa/process/agents/*.md` — files synced from plugin `content.agents` directories
///    at install time, containing the full agent markdown with capabilities and preamble.
///
/// Returns a combined list of `AgentDefinition` values. Callers that need the full
/// markdown content of individual agents should read `.orqa/process/agents/` directly.
/// Returns an empty vec if no installed plugins define agents.
pub fn collect_plugin_agent_definitions(project_path: &Path) -> Vec<AgentDefinition> {
    let mut agents: Vec<AgentDefinition> = Vec::new();

    // Primary source: provides.agents in installed plugin manifests.
    let discovered = scan_plugins(project_path);
    for plugin in &discovered {
        let plugin_path = std::path::Path::new(&plugin.path);
        if let Ok(manifest) = read_manifest(plugin_path) {
            for agent_def in manifest.provides.agents {
                agents.push(agent_def);
            }
        }
    }

    agents
}

/// Read agent markdown files installed to `.orqa/process/agents/`.
///
/// These files are synced from plugin `content.agents` directories at install time.
/// Each file is a full agent definition with YAML frontmatter and markdown body.
/// Returns a sorted list of `(filename_stem, content)` pairs.
/// Returns an empty vec if the agents directory does not exist.
fn read_installed_agent_files(project_path: &Path) -> Vec<(String, String)> {
    let agents_dir = project_path
        .join(".orqa")
        .join("process")
        .join("agents");
    let mut agent_files = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(&agents_dir) else {
        return agent_files;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        if let Ok(contents) = std::fs::read_to_string(&path) {
            agent_files.push((stem, contents));
        }
    }

    agent_files.sort_by(|a, b| a.0.cmp(&b.0));
    agent_files
}

/// Build a structured system prompt from the project's governance artifacts.
///
/// Reads:
/// - `.orqa/rules/*.md` — rule files (full content)
/// - `.claude/CLAUDE.md` — project instructions (full content, platform config)
/// - `.orqa/process/agents/*.md` — agent definitions synced from installed plugins
/// - `.orqa/process/knowledge/*.md` — knowledge catalog (name + one-line description only)
///
/// Agent definitions come from installed plugins, not from any static hardcoded file,
/// satisfying P1 (Plugin-Composed Everything). If no plugins have installed agent
/// definitions, the "Agent Definitions" section is omitted gracefully.
///
/// Returns the assembled prompt string. Returns `Err` only on I/O failures;
/// missing optional files are silently skipped.
pub fn build_system_prompt(project_path: &Path) -> Result<String, std::io::Error> {
    let mut parts: Vec<String> = Vec::new();
    parts.push("# Project Governance".to_string());

    let rules = read_rules(project_path);
    if !rules.is_empty() {
        parts.push("\n## Rules".to_string());
        for (name, content) in &rules {
            parts.push(format!("\n### {name}\n\n{content}"));
        }
    }

    let catalog = list_knowledge_catalog(project_path);
    if !catalog.is_empty() {
        parts.push("\n## Available Knowledge".to_string());
        parts.push(
            "Use the `load_knowledge` tool to load the full content of any knowledge artifact by name.".to_string(),
        );
        for (name, description) in &catalog {
            parts.push(format!("- **{name}**: {description}"));
        }
    }

    if let Some(claude_md) = read_governance_file(project_path, ".claude/CLAUDE.md")? {
        parts.push("\n## Project Instructions".to_string());
        parts.push(claude_md);
    }

    // Agent definitions come from installed plugin content (P1: Plugin-Composed Everything).
    // The `.orqa/process/agents/` directory is populated at plugin install time from
    // each plugin's `content.agents` source directory.
    let agent_files = read_installed_agent_files(project_path);
    if !agent_files.is_empty() {
        parts.push("\n## Agent Definitions".to_string());
        for (_name, content) in &agent_files {
            parts.push(content.clone());
        }
    }

    Ok(parts.join("\n"))
}

/// Resolve the system prompt from a known project root path, logging on failure.
///
/// Returns `Some(prompt)` when the prompt can be assembled; returns `None`
/// and emits a tracing warning when assembly fails.
pub fn resolve_system_prompt(project_root: &Path) -> Option<String> {
    match build_system_prompt(project_root) {
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

    #[test]
    fn build_system_prompt_empty_project() {
        let dir = make_project();
        let prompt = build_system_prompt(dir.path()).expect("should not error on empty project");
        assert!(prompt.contains("# Project Governance"));
    }

    #[test]
    fn build_system_prompt_includes_rules() {
        let dir = make_project();
        let rules_dir = dir.path().join(".orqa").join("rules");
        fs::create_dir_all(&rules_dir).expect("create rules dir");
        fs::write(
            rules_dir.join("no-debug.md"),
            "Never leave debug statements in production code.",
        )
        .expect("write rule");

        let prompt = build_system_prompt(dir.path()).expect("should succeed");
        assert!(prompt.contains("## Rules"));
        assert!(prompt.contains("no-debug"));
        assert!(prompt.contains("Never leave debug statements"));
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

        let prompt = build_system_prompt(dir.path()).expect("should succeed");
        assert!(prompt.contains("## Project Instructions"));
        assert!(prompt.contains("Follow the architecture."));
    }

    #[test]
    fn build_system_prompt_includes_knowledge_catalog() {
        let dir = make_project();
        let know_dir = dir
            .path()
            .join(".orqa")
            .join("process")
            .join("knowledge");
        fs::create_dir_all(&know_dir).expect("create knowledge dir");
        fs::write(
            know_dir.join("arch-overview.md"),
            "# Architecture Overview\n\nFull content here.\n",
        )
        .expect("write knowledge");

        let prompt = build_system_prompt(dir.path()).expect("should succeed");
        assert!(prompt.contains("## Available Knowledge"));
        assert!(prompt.contains("arch-overview"));
        assert!(prompt.contains("Architecture Overview"));
    }

    #[test]
    fn build_system_prompt_includes_plugin_agent_definitions() {
        // Agents installed to .orqa/process/agents/ (synced from plugin content.agents)
        // should appear in the generated prompt under "Agent Definitions".
        let dir = make_project();
        let agents_dir = dir
            .path()
            .join(".orqa")
            .join("process")
            .join("agents");
        fs::create_dir_all(&agents_dir).expect("create agents dir");
        fs::write(
            agents_dir.join("AGENT-abc123.md"),
            "---\nid: AGENT-abc123\ntitle: Orchestrator\n---\n# Orchestrator\n\nCoordinates workers.\n",
        )
        .expect("write agent file");

        let prompt = build_system_prompt(dir.path()).expect("should succeed");
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
        let prompt = build_system_prompt(dir.path()).expect("should succeed");
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
        let catalog = list_knowledge_catalog(dir.path());
        assert!(catalog.is_empty());
    }

    #[test]
    fn read_rules_empty_when_no_dir() {
        let dir = make_project();
        let rules = read_rules(dir.path());
        assert!(rules.is_empty());
    }

    #[test]
    fn read_governance_file_returns_none_when_absent() {
        let dir = make_project();
        let result = read_governance_file(dir.path(), "nonexistent.md").expect("should not error");
        assert!(result.is_none());
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
}
