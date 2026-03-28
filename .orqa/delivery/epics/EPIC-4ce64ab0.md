---
id: "EPIC-4ce64ab0"
type: "epic"
title: "Three-Layer Governance Classification"
description: "Classify all governance artifacts (agents, skills, rules, hooks) into three layers: canon (platform principles), project (project-specific), and plugin (extensible/ecosystem). Add scope categorisation to agents (software-engineering, governance, general). This is foundational architecture enabling multi-project support."
status: archived
priority: "P1"
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---
Note: This epic used 'canon' terminology which has since been renamed to 'core'.

## Findings Addressed

- **F25**: Agent scope needs categorisation (software-engineering vs governance vs general)
- **F26**: Skills need categorisation (project-type-specific vs universal platform)
- **F27**: Rules and hooks need canon vs project classification

## Implementation Design

### Three Layers

| Layer | Meaning | Ships With |
| ------- | --------- | ------------ |
| `core` | Platform principles — applies to ALL projects managed by OrqaStudio | The app |
| `project` | Project-specific — additive enforcement for this particular project | The project's `.orqa/` |
| `plugin` | Ecosystem-extensible — third-party or community contributions | Installed via skills CLI or plugin system |

### Agent Scope Categories

| Scope | Meaning | Examples |
| ------- | --------- | --------- |
| `software-engineering` | Code-writing agents for software projects | backend-engineer, frontend-engineer, designer |
| `governance` | Process/governance framework agents | agent-maintainer |
| `general` | Universal agents needed for any project type | orchestrator, code-reviewer, documentation-writer |

### Classification

All current agents are `layer: core` (generic roles shipping with the platform).
All `orqa-*` skills are `layer: project`. Portable skills are `layer: core`.
Most rules are `layer: core` (platform principles). Project-specific rules are `layer: project`.

### Tasks

| Task | Title | Assignee |
| ------ | ------- | ---------- |
| [TASK-d8f219b7](TASK-d8f219b7) | Classify agents with layer and scope fields | agent-maintainer |
| [TASK-7e3796b7](TASK-7e3796b7) | Classify skills with layer field | agent-maintainer |
| [TASK-bba9cf98](TASK-bba9cf98) | Classify rules and hooks with layer field | agent-maintainer |
| [TASK-142f63d5](TASK-142f63d5) | Update artifact-framework with classification schema | agent-maintainer |

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
