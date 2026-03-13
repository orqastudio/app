---
id: EPIC-060
title: "Pipeline health dashboard"
description: "Surface pipeline integrity on the app dashboard with scan/fix actions, add pipeline visualization and temporal analytics."
status: draft
priority: P1
created: "2026-03-13"
updated: "2026-03-13"
deadline: null
milestone: MS-001
pillars:
  - PILLAR-001
  - PILLAR-002
  - PILLAR-003
depends-on:
  - EPIC-061
blocks: []
research-refs:
  - RES-055
docs-required: []
docs-produced: []
scoring:
  dogfood-value: 9
  pipeline-visibility: 10
  implementation-complexity: 7
rule-overrides: []
---

## Context

Pipeline integrity checks (`make verify`) only run from the CLI or pre-commit hook. The artifact graph already computes orphans and broken refs during its two-pass build, and the dashboard already shows GraphStats (nodes, edges, orphans, broken refs) — but there's no way to run targeted integrity checks on-demand, see categorised issues, or fix them from the UI.

**Bundled ideas**: [IDEA-077](IDEA-077), [IDEA-078](IDEA-078), [IDEA-064](IDEA-064), [IDEA-049](IDEA-049)

**Depends on**: [EPIC-061](EPIC-061) — prioritization and process enforcement must be in place first so this epic's priority is properly assessed and its process improvements are captured automatically.

### Existing Infrastructure

- `ProjectDashboard.svelte` — dashboard with GraphStats display (nodes, edges, orphans, broken refs)
- `artifact_graph.rs` — bidirectional graph builder with two-pass processing, already tracks orphans and broken refs
- `artifactGraphSDK` — in-memory graph with auto-refresh on file changes
- `verify-links.mjs` / `verify-pipeline-integrity.mjs` — Node-based CLI checks (reference implementations)
- Pre-commit hook runs checks on staged files

## Implementation Design

### Phase 0: Graph SDK — Typed Relationship Traversal

**Foundation for all subsequent phases.** [RES-055](RES-055) found the `artifactGraphSDK` already holds the full graph in reactive state. The gap is typed relationship edges.

**Backend change:** Extend `ArtifactRef` (Rust + TypeScript) with `relationship_type: Option<String>`. During graph construction, when processing `relationships` arrays, populate `relationship_type` from the relationship's `type` field (e.g., `enforced-by`, `grounded`).

**SDK extensions:**
- `traverse(id, relationshipType)` → `ArtifactNode[]` — follow edges of a specific type
- `pipelineChain(id)` → full upstream/downstream pipeline chain
- `missingInverses()` → `ArtifactRef[]` — A→B exists but B→A doesn't
- `pipelineGaps()` → ADs without enforcement chains, rules without grounding

**Component migration:** Replace per-component `invoke()` calls with SDK lookups where the graph already has the data.

### Phase 1: Native Integrity Engine

Extend `artifact_graph.rs` with integrity check methods:

```
IntegrityCheck {
  category: BrokenLink | MissingRelationship | SchemaViolation | ReconciliationGap | NullTarget
  severity: Error | Warning
  artifact_id: String
  message: String
  auto_fixable: bool
  fix_description: Option<String>
}
```

