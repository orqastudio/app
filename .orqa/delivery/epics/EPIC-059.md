---
id: "EPIC-942c7678"
title: "Philosophy Alignment & Data Integrity"
description: "Content migration and directory reorganization to express the Knowledge Maturity Pipeline philosophy. Moves artifacts to process/delivery/documentation structure, audits all content, integrates integrity checks, and ensures app layout picks up the new structure."
status: "completed"
priority: "P1"
created: "2026-03-13"
updated: "2026-03-13"
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 4
  dependencies: 3
rule-overrides:
  - "rule: RULE-6c0496e0"
  - "rule: RULE-a764b2ae"
relationships:
  - target: "RES-07667737"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-07667737"
  - target: "RES-132fbb38"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-132fbb38"
  - target: "RES-cd3d33bf"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-cd3d33bf"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-e7324438"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d5d3e417"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8954343c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c4a7b6bb"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-5da55ccb"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8b9c68ae"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4b293b82"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-1dad83f7"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0cddbf9b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c42dac0c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-191958e7"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b3aadbcd"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a86c3565"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b6b5c31c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-9f1f8b4b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f6e9b767"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6e4fd8b9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a0d3f53c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c70b9d8a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7f5b6792"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7b8ded48"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2e138cb1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7fbe6eca"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0e7221a8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-65f6b166"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0bcbb927"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-53493d31"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-bd4fb2bc"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-cdfd039f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6dab59a2"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-cea1bc37"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "AD-71d44f5c"
    type: "driven-by"
  - target: "AD-8b3962f6"
    type: "driven-by"
---
## Context

[EPIC-ca7b398b](EPIC-ca7b398b) gave every governance artifact a `relationships` array and pipeline stage fields. That was the **schema migration**. This epic is the **content migration** — ensuring every artifact's body text, organizational placement, and cross-references actually express the Knowledge Maturity Pipeline philosophy.

**The schema says "here are the edges." This epic says "are the edges meaningful, are the nodes in the right places, and does the whole structure tell a coherent story?"**

### The First-Class Artifact Principle

**If an artifact is needed by the learning loop, enforcement mechanism, or delivery cycle, it must be a first-class artifact** — schema-validated, relationship-tracked, integrity-enforced.

**Documentation is a human-readable guide maintained by AI.** It explains what things mean, how to use the app, and documents the product being built. It references process artifacts but never defines them. The process artifacts define themselves.

### Directory Reorganization

```
.orqa/
  process/              # The machine — pipeline artifacts
    pillars/            #   PILLAR-NNN — guiding principles
    lessons/            #   IMPL-NNN — observations & understanding
    decisions/          #   AD-NNN — architectural principles
    rules/              #   RULE-NNN — enforcement
    skills/             #   Skill directories — practice
    agents/             #   Agent definitions — roles

  delivery/             # The work — idea to verified outcome
    ideas/              #   IDEA-NNN
    research/           #   RES-NNN
    milestones/         #   MS-NNN
    epics/              #   EPIC-NNN
    tasks/              #   TASK-NNN
    verification/       #   VER-NNN — verification records (new)

  documentation/        # The human layer — guides, product docs
    guide/              #   What it all means, how to use the app
    product/            #   Vision, roadmap, product narrative
    architecture/       #   Contributor-facing architecture overview
    development/        #   Dev setup, commands, coding standards ref
```

Key changes from current structure:
- `governance/` dissolved — lessons, decisions, rules move to `process/`
- `governance/hooks/` removed — hooks are plugin implementation, not artifacts
- `team/` dissolved — skills, agents move to `process/`
- `planning/` renamed to `delivery/`, pillars move to `process/`
- `verification/` added under `delivery/` (new artifact type)
- `documentation/` stays but scoped to human-consumption content only

### App Layout Integration

