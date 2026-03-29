---
id: "RES-94a3a6ca"
type: discovery-research
title: "Documentation Drift Audit"
description: "Comprehensive audit of contradictions and drift between documentation, rules, skills, lessons, schemas, and implementation code."
status: completed
created: "2026-03-10"
updated: "2026-03-10"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "guides"
    rationale: "Research findings informed epic design"
  - target: "RES-27120af2"
    type: "merged-into"
  - target: "RES-e9566e49"
    type: "merged-into"
  - target: "RES-9e03dcdc"
    type: "merged-into"
---

## Purpose

Dogfooding and rapid iteration have caused drift between documentation, governance artifacts, schemas, and the actual implementation. This audit catalogs every contradiction found so the user can decide what the canon should be for each.

## Methodology

Four parallel research agents audited different domains:

1. **Architecture docs vs implementation** — compared `.orqa/documentation/development/` against actual Rust, TypeScript, and sidecar source code
2. **Rules & skills consistency** — checked `.orqa/process/rules/` and `.orqa/process/skills/` for internal contradictions and outdated references
3. **Product/process docs & READMEs** — audited `.orqa/documentation/about/`, `guide/`, `development/`, schemas, and README files
4. **Cross-references & artifact integrity** — verified all inter-artifact references, status consistency, and promotion chains

---

## Findings

### A. Systemic Issues (appear across many files)

These are patterns of drift that affect multiple files and should be resolved as a single decision.

#### A1. Pillar field name: `test-questions` vs `gate`

The pillar schema uses `gate`. Multiple rules, docs, and the orchestrator itself reference `test-questions` which does not exist.

| File | References `test-questions` |
| ------ | ----------------------------- |
| [RULE-1b238fc8](RULE-1b238fc8) lines 19, 23, 39 | "read each pillar's `test-questions`" |
| [RULE-05562ed4](RULE-05562ed4) line 30 | "Each pillar has a `title`, `description`, `test-questions`, and `priority` field" |
| CLAUDE.md (orchestrator) line 361 | "read each pillar's `test-questions`" |
| governance.md (DOC-06224bf6) lines 23, 26 | References `test-questions` field |
| vision.md (VISION-4893db55) line 247 | "pillar artifacts are authoritative for test-questions" |
| artifact-framework.md (DOC-28344cd7) line 621 | Lists `test-questions` in field ordering |
| [KNOW-936e5944](KNOW-936e5944) line 303 | "read each pillar's `test-questions`" |
| **Schema (source of truth)** | **`pillars/schema.json` uses `gate`** |
| **Actual pillar files** | **[PILLAR-c9e0a695](PILLAR-c9e0a695).md, [PILLAR-2acd86c1](PILLAR-2acd86c1).md use `gate:`** |

**Decision needed:** Rename schema field to `test-questions`, or update all 7+ references to use `gate`.

#### A2. Artifact directory paths: missing `planning/` and `governance/` prefixes

Multiple docs reference flat `.orqa/` paths (e.g., `.orqa/epics/`) but the actual structure uses `.orqa/implementation/epics/` and `.orqa/process/lessons/`.

| File | Wrong paths referenced |
| ------ | ----------------------- |
| artifact-framework.md (DOC-28344cd7) lines 170-181, 738-759 | `.orqa/milestones/`, `.orqa/epics/`, `.orqa/tasks/`, `.orqa/ideas/`, `.orqa/lessons/`, `.orqa/research/`, `.orqa/decisions/` |
| [RULE-b10fe6d1](RULE-b10fe6d1) lines 23-32 | `.orqa/ideas/`, `.orqa/epics/`, `.orqa/research/`, `.orqa/tasks/`, `.orqa/milestones/`, `.orqa/lessons/`, `.orqa/decisions/` |
| [RULE-ec9462d8](RULE-ec9462d8) line 22 | `.orqa/epics/` |
| [RULE-c603e90e](RULE-c603e90e) lines 15, 19, 21, 47, 69 | `.orqa/lessons/` |
| [RULE-5dd9decd](RULE-5dd9decd) lines 60, 62 | `.orqa/lessons/` |
| **Actual paths** | **`.orqa/implementation/{milestones,epics,tasks,ideas,research}/` and `.orqa/process/{lessons,decisions,rules}/`** |

**Decision needed:** Update all references to use full paths.

#### A3. Deprecated plan artifact type references

Plans were deprecated ([PD-3b986859](PD-3b986859)) and merged into Research/Epics. Multiple files still reference them.