**Checks to implement natively:**
- Broken cross-references (target doesn't exist)
- Missing bidirectional inverses (A→B exists, B→A doesn't)
- Null relationship targets without `intended: true`
- Epic reconciliation task existence and dependency completeness
- Schema field validation (required fields present, enum values valid)

**Auto-fix logic ([IDEA-078](IDEA-078)):**
- Null targets: scan all artifacts for keyword matches on rationale text, shared relationships, same epic scope. Single strong candidate → auto-fix. Multiple → suggest.
- Missing inverses: deterministic — add the inverse relationship to the target artifact.
- Reconciliation tasks: create missing ones with correct dependencies.

**IPC commands:**
- `run_integrity_scan` → returns `Vec<IntegrityCheck>`
- `apply_auto_fixes` → applies deterministic fixes, returns what was changed
- `package_agent_tasks` → creates task artifacts for non-deterministic issues

### Phase 2: Dashboard Integrity Widget

**Health score card:**
- Traffic light indicator (green/amber/red based on error count)
- Summary: "3 errors, 7 warnings" or "All clear"
- Two action buttons: Scan (refresh checks) and Fix (apply auto-fixes + delegate rest)

**Issue list (expandable by category):**
- Broken Links (count)
- Missing Relationships (count)
- Schema Violations (count)
- Reconciliation Gaps (count)
- Null Targets (count)

Each issue row: artifact ID (clickable → navigates to artifact), message, severity badge, auto-fixable indicator.

**Fix flow:**
1. User clicks Fix
2. Auto-fixable issues applied immediately, results shown (what changed)
3. Non-auto-fixable issues packaged as agent tasks
4. Dashboard shows delegation status and results when agent completes

### Phase 3: Pipeline Visualization ([IDEA-064](IDEA-064))

**Thread rendering:**
- Traverse relationship edges from the artifact graph to render emergent threads
- Show the knowledge maturity pipeline: Observation → Understanding → Principle → Practice → Enforcement → Verification
- Highlight bottlenecks: observations that never became principles, enforcement without observations

**Visualization:**
- Sankey-style flow diagram showing artifact movement through pipeline stages
- Node coloring by bottleneck status (stuck = red, flowing = green)
- Click any node to navigate to the artifact

### Phase 4: Temporal Analytics ([IDEA-049](IDEA-049))

**Trend data:**
- Periodic graph snapshots (on each scan, or from git history)
- Store snapshots as lightweight JSON in ephemeral storage or SQLite metrics table

**Dashboard widgets:**
- Health sparklines: broken refs, orphans, density over last N snapshots
- Velocity chart: artifact status transitions per week
- Staleness heatmap: artifacts not updated relative to their dependents
- "Attention needed" feed: ranked by combined metric signals

## Tasks

| ID | Title | Phase | Depends On |
|----|-------|-------|------------|
| TASK-TBD-0a | Extend ArtifactRef with relationship_type in Rust backend | 0 | — |
| TASK-TBD-0b | Add traverse/pipelineChain/missingInverses to graph SDK | 0 | TASK-TBD-0a |
| TASK-TBD-0c | Migrate components from invoke() to SDK lookups | 0 | TASK-TBD-0b |
| TASK-TBD-1 | Native integrity checks in artifact_graph.rs | 1 | TASK-TBD-0b |
| TASK-TBD-2 | Auto-fix engine for deterministic integrity issues | 1 | TASK-TBD-1 |
| TASK-TBD-3 | IPC commands for integrity scan, auto-fix, and agent delegation | 1 | TASK-TBD-1, TASK-TBD-2 |
| TASK-TBD-4 | Dashboard integrity widget — health score and issue list | 2 | TASK-TBD-3 |
| TASK-TBD-5 | Fix flow — auto-fix application and agent delegation UI | 2 | TASK-TBD-3, TASK-TBD-4 |
| TASK-TBD-6 | Pipeline thread visualization | 3 | TASK-TBD-3 |
| TASK-TBD-7 | Bottleneck detection and flow analysis | 3 | TASK-TBD-6 |
| TASK-TBD-8 | Graph snapshot storage and trend data | 4 | TASK-TBD-1 |
| TASK-TBD-9 | Dashboard trend widgets — sparklines, velocity, staleness | 4 | TASK-TBD-4, TASK-TBD-8 |
| TASK-TBD-10 | Reconcile [EPIC-060](EPIC-060) | — | all above |

## Out of Scope

- Replacing the Node-based CLI tools (they continue to work for CLI users)
- Full graph visualization (node-link diagram) — covered by [EPIC-048](EPIC-048)
- Plugin ecosystem for custom integrity checks (future)
- Notification system for integrity degradation (future idea)
- Process automation (related idea surfacing, observation capture) — moved to [EPIC-061](EPIC-061)
