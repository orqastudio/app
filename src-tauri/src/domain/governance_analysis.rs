use std::path::{Path, PathBuf};

use crate::domain::governance::{
    ClaudeAnalysisOutput, GovernanceScanResult, Recommendation, RecommendationPriority,
    RecommendationStatus,
};
use crate::error::OrqaError;
use crate::sidecar::manager::SidecarManager;
use crate::sidecar::types::{SidecarRequest, SidecarResponse};

/// System prompt instructing Claude to analyze governance and produce structured output.
pub const GOVERNANCE_SYSTEM_PROMPT: &str = "You are a governance advisor for agentic development \
    projects. Your task is to analyze existing governance files and generate recommendations \
    for improving Claude Code governance coverage. \
    Always respond with a JSON object inside a ```json code block. \
    Generate real, project-appropriate governance content — not placeholder text. \
    Focus on the most impactful gaps first.";

// ── Sidecar communication ──

/// Send a message to the sidecar and collect the full text response.
///
/// Loops over sidecar events until `TurnComplete`, accumulating `TextDelta` content.
/// Tool calls are auto-approved so analysis can proceed unattended.
pub fn send_and_collect(
    sidecar: &SidecarManager,
    session_id: i64,
    content: &str,
) -> Result<String, OrqaError> {
    let request = SidecarRequest::SendMessage {
        session_id,
        content: content.to_string(),
        model: None,
        system_prompt: Some(GOVERNANCE_SYSTEM_PROMPT.to_string()),
        sdk_session_id: None,
    };
    sidecar.send(&request)?;

    let mut accumulated = String::new();
    loop {
        match sidecar.read_line()? {
            None => {
                return Err(OrqaError::Sidecar(
                    "sidecar closed stdout before completing governance analysis".to_string(),
                ))
            }
            Some(response) => {
                if handle_sidecar_response(sidecar, response, &mut accumulated)? {
                    break;
                }
            }
        }
    }
    Ok(accumulated)
}

/// Handle a single sidecar response event during analysis.
///
/// Returns `true` when the stream is complete and the loop should exit.
/// Returns an error if the stream reports a failure or cancellation.
/// Auto-approves tool calls so the analysis can proceed unattended.
pub fn handle_sidecar_response(
    sidecar: &SidecarManager,
    response: SidecarResponse,
    accumulated: &mut String,
) -> Result<bool, OrqaError> {
    match response {
        SidecarResponse::TextDelta { content } => {
            accumulated.push_str(&content);
            Ok(false)
        }
        SidecarResponse::TurnComplete { .. } => Ok(true),
        SidecarResponse::StreamError { message, .. } => Err(OrqaError::Sidecar(format!(
            "sidecar stream error during analysis: {message}"
        ))),
        SidecarResponse::StreamCancelled => Err(OrqaError::Sidecar(
            "governance analysis stream cancelled".to_string(),
        )),
        SidecarResponse::ToolExecute { tool_call_id, .. } => {
            approve_tool_call(sidecar, tool_call_id)?;
            Ok(false)
        }
        SidecarResponse::ToolApprovalRequest { tool_call_id, .. } => {
            approve_tool_call(sidecar, tool_call_id)?;
            Ok(false)
        }
        _ => Ok(false), // Ignore other events (StreamStart, BlockComplete, etc.)
    }
}

/// Send an auto-approval for a tool call during governance analysis.
fn approve_tool_call(sidecar: &SidecarManager, tool_call_id: String) -> Result<(), OrqaError> {
    sidecar
        .send(&SidecarRequest::ToolApproval {
            tool_call_id,
            approved: true,
            reason: None,
        })
        .map_err(|e| OrqaError::Sidecar(format!("failed to send tool approval: {e}")))?;
    Ok(())
}

// ── Claude output parsing ──

/// Parse Claude's JSON output from a raw text response.
///
/// Expects a JSON code block (```json ... ```) or bare JSON object in the response.
pub fn parse_claude_output(raw: &str) -> Result<ClaudeAnalysisOutput, OrqaError> {
    let json_str = extract_json_block(raw).unwrap_or(raw);

    serde_json::from_str::<ClaudeAnalysisOutput>(json_str.trim()).map_err(|e| {
        OrqaError::Serialization(format!(
            "failed to parse Claude governance analysis output: {e}. \
             Raw response snippet: {}",
            &raw[..raw.len().min(200)]
        ))
    })
}

/// Extract JSON from a markdown code block if present.
fn extract_json_block(text: &str) -> Option<&str> {
    let start = text.find("```json")?;
    let after_tag = start + "```json".len();
    let end = text[after_tag..].find("```")?;
    Some(text[after_tag..after_tag + end].trim())
}

// ── Prompt construction ──

