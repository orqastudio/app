# Governance Bootstrap

**Date:** 2026-03-04
**Phase:** 2b
**Status:** Implemented

## Overview

When a user opens a project in Orqa Studio, Claude (via the Agent SDK sidecar) scans existing governance files, analyzes their coverage and quality, and generates recommendations for improvements. The output is always native Claude Code artifacts — `.claude/rules/*.md`, `.claude/hooks/*.sh`, `.claude/agents/*.md`, `.claude/skills/*/SKILL.md`, `.claude/settings.json`, `CLAUDE.md`, and `AGENTS.md`.

The governance bootstrap is the bridge between "project has some governance" and "project has comprehensive, enforceable governance." It is Claude-powered because static analysis cannot understand intent, detect gaps in coverage, or generate contextually appropriate rules.

## Why Claude, Not a Static Scanner

A static scanner can count files and check formats. Claude can:

- **Read and understand** governance files — detect what a rule actually enforces vs. what it claims to
- **Detect gaps** — identify areas of the codebase with no governance coverage (e.g., testing standards exist but no error handling rules)
- **Suggest improvements** — generate rules, hooks, and agent definitions tailored to the project's tech stack and domain
- **Translate formats** — convert Cursor rules, Copilot instructions, or Continue configs into Claude Code artifacts
- **Generate tech-stack-specific content** — a Rust project gets different rules than a Python project

## Scan Sources

The governance scanner walks the filesystem to collect governance files from multiple tool ecosystems.

| Source | Path Pattern | Description |
|--------|-------------|-------------|
| Claude rules | `.claude/rules/*.md` | Enforcement rules for Claude Code agents |
| Claude agents | `.claude/agents/*.md` | Agent role definitions |
| Claude skills | `.claude/skills/*/SKILL.md` | Skill definitions |
| Claude hooks | `.claude/hooks/*` | Session hooks (bash scripts) |
| Claude settings | `.claude/settings.json` | Hook configuration, permissions |
| CLAUDE.md | `.claude/CLAUDE.md` or root `CLAUDE.md` | Project-level Claude instructions |
| AGENTS.md | Root `AGENTS.md` | Cross-agent instructions |
| Cursor rules | `.cursor/rules/*.md`, `.cursorrules` | Cursor AI rules (translatable) |
| Copilot instructions | `.github/copilot-instructions.md` | GitHub Copilot instructions (translatable) |
| Continue config | `.continue/` | Continue IDE extension config (translatable) |

## Output: Native Artifacts Only

The governance bootstrap produces native Claude Code artifacts exclusively. It never creates Orqa Studio-specific files or proprietary formats. This means:

- Governance works in the CLI without Orqa Studio installed
- Users can edit artifacts with any text editor
- No vendor lock-in — the artifacts are the Claude Code standard

| Artifact Type | Output Path | Description |
|---------------|-------------|-------------|
| Rules | `.claude/rules/*.md` | Enforcement rules |
| Hooks | `.claude/hooks/*.sh` | Session and pre-commit hooks |
| Agents | `.claude/agents/*.md` | Agent role definitions |
| Skills | `.claude/skills/*/SKILL.md` | Skill definitions |
| Settings | `.claude/settings.json` | Hook registration and permissions |
| CLAUDE.md | `.claude/CLAUDE.md` | Project instructions |
| AGENTS.md | `AGENTS.md` | Cross-agent instructions |

## Sidecar Integration

The governance bootstrap uses the existing `send_message` flow with a dedicated "governance" session. No new sidecar request types are needed — it is a specialized system prompt that instructs Claude to analyze governance files and produce structured output.

**Flow:**
1. Rust backend collects governance files via filesystem walk
2. File contents are formatted into a structured prompt
3. A governance session is created with a system prompt instructing Claude to analyze and recommend
4. Claude's response is parsed into structured recommendations
5. Recommendations are stored in SQLite for review

