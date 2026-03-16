---
id: EPIC-075
title: Documentation reorganisation — book-chapter structure with how-to guides
description: "Reorganise .orqa/documentation/ from ad-hoc sections (architecture/, product/, process/, ui/, wireframes/) into book chapters (about/, guide/, development/, how-to/, reference/, grounding/). Write missing how-to guides for plugin SDK, testing, and linting. Migrate relationships and project.json paths."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: active
scoring: null
relationships:
  - target: RES-063
    type: informed-by
    rationale: Documentation audit and migration inventory drive this epic
  - target: IDEA-095
    type: informed-by
    rationale: Documentation-as-graph-knowledge idea initiated this work
  - target: EPIC-064
    type: informed-by
    rationale: Documentation restructuring started in EPIC-064 Phase 1
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-484
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-484"
  - target: TASK-488
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-488"
  - target: TASK-489
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-489"
  - target: TASK-485
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-485"
  - target: TASK-486
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-486"
  - target: TASK-487
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-487"
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
  - target: RES-063
    type: informs
---
## Context

RES-063 produced a complete migration inventory: 58 files mapped, 46 need moving. The architecture/ and process/ directories dissolve. product/ becomes about/. ui/ and wireframes/ consolidate into reference/. A new how-to/ chapter needs 10 guides written (3 P1).

DOC-ID relationships (187 references) are stable across moves. 152 body-text path references across 71 files need updating.

## Implementation Design

### Migration Strategy (per RES-063 recommendation: atomic per source directory)

1. Move product/ → about/ (11 files)
2. Move architecture/ → development/ and reference/ (18 files)
3. Move process/ → guide/, about/, development/, reference/ (6 files)
4. Move ui/ → reference/ (6 files)
5. Move wireframes/ → reference/wireframes/ (5 files)
6. Update project.json artifact config
7. Sweep body-text path references
8. Write P1 how-to guides
9. Add grounding/ to project.json (pre-existing gap)

## Tasks

- [ ] [TASK-484](TASK-484): Move product/ → about/ (11 files) + update project.json
- [ ] [TASK-485](TASK-485): Move architecture/ → development/ and reference/ (18 files) + assign missing DOC IDs
- [ ] [TASK-486](TASK-486): Move process/ and ui/ and wireframes/ → target chapters (17 files)
- [ ] [TASK-487](TASK-487): Sweep body-text path references across all .orqa/ files
- [ ] [TASK-488](TASK-488): Write P1 how-to guides (plugin SDK, Rust testing, frontend testing)
- [ ] [TASK-489](TASK-489): Add grounding/ to project.json + create chapter READMEs
