---
id: EPIC-060
title: Pipeline health dashboard
description: "Surface pipeline integrity on the app dashboard with scan/fix actions, add pipeline visualization and temporal analytics."
status: completed
priority: P1
created: 2026-03-13
updated: 2026-03-13
deadline: null
horizon: active
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
rule-overrides: []
relationships:
  - target: IDEA-091
    type: informs
    rationale: Auto-generated inverse of informs relationship from IDEA-091
  - target: EPIC-063
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from EPIC-063
  - target: EPIC-065
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from EPIC-065
  - target: RES-055
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-055
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-388
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-389
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-390
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-391
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-392
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-393
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-394
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-395
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-396
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-397
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-400
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-401
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-402
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-061
    type: depends-on
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
  - target: EPIC-063
    type: depended-on-by
  - target: EPIC-065
    type: depended-on-by
  - target: IDEA-049
    type: evolves-from
  - target: IDEA-064
    type: evolves-from
  - target: IDEA-077
    type: evolves-from
  - target: IDEA-078
    type: evolves-from
  - target: DOC-060
    type: informs
  - target: PILLAR-001
    type: grounded-by
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
| [TASK-388](TASK-388) | Extend ArtifactRef with relationship_type in Rust backend | 0 | — |
| [TASK-389](TASK-389) | Add traverse/pipelineChain/missingInverses to graph SDK | 0 | [TASK-388](TASK-388) |
| [TASK-396](TASK-396) | Audit components for invoke()-to-SDK migration | 0 | [TASK-389](TASK-389) |
| [TASK-390](TASK-390) | Native integrity checks in artifact_graph.rs | 1 | [TASK-389](TASK-389) |
| [TASK-392](TASK-392) | Auto-fix engine for deterministic integrity issues | 1 | [TASK-390](TASK-390) |
| ~~TASK-TBD-3~~ | ~~IPC commands~~ (merged into [TASK-390](TASK-390) + [TASK-392](TASK-392)) | — | — |
| [TASK-391](TASK-391) | Dashboard integrity widget — health score and issue list | 2 | [TASK-390](TASK-390) |
| [TASK-393](TASK-393) | Fix flow — auto-fix button and results display in IntegrityWidget | 2 | [TASK-391](TASK-391), [TASK-392](TASK-392) |
| [TASK-394](TASK-394) | Pipeline stage visualization widget | 3 | [TASK-389](TASK-389) |
| [TASK-395](TASK-395) | Graph health snapshot storage and trend sparklines | 4 | [TASK-390](TASK-390) |
| [TASK-400](TASK-400) | Integrity check: delivered ideas must have research-needed resolved | 1 | [TASK-390](TASK-390) |
| [TASK-401](TASK-401) | Audit integrity checking system for coverage gaps | 1 | [TASK-400](TASK-400) |
| [TASK-402](TASK-402) | Implement 7 high-priority integrity checks from audit | 1 | [TASK-401](TASK-401) |
| [TASK-397](TASK-397) | Reconcile [EPIC-060](EPIC-060) | — | all above |

## Out of Scope

- Replacing the Node-based CLI tools (they continue to work for CLI users)
- Full graph visualization (node-link diagram) — covered by [EPIC-048](EPIC-048)
- Plugin ecosystem for custom integrity checks (future)
- Notification system for integrity degradation (future idea)
- Process automation (related idea surfacing, observation capture) — moved to [EPIC-061](EPIC-061)