| File | References plans |
| ------ | ----------------- |
| [RULE-87ba1b81](RULE-87ba1b81) line 60 | "Plan files (`.orqa/implementation/plans/`)" |
| [RULE-484872ef](RULE-484872ef) line 32 | Lists "Plans (`.orqa/implementation/plans/`)" |
| [RULE-8ee65d73](RULE-8ee65d73) line 23 | "The epic's plan in `.orqa/implementation/plans/`" |
| [RULE-dccf4226](RULE-dccf4226) title/body | "plan-mode-compliance" — governs planning activity, not plan artifacts |
| DOC-028 line 43 | "If the epic has a `plan` field" |
| DOC-f6c4ac69 line 142 | "Follow the plan referenced by the epic's `plan` field" |
| [KNOW-0619a413](KNOW-0619a413) line 101 | `parse_plan_frontmatter` code example |
| [EPIC-9ddef7f9](EPIC-9ddef7f9) line 94 | References `.orqa/plans/epic-005-artifact-browser.md` |
| [EPIC-c1833545](EPIC-c1833545) line 33 | References `.orqa/plans/composability-gate.md` |

**Decision needed:** Remove all plan references from rules/docs. Update [RULE-dccf4226](RULE-dccf4226) body to clarify it governs planning activity.

#### A4. Stale lesson file path references

Multiple docs reference a monolithic `development/lessons.md` file. Lessons are now individual `IMPL-NNN.md` files in `.orqa/process/lessons/`.

| File | References old path |
| ------ | --------------------- |
| DOC-028 line 55 | "development/lessons.md checked for known patterns" |
| DOC-027 line 86 | "patterns discovered during this task logged in `development/lessons.md`" |
| DOC-db5b37dc line 24 | "Checked `.orqa/documentation/development/lessons.md`" |
| DOC-939d8636 line 139 | "Tracked in `development/lessons.md`" |

**Decision needed:** Update all to reference `.orqa/process/lessons/`.

#### A5. `docs/` path references (directory doesn't exist)

The `planning` skill and `composability` skill reference `docs/` paths. No `docs/` directory exists — all documentation is under `.orqa/documentation/`.

| File | Wrong paths |
| ------ | ------------- |
| [KNOW-21d28aa0](KNOW-21d28aa0) lines 47-59, 171, 228 | `docs/ui/`, `docs/architecture/decisions.md`, `docs/product/roadmap.md`, etc. |
| [KNOW-21d28aa0](KNOW-21d28aa0) lines 225-227 | `.orqa/rules/plan-mode-compliance.md` (should be `.orqa/process/rules/[RULE-dccf4226](RULE-dccf4226).md`) |
| [KNOW-0619a413](KNOW-0619a413) line 419 | `docs/architecture/decisions.md` |

**Decision needed:** Update all to `.orqa/documentation/...` paths.

#### A6. `.claude/` write directives contradict `.orqa/` source of truth

[RULE-4603207a](RULE-4603207a) directs writing to `.claude/rules/`, `.claude/skills/`, `.claude/agents/`. [RULE-63cc16ad](RULE-63cc16ad) and MEMORY.md say "NEVER write to `.claude/`."

| File | Contradiction |
| ------ | --------------- |
| [RULE-4603207a](RULE-4603207a) lines 23-27 | "Location: `.claude/rules/`", "`.claude/skills/`", "`.claude/agents/`" |
| [RULE-63cc16ad](RULE-63cc16ad) | "NEVER write directly to `.claude/` directories — always write to `.orqa/` source of truth" |

**Decision needed:** Update [RULE-4603207a](RULE-4603207a) to reference `.orqa/` paths.

---

### B. Architecture Documentation vs Implementation

These are docs that describe systems significantly different from what's built.