/// Build the analysis prompt from governance scan data.
pub fn build_analysis_prompt(scan: &GovernanceScanResult) -> String {
    let header = format_coverage_header(scan);
    let file_list = format_file_list(scan);

    format!(
        "Analyze the governance files found in this project.\n\n\
         {header}\n\n\
         Found governance files:\n{file_list}\n\n\
         Return a JSON object (inside a ```json code block) with this exact structure:\n\
         {{\n\
           \"summary\": \"<2-3 sentence overall assessment>\",\n\
           \"strengths\": [\"<strength 1>\", \"<strength 2>\"],\n\
           \"gaps\": [\"<gap 1>\", \"<gap 2>\"],\n\
           \"recommendations\": [\n\
             {{\n\
               \"category\": \"rule|hook|agent|skill|settings|claude_md|agents_md\",\n\
               \"priority\": \"critical|recommended|optional\",\n\
               \"title\": \"<short title>\",\n\
               \"description\": \"<what this does and why>\",\n\
               \"artifact_type\": \"markdown|shell|json\",\n\
               \"target_path\": \"<relative path e.g. .claude/rules/error-handling.md>\",\n\
               \"content\": \"<full file content to write>\",\n\
               \"rationale\": \"<why Claude recommends this>\"\n\
             }}\n\
           ]\n\
         }}"
    )
}

/// Build the coverage header section for the analysis prompt.
fn format_coverage_header(scan: &GovernanceScanResult) -> String {
    let covered: Vec<&str> = scan
        .areas
        .iter()
        .filter(|a| a.covered && a.source == "claude")
        .map(|a| a.name.as_str())
        .collect();

    let uncovered: Vec<&str> = scan
        .areas
        .iter()
        .filter(|a| !a.covered && a.source == "claude")
        .map(|a| a.name.as_str())
        .collect();

    let covered_str = if covered.is_empty() {
        "none".to_string()
    } else {
        covered.join(", ")
    };
    let uncovered_str = if uncovered.is_empty() {
        "none".to_string()
    } else {
        uncovered.join(", ")
    };

    format!(
        "Coverage: {:.0}% ({} of 7 canonical areas covered)\nCovered areas: {}\nMissing areas: {}",
        scan.coverage_ratio * 100.0,
        covered.len(),
        covered_str,
        uncovered_str,
    )
}

