---
id: "EPIC-e37794bf"
title: "Pipeline health dashboard"
description: "Surface pipeline integrity on the app dashboard with scan/fix actions, add pipeline visualization and temporal analytics."
status: "completed"
priority: "P1"
created: "2026-03-13"
updated: "2026-03-13"
deadline: null
horizon: "active"
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
rule-overrides: []
relationships:
  - target: "RES-e757fc6e"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-e757fc6e"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-2ffc45f7"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ce651394"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f6fd3161"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ee4681f4"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-5fc0e2dc"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-614122c8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c5f53141"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e188a8c8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b155317a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d624db8f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d746777d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-07218422"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ab087863"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-c3b01bfd"
    type: "realised-by"
  - target: "IDEA-6522afe2"
    type: "realised-by"
  - target: "IDEA-289ecd64"
    type: "realised-by"
  - target: "IDEA-3c8085b6"
    type: "realised-by"
---
## Context

Pipeline integrity checks (`make verify`) only run from the CLI or pre-commit hook. The artifact graph already computes orphans and broken refs during its two-pass build, and the dashboard already shows GraphStats (nodes, edges, orphans, broken refs) — but there's no way to run targeted integrity checks on-demand, see categorised issues, or fix them from the UI.

**Bundled ideas**: [IDEA-289ecd64](IDEA-289ecd64), [IDEA-3c8085b6](IDEA-3c8085b6), [IDEA-6522afe2](IDEA-6522afe2), [IDEA-c3b01bfd](IDEA-c3b01bfd)

**Depends on**: [EPIC-4e6e9eae](EPIC-4e6e9eae) — prioritization and process enforcement must be in place first so this epic's priority is properly assessed and its process improvements are captured automatically.

### Existing Infrastructure

- `ProjectDashboard.svelte` — dashboard with GraphStats display (nodes, edges, orphans, broken refs)
- `artifact_graph.rs` — bidirectional graph builder with two-pass processing, already tracks orphans and broken refs
- `artifactGraphSDK` — in-memory graph with auto-refresh on file changes
- `verify-links.mjs` / `verify-pipeline-integrity.mjs` — Node-based CLI checks (reference implementations)
- Pre-commit hook runs checks on staged files

## Implementation Design

### Phase 0: Graph SDK — Typed Relationship Traversal

**Foundation for all subsequent phases.** [RES-e757fc6e](RES-e757fc6e) found the `artifactGraphSDK` already holds the full graph in reactive state. The gap is typed relationship edges.

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

**Auto-fix logic ([IDEA-3c8085b6](IDEA-3c8085b6)):**
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

### Phase 3: Pipeline Visualization ([IDEA-6522afe2](IDEA-6522afe2))

**Thread rendering:**
- Traverse relationship edges from the artifact graph to render emergent threads
- Show the knowledge maturity pipeline: Observation → Understanding → Principle → Practice → Enforcement → Verification
- Highlight bottlenecks: observations that never became principles, enforcement without observations

**Visualization:**
- Sankey-style flow diagram showing artifact movement through pipeline stages
- Node coloring by bottleneck status (stuck = red, flowing = green)
- Click any node to navigate to the artifact

### Phase 4: Temporal Analytics ([IDEA-c3b01bfd](IDEA-c3b01bfd))

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
| [TASK-2ffc45f7](TASK-2ffc45f7) | Extend ArtifactRef with relationship_type in Rust backend | 0 | — |
| [TASK-ce651394](TASK-ce651394) | Add traverse/pipelineChain/missingInverses to graph SDK | 0 | [TASK-2ffc45f7](TASK-2ffc45f7) |
| [TASK-b155317a](TASK-b155317a) | Audit components for invoke()-to-SDK migration | 0 | [TASK-ce651394](TASK-ce651394) |
| [TASK-f6fd3161](TASK-f6fd3161) | Native integrity checks in artifact_graph.rs | 1 | [TASK-ce651394](TASK-ce651394) |
| [TASK-5fc0e2dc](TASK-5fc0e2dc) | Auto-fix engine for deterministic integrity issues | 1 | [TASK-f6fd3161](TASK-f6fd3161) |
| ~~TASK-TBD-3~~ | ~~IPC commands~~ (merged into [TASK-f6fd3161](TASK-f6fd3161) + [TASK-5fc0e2dc](TASK-5fc0e2dc)) | — | — |
| [TASK-ee4681f4](TASK-ee4681f4) | Dashboard integrity widget — health score and issue list | 2 | [TASK-f6fd3161](TASK-f6fd3161) |
| [TASK-614122c8](TASK-614122c8) | Fix flow — auto-fix button and results display in IntegrityWidget | 2 | [TASK-ee4681f4](TASK-ee4681f4), [TASK-5fc0e2dc](TASK-5fc0e2dc) |
| [TASK-c5f53141](TASK-c5f53141) | Pipeline stage visualization widget | 3 | [TASK-ce651394](TASK-ce651394) |
| [TASK-e188a8c8](TASK-e188a8c8) | Graph health snapshot storage and trend sparklines | 4 | [TASK-f6fd3161](TASK-f6fd3161) |
| [TASK-d746777d](TASK-d746777d) | Integrity check: delivered ideas must have research-needed resolved | 1 | [TASK-f6fd3161](TASK-f6fd3161) |
| [TASK-07218422](TASK-07218422) | Audit integrity checking system for coverage gaps | 1 | [TASK-d746777d](TASK-d746777d) |
| [TASK-ab087863](TASK-ab087863) | Implement 7 high-priority integrity checks from audit | 1 | [TASK-07218422](TASK-07218422) |
| [TASK-d624db8f](TASK-d624db8f) | Reconcile [EPIC-e37794bf](EPIC-e37794bf) | — | all above |

## Out of Scope

- Replacing the Node-based CLI tools (they continue to work for CLI users)
- Full graph visualization (node-link diagram) — covered by [EPIC-0a8a5e72](EPIC-0a8a5e72)
- Plugin ecosystem for custom integrity checks (future)
- Notification system for integrity degradation (future idea)
- Process automation (related idea surfacing, observation capture) — moved to [EPIC-4e6e9eae](EPIC-4e6e9eae)