| # | Doc | Claim | Implementation Reality | Severity |
| --- | ----- | ------- | ------------------------ | ---------- |
| B1 | error-taxonomy.md (DOC-bcd7fef4) | 8 nested error enums (DatabaseError, IpcError, SidecarError, etc.) with `IpcErrorPayload` struct, `user_message()` method, custom Serialize impl | Flat `OrqaError` enum with 9 string variants. `#[derive(Serialize)]` with `#[serde(tag="code", content="message")]`. No nested enums, no custom serialization. | **Critical** — entire doc is fiction |
| B2 | ipc-commands.md (DOC-23175cea) | 39 commands across 11 modules | ~81 commands across 15 modules. Missing: setup_commands (6), lesson_commands (5), governance_commands (7), enforcement_commands (3), graph_commands (8), plus others | **Critical** — missing >50% of commands |
| B3 | rust-modules.md (DOC-921ab420) | 7 domain modules, 6 repos, 39 commands, AppState with 4 fields | 27 domain modules, 10 repos, ~81 commands, AppState with 9 fields. Claims `domain/` depends on nothing — violated by tool_executor.rs importing AppState and repos | **High** |
| B4 | svelte-components.md (DOC-2c94f7ba) | Lists `commands/` directory, 6 stores, specific component names | `commands/` dir doesn't exist. 10+ stores. Many listed components don't exist (ConversationView, SessionHeader, EmptyState, ErrorDisplay). Missing 4 entire component directories (governance/, enforcement/, lessons/, setup/) | **High** |
| B5 | streaming-pipeline.md (DOC-39e2fb81) | Type is `ProviderEvent` with 9 variants. Handler is `sidecars/claude-agentsdk-sidecar/stream.rs` with `StreamHandler` struct | Type is `StreamEvent` with 16 variants. Handler is `domain/stream_loop.rs`. No `stream.rs` file exists. | **High** |
| B6 | tool-definitions.md (DOC-ec909ab0) | 6 tools, `orqa_` prefix, tools run as separate MCP server with JSON-RPC 2.0, stored in `message_blocks` table | 10+ tools, no prefix, tools run in-process via `tool_executor.rs`, no `message_blocks` table | **High** |
| B7 | sqlite-schema.md (DOC-e3a0462c) | 1 migration file. No `provider_session_id` or `title_manually_set` columns. Lessons table "NOT YET CREATED" | 5+ migration files. Both columns exist. Lesson repo implemented. | **Medium** |
| B8 | search-engine.md (DOC-07f98a90) | `code_research` is "Future — Phase 4". Commands return `Result\<T, String\>` | `code_research` is active. Commands return `Result\<T, OrqaError\>` | **Low** |
| B9 | rust-modules.md | "watcher/ — deferred to later phase", "tools/ — MCP tool implementations deferred" | Both implemented: `watcher.rs` exists, `tool_executor.rs` implements native tools | **Medium** |

---

### C. Rules & Skills Internal Issues

| # | File | Issue | Reality | Severity |
| --- | ------ | ------- | --------- | ---------- |
| C1 | [RULE-05562ed4](RULE-05562ed4) line 30 | Claims pillars have `priority` field | Pillar schema has no `priority` field | **Medium** |
| C2 | [RULE-1b238fc8](RULE-1b238fc8) line 45 | "the pillar with the lower `priority` number takes precedence" | No `priority` field exists. [RULE-1b238fc8](RULE-1b238fc8) itself says (line 33) "Pillars are equal in importance — there is no numeric priority ranking" — contradicts itself | **High** |
| C3 | [RULE-c603e90e](RULE-c603e90e) lines 22, 25, 36, 69 | References `agent-maintainer` role | No `agent-maintainer` agent exists in `.orqa/process/agents/` | **Low** |
| C4 | [KNOW-936e5944](KNOW-936e5944) | Shows epic scoring dimensions: `dogfood-value`, `foundation`, `user-visible`, `scope`, `dependency-risk` | DOC-28344cd7 defines: `pillar`, `impact`, `dependency`, `effort`. Both coexist in actual epics — old vs new | **Medium** |
| C5 | [KNOW-936e5944](KNOW-936e5944) | Task frontmatter shows `status: surpassed` and `layer:` field | Task schema only allows `todo`, `in-progress`, `done`. No `layer` field. | **Medium** |
| C6 | [KNOW-936e5944](KNOW-936e5944) | Lesson frontmatter shows `category:` and `tags:` fields | Lesson schema has `additionalProperties: false` — no `category` or `tags` allowed | **Medium** |
| C7 | DOC-28344cd7 | Rule schema requires `slug` field | Rule schema.json has no `slug`. No rule file uses `slug`. | **Medium** |
| C8 | DOC-28344cd7 | Milestone schema has `epic-count` and `completed-epics` fields | Milestone schema.json does not include these. No milestone uses them. | **Medium** |
| C9 | DOC-28344cd7 | Decision schema omits `category` field | Decision schema.json includes `category` in propertyOrder | **Low** |
| C10 | DOC-d9cc1f84 lines 94-108 | Lists specific agent names: `backend-engineer`, `frontend-engineer` | [PD-48b310f9](PD-48b310f9) universal model uses role+skills (Implementer + backend skills), not named agents | **Medium** |
| C11 | DOC-68a7420e line 32 | "All 15 agents" | 7 universal roles per [PD-48b310f9](PD-48b310f9): Orchestrator, Researcher, Planner, Implementer, Reviewer, Writer, Designer | **Medium** |
| C12 | DOC-c43c7d5d lines 146-201 | Documents `.orqa/hookify/` directory | Directory does not exist | **Medium** |
| C13 | DOC-d2c2063a | `make dev-sidecar` described as hot reload | Target removed — use `make build-sidecar` instead | **Resolved** |
| C14 | DOC-d2c2063a | `make install` described as "Install frontend Node.js dependencies" | Also does `cd sidecar && bun install` and `cargo fetch` | **Low** |

---

### D. Cross-Reference & Artifact Integrity Issues