After directory moves, `project.json` artifact config and all directory READMEs must be updated so the artifact scanner renders the new three-level structure correctly. Every directory needs README frontmatter (icon, label, description, sort) that the scanner extracts for nav tree rendering.

### Relationship & Metadata Display Research

A research document ([RES-07667737](RES-07667737)) investigates how artifacts should present their relationships and metadata to users in the app, given the revised structure and pipeline philosophy.

## Implementation Design

### Phase 0: Data Integrity (BLOCKING)

Broken links and missing inverses mean the graph is lying. Nothing else starts until this is clean.

1. Integrate integrity checks into pre-commit hook (staged-file mode)
2. Add `make verify` targets
3. Fix all broken links and frontmatter refs
4. Backfill missing bidirectional inverses
5. Codify as [RULE-130f1f63](RULE-130f1f63)

### Phase 1: Directory Reorganization

1. Create AD for directory reorganization ([AD-71d44f5c](AD-71d44f5c))
2. Move governance artifacts to `process/`
3. Move team artifacts to `process/`
4. Move pillars to `process/`, rename `planning/` to `delivery/`
5. Create verification artifact type (VER-NNN)
6. Scope `documentation/` to human-consumption content
7. Update `project.json`, README frontmatter, `.claude/` symlinks for app layout

### Phase 2: Content Alignment Audit

1. Rules content audit (44 rules)
2. Skills content audit (48 skills)
3. Agents content audit (7 agents)
4. Lessons maturity review (16 lessons)
5. Documentation inventory

### Phase 3: Pipeline Traceability

1. Create AD for standards distribution pattern ([AD-8b3962f6](AD-8b3962f6))
2. README alignment audit — every directory README describes pipeline role
3. Update all path references across codebase
4. Research: relationship and metadata display UX ([RES-07667737](RES-07667737))

## Tasks

| ID | Title | Depends On | Phase |
|----|-------|-----------|-------|
| [TASK-e7324438](TASK-e7324438) | Integrate integrity checks into pre-commit hook | — | 0 |
| [TASK-d5d3e417](TASK-d5d3e417) | Add make verify targets | — | 0 |
| [TASK-8954343c](TASK-8954343c) | Fix all broken links and frontmatter refs | [TASK-e7324438](TASK-e7324438), [TASK-d5d3e417](TASK-d5d3e417) | 0 |
| [TASK-c4a7b6bb](TASK-c4a7b6bb) | Backfill missing bidirectional inverses | [TASK-e7324438](TASK-e7324438), [TASK-d5d3e417](TASK-d5d3e417) | 0 |
| [TASK-5da55ccb](TASK-5da55ccb) | Create data integrity rule (RULE-130f1f63) | [TASK-8954343c](TASK-8954343c), [TASK-c4a7b6bb](TASK-c4a7b6bb) | 0 |
| [TASK-8b9c68ae](TASK-8b9c68ae) | Create AD for directory reorganization (AD-71d44f5c) | — | 1 |
| [TASK-4b293b82](TASK-4b293b82) | Move governance artifacts to process/ | [TASK-8b9c68ae](TASK-8b9c68ae) | 1 |
| [TASK-1dad83f7](TASK-1dad83f7) | Move team artifacts to process/ | [TASK-8b9c68ae](TASK-8b9c68ae) | 1 |
| [TASK-0cddbf9b](TASK-0cddbf9b) | Move pillars to process/ and rename planning to delivery | [TASK-8b9c68ae](TASK-8b9c68ae) | 1 |
| [TASK-c42dac0c](TASK-c42dac0c) | Create verification artifact type | — | 1 |
| [TASK-191958e7](TASK-191958e7) | Scope documentation to human-consumption content | [TASK-4b293b82](TASK-4b293b82), [TASK-1dad83f7](TASK-1dad83f7), [TASK-0cddbf9b](TASK-0cddbf9b) | 1 |
| [TASK-b3aadbcd](TASK-b3aadbcd) | Update project.json, READMEs, and symlinks for app layout | [TASK-4b293b82](TASK-4b293b82), [TASK-1dad83f7](TASK-1dad83f7), [TASK-0cddbf9b](TASK-0cddbf9b) | 1 |
| [TASK-a86c3565](TASK-a86c3565) | Rules content audit (44 rules) | [TASK-5da55ccb](TASK-5da55ccb) | 2 |
| [TASK-b6b5c31c](TASK-b6b5c31c) | Skills content audit (48 skills) | [TASK-5da55ccb](TASK-5da55ccb) | 2 |
| [TASK-9f1f8b4b](TASK-9f1f8b4b) | Agents content audit (7 agents) | [TASK-b6b5c31c](TASK-b6b5c31c) | 2 |
| [TASK-f6e9b767](TASK-f6e9b767) | Lessons maturity review (16 lessons) | [TASK-5da55ccb](TASK-5da55ccb) | 2 |
| [TASK-6e4fd8b9](TASK-6e4fd8b9) | Documentation inventory | [TASK-5da55ccb](TASK-5da55ccb) | 2 |
| [TASK-a0d3f53c](TASK-a0d3f53c) | Create AD for standards distribution pattern (AD-8b3962f6) | — | 3 |
| [TASK-c70b9d8a](TASK-c70b9d8a) | README alignment audit | [TASK-b3aadbcd](TASK-b3aadbcd) | 3 |
| [TASK-7f5b6792](TASK-7f5b6792) | Update all path references across codebase | [TASK-b3aadbcd](TASK-b3aadbcd) | 3 |
| [TASK-7b8ded48](TASK-7b8ded48) | Research: relationship and metadata display UX (RES-07667737) | [TASK-b3aadbcd](TASK-b3aadbcd) | 3 |

