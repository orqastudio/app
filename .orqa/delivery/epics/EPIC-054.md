---
id: EPIC-6f2d06d4
title: Artifact Graph Alignment Audit
description: |
  Comprehensive audit and cleanup of all .orqa/ artifacts to align with graph-based
  knowledge injection principles, correct layer classifications, fix data integrity
  issues, and eliminate sources of context confusion.
status: completed
priority: P1
created: 2026-03-12
updated: 2026-03-12
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: RES-87c6d062
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-87c6d062
  - target: RES-5d1fa2a8
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-5d1fa2a8
  - target: RES-0a32a350
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-0a32a350
  - target: MS-eea45fa8
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-3f426c14
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-bd34be90
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-6856f61d
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-8af47bbd
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-15bc3749
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-6c46014d
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-b40669f0
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-e1d73ed7
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-84fc1180
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-507ce554
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-97ae6841
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-025a0d1e
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-b2562f5a
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-df494469
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-6e243257
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-fa3388c2
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-3655ff27
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-55c08ed7
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-3b2cea80
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-94149697
    type: delivered-by
    rationale: Epic contains this task
  - target: MS-654badde
    type: fulfils
  - target: RES-0a32a350
    type: guided-by
  - target: RES-5d1fa2a8
    type: guided-by
  - target: RES-87c6d062
    type: guided-by
---
## Governing Principles

These are the architectural principles established in recent sessions that this epic
enforces. Every task in this epic must preserve and strengthen these principles.

### 1. Graph-Based Knowledge Injection

The artifact graph is the knowledge system. Hooks inject **artifact IDs only** (TASK-NNN,
SKILL-NNN, DOC-NNN) — never content, never indexes. Agents have Read tooling and graph
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

Three research investigations ([RES-0a32a350](RES-0a32a350), [RES-5d1fa2a8](RES-5d1fa2a8), [RES-87c6d062](RES-87c6d062))
audited 476+ artifacts across all layers. Critical findings:

**Data Integrity:**
- [KNOW-e3a559c9](KNOW-e3a559c9) ID assigned to 3 different skills (graph traversal breaks)
- KNOW-bcfeb64e exists as divergent copies (not symlinked)
- 20+ epics reference DOC-NNN phantom IDs (unresolvable graph edges)
- 4 different scoring dimension sets across epics (priority comparison meaningless)
- [EPIC-0a7b21cf](EPIC-0a7b21cf) fully complete but all tasks marked todo
- [EPIC-03900ea5](EPIC-03900ea5) marked done but superseded by [EPIC-897bbe8f](EPIC-897bbe8f)

**Layer Violations:**
- 8 core rules embed project-specific Tauri/Svelte/Rust content
- composability skill (core) has 37 project-specific references
- orqa-native-search marked core but is project-specific
- orqa-code-search marked project but treated as universal

**Content Staleness:**
- orchestration.md and workflow.md describe pre-graph patterns
- Planning README mentions deprecated "plans" artifact type
- Documentation README uses web-style links violating [RULE-2f7b6a31](RULE-2f7b6a31)
- [EPIC-31c9baca](EPIC-31c9baca) uses `canon` terminology (now `core`)
- [RES-8b971acf](RES-8b971acf) references non-existent `.orqa/agents/` path

**Structural:**
- Tasks README duplicates and diverges from schema.json
- No architecture doc for graph-based injection model
- 3 rules have empty scope arrays
- [RES-5a9e6375](RES-5a9e6375)-tauri-dev-process-management.md has mismatched filename/ID

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
| [TASK-bd34be90](TASK-bd34be90) | Fix [KNOW-e3a559c9](KNOW-e3a559c9) ID collision — assign unique IDs |
| [TASK-6856f61d](TASK-6856f61d) | Fix KNOW-bcfeb64e rule-enforcement duplication — symlink or split |
| [TASK-8af47bbd](TASK-8af47bbd) | Fix epic/task status mismatches (EPIC-0a7b21cf, [EPIC-dc1e3e4b](EPIC-dc1e3e4b), EPIC-03900ea5) |
| [TASK-15bc3749](TASK-15bc3749) | Audit [EPIC-3a8ad459](EPIC-3a8ad459) tasks against plugin codebase |
| [TASK-6c46014d](TASK-6c46014d) | Resolve DOC-NNN phantom references across all epics |
| [TASK-b40669f0](TASK-b40669f0) | Standardise scoring dimensions across all epics |
| [TASK-e1d73ed7](TASK-e1d73ed7) | Rename [RES-5a9e6375](RES-5a9e6375)-tauri-dev-process-management.md to match its ID |

### Phase 2: Layer Reclassification

| ID | Title |
|----|-------|
| [TASK-84fc1180](TASK-84fc1180) | Split 8 core rules — extract project-specific content |
| [TASK-507ce554](TASK-507ce554) | Split composability skill — core principle vs project examples |
| [TASK-97ae6841](TASK-97ae6841) | Fix skill layer misclassifications (orqa-native-search, rule-enforcement, orqa-code-search) |
| [TASK-025a0d1e](TASK-025a0d1e) | Assign scope to [RULE-c95f4444](RULE-c95f4444), [RULE-f9d0279c](RULE-f9d0279c), [RULE-7f416d7d](RULE-7f416d7d) |

### Phase 3: Content Accuracy

| ID | Title |
|----|-------|
| [TASK-b2562f5a](TASK-b2562f5a) | Fix all README inaccuracies (Planning, Documentation, Skills, Tasks, Lessons, Decisions) |
| [TASK-df494469](TASK-df494469) | Update orchestration.md and workflow.md for graph-based model |
| [TASK-6e243257](TASK-6e243257) | Remove scope reference from CLAUDE.md / orchestrator prompt |
| [TASK-fa3388c2](TASK-fa3388c2) | Update [EPIC-31c9baca](EPIC-31c9baca) body — canon → core terminology |
| [TASK-3f426c14](TASK-3f426c14) | Backfill missing description fields across all artifact types |

### Phase 4: Structural Cleanup

| ID | Title |
|----|-------|
| [TASK-3655ff27](TASK-3655ff27) | Archive stale ideas (IDEA-b153fabd, [IDEA-9334b770](IDEA-9334b770), [IDEA-325a4ddc](IDEA-325a4ddc), [IDEA-889dffd4](IDEA-889dffd4) status fix) |
| [TASK-55c08ed7](TASK-55c08ed7) | Mark surpassed research (RES-8b971acf) and connect Phase 0 orphans |
| [TASK-3b2cea80](TASK-3b2cea80) | Address .claude/rules/ full-body loading vs graph-based injection |

---

## Out of Scope

- Implementing new enforcement layers (EPIC-4440cdd4 scope)
- Building auto-generation tooling for READMEs (future idea, not this epic)
- Rewriting the plugin hooks (already aligned with graph principles)
- Creating the DOC-NNN artifact type (separate epic if decided)
- Wireframe/UI doc accuracy audit (separate review needed post-implementation)