| # | Source | Reference | Issue | Severity |
| --- | -------- | ----------- | ------- | ---------- |
| D1 | [PD-859ed163](PD-859ed163) | Missing `supersedes: [PD-75bb14ae](PD-75bb14ae)` | [PD-75bb14ae](PD-75bb14ae) has `superseded-by: [PD-859ed163](PD-859ed163)` but [PD-859ed163](PD-859ed163) does NOT have `supersedes: [PD-75bb14ae](PD-75bb14ae)`. One-sided supersession. | **High** |
| D2 | [EPIC-489c0a47](EPIC-489c0a47) | `status: in-progress` | All 10 tasks are `done`. Epic should be `review` or `done`. | **High** |
| D3 | [EPIC-57dd7d4c](EPIC-57dd7d4c) | `status: in-progress` | All 9 tasks are `done`. Epic should be `review` or `done`. | **High** |
| D4 | [EPIC-2f1efbd5](EPIC-2f1efbd5) | `status: in-progress` | Zero tasks reference this epic. In-progress with no tasks. | **High** |
| D5 | [IMPL-91d951b6](IMPL-91d951b6), [IMPL-286bdc1f](IMPL-286bdc1f) | `promoted-to: [RULE-63cc16ad](RULE-63cc16ad)` | [RULE-63cc16ad](RULE-63cc16ad) has no `promoted-from` back-reference. | **Medium** |
| D6 | [IMPL-5b380b2e](IMPL-5b380b2e), [IMPL-42dd183e](IMPL-42dd183e) | `promoted-to: [RULE-71352dc8](RULE-71352dc8)` | [RULE-71352dc8](RULE-71352dc8) has no `promoted-from` back-reference. | **Medium** |
| D7 | [IMPL-53fc59b5](IMPL-53fc59b5) | `promoted-to: [RULE-f609242f](RULE-f609242f)` | [RULE-f609242f](RULE-f609242f) has no `promoted-from` back-reference. | **Medium** |
| D8 | [RES-8c29af5d](RES-8c29af5d) | `status: draft` | Referenced by done [EPIC-7394ba2a](EPIC-7394ba2a). Research should be `complete`. | **Medium** |
| D9 | [RES-f7bd7ab1](RES-f7bd7ab1), [RES-156f2188](RES-156f2188) | `status: draft` | Referenced by review [EPIC-d45b4dfd](EPIC-d45b4dfd). Research should be `complete`. | **Medium** |
| D10 | [EPIC-7394ba2a](EPIC-7394ba2a) | `milestone: [MS-21d5096a](MS-21d5096a)` | [EPIC-7394ba2a](EPIC-7394ba2a) is `done` but [MS-21d5096a](MS-21d5096a) is `planning`. | **Medium** |
| D11 | [EPIC-489c0a47](EPIC-489c0a47) | `scoring` dimensions | Uses `user-value`, `pillar-alignment`, `dependency-weight`, `effort`, `risk` — different from [MS-b1ac0a20](MS-b1ac0a20) convention (`dogfood-value`, `foundation`, `user-visible`, `scope`, `dependency-risk`) | **Medium** |
| D12 | Rules schema | No `promoted-from` field | [RULE-b10fe6d1](RULE-b10fe6d1) requires bidirectional promotion chain integrity, but the rules schema doesn't define `promoted-from` and only [RULE-83411442](RULE-83411442) has it | **Medium** |
| D13 | [IDEA-7ff4a905](IDEA-7ff4a905), [IDEA-63444da6](IDEA-63444da6) | Missing `evolves-into` field | All other ideas include it (as null or value). Minor consistency gap. | **Low** |
| D14 | [IDEA-bac195b7](IDEA-bac195b7) | `status: promoted` | Skipped `exploring` and `shaped` — forbidden transition per [RULE-b10fe6d1](RULE-b10fe6d1). Likely backfilled. | **Low** |
| D15 | [TASK-d3085ce2](TASK-d3085ce2) | `epic: [EPIC-560cf78c](EPIC-560cf78c)` | Task is `todo`, parent epic is `draft`. Task created prematurely. | **Low** |

---

## Summary

| Category | Count | High | Medium | Low |
| ---------- | ------- | ------ | -------- | ----- |
| A. Systemic (multi-file) | 6 themes | 3 | 2 | 1 |
| B. Architecture docs vs code | 9 | 4 | 3 | 2 |
| C. Rules & skills internal | 14 | 1 | 9 | 4 |
| D. Cross-references & integrity | 15 | 4 | 8 | 3 |
| **Total distinct findings** | **~80** | **12** | **22** | **10** |

The systemic issues (A1-A6) account for the bulk of individual file changes needed. Fixing A1 alone touches 7+ files. Fixing A2 touches 6+ files. The architecture docs (B1-B6) are the most severely drifted — some describe systems that were never built as described.
