---
id: EPIC-014
title: Project Type System
description: Add a project type system so OrqaStudio adapts its agents, tools, and scanning to the domain (software, research, product, personal).
status: draft
priority: P1
created: 2026-03-07
updated: 2026-03-07
milestone: MS-002
pillars:
  - PILLAR-001
depends-on: []
blocks: []
docs-required:
  - docs/product/vision.md (domain-agnostic principle)
  - docs/architecture/project-configuration.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (project type system plan)
  - docs/architecture/project-configuration.md (update with project_type field)
  - docs/architecture/decisions.md (AD for project type taxonomy)
scoring:
  pillar: 4
  impact: 5
  dependency: 3
  effort: 3
  score: 10.3
---
## Why P1

Without this, non-software users encounter software-specific features that don't apply to them. Domain-agnostic thinking is a core principle.

## Tasks

- [ ] `project_type` field in `.orqa/project.json` and `ProjectSettings` — Software, Research, Product, Personal, Custom
- [ ] Project type determines which agents, tools, and scanning are available
- [ ] Software projects get code tools (ChunkHound, file tools, git); others get domain-appropriate tooling
- [ ] Non-software projects work without requiring a codebase
- [ ] Project type selector in project creation/settings
