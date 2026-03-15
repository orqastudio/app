---
id: EPIC-054
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
milestone: MS-001
horizon: null
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on: []
blocks: []
research-refs:
  - RES-040
  - RES-041
  - RES-042
docs-required: []
docs-produced: []
scoring:
  dogfood-value: 5
  user-facing: 3
  foundation: 5
  complexity: 4
  score: 4.25
relationships:
  - target: RES-042
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-042
  - target: RES-041
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-041
  - target: RES-040
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-040
  - target: MS-002
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-224
    type: contains
    rationale: Epic contains this task
  - target: TASK-225
    type: contains
    rationale: Epic contains this task
  - target: TASK-226
    type: contains
    rationale: Epic contains this task
  - target: TASK-227
    type: contains
    rationale: Epic contains this task
  - target: TASK-228
    type: contains
    rationale: Epic contains this task
  - target: TASK-229
    type: contains
    rationale: Epic contains this task
  - target: TASK-230
    type: contains
    rationale: Epic contains this task
  - target: TASK-231
    type: contains
    rationale: Epic contains this task
  - target: TASK-232
    type: contains
    rationale: Epic contains this task
  - target: TASK-233
    type: contains
    rationale: Epic contains this task
  - target: TASK-234
    type: contains
    rationale: Epic contains this task
  - target: TASK-235
    type: contains
    rationale: Epic contains this task
  - target: TASK-236
    type: contains
    rationale: Epic contains this task
  - target: TASK-237
    type: contains
    rationale: Epic contains this task
  - target: TASK-238
    type: contains
    rationale: Epic contains this task
  - target: TASK-239
    type: contains
    rationale: Epic contains this task
  - target: TASK-240
    type: contains
    rationale: Epic contains this task
  - target: TASK-241
    type: contains
    rationale: Epic contains this task
  - target: TASK-242
    type: contains
    rationale: Epic contains this task
  - target: TASK-344
    type: contains
    rationale: Epic contains this task
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

Three research investigations ([RES-040](RES-040), [RES-041](RES-041), [RES-042](RES-042))
audited 476+ artifacts across all layers. Critical findings:

**Data Integrity:**
- [SKILL-046](SKILL-046) ID assigned to 3 different skills (graph traversal breaks)
- SKILL-045 exists as divergent copies (not symlinked)
- 20+ epics reference DOC-NNN phantom IDs (unresolvable graph edges)
- 4 different scoring dimension sets across epics (priority comparison meaningless)
- [EPIC-051](EPIC-051) fully complete but all tasks marked todo
- [EPIC-007](EPIC-007) marked done but superseded by [EPIC-039](EPIC-039)

**Layer Violations:**
- 8 core rules embed project-specific Tauri/Svelte/Rust content
- composability skill (core) has 37 project-specific references
- orqa-native-search marked core but is project-specific
- orqa-code-search marked project but treated as universal

**Content Staleness:**
- orchestration.md and workflow.md describe pre-graph patterns
- Planning README mentions deprecated "plans" artifact type
- Documentation README uses web-style links violating [RULE-034](RULE-034)
- [EPIC-044](EPIC-044) uses `canon` terminology (now `core`)
- [RES-024](RES-024) references non-existent `.orqa/agents/` path

**Structural:**
- Tasks README duplicates and diverges from schema.json
- No architecture doc for graph-based injection model
- 3 rules have empty scope arrays
- [RES-016](RES-016)-tauri-dev-process-management.md has mismatched filename/ID

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
| [TASK-225](TASK-225) | Fix [SKILL-046](SKILL-046) ID collision — assign unique IDs |
| [TASK-226](TASK-226) | Fix SKILL-045 rule-enforcement duplication — symlink or split |
| [TASK-227](TASK-227) | Fix epic/task status mismatches (EPIC-051, [EPIC-053](EPIC-053), EPIC-007) |
| [TASK-228](TASK-228) | Audit [EPIC-050](EPIC-050) tasks against plugin codebase |
| [TASK-229](TASK-229) | Resolve DOC-NNN phantom references across all epics |
| [TASK-230](TASK-230) | Standardise scoring dimensions across all epics |
| [TASK-231](TASK-231) | Rename [RES-016](RES-016)-tauri-dev-process-management.md to match its ID |

### Phase 2: Layer Reclassification

| ID | Title |
|----|-------|
| [TASK-232](TASK-232) | Split 8 core rules — extract project-specific content |
| [TASK-233](TASK-233) | Split composability skill — core principle vs project examples |
| [TASK-234](TASK-234) | Fix skill layer misclassifications (orqa-native-search, rule-enforcement, orqa-code-search) |
| [TASK-235](TASK-235) | Assign scope to [RULE-041](RULE-041), [RULE-042](RULE-042), [RULE-043](RULE-043) |

### Phase 3: Content Accuracy

| ID | Title |
|----|-------|
| [TASK-236](TASK-236) | Fix all README inaccuracies (Planning, Documentation, Skills, Tasks, Lessons, Decisions) |
| [TASK-237](TASK-237) | Update orchestration.md and workflow.md for graph-based model |
| [TASK-238](TASK-238) | Remove scope reference from CLAUDE.md / orchestrator prompt |
| [TASK-239](TASK-239) | Update [EPIC-044](EPIC-044) body — canon → core terminology |
| [TASK-224](TASK-224) | Backfill missing description fields across all artifact types |

### Phase 4: Structural Cleanup

| ID | Title |
|----|-------|
| [TASK-240](TASK-240) | Archive stale ideas (IDEA-025, [IDEA-032](IDEA-032), [IDEA-045](IDEA-045), [IDEA-057](IDEA-057) status fix) |
| [TASK-241](TASK-241) | Mark surpassed research (RES-024) and connect Phase 0 orphans |
| [TASK-242](TASK-242) | Address .claude/rules/ full-body loading vs graph-based injection |

---

## Out of Scope

- Implementing new enforcement layers (EPIC-052 scope)
- Building auto-generation tooling for READMEs (future idea, not this epic)
- Rewriting the plugin hooks (already aligned with graph principles)
- Creating the DOC-NNN artifact type (separate epic if decided)
- Wireframe/UI doc accuracy audit (separate review needed post-implementation)
