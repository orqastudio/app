---
id: KNOW-477f2c9c
type: knowledge
title: Agentic Workflow and Enforcement Pipeline
description: |
summary: "|. This is the master reference for how OrqaStudio works as a system. It describes the complete pipeline from user request to committed code, covering agent delegation, knowledge injection, artifact graph traversal, schema-driven validation, and the three-layer enforcement stack."
  The complete OrqaStudio execution model: how user requests become agent-delegated work,
  how knowledge flows into agent context via declared and semantic injection, how the
  artifact graph connects everything, and how the three-layer enforcement stack (LSP,
  behavioral rules, pre-commit) validates quality. Use when: understanding the system
  architecture, designing new enforcement, configuring agent knowledge, adding artifact
  types, or explaining how OrqaStudio works.
status: active
created: 2026-03-24
updated: 2026-03-24
category: architecture
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-e16aea3b
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
  - target: KNOW-22783288
    type: related
    rationale: "CLI architecture is the developer interface into this workflow"
  - target: KNOW-dd5062c9
    type: related
    rationale: "Shared validation engine implements enforcement Layers 1 and 3"
  - target: KNOW-1f4aba8f
    type: related
    rationale: "Three-layer enforcement model is the detailed enforcement breakdown"
  - target: KNOW-586bfa9a
    type: related
    rationale: "Knowledge auto-injection is the detailed injection mechanism"
---

## Purpose

This is the master reference for how OrqaStudio works as a system. It describes the complete pipeline from user request to committed code, covering agent delegation, knowledge injection, artifact graph traversal, schema-driven validation, and the three-layer enforcement stack.

---

## System Model

```
User request
    │
    ▼
Orchestrator (coordinates, never implements)
    │
    ├── Queries artifact graph for context
    ├── Assigns tasks to specialist agents
    │
    ▼
Agent spawned with knowledge
    │
    ├── Declared knowledge (employs relationships → always loaded)
    ├── Semantic knowledge (ONNX search on task description → context-dependent)
    ├── Required Reading (governing documentation)
    │
    ▼
Agent produces work
    │
    ▼
Independent Reviewer verifies
    │
    ├── PASS → commit
    └── FAIL → fix cycle
```

---

## Agent Roles and Boundaries

| Role | May Do | May NOT Do |
|------|--------|-----------|
| **Orchestrator** | Coordinate, delegate, query graph, write session state | Write implementation code, run tests |
| **Implementer** | Write/edit code, run builds | Self-certify quality |
| **Researcher** | Read files, search code, investigate | Modify any files |
| **Planner** | Read files, design approaches | Write code or modify files |
| **Reviewer** | Run quality checks (shell), read files | Edit files, fix issues |
| **Writer** | Write/edit documentation files | Run shell commands |
| **Designer** | Write/edit UI components | Run shell commands |

Boundaries are enforced by **capability restrictions** — each role has a defined set of tool capabilities it may use (file_read, file_edit, shell_execute, etc.). Capabilities resolve to provider-specific tools at delegation time.

---

## Knowledge Injection

### Mechanism 1: Declared (Deterministic)

Agent YAML frontmatter contains `employs` relationships:

```yaml
relationships:
  - target: KNOW-xxxxxxxx
    type: employs
```

At spawn time: platform reads agent definition → follows all `employs` edges → loads target knowledge artifacts into context.

**Same agent type = same base knowledge every time.**

### Mechanism 2: Semantic Search (Dynamic)

At spawn time: platform takes task description → runs ONNX embedding search against knowledge corpus → injects top-N relevant results (deduplicated against declared knowledge).

**Same agent type + different task = different supplemental knowledge.**

### Enforcement

Hooks enforce injection at every agent spawn. Agents cannot be created without going through the injection pipeline. A per-session dedup cache prevents re-injection.

---

## The Artifact Graph

### Structure

- **Nodes** = Markdown files with YAML frontmatter in `.orqa/` directories
- **Edges** = Relationships declared in frontmatter (`relationships` array)
- **Types** = Defined by plugin schemas (rule, knowledge, doc, agent, idea, decision, etc.)

### Relationship Semantics

| Semantic Group | Key Relationships | Purpose |
|---------------|-------------------|---------|
| Foundation | upholds, grounded, benefits, serves, revises | Anchor work to vision/pillars/personas |
| Lineage | crystallises, spawns, merged-into | Track artifact evolution |
| Governance | drives, governs, enforces, codifies, promoted-to | Connect decisions → rules → lessons |
| Knowledge flow | informs, teaches, guides, cautions, documents | Route knowledge between artifacts |
| Agency | employs | Connect agents to knowledge |
| Synchronisation | synchronised-with | Pair doc + knowledge artifacts |

### Graph Query

The daemon serves the graph in memory. All queries go through it:

- CLI: `orqa graph --type rule --status active`
- MCP: `graph_query({ type: "rule", status: "active" })`
- Both route to the same daemon API.

---

## Schema-Driven Validation

### Schema Source

Plugins declare schemas in `orqa-plugin.json` under `provides.schemas`. Each schema defines:

- `frontmatter` — JSON Schema for required fields, types, enums
- `statusTransitions` — legal state changes
- `idPrefix` — ID format (RULE, KNOW, DOC, etc.)