The system prompt instructs Claude to return recommendations in a structured JSON format within a code block, which Rust parses into `Recommendation` objects.

## Wizard Flow

### Trigger Conditions

The governance bootstrap wizard appears when a project is opened and meets one of these conditions:

- **First open:** No governance analysis exists for this project in SQLite
- **Sparse governance:** The scan finds fewer than 3 governance areas covered (e.g., has rules but no agents, hooks, or skills)
- **Manual trigger:** User clicks "Re-analyze" on the project dashboard

### Step-by-Step Flow

1. **Project opens** — Rust backend automatically scans the filesystem for governance files
2. **Sparse detection** — If fewer than 3 governance areas have files, the wizard overlay appears
3. **Scan panel** — Displays findings: which files were found, which areas are covered, a coverage indicator (e.g., "3 of 7 areas covered")
4. **"Analyze with Claude"** — User clicks to start the Claude-powered analysis. Creates a governance session via the sidecar
5. **Analysis results** — Claude returns structured analysis: strengths, gaps, and recommendations. Displayed in a review panel
6. **Review panel** — Each recommendation is a card showing: category, priority, title, description, target file path, and content preview. User can approve, reject, or edit each recommendation
7. **Conversational refinement** — User can send follow-up messages in the governance session to refine recommendations (e.g., "Make that rule stricter" or "We don't use Playwright, use Vitest instead")
8. **Apply** — Approved recommendations are written to disk as native files. Orqa Studio calls `fs::write` for each approved artifact
9. **Done** — Dashboard updated with new governance health status

## Backend Changes

| File | Change |
|------|--------|
| `src-tauri/src/domain/governance.rs` | New — governance domain types (scan results, recommendations, analysis) |
| `src-tauri/src/domain/governance_scanner.rs` | New — filesystem walk to collect governance files |
| `src-tauri/src/domain/mod.rs` | Add `pub mod governance;` and `pub mod governance_scanner;` |
| `src-tauri/src/repo/governance_repo.rs` | New — CRUD for analyses and recommendations (SQLite) |
| `src-tauri/src/repo/mod.rs` | Add `pub mod governance_repo;` |
| `src-tauri/src/commands/governance_commands.rs` | New — Tauri commands for scan, analyze, list, update, apply |
| `src-tauri/src/commands/mod.rs` | Add `pub mod governance_commands;` |
| `src-tauri/src/lib.rs` | Register governance commands in Tauri app builder |
| `src-tauri/migrations/002_governance_bootstrap.sql` | New — governance tables |

## SQLite Schema (Migration 002)

```sql
CREATE TABLE governance_analyses (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    scan_data TEXT NOT NULL,        -- JSON: raw scan results
    summary TEXT NOT NULL,          -- Claude's overall assessment
    strengths TEXT NOT NULL,        -- JSON array: what's working well
    gaps TEXT NOT NULL,             -- JSON array: what's missing
    session_id INTEGER REFERENCES sessions(id),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE governance_recommendations (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    analysis_id INTEGER NOT NULL REFERENCES governance_analyses(id) ON DELETE CASCADE,
    category TEXT NOT NULL,         -- 'rule', 'hook', 'agent', 'skill', 'settings', 'claude_md', 'agents_md'
    priority TEXT NOT NULL,         -- 'critical', 'recommended', 'optional'
    title TEXT NOT NULL,            -- Human-readable title
    description TEXT NOT NULL,      -- What this recommendation does and why
    artifact_type TEXT NOT NULL,    -- File type: 'markdown', 'shell', 'json'
    target_path TEXT NOT NULL,      -- Where the file will be written (relative to project root)
    content TEXT NOT NULL,          -- Full file content to write
    rationale TEXT NOT NULL,        -- Why Claude recommends this
    status TEXT NOT NULL DEFAULT 'pending',  -- 'pending', 'approved', 'rejected', 'applied'
    applied_at TEXT,                -- When the recommendation was written to disk
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
```

