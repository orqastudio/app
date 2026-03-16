---
id: EPIC-044
title: Three-Layer Governance Classification
description: "Classify all governance artifacts (agents, skills, rules, hooks) into three layers: canon (platform principles), project (project-specific), and plugin (extensible/ecosystem). Add scope categorisation to agents (software-engineering, governance, general). This is foundational architecture enabling multi-project support."
status: completed
priority: P1
created: 2026-03-09
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-047
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-048
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-049
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-050
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-334
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
---
Note: This epic used 'canon' terminology which has since been renamed to 'core'.

## Findings Addressed

- **F25**: Agent scope needs categorisation (software-engineering vs governance vs general)
- **F26**: Skills need categorisation (project-type-specific vs universal platform)
- **F27**: Rules and hooks need canon vs project classification

## Implementation Design

### Three Layers

| Layer | Meaning | Ships With |
|-------|---------|------------|
| `core` | Platform principles — applies to ALL projects managed by OrqaStudio | The app |
| `project` | Project-specific — additive enforcement for this particular project | The project's `.orqa/` |
| `plugin` | Ecosystem-extensible — third-party or community contributions | Installed via skills CLI or plugin system |

### Agent Scope Categories

| Scope | Meaning | Examples |
|-------|---------|---------|
| `software-engineering` | Code-writing agents for software projects | backend-engineer, frontend-engineer, designer |
| `governance` | Process/governance framework agents | agent-maintainer |
| `general` | Universal agents needed for any project type | orchestrator, code-reviewer, documentation-writer |

### Classification

All current agents are `layer: core` (generic roles shipping with the platform).
All `orqa-*` skills are `layer: project`. Portable skills are `layer: core`.
Most rules are `layer: core` (platform principles). Project-specific rules are `layer: project`.

### Tasks

| Task | Title | Assignee |
|------|-------|----------|
| [TASK-047](TASK-047) | Classify agents with layer and scope fields | agent-maintainer |
| [TASK-048](TASK-048) | Classify skills with layer field | agent-maintainer |
| [TASK-049](TASK-049) | Classify rules and hooks with layer field | agent-maintainer |
| [TASK-050](TASK-050) | Update artifact-framework with classification schema | agent-maintainer |

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
