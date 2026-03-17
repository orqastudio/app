---
id: EPIC-059
title: "Philosophy Alignment & Data Integrity"
description: "Content migration and directory reorganization to express the Knowledge Maturity Pipeline philosophy. Moves artifacts to process/delivery/documentation structure, audits all content, integrates integrity checks, and ensures app layout picks up the new structure."
status: completed
priority: P1
created: 2026-03-13
updated: 2026-03-13
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 4
  dependencies: 3
rule-overrides:
  - "rule: RULE-003"
  - "rule: RULE-032"
relationships:
  - target: RES-049
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-049
  - target: RES-048
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-048
  - target: RES-052
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-052
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-281
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-282
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-283
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-284
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-285
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-286
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-287
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-288
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-289
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-290
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-291
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-292
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-293
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-294
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-295
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-296
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-297
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-298
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-299
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-300
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-301
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-302
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-303
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-304
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-305
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-306
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-307
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-308
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-309
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-310
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-349
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-058
    type: depends-on
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
  - target: AD-042
    type: informs
  - target: AD-043
    type: informed-by
  - target: AD-044
    type: informed-by
  - target: RULE-045
    type: informed-by
  - target: PILLAR-001
    type: grounded-by
---
## Context

[EPIC-058](EPIC-058) gave every governance artifact a `relationships` array and pipeline stage fields. That was the **schema migration**. This epic is the **content migration** — ensuring every artifact's body text, organizational placement, and cross-references actually express the Knowledge Maturity Pipeline philosophy.

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

A research document ([RES-049](RES-049)) investigates how artifacts should present their relationships and metadata to users in the app, given the revised structure and pipeline philosophy.

## Implementation Design

### Phase 0: Data Integrity (BLOCKING)

Broken links and missing inverses mean the graph is lying. Nothing else starts until this is clean.

1. Integrate integrity checks into pre-commit hook (staged-file mode)
2. Add `make verify` targets
3. Fix all broken links and frontmatter refs
4. Backfill missing bidirectional inverses
5. Codify as [RULE-045](RULE-045)

### Phase 1: Directory Reorganization

1. Create AD for directory reorganization ([AD-043](AD-043))
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

1. Create AD for standards distribution pattern ([AD-044](AD-044))
2. README alignment audit — every directory README describes pipeline role
3. Update all path references across codebase
4. Research: relationship and metadata display UX ([RES-049](RES-049))

## Tasks

| ID | Title | Depends On | Phase |
|----|-------|-----------|-------|
| [TASK-281](TASK-281) | Integrate integrity checks into pre-commit hook | — | 0 |
| [TASK-282](TASK-282) | Add make verify targets | — | 0 |
| [TASK-283](TASK-283) | Fix all broken links and frontmatter refs | [TASK-281](TASK-281), [TASK-282](TASK-282) | 0 |
| [TASK-284](TASK-284) | Backfill missing bidirectional inverses | [TASK-281](TASK-281), [TASK-282](TASK-282) | 0 |
| [TASK-285](TASK-285) | Create data integrity rule (RULE-045) | [TASK-283](TASK-283), [TASK-284](TASK-284) | 0 |
| [TASK-286](TASK-286) | Create AD for directory reorganization (AD-043) | — | 1 |
| [TASK-287](TASK-287) | Move governance artifacts to process/ | [TASK-286](TASK-286) | 1 |
| [TASK-288](TASK-288) | Move team artifacts to process/ | [TASK-286](TASK-286) | 1 |
| [TASK-289](TASK-289) | Move pillars to process/ and rename planning to delivery | [TASK-286](TASK-286) | 1 |
| [TASK-290](TASK-290) | Create verification artifact type | — | 1 |
| [TASK-291](TASK-291) | Scope documentation to human-consumption content | [TASK-287](TASK-287), [TASK-288](TASK-288), [TASK-289](TASK-289) | 1 |
| [TASK-292](TASK-292) | Update project.json, READMEs, and symlinks for app layout | [TASK-287](TASK-287), [TASK-288](TASK-288), [TASK-289](TASK-289) | 1 |
| [TASK-293](TASK-293) | Rules content audit (44 rules) | [TASK-285](TASK-285) | 2 |
| [TASK-294](TASK-294) | Skills content audit (48 skills) | [TASK-285](TASK-285) | 2 |
| [TASK-295](TASK-295) | Agents content audit (7 agents) | [TASK-294](TASK-294) | 2 |
| [TASK-296](TASK-296) | Lessons maturity review (16 lessons) | [TASK-285](TASK-285) | 2 |
| [TASK-297](TASK-297) | Documentation inventory | [TASK-285](TASK-285) | 2 |
| [TASK-298](TASK-298) | Create AD for standards distribution pattern (AD-044) | — | 3 |
| [TASK-299](TASK-299) | README alignment audit | [TASK-292](TASK-292) | 3 |
| [TASK-300](TASK-300) | Update all path references across codebase | [TASK-292](TASK-292) | 3 |
| [TASK-301](TASK-301) | Research: relationship and metadata display UX (RES-049) | [TASK-292](TASK-292) | 3 |

### Phase 4: Open Items & Process Gaps (discovered during implementation)

| ID | Title | Depends On | Phase |
|----|-------|-----------|-------|
| [TASK-302](TASK-302) | Complete [RES-051](RES-051): behavioral directives research | — | 4 |
| [TASK-303](TASK-303) | Clean up Claude memory files that duplicate artifact knowledge | [TASK-302](TASK-302) | 4 |
| [TASK-304](TASK-304) | Remove ArtifactType::Hook dead code | — | 4 |
| [TASK-305](TASK-305) | Fix broken forward-references to non-existent artifacts | — | 4 |
| [TASK-306](TASK-306) | Investigate configurable .orqa/ paths (IMPL-018) | — | 4 |
| [TASK-307](TASK-307) | Design plugin-sidecar pairing mechanism (IMPL-019, IMPL-020) | — | 4 |
| [TASK-308](TASK-308) | Establish learning loop and completion discipline (IMPL-021 through 025) | — | 4 |
| [TASK-309](TASK-309) | Triage [EPIC-059](EPIC-059) observations (IMPL-018 through IMPL-025) | [TASK-308](TASK-308) | 4 |
| [TASK-310](TASK-310) | Plugin: maintain memory entries for unimplemented ADs | — | 4 |

## Out of Scope

- Populating VER-NNN verification artifacts (this epic creates the type)
- Building pipeline visualization in the app
- Plugin ecosystem implementation (TASK-307 covers the design only, not implementation)
- App UI changes for new directory structure (scanner handles it via config)
- Implementing any UX changes from [RES-049](RES-049) (that's a future epic)
- Implementing configurable paths runtime cache (TASK-306 investigates, implementation is a future epic if Option C is chosen)