### Phase 4: Open Items & Process Gaps (discovered during implementation)

| ID | Title | Depends On | Phase |
|----|-------|-----------|-------|
| [TASK-2e138cb1](TASK-2e138cb1) | Complete [RES-c4dd4c4d](RES-c4dd4c4d): behavioral directives research | — | 4 |
| [TASK-7fbe6eca](TASK-7fbe6eca) | Clean up Claude memory files that duplicate artifact knowledge | [TASK-2e138cb1](TASK-2e138cb1) | 4 |
| [TASK-0e7221a8](TASK-0e7221a8) | Remove ArtifactType::Hook dead code | — | 4 |
| [TASK-65f6b166](TASK-65f6b166) | Fix broken forward-references to non-existent artifacts | — | 4 |
| [TASK-0bcbb927](TASK-0bcbb927) | Investigate configurable .orqa/ paths (IMPL-c306b136) | — | 4 |
| [TASK-53493d31](TASK-53493d31) | Design plugin-sidecar pairing mechanism (IMPL-e22b63b4, IMPL-a97eccb6) | — | 4 |
| [TASK-bd4fb2bc](TASK-bd4fb2bc) | Establish learning loop and completion discipline (IMPL-97e2788f through 025) | — | 4 |
| [TASK-cdfd039f](TASK-cdfd039f) | Triage [EPIC-942c7678](EPIC-942c7678) observations (IMPL-c306b136 through IMPL-257c8303) | [TASK-bd4fb2bc](TASK-bd4fb2bc) | 4 |
| [TASK-6dab59a2](TASK-6dab59a2) | Plugin: maintain memory entries for unimplemented ADs | — | 4 |

## Out of Scope

- Populating VER-NNN verification artifacts (this epic creates the type)
- Building pipeline visualization in the app
- Plugin ecosystem implementation (TASK-53493d31 covers the design only, not implementation)
- App UI changes for new directory structure (scanner handles it via config)
- Implementing any UX changes from [RES-07667737](RES-07667737) (that's a future epic)
- Implementing configurable paths runtime cache (TASK-0bcbb927 investigates, implementation is a future epic if Option C is chosen)