## Key Types

```rust
/// Result of scanning the filesystem for governance files.
/// project_id is not stored here — the command takes it as a parameter.
/// total_files is derivable from areas.iter().map(|a| a.files.len()).sum().
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceScanResult {
    pub areas: Vec<GovernanceArea>,
    pub coverage_ratio: f64,  // 0.0 to 1.0 — covered canonical areas / 7
}

/// A governance area (rules, hooks, agents, etc.) found during scanning.
/// `source` identifies the tool ecosystem: "claude", "cursor", "copilot", "continue".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceArea {
    pub name: String,        // e.g. "claude_rules", "cursor", "copilot"
    pub source: String,      // tool ecosystem identifier
    pub files: Vec<GovernanceFile>,
    pub covered: bool,
}

/// A single governance file found on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFile {
    pub path: String,
    pub size_bytes: u64,
    pub content_preview: String,  // first 500 chars of file content
}

/// Claude's analysis of the project's governance state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAnalysis {
    pub id: i64,
    pub project_id: i64,
    pub scan_data: GovernanceScanResult,  // raw scan results for display
    pub summary: String,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub session_id: Option<i64>,
    pub created_at: String,
}

/// Priority of a governance recommendation (type-safe enum).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationPriority {
    Critical,
    Recommended,
    Optional,
}

/// Lifecycle status of a governance recommendation (type-safe enum).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationStatus {
    Pending,
    Approved,
    Rejected,
    Applied,
}

/// A single recommendation from Claude's governance analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: i64,
    pub project_id: i64,
    pub analysis_id: i64,
    pub category: String,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub artifact_type: String,
    pub target_path: String,
    pub content: String,
    pub rationale: String,
    pub status: RecommendationStatus,
    pub applied_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A raw recommendation as parsed from Claude's JSON output (before DB persistence).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRecommendation {
    pub category: String,
    pub priority: String,
    pub title: String,
    pub description: String,
    pub artifact_type: String,
    pub target_path: String,
    pub content: String,
    pub rationale: String,
}

/// Claude's structured analysis output (parsed from JSON code block in the response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeAnalysisOutput {
    pub summary: String,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub recommendations: Vec<RawRecommendation>,
}
```

## IPC Commands

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `governance_scan` | `project_id: i64` | `GovernanceScanResult` | Scan filesystem for governance files |
| `governance_analyze` | `project_id: i64, scan_result: GovernanceScanResult` | `GovernanceAnalysis` | Send to Claude for analysis (creates governance session) |
| `governance_analysis_get` | `project_id: i64` | `Option<GovernanceAnalysis>` | Get latest analysis for a project |
| `recommendations_list` | `project_id: i64` | `Vec<Recommendation>` | List all recommendations for a project |
| `recommendation_update` | `id: i64, status: String` | `Recommendation` | Approve or reject a recommendation |
| `recommendation_apply` | `id: i64` | `Recommendation` | Write approved recommendation to disk |
| `recommendations_apply_all` | `project_id: i64` | `Vec<Recommendation>` | Apply all approved recommendations |

## Frontend Changes

| File | Change |
|------|--------|
| `ui/lib/types/governance.ts` | New — TypeScript interfaces matching Rust types |
| `ui/lib/stores/governance.svelte.ts` | New — governance store: scan state, analysis state, recommendations list, actions |
| `ui/lib/components/governance/GovernanceBootstrapWizard.svelte` | New — wizard overlay container with step navigation |
| `ui/lib/components/governance/GovernanceScanPanel.svelte` | New — scan results display with coverage indicator |
| `ui/lib/components/governance/RecommendationList.svelte` | New — scrollable list of recommendation cards |
| `ui/lib/components/governance/RecommendationCard.svelte` | New — individual recommendation with approve/reject/preview |
| `ui/lib/components/governance/CoverageIndicator.svelte` | New — visual indicator of governance area coverage |
| Project dashboard component | Add governance health badge and "Re-analyze" button |

