---
id: "EPIC-0497a1be"
type: "epic"
title: "Business logic deduplication — daemon delegation model"
description: "Consolidate 7 categories of business logic duplication across the codebase by migrating JavaScript/TypeScript reimplementations to delegate to the canonical Rust validation crate via the daemon HTTP API. Eliminates logic drift risk in graph scanning, schema validation, status transitions, frontmatter parsing, relationship validation, Cytoscape analysis, and type inference."
status: review
priority: "P1"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
horizon: "active"
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 2
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Dogfooding requires single source of truth — duplicate logic causes divergent behaviour in the app being dogfooded"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "Clarity Through Structure — one canonical implementation per concern eliminates ambiguity about which copy is authoritative"
  - target: "PILLAR-2acd86c1"
    type: "grounded"
    rationale: "Learning Through Reflection — the audit identified a systemic pattern (JS reimplementing Rust) that must be encoded as an enforceable constraint"
---

## Context

A full-codebase duplication audit (2026-03-24) found 7 categories of business logic duplicated across `libs/`, `app/`, `connectors/`, and `plugins/`. The canonical source of truth is `libs/validation/` (the `orqa_validation` Rust crate). Multiple JavaScript/TypeScript consumers have reimplemented logic that should be delegated to the Rust crate via the daemon HTTP API.

The codebase already has the correct pattern established in `connectors/claude-code/src/hooks/validate-artifact.ts` — a thin adapter that calls `POST /parse` on the daemon with zero business logic. Every duplicated implementation should follow this pattern.

### Research References

- `.state/business-logic-duplication-audit.md` — full audit findings with file locations, risk assessment, and consolidation priorities
- [RES-16fd5aea](RES-16fd5aea) — source of truth duplication audit research artifact

## Architectural Pattern: Daemon Delegation Model

```text
Pre-commit hook / CLI command / MCP tool
        |
        v
   Daemon HTTP API  ---------> orqa_validation (Rust)
   (libs/mcp-server)            (libs/validation)
        |
        v
   JSON response with findings
```

All hooks, CLI commands, and MCP tools should be thin adapters that call the daemon. Zero business logic in JavaScript/TypeScript.

## Implementation Design

### Priority 1 — Fix Now (logic drift actively causing or will cause bugs)

**P1a: Status transition hooks** (category 3 from audit)

- `app/.githooks/validate-status-transitions.mjs` has hardcoded `VALID_TRANSITIONS` map that WILL drift from the config-driven Rust definitions in `app/backend/src-tauri/src/domain/status_transitions.rs`
- Replace with daemon delegation: hook calls `orqa validate` or daemon `/parse` endpoint
- Remove the hardcoded transition map entirely

**P1b: MCP server frontmatter trim fix** (category 5, copy 2)

- `libs/mcp-server/src/graph.rs::extract_frontmatter()` is missing `.trim()` on the frontmatter text, producing subtly different parse results from the canonical `libs/validation/src/graph.rs::extract_frontmatter()`
- Fix: re-export `extract_frontmatter` from `orqa_validation` directly, or add the missing `.trim()`

### Priority 2 — Fix Soon (significant maintenance burden, divergence risk)

**P2a: CLI graph scanner** (category 2)

- `libs/cli/src/lib/graph.ts` (~225 lines) is a complete TypeScript reimplementation of `libs/validation/src/graph.rs`
- `inferType()` uses path-only inference with a hardcoded allowlist of 14 type names vs the Rust version's frontmatter-first approach
- CLI graph commands should call daemon HTTP API instead
- Connector re-exports from `connectors/claude-code/src/index.ts` should also migrate

**P2b: Schema validation hooks** (category 4)

- Three independent schema builders: Rust (`libs/validation/src/checks/schema.rs`), app hooks (`app/.githooks/validate-schema.mjs`), plugin hooks (`plugins/githooks/hooks/validate-frontmatter.mjs`)
- Migrate both JS hooks to delegate to daemon `/parse` endpoint

**P2c: Cytoscape analysis functions** (category 1)

- `libs/graph-visualiser/src/analysis.ts` reimplements health metrics, BFS traversal, PageRank, knowledge gap detection
- Dead functions: `computeGraphHealth()`, `computeBackboneArtifacts()`, `computeKnowledgeGaps()`, `computeImpact()`
- Add missing Rust equivalents (PageRank, knowledge gaps, impact analysis) to `libs/validation/src/metrics.rs`
- Strip `analysis.ts` to a thin fetch layer calling daemon endpoints
- Keep Cytoscape ONLY for layout/rendering in `elements.ts`

### Priority 3 — Fix When Touching (low immediate risk, high long-term burden)

**P3a: Frontmatter parsing copies** (category 5)

- 7 copies across Rust, SDK, app UI, connector, and CLI
- SDK (`libs/sdk/src/utils/frontmatter.ts`) should expose one canonical TS implementation
- App UI (`app/ui/src/lib/utils/frontmatter.ts`) should import from SDK
- CLI should have one shared utility in `libs/cli/src/lib/`

**P3b: Relationship validation hooks** (category 6)

- App hooks (`app/.githooks/validate-relationships.mjs`) and plugin hooks (`plugins/githooks/hooks/validate-relationships.mjs`) — both should delegate to daemon
- App version loads rich metadata (Map), plugin version loads only keys (Set) — different validation depth

**P3c: Type inference consolidation** (category 7)

- Three implementations with different strategies (frontmatter-first, path-only, strip-trailing-s)
- Subsumed by P2a (CLI graph scanner migration)

### Standalone fix: Pre-commit hook naming bug (from SoT Finding 6)

- `plugins/githooks/hooks/pre-commit.sh` has `.sh` extension — Git hooks via `core.hooksPath` must not have extensions
- The pre-commit hook may not be firing in plugin consumers
- Also: pre-commit hook shows `integer expression expected` bug (line 57) when stdin buffer has multi-line content

## Tasks

| ID | Title | Priority | Depends On |
| ---- | ------- | ---------- | ------------ |
| [TASK-8ba5ac58](TASK-8ba5ac58) | Replace status transition hook with daemon delegation | P1 | — |
| [TASK-c18c4cae](TASK-c18c4cae) | Fix MCP server frontmatter missing trim | P1 | — |
| [TASK-a32d1929](TASK-a32d1929) | Migrate CLI graph scanner to daemon API | P2 | — |
| [TASK-849b82b4](TASK-849b82b4) | Migrate schema validation hooks to daemon delegation | P2 | — |
| [TASK-2ab6cb6f](TASK-2ab6cb6f) | Consolidate Cytoscape analysis to Rust daemon endpoints | P2 | — |
| [TASK-bad9cd52](TASK-bad9cd52) | Consolidate frontmatter parsing to SDK single export | P3 | — |
| [TASK-14c5b276](TASK-14c5b276) | Migrate relationship validation hooks to daemon delegation | P3 | — |
| [TASK-f7bc3afd](TASK-f7bc3afd) | Fix pre-commit hook extension and integer expression bug | P1 | — |
| [TASK-11090f14](TASK-11090f14) | Reconcile EPIC-0497a1be | — | all above |

## Out of Scope

To be confirmed with user.
