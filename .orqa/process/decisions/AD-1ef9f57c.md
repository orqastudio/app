---
id: "AD-1ef9f57c"
type: discovery-decision
title: "Team design v2 — open question resolutions"
status: active
created: 2026-03-25T00:00:00.000Z
updated: 2026-03-25T00:00:00.000Z
description: "Resolutions for the 10 open questions raised in RES-d6e8ab11 (Agent Team Design v2). Covers workflow inheritance, guard language, cross-plugin coupling, versioning, summary generation, latency, budgets, daemon boundary, migration strategy, and backwards compatibility."
relationships: []
---

## Decision

The following resolutions were agreed for the open questions in RES-d6e8ab11:

### Architecture Questions

| # | Question | Resolution |
|---|----------|-----------|
| 1 | Workflow inheritance vs composition | **No inheritance.** Plugin owns its complete state machine. Add extension points only if clear need emerges. |
| 2 | Guard expression language | **Declarative only.** Field checks, relationship checks, graph queries. Code hooks for anything beyond that. |
| 3 | Cross-plugin workflow coordination | **Implicit coupling within a domain is acceptable** as long as the expected vocabulary (artifact type names, status values) is documented. Alternative plugins that provide the same stage must use the same terminology. |
| 4 | Workflow versioning | **Forward-compatible addition only.** No backwards compatibility during pre-release. Breaking changes expected and necessary. Data migrated via `orqa migrate`. |

### Token Efficiency Questions

| # | Question | Resolution |
|---|----------|-----------|
| 5 | Compressed summary generation | **Author writes summaries** (including agent authors). `summary` field in frontmatter. `orqa summarize` CLI generates drafts. |
| 6 | On-demand retrieval latency | **Acceptable.** 1-2s per query paid once at task start beats 10x token cost compounding throughout agent lifetime. |
| 7 | Token budget granularity | **Per-agent for prompt size, per-session for total cost.** Team-level adds complexity without benefit. |

### Process Questions

| # | Question | Resolution |
|---|----------|-----------|
| 8 | MCP server as application boundary | **No — daemon is the business logic boundary.** MCP and LSP are access protocols. Prompt generation belongs in the daemon. |
| 9 | Migration timeline | **Incremental, one epic, sequential tasks with validation between each.** Start with one plugin as proving ground. |
| 10 | Backwards compatibility during transition | **Short fallback period only.** CLAUDE.md loading as safety net while the LLM performs migration. Remove fallback code after migration complete. |

## Rationale

These resolutions follow the research recommendations in most cases, with three user-directed corrections:

1. **No backwards compatibility** (Q4, Q10) — we are pre-release. Carrying migration debt slows the architecture work that defines the product.
2. **Daemon, not MCP** (Q8) — the daemon is where business logic lives. MCP is an access method, not an application boundary.
3. **One epic for migration** (Q9) — sequential with validation gates catches integration issues early.

## Consequences

- Workflow engine does not need inheritance/extension machinery in v1
- Guard language stays simple — no custom DSL
- Plugin documentation must declare expected vocabulary for cross-plugin queries
- All migration work is scoped to a single epic with sequential delivery
- `.state/` directory renamed to `.state/` (see AD-8727f99a)
