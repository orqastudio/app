use crate::error::OrqaError;
use crate::state::AppState;

use std::path::Path;

/// Read a governance file from the project directory, returning its contents.
/// Returns `None` if the file does not exist, `Err` on read errors.
pub fn read_governance_file(
    project_path: &Path,
    relative: &str,
) -> Result<Option<String>, OrqaError> {
    let full_path = project_path.join(relative);
    if !full_path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&full_path)?;
    Ok(Some(contents))
}

/// List skill names with one-line descriptions from `.claude/skills/*/SKILL.md`.
///
/// Reads only the first non-empty line of each SKILL.md as the description.
/// Full skill content is intentionally NOT loaded here — skills are loaded
/// on demand via the `load_skill` tool.
pub fn list_skill_catalog(project_path: &Path) -> Vec<(String, String)> {
    let skills_dir = project_path.join(".claude").join("skills");
    let mut catalog = Vec::new();

    let read_dir = match std::fs::read_dir(&skills_dir) {
        Ok(rd) => rd,
        Err(_) => return catalog,
    };

    for entry in read_dir.flatten() {
        let skill_md = entry.path().join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let skill_name = entry.file_name().to_string_lossy().to_string();
        let description = std::fs::read_to_string(&skill_md)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .map(|l| l.trim_start_matches('#').trim().to_string())
            })
            .unwrap_or_else(|| "No description".to_string());

        catalog.push((skill_name, description));
    }

    catalog.sort_by(|a, b| a.0.cmp(&b.0));
    catalog
}

/// Read all rule files from `.claude/rules/*.md`.
pub fn read_rules(project_path: &Path) -> Vec<(String, String)> {
    let rules_dir = project_path.join(".claude").join("rules");
    let mut rules = Vec::new();

    let read_dir = match std::fs::read_dir(&rules_dir) {
        Ok(rd) => rd,
        Err(_) => return rules,
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

/// Build a structured system prompt from the project's governance artifacts.
///
/// Reads:
/// - `.claude/rules/*.md` — rule files (full content)
/// - `.claude/CLAUDE.md` — project instructions (full content)
/// - `AGENTS.md` — agent definitions (full content)
/// - `.claude/skills/*/SKILL.md` — skill catalog (name + one-line description only)
///
/// Returns `Ok(None)` when the project path cannot be resolved (no active project).
pub fn build_system_prompt(project_path: &Path) -> Result<String, OrqaError> {
    let mut parts: Vec<String> = Vec::new();
    parts.push("# Project Governance".to_string());

    let rules = read_rules(project_path);
    if !rules.is_empty() {
        parts.push("\n## Rules".to_string());
        for (name, content) in &rules {
            parts.push(format!("\n### {name}\n\n{content}"));
        }
    }

    let catalog = list_skill_catalog(project_path);
    if !catalog.is_empty() {
        parts.push("\n## Available Skills".to_string());
        parts.push(
            "Use the `load_skill` tool to load the full content of any skill by name.".to_string(),
        );
        for (name, description) in &catalog {
            parts.push(format!("- **{name}**: {description}"));
        }
    }

    if let Some(claude_md) = read_governance_file(project_path, ".claude/CLAUDE.md")? {
        parts.push("\n## Project Instructions".to_string());
        parts.push(claude_md);
    }

    if let Some(agents_md) = read_governance_file(project_path, "AGENTS.md")? {
        parts.push("\n## Agent Definitions".to_string());
        parts.push(agents_md);
    }

    Ok(parts.join("\n"))
}

/// A condensed message record used for context injection into the system prompt.
#[derive(serde::Serialize)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
}

/// Load recent text messages from a session for context injection.
///
/// Returns up to 20 recent user/assistant text messages in chronological order.
/// Returns `None` if the database lock fails or the session has no qualifying messages.
pub fn load_context_messages(state: &AppState, session_id: i64) -> Option<Vec<ContextMessage>> {
    use crate::domain::message::{ContentType, MessageRole};
    use crate::repo::message_repo;

    let db = state.db.lock().ok()?;
    // Load a generous window; we'll slice from the end below.
    let messages = message_repo::list(&db, session_id, 200, 0).ok()?;

    let context: Vec<ContextMessage> = messages
        .iter()
        .filter(|m| {
            matches!(m.role, MessageRole::User | MessageRole::Assistant)
                && m.content_type == ContentType::Text
                && m.content.is_some()
        })
        .rev()
        .take(20)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| ContextMessage {
            role: match m.role {
                MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "assistant".to_string(),
                MessageRole::System => "system".to_string(),
            },
            content: m.content.clone().unwrap_or_default(),
        })
        .collect();

    if context.is_empty() {
        None
    } else {
        Some(context)
    }
}
