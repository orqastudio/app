---
id: TASK-ff0a2460
type: task
title: "Forward-only relationship storage — remove stored inverses, compute at query time"
status: captured
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "MissingInverse validation removed from structural.rs — no longer produces errors"
  - "apply_missing_inverse_fix() in auto_fix.rs removed or disabled"
  - "Bidirectionality metrics in metrics.rs count schema-defined inverses, not stored edges"
  - "Graph builder Pass 2 (graph.rs) still computes backlinks for queries — unchanged"
  - "Plugin manifest inverse field preserved — defines semantic inverse for graph engine"
  - "All stored inverse entries removed from .orqa/ artifact files (bulk cleanup)"
  - "Pre-commit hook passes on artifacts with forward-only relationships"
  - "cargo build/test/clippy clean on orqa-validation"
relationships:
  - target: EPIC-4304bdcc
    type: delivers
    rationale: "Relationship storage model is governance foundation work"
  - target: TASK-d28b2909
    type: depends-on
    rationale: "Vocabulary must be confirmed before changing storage model"
---

## Research Findings

Graph engine already computes inverse edges in Pass 2 of graph building (`graph.rs` lines 257-268). Stored inverses on artifacts are redundant — they exist only because the validator requires them.

### Code Locations

| File | What | Lines |
|------|------|-------|
| `libs/validation/src/structural.rs` | MissingInverse check — requires stored inverse on target | 32-81 |
| `libs/validation/src/auto_fix.rs` | `apply_missing_inverse_fix()` — adds inverse entries to files | 240-297 |
| `libs/validation/src/metrics.rs` | Bidirectionality metrics — counts stored bidirectional edges | 507-548 |
| `libs/validation/src/graph.rs` | Pass 2 — computes backlinks from forward refs (KEEP) | 257-268 |
| `libs/validation/src/context.rs` | Inverse map construction from schemas (KEEP) | 97-187 |
| `libs/validation/src/platform.rs` | Plugin manifest scanning — extracts inverse field (KEEP) | 304-331 |

### Current Problem

Every forward relationship requires a stored inverse on the target artifact:
- Persona files accumulate 100+ `benefited-by` entries
- Every commit touching relationships gets MissingInverse errors
- Auto-fix adds inverse entries, growing target files indefinitely
- Double maintenance burden for every relationship change

### Design Decision

- **Store forward edges only** — the traceability direction
- **Compute inverses at query time** — the graph engine already does this
- **Plugin schemas still define inverses** — semantic definition stays, storage requirement goes
- **Narrow from/to constraints stay** — specificity is the point

### Bulk Cleanup

After code changes, remove all stored inverse entries from artifacts. These are entries where the relationship type is an inverse (e.g. `benefited-by`, `delivered-by`, `fulfilled-by`, `informed-by`, etc.) that exist only because the validator required them.
