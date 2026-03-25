---
id: "EPIC-770f9ce9"
type: "epic"
title: "Artifact Graph Alignment Audit"
description: "Comprehensive audit and cleanup of all .orqa/ artifacts to align with graph-based\nknowledge injection principles, correct layer classifications, fix data integrity\nissues, and eliminate sources of context confusion.\n"
status: "completed"
priority: "P1"
created: "2026-03-12"
updated: "2026-03-12"
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---
## Governing Principles

These are the architectural principles established in recent sessions that this epic
enforces. Every task in this epic must preserve and strengthen these principles.

### 1. Graph-Based Knowledge Injection

The artifact graph is the knowledge system. Hooks inject **artifact IDs only** (TASK-NNN,
KNOW-NNN, DOC-NNN) — never content, never indexes. Agents have Read tooling and graph
traversal knowledge to resolve IDs to files on demand. This keeps injection lightweight
and context-efficient.

**Implication:** Any artifact whose content would confuse an agent if loaded via graph
traversal is a data integrity problem, not a "historical record." Completed artifacts
that describe superseded approaches must be clearly marked so the graph doesn't inject
stale patterns.

### 2. Three-Layer Architecture

| Layer | What | Portability Test |
|-------|------|-----------------|
| **Core** | Clarity engine firmware — universal principles, process model, schemas | Would this work unchanged on a Python/Django project? |
| **Project** | This project's patterns — Tauri/Svelte/Rust conventions, orqa-* skills | Does this reference OrqaStudio-specific paths, technologies, or patterns? |
| **Plugin** | CLI compatibility — hooks, commands, session management | Does this exist to make the graph work in Claude Code specifically? |

**Implication:** Core rules must not contain project-specific examples. Core skills must
not reference project-specific paths. The layer field on every artifact must be accurate.

### 3. Schema as Single Source of Truth for Field Shapes

`schema.json` defines what fields exist, their types, constraints, and relationships.
`artifact-framework.md` defines lifecycle and process semantics. READMEs provide
orientation but must not duplicate or contradict either source.

**Implication:** No README should contain a field reference table. No artifact should
have fields not in the schema. No schema should allow fields the process doesn't define.

### 4. Enforcement via Relationships

Rules have enforcement entries (block/warn/inject actions). Skills are injected based
on path patterns. The graph-guardian checks relationship integrity. Process gates fire
at transitions. This replaces content-heavy injection with lightweight ID-based injection.

**Implication:** The `.claude/rules/` symlink loading ALL 44 rule bodies into every CLI
session is an architectural tension. Core rules should be minimal principles; project
detail belongs in skills injected on demand.

### 5. Honest Status

Every artifact's status must reflect reality. A done epic with incomplete deliverables,
a draft epic whose tasks are all complete, or a todo task whose work is verified done —
these are lies in the graph that cause downstream confusion.

---

## Context

Three research investigations ([RES-b7062d7b](RES-b7062d7b), [RES-1a888755](RES-1a888755), [RES-22e4c59c](RES-22e4c59c))
audited 476+ artifacts across all layers. Critical findings:

**Data Integrity:**
- [KNOW-f5ee4e0d](KNOW-f5ee4e0d) ID assigned to 3 different skills (graph traversal breaks)
- KNOW-bcfeb64e exists as divergent copies (not symlinked)
- 20+ epics reference DOC-NNN phantom IDs (unresolvable graph edges)
- 4 different scoring dimension sets across epics (priority comparison meaningless)
- [EPIC-709a6f76](EPIC-709a6f76) fully complete but all tasks marked todo
- [EPIC-c1833545](EPIC-c1833545) marked done but superseded by [EPIC-c1833545](EPIC-c1833545)

**Layer Violations:**
- 8 core rules embed project-specific Tauri/Svelte/Rust content
- composability skill (core) has 37 project-specific references
- orqa-native-search marked core but is project-specific
- orqa-code-search marked project but treated as universal

**Content Staleness:**
- orchestration.md and workflow.md describe pre-graph patterns
- Planning README mentions deprecated "plans" artifact type
- Documentation README uses web-style links violating RULE-2f7b6a31
- [EPIC-4ce64ab0](EPIC-4ce64ab0) uses `canon` terminology (now `core`)
- [RES-5435cae9](RES-5435cae9) references non-existent `.orqa/agents/` path