/// Format covered governance files as a markdown file list for the prompt.
fn format_file_list(scan: &GovernanceScanResult) -> String {
    scan.areas
        .iter()
        .filter(|a| a.covered)
        .flat_map(|a| {
            a.files.iter().map(|f| {
                format!(
                    "### {} ({})\n```\n{}\n```\n",
                    f.path, a.name, f.content_preview
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Recommendation building ──

/// Convert raw Claude recommendations into `Recommendation` domain objects.
pub fn build_recommendations(
    project_id: i64,
    analysis_id: i64,
    output: &ClaudeAnalysisOutput,
    now: &str,
) -> Vec<Recommendation> {
    output
        .recommendations
        .iter()
        .map(|raw| {
            let priority = RecommendationPriority::parse(&raw.priority)
                .unwrap_or(RecommendationPriority::Recommended);
            Recommendation {
                id: 0,
                project_id,
                analysis_id,
                category: raw.category.clone(),
                priority,
                title: raw.title.clone(),
                description: raw.description.clone(),
                artifact_type: raw.artifact_type.clone(),
                target_path: raw.target_path.clone(),
                content: raw.content.clone(),
                rationale: raw.rationale.clone(),
                status: RecommendationStatus::Pending,
                applied_at: None,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            }
        })
        .collect()
}

// ── File output ──

/// Write a recommendation's content to its target path within the project.
///
/// Creates parent directories as needed. Performs filesystem I/O.
pub fn write_recommendation_file(rec: &Recommendation, project_path: &str) -> Result<(), OrqaError> {
    let target = resolve_target_path(&rec.target_path, project_path)?;

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&target, &rec.content)?;
    Ok(())
}

// ── Path utilities ──

/// Resolve the target path relative to the project root, rejecting path traversal.
pub fn resolve_target_path(target_path: &str, project_root: &str) -> Result<PathBuf, OrqaError> {
    let root = PathBuf::from(project_root);
    let candidate = if Path::new(target_path).is_absolute() {
        PathBuf::from(target_path)
    } else {
        root.join(target_path)
    };

    let normalized = normalize_path(&candidate);
    let root_normalized = normalize_path(&root);

    if !normalized.starts_with(&root_normalized) {
        return Err(OrqaError::PermissionDenied(format!(
            "target path '{target_path}' is outside the project root"
        )));
    }
    Ok(normalized)
}

/// Normalize a path by resolving `.` and `..` without touching the filesystem.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            c => components.push(c),
        }
    }
    components.iter().collect()
}

// ── Timestamp utilities ──

/// Return the current UTC timestamp in RFC3339 format.
pub fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_unix_timestamp(secs)
}

/// Format a Unix timestamp as a compact ISO-8601 string matching SQLite's default.
fn format_unix_timestamp(secs: u64) -> String {
    let (year, month, day, hour, minute, second) = unix_to_datetime(secs);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.000Z")
}

/// Decompose a Unix timestamp into (year, month, day, hour, minute, second).
fn unix_to_datetime(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let second = secs % 60;
    let minutes = secs / 60;
    let minute = minutes % 60;
    let hours = minutes / 60;
    let hour = hours % 24;
    let total_days = hours / 24;

    let mut year = 1970u64;
    let mut remaining_days = total_days;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let leap = is_leap_year(year);
    let month_days: [u64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1u64;
    for &days in &month_days {
        if remaining_days < days {
            break;
        }
        remaining_days -= days;
        month += 1;
    }
    let day = remaining_days + 1;

    (year, month, day, hour, minute, second)
}

fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_block_finds_code_block() {
        let text = "Here is the analysis:\n\n```json\n{\"summary\": \"ok\"}\n```\n\nDone.";
        let extracted = extract_json_block(text).expect("should find json block");
        assert_eq!(extracted, "{\"summary\": \"ok\"}");
    }

    #[test]
    fn extract_json_block_returns_none_when_absent() {
        let text = "No code block here, just plain text.";
        assert!(extract_json_block(text).is_none());
    }

    #[test]
    fn parse_claude_output_valid_json() {
        let raw = r#"```json
{
  "summary": "Good governance",
  "strengths": ["Has rules"],
  "gaps": ["No hooks"],
  "recommendations": []
}
```"#;
        let output = parse_claude_output(raw).expect("should parse");
        assert_eq!(output.summary, "Good governance");
        assert_eq!(output.strengths, ["Has rules"]);
    }

    #[test]
    fn parse_claude_output_invalid_json_returns_error() {
        let result = parse_claude_output("```json\n{bad json\n```");
        assert!(result.is_err());
        assert!(matches!(result, Err(OrqaError::Serialization(_))));
    }

    #[test]
    fn normalize_path_removes_double_dot() {
        let path = Path::new("/project/foo/../bar");
        let normalized = normalize_path(path);
        assert_eq!(normalized, PathBuf::from("/project/bar"));
    }

    #[test]
    fn resolve_target_path_relative_within_root() {
        let result = resolve_target_path(".claude/rules/test.md", "/project");
        assert!(result.is_ok());
        let path = result.expect("ok");
        assert!(path.starts_with("/project"));
    }

    #[test]
    fn resolve_target_path_traversal_rejected() {
        let result = resolve_target_path("../../etc/passwd", "/project");
        assert!(matches!(result, Err(OrqaError::PermissionDenied(_))));
    }

    #[test]
    fn format_unix_timestamp_produces_valid_format() {
        let ts = format_unix_timestamp(0);
        assert_eq!(ts, "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn build_analysis_prompt_includes_coverage() {
        use crate::domain::governance::GovernanceArea;
        let scan = GovernanceScanResult {
            areas: vec![GovernanceArea {
                name: "claude_rules".to_string(),
                source: "claude".to_string(),
                files: vec![],
                covered: false,
            }],
            coverage_ratio: 0.0,
        };
        let prompt = build_analysis_prompt(&scan);
        assert!(prompt.contains("0%"));
        assert!(prompt.contains("claude_rules"));
    }

    #[test]
    fn build_recommendations_maps_raw_correctly() {
        use crate::domain::governance::RawRecommendation;
        let output = ClaudeAnalysisOutput {
            summary: "s".to_string(),
            strengths: vec![],
            gaps: vec![],
            recommendations: vec![RawRecommendation {
                category: "rule".to_string(),
                priority: "critical".to_string(),
                title: "Error handling".to_string(),
                description: "desc".to_string(),
                artifact_type: "markdown".to_string(),
                target_path: ".claude/rules/error.md".to_string(),
                content: "# Error".to_string(),
                rationale: "missing".to_string(),
            }],
        };
        let recs = build_recommendations(1, 2, &output, "2026-01-01T00:00:00.000Z");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].priority, RecommendationPriority::Critical);
        assert_eq!(recs[0].status, RecommendationStatus::Pending);
    }

    #[test]
    fn build_recommendations_falls_back_on_unknown_priority() {
        use crate::domain::governance::RawRecommendation;
        let output = ClaudeAnalysisOutput {
            summary: "s".to_string(),
            strengths: vec![],
            gaps: vec![],
            recommendations: vec![RawRecommendation {
                category: "rule".to_string(),
                priority: "high".to_string(), // unknown
                title: "t".to_string(),
                description: "d".to_string(),
                artifact_type: "markdown".to_string(),
                target_path: ".claude/rules/t.md".to_string(),
                content: "c".to_string(),
                rationale: "r".to_string(),
            }],
        };
        let recs = build_recommendations(1, 2, &output, "2026-01-01T00:00:00.000Z");
        assert_eq!(recs[0].priority, RecommendationPriority::Recommended);
    }
}
