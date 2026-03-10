---
id: EPIC-044
title: Three-Layer Governance Classification
description: "Classify all governance artifacts (agents, skills, rules, hooks) into three layers: canon (platform principles), project (project-specific), and plugin (extensible/ecosystem). Add scope categorisation to agents (software-engineering, governance, general). This is foundational architecture enabling multi-project support."
status: done
priority: P1
created: "2026-03-09"
updated: "2026-03-09"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required: []
docs-produced:
  - DOC-036
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 5
  effort: 2
  risk: 1
  score: 18
---
## Findings Addressed

- **F25**: Agent scope needs categorisation (software-engineering vs governance vs general)
- **F26**: Skills need categorisation (project-type-specific vs universal platform)
- **F27**: Rules and hooks need canon vs project classification

## Implementation Design

### Three Layers

| Layer | Meaning | Ships With |
|-------|---------|------------|
| `canon` | Platform principles — applies to ALL projects managed by OrqaStudio | The app |
| `project` | Project-specific — additive enforcement for this particular project | The project's `.orqa/` |
| `plugin` | Ecosystem-extensible — third-party or community contributions | Installed via skills CLI or plugin system |

### Agent Scope Categories

| Scope | Meaning | Examples |
|-------|---------|---------|
| `software-engineering` | Code-writing agents for software projects | backend-engineer, frontend-engineer, designer |
| `governance` | Process/governance framework agents | agent-maintainer |
| `general` | Universal agents needed for any project type | orchestrator, code-reviewer, documentation-writer |

### Classification

All current agents are `layer: canon` (generic roles shipping with the platform).
All `orqa-*` skills are `layer: project`. Portable skills are `layer: canon`.
Most rules are `layer: canon` (platform principles). Project-specific rules are `layer: project`.

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
