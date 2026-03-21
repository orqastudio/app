---
id: EPIC-7b3d9f05
type: epic
title: "Validation consolidation: libs/validation crate as single source of truth"
description: Extract all validation logic into a shared libs/validation Rust crate. Consolidate 6 separate implementations into one. Add graph-theoretic metrics. All consumers (app, MCP, LSP, CLI, hooks, visualiser) use the same library for consistent results.
status: ready
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: MS-654badde
    type: fulfils
  - target: AD-a4f2c8e1
    type: driven-by
---

# EPIC-100: Validation Consolidation — libs/validation Crate as Single Source of Truth

## Problem

OrqaStudio has 6 validation implementations that produce different results. The app's `integrity_engine.rs` has 13 checks. The MCP server has a manual copy missing 2. The LSP has 8 file-level checks. The hook has its own JS parser. The graph visualiser computes metrics in a separate JS pipeline. The CLI depends on the app being alive.

When you run `orqa validate` and get 5 errors, then open the dashboard and see 3, and then look at the graph and see 12 orphans — all from the same artifact graph — none of those numbers are trustworthy. This is an enforcement gap: the product exists to make governance reliable, and its own governance reporting is inconsistent.

## Design

See AD-062 for the architecture decision.

### Phase 1 — Extract: Create libs/validation

Create `libs/validation` as a Rust workspace crate. Extract `app/integrity_engine.rs` as the starting implementation. Organise into `checks/` modules:

| Module | Checks |
|--------|--------|
| `structural.rs` | Broken references, missing inverses, type constraints |
| `cardinality.rs` | Min/max relationship counts per relationship type |
| `cycles.rs` | Cycle detection in the relationship graph |
| `status.rs` | Valid status transitions, terminal state rules |
| `delivery.rs` | Delivery path completeness (tasks → epics → milestones) |
| `parent_child.rs` | Parent-child consistency, orphan detection |
| `body_refs.rs` | Body-text reference validation against known artifact IDs |

Add graph-theoretic metrics (currently only in JS Cytoscape pipeline):

- Connected component / cluster detection
- Orphan nodes (degree 0)
- In-degree and out-degree per node
- PageRank approximation for artifact importance

Single output type throughout: `IntegrityCheck { category, severity, message, artifact_id, auto_fixable }`.

### Phase 2 — Consolidate App + MCP

App and MCP server import from `libs/validation` as a workspace dependency. Remove their independent implementations:

- Delete `backend/src-tauri/src/integrity_engine.rs`
- Delete `libs/mcp-server/src/integrity.rs`

Verify the app's `graph_validate` Tauri command and the MCP `graph_validate` tool both produce identical output for the same artifact graph.

### Phase 3 — Consolidate LSP + Hook

LSP server imports graph-level checks from `libs/validation`. The 8 LSP file-level checks become a subset of the shared implementation — no separate logic maintained.

`validate-artifact.mjs` hook delegates to MCP `graph_validate` instead of its own JS YAML parser. The hook becomes a thin adapter: call the tool, interpret the result, report to the pre-commit pipeline.

### Phase 4 — Metrics Integration

Graph visualiser calls server-computed metrics from `libs/validation` (via MCP) instead of running its own Cytoscape analysis. Dashboard clarity view and graph visualiser show identical numbers because they query the same source.

Wire `auto_fixable: true` checks to `orqa validate --fix` across all surfaces.

## Acceptance Criteria

- [ ] `libs/validation` crate exists as a Rust workspace member with all check modules
- [ ] App imports `libs/validation` — `integrity_engine.rs` deleted
- [ ] MCP server imports `libs/validation` — `mcp/integrity.rs` deleted
- [ ] LSP server imports graph-level checks from `libs/validation`
- [ ] CLI `orqa validate` uses `libs/validation` directly (no app dependency)
- [ ] `validate-artifact.mjs` hook delegates to MCP `graph_validate` — no JS YAML parsing
- [ ] Graph health metrics (clusters, orphans, degree) computed server-side in `libs/validation`
- [ ] Dashboard clarity view matches MCP `graph_validate` output exactly
- [ ] Graph visualiser metrics match server-computed metrics
- [ ] `IntegrityCheck` is the single output type across all consumers
- [ ] Zero code duplication across validation implementations
- [ ] `make check` passes after all changes

## Risks

- **Type interface design** — shared types between app, MCP, and LSP need careful design to avoid circular workspace dependencies
- **LSP file-level vs graph-level checks** — some LSP checks are inherently per-file (can run without loading the full graph); the interface must support both modes
- **Hook MCP dependency** — the hook now requires the MCP server to be running; validate this is always the case in the pre-commit environment
- **Metric algorithm parity** — JS Cytoscape and Rust graph metrics may use different algorithms; parity must be verified before deleting the JS pipeline