## Component State Table

| Component | States | What the user sees |
|-----------|--------|-------------------|
| GovernanceBootstrapWizard | `scanning`, `scan_complete`, `analyzing`, `review`, `applying`, `done` | Scanning: spinner. Scan complete: results + "Analyze" button. Analyzing: spinner with streamed progress. Review: recommendation cards. Applying: progress. Done: summary. |
| GovernanceScanPanel | `loading`, `loaded`, `empty` | Loading: spinner. Loaded: file list + coverage chart. Empty: "No governance files found" message. |
| RecommendationCard | `pending`, `approved`, `rejected`, `applying`, `applied`, `error` | Pending: neutral card with approve/reject buttons. Approved: green accent. Rejected: dimmed. Applying: spinner. Applied: check mark + file path. Error: error message + retry. |
| CoverageIndicator | `loading`, `loaded` | Loading: skeleton. Loaded: segmented bar showing covered (green) vs. uncovered (gray) areas. |

## User Journeys

**New project with no governance:**
1. User opens a project directory with no `.claude/` folder
2. Scan finds 0 governance files — wizard appears automatically
3. Scan panel shows "No governance files found. 0 of 7 areas covered."
4. User clicks "Analyze with Claude"
5. Claude analyzes the project's package.json, Cargo.toml, etc. to understand the tech stack
6. Claude generates recommendations: coding standards rule, error handling rule, testing rule, pre-commit hook, etc.
7. User reviews each recommendation — approves most, rejects one, edits another via chat
8. User clicks "Apply" — files written to `.claude/rules/`, `.claude/hooks/`, etc.
9. Dashboard shows governance health: "5 of 7 areas covered"

**Existing project with Cursor rules:**
1. User opens a project with `.cursorrules` and some `.cursor/rules/*.md`
2. Scan finds Cursor-format rules — wizard appears (Claude tools not configured)
3. Scan panel shows: "Found 4 Cursor rules. Claude Code governance: 0 of 7 areas covered."
4. User clicks "Analyze with Claude"
5. Claude reads the Cursor rules, understands their intent, and generates equivalent Claude Code rules
6. Recommendations include translated rules plus additional gap-filling suggestions
7. User reviews, approves, applies

**Existing project with comprehensive governance:**
1. User opens the Orqa Studio project itself (which has extensive `.claude/` governance)
2. Scan finds 20+ files across all 7 areas
3. Coverage is 7/7 — wizard does NOT auto-trigger
4. User can still click "Re-analyze" on the dashboard to get improvement suggestions

## Verification Criteria

1. **Scan accuracy:** Open a project with `.claude/` files — scan finds all files, coverage ratio is correct
2. **Sparse trigger:** Open a project with < 3 governance areas — wizard triggers automatically
3. **Claude analysis:** "Analyze with Claude" produces structured recommendations with real content (not stubs)
4. **Approve/reject:** Recommendation status changes persist in SQLite
5. **Apply writes files:** Approved recommendations create real files on disk at the specified paths
6. **Re-analyze detects changes:** After applying recommendations, re-scanning detects the new files

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | Governance bootstrap is the first turn of the learning loop — Claude reads what exists and improves it. Each re-analysis captures what changed and suggests further improvements, building knowledge over time. |
| Process Governance | Creates native Claude Code governance artifacts from scratch or by translating from other tools, making governance visible and enforceable from the moment a project is opened in Orqa Studio. |

## Related Documents

- [Roadmap — Phase 2b](/product/roadmap)
- [Setup Wizard (Phase 2a)](/architecture/setup-wizard)
- [SQLite Schema](/architecture/sqlite-schema)
- [IPC Commands](/architecture/ipc-commands)
- [Streaming Pipeline](/architecture/streaming-pipeline)
