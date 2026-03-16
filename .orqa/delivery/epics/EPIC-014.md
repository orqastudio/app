---
id: EPIC-014
title: Project Type System
description: "Add a project type system so OrqaStudio adapts its agents, tools, and scanning to the domain (software, research, product, personal)."
status: captured
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring: null
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
  - target: AD-029
    type: informed-by
  - target: AD-030
    type: informed-by
---
## Why P1

Without this, non-software users encounter software-specific features that don't apply to them. Domain-agnostic thinking is a core principle.

## Tasks

- [ ] `project_type` field in `.orqa/project.json` and `ProjectSettings` — Software, Research, Product, Personal, Custom
- [ ] Project type determines which agents, tools, and scanning are available
- [ ] Software projects get code tools (ChunkHound, file tools, git); others get domain-appropriate tooling
- [ ] Non-software projects work without requiring a codebase
- [ ] Project type selector in project creation/settings

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
