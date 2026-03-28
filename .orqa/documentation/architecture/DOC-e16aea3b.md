---
id: DOC-e16aea3b
type: doc
status: active
title: OrqaStudio Agentic Workflow and Enforcement Pipeline
domain: architecture
description: "The core concept of OrqaStudio: how agents do structured work, how knowledge flows into their context, how the artifact graph connects everything, and how the three-layer enforcement stack ensures quality from real-time diagnostics through hard commit gates."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-477f2c9c
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
  - target: DOC-22783288
    type: related
    rationale: "CLI architecture is the developer interface into this workflow"
  - target: DOC-dd5062c9
    type: related
    rationale: "Shared validation engine implements Layer 1 and Layer 3 of the enforcement stack"
  - target: DOC-1f4aba8f
    type: related
    rationale: "Three-layer enforcement model is the detailed breakdown of the enforcement stack described here"
  - target: DOC-586bfa9a
    type: related
    rationale: "Knowledge auto-injection is the detailed breakdown of knowledge injection described here"
---

# OrqaStudio Agentic Workflow and Enforcement Pipeline

## Overview

OrqaStudio is a structured thinking framework for software projects. The core idea: **agents do the work, the framework ensures quality.**

A human describes what they want. An orchestrator breaks the request into tasks and delegates each task to a specialist agent. Each agent receives domain knowledge automatically, produces artifacts or code, and submits work for independent review. The framework validates everything at three layers: real-time diagnostics as you type, behavioral rules that guide agent judgement, and a hard gate that blocks bad commits.

Nothing in this pipeline is hardcoded. Plugin schemas define what artifact types exist, what statuses are valid, what relationships connect them. The validation engine reads those schemas. The enforcement stack reads the validation engine. Adding a new artifact type or relationship means writing a schema — the pipeline picks it up automatically.

## The Agentic Workflow

### Request to Delivery

```text
User request
    │
    ▼
Orchestrator
    ├── Breaks request into tasks
    ├── Queries the artifact graph for context
    ├── Assigns each task to a specialist agent
    │
    ▼
Agent spawned with:
    ├── Role (Implementer, Researcher, Reviewer, Writer, Designer, Planner)
    ├── Declared knowledge (from agent employs relationships)
    ├── Task-relevant knowledge (from ONNX semantic search)
    ├── Governing documentation (Required Reading)
    │
    ▼
Agent produces work
    ├── Code changes, artifact modifications, documentation
    │
    ▼
Independent Reviewer
    ├── Verifies against acceptance criteria
    ├── PASS → work committed
    └── FAIL → feedback to implementing agent → fix cycle
```text

### Agent Roles

| Role | What They Do | What They Cannot Do |
| ------ | ------------- | ------------------- |
| **Orchestrator** | Coordinates, delegates, reports status | Write implementation code |
| **Implementer** | Build code and deliverables | Self-certify quality |
| **Researcher** | Investigate and gather information | Modify files |
| **Planner** | Design approaches, map dependencies | Write code |
| **Reviewer** | Check quality, produce PASS/FAIL verdicts | Fix the issues they find |
| **Writer** | Create and edit documentation | Run system commands |
| **Designer** | Design interfaces and experiences | Run system commands |

These boundaries are enforced by capability restrictions — each role has a defined set of tools it may use. A Reviewer can run quality checks but cannot edit files. A Researcher can read files but cannot write them.

### The Orchestrator's Job

The orchestrator does not implement. It:

1. **Queries the artifact graph** to understand current state before every decision
2. **Delegates** each piece of work to the right role with the right knowledge loaded
3. **Verifies** agent output against acceptance criteria
4. **Reports** status honestly to the user

The orchestrator coordinates work across agents but never writes implementation code itself. If the orchestrator is editing source files, the delegation model has broken down.

## Knowledge Injection

Agents need domain knowledge to work correctly. OrqaStudio provides two complementary injection mechanisms that ensure agents always have the right context.

### Declared Knowledge (Agent Definition)

Every agent definition includes `employs` relationships pointing to knowledge artifacts. When an agent is spawned, the platform reads these relationships and loads the target knowledge into the agent's context automatically.

This is **deterministic** — the same agent type always receives the same foundational knowledge. An Implementer always gets composability patterns, search methodology, and reasoning frameworks.

### Semantic Knowledge (Task-Relevant)

When a task is assigned, the platform runs a semantic search using ONNX embeddings against all knowledge artifacts. The task description is the query. Top results are injected into the agent's context, deduplicated against already-loaded declared knowledge.

This is **dynamic** — an Implementer working on "add a Tauri command for artifact validation" receives different task-relevant knowledge than one working on "refactor the Svelte store layer."

### How They Combine

```text
Agent spawn
    │
    ├── Agent definition → employs → Declared knowledge (always loaded)
    │
    ├── Task description → ONNX semantic search → Task-relevant knowledge (context-dependent)
    │
    └── Dedup + inject into agent context
```text

The hook system enforces this at every agent spawn. Agents cannot be created without going through the knowledge injection pipeline.

