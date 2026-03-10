---
id: EPIC-005
title: Artifact Browser Enhancements
description: Make all .orqa/ artifacts browsable, restructure navigation into groups, establish .orqa/ as the single source of truth with platform adapters, and enable cross-artifact navigation.
status: draft
priority: P1
created: "2026-03-07"
updated: "2026-03-08"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-029
docs-required:
  - DOC-036
docs-produced:
  - DOC-005
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 4
  score: 8.8
---
## Why P1

This is the **underlying UX model**. Markdown documents visible in the UI is the foundational layer. Without grouped navigation, adding artifact types creates bloat. Without platform portability, governance is locked to Claude CLI's `.claude/` convention. Without cross-linking, the traceability web exists in frontmatter but is invisible.

## Design Principles

> "Structuring them as markdown documents that are visible within the UI is an important first step and is the underlying UX model."

> ".orqa/ should be the source of truth and what is read by the viewer."

> "Platform compatibility should be a setting — when a platform is 'turned on', symlinks should be created."

## Scope — Three Pillars

### 1. Navigation Restructuring
Replace 8 individual activity bar items with 4 grouped categories:
- **Documentation** — Docs
- **Planning** — Research, Plans, Milestones, Epics, Tasks, Ideas
- **Team** — Agents, Skills, Orchestrator
- **Governance** — Rules, Hooks, Lessons, Decisions

### 2. Platform Portability
- Move governance artifacts from `.claude/` to `.orqa/` (agents, rules, skills, hooks)
- Merge `CLAUDE.md` + `AGENTS.md` into `.orqa/agents/orchestrator.md`
- Platform compatibility as a project setting
- Claude CLI adapter: symlinks from `.claude/` → `.orqa/` and `CLAUDE.md` → `.orqa/agents/orchestrator.md`
- Remove `TODO.md` (content already in artifacts)

### 3. Cross-Linking
- Frontmatter fields containing artifact IDs render as clickable links
- Markdown body references (EPIC-001, AD-017, etc.) detected and linked
- Clicking navigates to the correct group/sub-category/artifact

## Phases

### Phase 1: Directory Migration & Platform Portability
- Move governance files to `.orqa/`
- Create orchestrator agent definition
- Symlink adapter for Claude CLI
- Update all path references in agents, rules, skills, docs
- Remove TODO.md

### Phase 2: Navigation Restructuring
- Activity bar: 4 group icons
- NavSubPanel: two-level navigation (group → sub-category → artifact list)
- Navigation store updates

### Phase 3: Backend Readers
- Scan/read commands for milestones, epics, tasks, ideas, decisions, lessons
- Update existing governance commands to read from `.orqa/`
- Store methods for each type

### Phase 4: Artifact Viewers with Structured Frontmatter
- FrontmatterHeader component (badges, links, tags, dates, progress)
- Type-specific viewers (MilestoneViewer, EpicViewer, etc.)
- Orchestrator viewer under Team

### Phase 5: Cross-Linking
- Frontmatter cross-links (clickable artifact ID chips)
- Body cross-links (regex detection + inline link rendering)
- navigateToArtifact() resolver

### Phase 6: Cleanup & Polish
- Remove all old path references
- Update governance artifacts for new structure
- Final verification

## Full Plan

See `.orqa/plans/epic-005-artifact-browser.md` for detailed implementation plan with architectural compliance, UX design, component states, and verification criteria.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

Task breakdown to be defined.
