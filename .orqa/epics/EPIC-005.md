---
id: EPIC-005
title: "Artifact Browser & Visibility"
status: draft
priority: P1
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: [EPIC-016]
assignee: null
pillar:
  - clarity-through-structure
  - learning-through-reflection
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 3
score: 11.7
roadmap-ref: "D5"
docs-required:
  - docs/product/artifact-framework.md
  - docs/wireframes/artifact-browser.md
  - docs/architecture/ipc-commands.md
docs-produced:
  - docs/wireframes/artifact-browser.md (update with .orqa/ browser wireframes)
  - docs/architecture/ipc-commands.md (new artifact scanning/parsing commands)
  - docs/architecture/decisions.md (AD for .orqa/ parsing and indexing strategy)
tags: [artifacts, browser, markdown, ux-model]
---

# Artifact Browser & Visibility

Make `.orqa/` artifacts (milestones, epics, ideas, plans, research, lessons) browsable as rendered markdown documents in OrqaStudio's UI.

## Why P1

This is the **underlying UX model**. Markdown documents visible in the UI is the foundational layer. All richer views (kanban boards, dashboards, graph visualisations) are optional layers built on top. Without this, structured thinking artifacts are invisible inside the app.

## Design Principle

> "Structuring them as markdown documents that are visible within the UI is an important first step and is the underlying UX model. Other ways of displaying and interacting with this content all become optional layers on top."

## Scope

This epic covers the `.orqa/` artifact types defined in the artifact framework. The existing `.claude/` governance artifact browser (agents, rules, skills, hooks) already works and is a separate concern (EPIC-004 adds editing).

## Tasks

- [ ] `.orqa/` directory scanner — read and parse all artifact types (milestones, epics, ideas, plans, lessons, research)
- [ ] Frontmatter parser — extract YAML frontmatter from markdown files into structured data
- [ ] Artifact browser sidebar — tree navigation by type (Milestones > Epics > Tasks, Ideas, Plans, Lessons, Research)
- [ ] Markdown renderer view — render artifact body with syntax highlighting
- [ ] Frontmatter metadata panel — display structured frontmatter alongside the document (status, priority, connections)
- [ ] Connection links — clickable references to related artifacts (milestone, epic, depends-on, blocks)
- [ ] Status badges — colour-coded status indicators per artifact type
- [ ] Priority band indicators — P1/P2/P3 badges for epics
- [ ] File watcher integration — refresh when `.orqa/` files change on disk (coordinate with EPIC-006)