## Schema-Driven Validation

Everything is defined by schemas. Plugins provide `schema.json` files that declare:

- **Artifact types** — what types exist (rule, knowledge, agent, doc, idea, epic, etc.)
- **Required fields** — what frontmatter every artifact of that type must have
- **Valid statuses** — what states an artifact can be in
- **Status transitions** — which state changes are legal
- **Valid relationships** — what relationship types can connect which artifact types

The validation engine reads these schemas. It never hardcodes valid values. Adding a new status to a schema makes it immediately valid across all three enforcement layers.

### Three Consumers, One Engine

The shared validation engine (`libs/validation/`) is consumed by three adapters:

| Consumer | When | Response |
| ---------- | ------ | ---------- |
| **LSP** (`orqa lsp`) | Real-time, on every file save | Red squiggles, warnings, completions |
| **CLI** (`orqa check`) | On demand | Human-readable error report |
| **Pre-commit** (`.githooks/pre-commit`) | On every `git commit` | Commit blocked if errors |

No consumer implements its own validation logic. All three read the same schemas through the same engine.

## The Enforcement Stack

Governance enforcement operates in three layers, each handling a different class of violations.

### Layer 1: LSP Real-Time Diagnostics

The LSP server (`orqa lsp`) runs in the developer's editor and provides instant feedback:

| Check | Example |
| ------- | --------- |
| Invalid statuses | `status: enabled` when the schema says `active/inactive/archived` |
| Wrong relationship types | `type: synced-with` when the schema defines `synchronised-with` |
| Missing required fields | A rule without an `enforcement` array |
| Broken artifact references | `target: KNOW-999999` pointing to nothing |
| Type mismatches | A string where an array is expected |

Layer 1 checks are **mechanical and deterministic**. If the schema defines it, the LSP validates it. No judgement required.

### Layer 2: Behavioral Rules (Prompt Injection)

Some governance cannot be reduced to a schema check. These are enforced as behavioral rules injected into agent context:

| Check | Example |
| ------- | --------- |
| Documentation-before-code | Agent must verify docs exist before implementing |
| Delegation boundaries | Orchestrator must not write code; Reviewer must not fix issues |
| Pillar alignment | Every feature must serve at least one active pillar |
| Process sequencing | Artifacts must exist before implementation begins |
| Honest reporting | Completion reports must include "What Is NOT Done" section |

Layer 2 rules require **judgement** — they cannot be mechanically verified by a schema. They are loaded into agent context at delegation time so agents follow them as part of their instructions.

### Layer 3: Pre-Commit Hard Gate

The final line of defense. The pre-commit hook calls `orqa check` which runs:

- All schema validation (redundant with LSP, but a safety net)
- Language-specific linting (Rust clippy, ESLint)
- Type checking (svelte-check, TypeScript compiler)
- Test suite execution
- Artifact schema validation

Nothing passes without passing all checks. The hook cannot be bypassed with `--no-verify`.

### How the Layers Interact

```text
Developer edits artifact
    │
    ├── Layer 1: LSP flags invalid status immediately (red squiggle)
    │
    ├── Layer 2: Agent follows behavioral rules while working (documentation-first, delegation)
    │
    └── Layer 3: Pre-commit blocks the commit if anything slipped through
```text

Each layer catches different things. LSP catches schema errors in real time. Behavioral rules guide agent decisions. Pre-commit catches everything that made it past both.

## The Artifact Graph

OrqaStudio's data model is a **graph** where nodes are artifacts and edges are relationships.

### Nodes

Artifacts are markdown files with YAML frontmatter, stored in `.orqa/` directories. Each artifact has a type (rule, knowledge, doc, agent, idea, decision, etc.), an ID (`RULE-05ae2ce7`, `KNOW-586bfa9a`), and typed metadata in its frontmatter.

### Edges

Relationships are declared in YAML frontmatter:

```yaml
relationships:
  - target: KNOW-e89753ad
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
  - target: AD-a44384d1
    type: implements
    rationale: "Documents the CLI architecture decision"
```text

Every relationship type is defined by a plugin schema — valid types, which artifact types can be source/target, inverses, semantic groupings. The validation engine enforces that only valid relationship types are used.

### Semantic Groupings

Relationships are grouped by semantic meaning:

| Semantic | Relationships | Purpose |
| ---------- | -------------- | --------- |
| **Foundation** | upholds, grounded, benefits, serves, revises | Anchor work to vision, pillars, personas |
| **Lineage** | crystallises, spawns, merged-into | Track how artifacts evolve |
| **Governance** | drives, governs, enforces, codifies, promoted-to | Connect decisions to rules to lessons |
| **Knowledge flow** | informs, teaches, guides, cautions, documents | Route knowledge between artifacts |
| **Agency** | employs | Connect agents to their knowledge |
| **Synchronisation** | synchronised-with | Pair doc + knowledge artifacts |

### Graph Query

The daemon serves the graph in memory. All tools query it through the daemon's API:

```bash
orqa graph --type rule --status active      # Find active rules
orqa graph --related-to KNOW-586bfa9a       # Find related artifacts
orqa graph --id DOC-e16aea3b                # Read one artifact
```text

MCP tools expose the same queries to AI agents (`graph_query`, `graph_resolve`, `graph_relationships`).

## Plugin Architecture

Plugins are the **canonical source of truth** for schemas, agents, knowledge, rules, and documentation. The `.orqa/` directory in a project contains installed copies, not the originals.

### The Flow

```text
Plugin source (plugins/<name>/)
    │
    orqa install
    │
    ▼
Installed copy (.orqa/)
    │
    Developer edits locally
    │
    ▼
Three-way diff on next orqa plugin refresh
```text

### Three-Way Diff Model

When a plugin is updated, OrqaStudio compares three versions:

1. **Plugin source** — the new version from the plugin
2. **Installed baseline** — what was installed last time (recorded in `manifest.json`)
3. **Project copy** — what exists in `.orqa/` right now (may have local edits)

This detects both plugin updates and user local edits, and merges them intelligently.

### What Plugins Provide

| Content Type | Source | Installed To |
| ------------- | -------- | ------------- |
| Agents | `plugins/<name>/agents/` | `.orqa/process/agents/` |
| Knowledge | `plugins/<name>/knowledge/` | `.orqa/process/knowledge/` |
| Rules | `plugins/<name>/rules/` | `.orqa/process/rules/` |
| Docs | `plugins/<name>/docs/` | `.orqa/documentation/` |
| Schemas | `orqa-plugin.json` provides.schemas | Used by validation engine |
| Relationships | `orqa-plugin.json` provides.relationships | Used by validation engine |

Schemas and relationships are not installed as files — they are declared in `orqa-plugin.json` and read directly by the validation engine and graph query tools.

## Documentation + Knowledge Pairing

Every documentation page has a paired knowledge artifact. They contain the same information structured for different audiences:

| Artifact | Audience | Purpose |
| ---------- | ---------- | --------- |
| **Doc** (DOC-xxxxxxxx) | Human developers | Explain concepts, provide context, show examples |
| **Knowledge** (KNOW-xxxxxxxx) | AI agents | Structured rules, decision tables, forbidden patterns |

The pair is linked by a `synchronised-with` relationship. When one is updated, the other must be updated in the same commit to stay in sync.

Both live in the same plugin. This ensures they travel together when the plugin is installed, updated, or refreshed.

## Stability Tracking

Behavioral rules (Layer 2) can be demoted to inactive when mechanical enforcement (Layer 1 or 3) covers the same check. Demotion enters a stability tracking period:

1. **Demote** — set `status: inactive`, add demotion metadata to the rule
2. **Track** — the stability tracker checks for violations each session
3. **Count** — each clean session increments the counter; any violation resets it
4. **Delete** — when the counter reaches the threshold (default 10), the rule is surfaced as safe to delete

This prevents premature deletion (removing a rule before its replacement is proven) and stale rules (keeping inactive rules forever because nobody checks).

### What Can Be Demoted

Rules about valid statuses, relationship types, required fields, ID formats — anything the LSP or pre-commit hook now validates mechanically.

### What Cannot Be Demoted

Rules requiring judgement — documentation-first, delegation boundaries, pillar alignment, honest reporting, process sequencing. These will always require behavioral enforcement.

## The Full Loop

The complete lifecycle from writing to committed code:

```text
1. Write artifact
       │
2. LSP validates in real-time (Layer 1)
       │  ← Red squiggles for schema violations
       │
3. Agent receives knowledge injection
       │  ← Declared (employs) + semantic (ONNX search)
       │
4. Agent produces work following behavioral rules (Layer 2)
       │  ← Documentation-first, delegation boundaries, honest reporting
       │
5. orqa check validates on demand (shared engine)
       │  ← Schema + linting + type checking + tests
       │
6. Pre-commit gates the commit (Layer 3)
       │  ← Hard block on any violation
       │
7. Committed and pushed
```text

At every stage, the enforcement is **schema-driven**. Plugin schemas define what is valid. The validation engine reads those schemas. The enforcement layers read the validation engine. Nothing is hardcoded. Extend the schema, and enforcement follows automatically.

## Related Documents

- [KNOW-477f2c9c](KNOW-477f2c9c) — Agent-facing knowledge pair for this documentation page
- [DOC-22783288](DOC-22783288) — CLI Architecture (the developer interface into this workflow)
- [DOC-dd5062c9](DOC-dd5062c9) — Shared Validation Engine (implements enforcement Layers 1 and 3)
- [DOC-1f4aba8f](DOC-1f4aba8f) — Three-Layer Enforcement Model (detailed breakdown)
- [DOC-586bfa9a](DOC-586bfa9a) — Knowledge Auto-Injection (detailed injection mechanisms)
- [DOC-a16b7bc7](DOC-a16b7bc7) — Demoted Rule Stability Tracking (rule lifecycle after demotion)