**Structural:**
- Tasks README duplicates and diverges from schema.json
- No architecture doc for graph-based injection model
- 3 rules have empty scope arrays
- [RES-44b34cba](RES-44b34cba)-tauri-dev-process-management.md has mismatched filename/ID

---

## Implementation Design

Work is organised into phases by blast radius and dependency order.

### Phase 1: Data Integrity Fixes

Fix broken graph edges and status lies. These are factual corrections, not opinion.

### Phase 2: Layer Reclassification

Correct layer fields on misclassified artifacts. Split core rules/skills that contain
project-specific content.

### Phase 3: Content Accuracy

Update stale content, fix README issues, resolve the canonical definition question.

### Phase 4: Structural Cleanup

Archive stale ideas, mark surpassed research, connect orphaned artifacts, standardise
scoring dimensions.

---

## Tasks

### Phase 1: Data Integrity

| ID | Title |
|----|-------|
| [TASK-25ef9bc2](TASK-25ef9bc2) | Fix [KNOW-f5ee4e0d](KNOW-f5ee4e0d) ID collision — assign unique IDs |
| [TASK-4a65565c](TASK-4a65565c) | Fix KNOW-bcfeb64e rule-enforcement duplication — symlink or split |
| [TASK-2106d3c4](TASK-2106d3c4) | Fix epic/task status mismatches (EPIC-709a6f76, [EPIC-f079c196](EPIC-f079c196), EPIC-c1833545) |
| [TASK-a20a2c9d](TASK-a20a2c9d) | Audit [EPIC-9a1eba3f](EPIC-9a1eba3f) tasks against plugin codebase |
| [TASK-f303c4e4](TASK-f303c4e4) | Resolve DOC-NNN phantom references across all epics |
| [TASK-584583c4](TASK-584583c4) | Standardise scoring dimensions across all epics |
| [TASK-c71e1808](TASK-c71e1808) | Rename [RES-44b34cba](RES-44b34cba)-tauri-dev-process-management.md to match its ID |

### Phase 2: Layer Reclassification

| ID | Title |
|----|-------|
| [TASK-1c15bc9a](TASK-1c15bc9a) | Split 8 core rules — extract project-specific content |
| [TASK-729a39f7](TASK-729a39f7) | Split composability skill — core principle vs project examples |
| [TASK-0fcd8ea1](TASK-0fcd8ea1) | Fix skill layer misclassifications (orqa-native-search, rule-enforcement, orqa-code-search) |
| [TASK-a297df32](TASK-a297df32) | Assign scope to [RULE-09a238ab](RULE-09a238ab), [RULE-e1f1afc1](RULE-e1f1afc1), [RULE-42d17086](RULE-42d17086) |

### Phase 3: Content Accuracy

| ID | Title |
|----|-------|
| [TASK-ce9f22cc](TASK-ce9f22cc) | Fix all README inaccuracies (Planning, Documentation, Skills, Tasks, Lessons, Decisions) |
| [TASK-0f837aa7](TASK-0f837aa7) | Update orchestration.md and workflow.md for graph-based model |
| [TASK-057c1430](TASK-057c1430) | Remove scope reference from CLAUDE.md / orchestrator prompt |
| [TASK-38912ce1](TASK-38912ce1) | Update [EPIC-4ce64ab0](EPIC-4ce64ab0) body — canon → core terminology |
| [TASK-275b380d](TASK-275b380d) | Backfill missing description fields across all artifact types |

### Phase 4: Structural Cleanup

| ID | Title |
|----|-------|
| [TASK-ec7d9989](TASK-ec7d9989) | Archive stale ideas (IDEA-119c5d54, [IDEA-8087e548](IDEA-8087e548), [IDEA-cc97aab3](IDEA-cc97aab3), [IDEA-1d6675c7](IDEA-1d6675c7) status fix) |
| [TASK-bd8caa3c](TASK-bd8caa3c) | Mark surpassed research (RES-5435cae9) and connect Phase 0 orphans |
| [TASK-97e9ba49](TASK-97e9ba49) | Address .claude/rules/ full-body loading vs graph-based injection |

---

## Out of Scope

- Implementing new enforcement layers (EPIC-56940fa8 scope)
- Building auto-generation tooling for READMEs (future idea, not this epic)
- Rewriting the plugin hooks (already aligned with graph principles)
- Creating the DOC-NNN artifact type (separate epic if decided)
- Wireframe/UI doc accuracy audit (separate review needed post-implementation)