### Single Validation Engine

`libs/validation/` is one TypeScript library consumed by three adapters:

| Adapter | When | Response |
|---------|------|----------|
| LSP (`orqa lsp`) | Real-time | Red squiggles, warnings |
| CLI (`orqa check`) | On demand | Error report, exit code |
| Pre-commit | On commit | Hard block |

No adapter implements its own validation logic. All read schemas through the shared engine.

---

## The Three-Layer Enforcement Stack

### Layer 1: LSP (Real-Time, Mechanical)

Validates against plugin schemas as you type:
- Invalid status values, wrong relationship types, missing required fields, broken references, type mismatches
- **Response**: Instant editor diagnostics (squiggles, completions)
- **Character**: Deterministic. No judgement. If the schema defines it, LSP validates it.

### Layer 2: Behavioral Rules (Prompt Injection, Judgement)

Injected into agent context at delegation time:
- Documentation-before-code, delegation boundaries, pillar alignment, process sequencing, honest reporting
- **Response**: Agent follows the rule as part of its instructions
- **Character**: Requires judgement. Cannot be reduced to a schema check.

### Layer 3: Pre-Commit (Hard Gate, Final)

Runs `orqa check` on every commit:
- All schema validation (safety net for Layer 1), linting, type checking, tests
- **Response**: Commit blocked on any failure
- **Character**: Nothing passes. Cannot be bypassed with `--no-verify`.

### Layer Interaction

LSP catches schema errors instantly. Behavioral rules guide agent decisions during work. Pre-commit catches anything that slipped through both.

---

## Plugin Architecture

### Canonical Source

Plugins (`plugins/<name>/`) are the source of truth for schemas, agents, knowledge, rules, and docs. The `.orqa/` directory contains **installed copies**, not originals.

### Content Flow

```
Plugin source → orqa install → .orqa/ (installed copy) → local edits → three-way diff on refresh
```

### Three-Way Diff

On `orqa plugin refresh`, compare:
1. **Plugin source** (new version)
2. **Installed baseline** (what was last synced, from `manifest.json`)
3. **Project copy** (current `.orqa/` state, may have local edits)

Detects both plugin updates AND user local edits.

### What Plugins Provide

| Content | Plugin Location | Installed To |
|---------|----------------|-------------|
| Agents | `agents/` | `.orqa/process/agents/` |
| Knowledge | `knowledge/` | `.orqa/process/knowledge/` |
| Rules | `rules/` | `.orqa/process/rules/` |
| Docs | `docs/` | `.orqa/documentation/` |
| Schemas | `orqa-plugin.json` | Read by validation engine (not installed as files) |
| Relationships | `orqa-plugin.json` | Read by validation engine (not installed as files) |

---

## Doc + Knowledge Pairing

Every documentation page has a paired knowledge artifact:

| Type | Audience | Content Style |
|------|----------|--------------|
| Doc (DOC-xxx) | Human developers | Explanatory, contextual, with examples |
| Knowledge (KNOW-xxx) | AI agents | Structured tables, decision rules, forbidden patterns |

Linked by `synchronised-with`. Updated together in the same commit. Both live in the same plugin so they travel together on install/update.

---

## Stability Tracking (Rule Demotion)

When a behavioral rule (Layer 2) becomes mechanically covered (Layer 1/3):

1. **Demote** — `status: inactive` + demotion metadata
2. **Track** — stability counter increments each clean session, resets on violation
3. **Delete** — counter reaches threshold → surfaced as safe to delete

### Demotable

Rules about valid statuses, relationship types, required fields, ID formats.

### Not Demotable

Rules requiring judgement: documentation-first, delegation boundaries, pillar alignment, honest reporting.

---

## The Full Loop

```
1. Write artifact         → LSP validates real-time (Layer 1)
2. Agent spawned          → Knowledge injected (declared + semantic)
3. Agent works            → Behavioral rules guide decisions (Layer 2)
4. Agent submits work     → Reviewer verifies independently
5. Developer commits      → Pre-commit gates (Layer 3)
6. Committed + pushed     → Work is done
```

**Everything is schema-driven.** Extend a plugin schema → validation engine picks it up → all three enforcement layers enforce it automatically.

---

## Agent Actions

| Situation | Action |
|-----------|--------|
| Adding a new artifact type | Define schema in plugin `orqa-plugin.json`. Validation engine picks it up. |
| Adding a new relationship type | Add to plugin `provides.relationships`. Graph and validation honor it. |
| Configuring agent knowledge | Add `employs` relationship in agent YAML frontmatter. |
| Adding task-relevant knowledge | Create knowledge artifact with clear title/description. Semantic search surfaces it. |
| Designing enforcement for a new rule | Determine layer: schema → LSP/pre-commit; judgement → behavioral. |
| Understanding missing knowledge | Check employs relationships + semantic search index freshness. |

---

## FORBIDDEN

- Hardcoding valid statuses, relationship types, or artifact types outside of plugin schemas
- Implementing validation logic in a consumer instead of the shared engine
- Bypassing pre-commit with `--no-verify`
- Spawning agents without going through the knowledge injection pipeline
- Orchestrator writing implementation code instead of delegating
- Reviewer fixing issues instead of reporting them
- Updating a doc without updating its paired knowledge (or vice versa)
