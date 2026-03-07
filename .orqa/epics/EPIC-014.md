---
id: EPIC-014
title: "Project Type System"
status: draft
priority: P1
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 4
  impact: 5
  dependency: 3
  effort: 3
score: 10.3
roadmap-ref: "M4"
docs-required:
  - docs/product/vision.md (domain-agnostic principle)
  - docs/architecture/project-configuration.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (project type system plan)
  - docs/architecture/project-configuration.md (update with project_type field)
  - docs/architecture/decisions.md (AD for project type taxonomy)
tags: [project-types, domain-agnostic]
---

# Project Type System

OrqaStudio's governance applies to any project, not just software. The app needs to know what kind of project it's managing.

## Why P1

Without this, non-software users encounter software-specific features that don't apply to them. Domain-agnostic thinking is a core principle.

## Tasks

- [ ] `project_type` field in `.orqa/project.json` and `ProjectSettings` — Software, Research, Product, Personal, Custom
- [ ] Project type determines which agents, tools, and scanning are available
- [ ] Software projects get code tools (ChunkHound, file tools, git); others get domain-appropriate tooling
- [ ] Non-software projects work without requiring a codebase
- [ ] Project type selector in project creation/settings
