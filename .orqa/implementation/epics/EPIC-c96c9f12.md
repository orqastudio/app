---
id: "EPIC-c96c9f12"
type: epic
title: "UAT round 3 — navigation architecture, artifact links, roadmap drill-down, doc reorg"
description: "30 UAT findings grouped into 10 systemic themes. Largest themes: navigation architecture overhaul (Exploring/Delivery/Documentation/Process), artifact link component rework with settings, roadmap hierarchical drill-down, documentation reorganisation into book chapters, and schema migrations for scope → relationship."
status: active
priority: "P1"
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 4
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
horizon: "active"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

UAT round 3 collected 30 findings across the app after implementing 31 tasks in 6 epics. F2 (agent artifact discipline — 57 integrity warnings) was addressed by EPIC-12fba656 (enforcement bootstrapping). The remaining 29 findings group into 10 systemic themes.

## UAT Findings

| # | Finding | Type | Area | Theme |
| --- | --------- | ------ | ------ | ------- |
| F1 | Dashboard needs holistic redesign of what data is shown | ux | dashboard | Dashboard |
| F2 | ~~57 integrity warnings~~ | ~~bug~~ | ~~process~~ | **Addressed by EPIC-12fba656** |
| F3 | Broken link: RES-94a3a6ca → TASK-d3085ce2 | bug | data | Quick fix |
| F4 | Roadmap kanban columns don't fill available space | ux | roadmap | Roadmap |
| F5 | Roadmap needs hierarchical drill-down: milestone → epic → task with breadcrumbs, drag-drop, collapsible done columns | ux | roadmap | Roadmap |
| F6 | Relationships appearing in metadata card — should only be in RelationshipsList | bug | artifact-viewer | Artifact links |
| F7 | Status dots positioned outside artifact link chips — should be inside | bug | artifact-links | Artifact links |
| F8 | Artifact link chips should have configurable per-type colours | ux | artifact-links | Artifact links |
| F9 | Title display on artifact chips needs max-width with ellipsis | ux | artifact-links | Artifact links |
| F10 | Hover over artifact chip → popover with metadata from graph | ux | artifact-links | Artifact links |
| F11 | Actions section should be below status pipeline on artifact views | ux | artifact-viewer | Viewer layout |
| F12 | Lessons show "Active" but should show "Captured" | bug | lessons | Schema fixes |
| F13 | Status pipeline connector line should center on circle, not stage component | ux | status-pipeline | Viewer layout |
| F14 | Rule scope field should be relationship type (scoped-to/scoped-by) | bug | rules | Schema fixes |
| F15 | Skill scope field should be relationship type | bug | skills | Schema fixes |
| F16 | IDEA-fab848e7 appears under "Other" and can't be viewed | bug | artifact-browser | Quick fix |
| F17 | Roadmap under Delivery, not top-level nav | ux | navigation | Navigation |
| F18 | Verification sub-nav has no icon | bug | navigation | Navigation |
| F19 | Verification section exists but no artifacts — evaluate removal | ux | navigation | Navigation |
| F20 | Delivery sub-nav split into Exploring + pipeline groups | ux | navigation | Navigation |
| F21 | Roadmap should be top item in pipeline group | ux | navigation | Navigation |
| F22 | Split Exploring and Delivery into two main-nav sections | ux | navigation | Navigation |
| F23 | Main nav order: Exploring → Delivery → Documentation → Process | ux | navigation | Navigation |
| F24 | Documentation main nav needs icon | bug | navigation | Navigation |
| F25 | Documentation needs book-chapter reorganisation + how-to guides (plugin SDK, testing, linting) | ux | documentation | Doc reorg |
| F26 | Doc list panel shows redundant category badge | ux | artifact-browser | Doc reorg |
| F27 | Artifact link display mode (ID vs title) settings not delivered from last round | bug | settings | Artifact links |
| F28 | Artifact link colour coding should be in project settings | ux | settings | Artifact links |
| F29 | Ctrl+Space search shortcut not in keyboard shortcuts config | bug | settings | Settings |
| F30 | Pipeline health table needs ScrollArea component | ux | dashboard | Dashboard |
| F31 | Toast notifications: animate in/out, top center, min-width | ux | notifications | UI polish |
| F32 | Pipeline health category dropdown not using SelectMenu | bug | dashboard | Dashboard |
| F33 | Pipeline health filter toggles need proper component pattern | ux | dashboard | Dashboard |
| F34 | Roadmap columns no longer evenly fill panel width after ScrollArea migration | bug | roadmap | Roadmap |
| F35 | ScrollArea inside roadmap columns doesn't scroll — cards clipped/hidden | bug | roadmap | Roadmap |
| F36 | All-done state: show "All completed" screen with "View board" button instead of empty columns. Only collapse Done when non-done items exist | ux | roadmap | Roadmap |
| F37 | Drag and drop between roadmap kanban columns not working | bug | roadmap | Roadmap |
| F38 | No sort/group dropdown on roadmap kanban (was in acceptance criteria) | bug | roadmap | Roadmap |
| F39 | Column headers: remove "X items", move "X/X done" into that spot, capitalize words, put in badge | ux | roadmap | Roadmap |
| F40 | Progress bar circles on milestone cards not centered on label | ux | roadmap | Roadmap |
| F41 | Roadmap still top-level nav — should be inside Delivery section as first item | bug | navigation | Navigation |
| F42 | Relationship graph needs proper library (zoom, pan, dynamic layout, click-to-navigate) | ux | artifact-viewer | Graph viz |
| F43 | Metadata panel: add icons to all fields (only scoring has one) | ux | artifact-viewer | Viewer layout |
| F44 | Metadata panel still shows relationship-specific fields (epic, milestone, depends-on) duplicating relationships list | bug | artifact-viewer | Viewer layout |
| F45 | Gap between metadata panel and acceptance criteria too large | ux | artifact-viewer | Viewer layout |
| F46 | No gap between acceptance criteria and body content on tasks | ux | artifact-viewer | Viewer layout |
| F47 | Artifact link display mode (ID vs title) should be per-type, not just global | ux | settings | Artifact links |

## Systemic Analysis

### Theme 1: Navigation Architecture (F17-F24)

**Root cause**: The nav structure was built incrementally and doesn't match the conceptual model of Exploring (divergent thinking) vs Delivery (convergent execution).

**Fix**: Restructure main nav into 4 sections: Exploring → Delivery → Documentation → Process. Exploring gets ideas + research. Delivery gets roadmap (top), milestones, epics, tasks. Remove or repurpose Verification. Fix missing icons.

### Theme 2: Artifact Link System (F6-F10, F27, F28)

**Root cause**: The ArtifactLink component was built for basic rendering. Now it needs to be a rich, configurable, interactive element with status dots inside, per-type colours, hover popovers, display mode settings, and proper text overflow.

**Fix**: Systematic rework of ArtifactLink component + project settings for colour/display configuration. Also fix relationships leaking into metadata card.

### Theme 3: Roadmap View (F4, F5)

**Root cause**: Roadmap kanban is a flat view. The conceptual model is hierarchical: Now/Next/Later → milestones → epics → tasks.

**Fix**: Build drill-down navigation with breadcrumbs, collapsible done columns, drag-drop between columns, and configurable grouping/sorting. This is a substantial UX feature.

### Theme 4: Schema/Data Fixes (F12, F14, F15)

**Root cause**: Legacy frontmatter fields (scope on rules/skills) that should be relationship types. Status vocabulary mismatch on lessons.

**Fix**: Migrate scope fields to scoped-to/scoped-by relationships. Fix lesson status vocabulary. Update schemas.

### Theme 5: Dashboard Polish (F1, F30, F32, F33)

**Root cause**: Dashboard components built quickly without using shared component library.

**Fix**: Replace native elements with shared components (ScrollArea, SelectMenu, proper filter toggles). F1 (holistic redesign) is a larger piece — defer to separate research/epic.

### Theme 6: Artifact Viewer Layout (F11, F13)

**Root cause**: Minor layout issues in artifact viewer.

**Fix**: Move actions below status pipeline. Fix connector line vertical centering.

### Theme 7: Documentation Reorganisation (F25, F26)

**Root cause**: Docs grew organically without structure. Needs research ([RES-29cf5ac5](RES-29cf5ac5)) then its own epic.

**Fix**: Research doc-chapter structure, plan migration, create epic. Remove redundant category badge.

### Theme 8: UI Polish (F31)

**Root cause**: Toast notification defaults not customised.

**Fix**: Configure toast position, animation, min-width.

### Theme 9: Settings (F29)

**Root cause**: Keyboard shortcut exists but isn't registered in config UI.

**Fix**: Add Ctrl+Space to keyboard shortcuts section.

### Theme 10: Quick Fixes (F3, F16)

**Root cause**: Individual bugs.

**Fix**: Fix broken link. Fix IDEA-fab848e7 categorisation.

## Tasks

### Quick Fixes

- [ ] [TASK-c8ea3e2a](TASK-c8ea3e2a): Fix broken link RES-94a3a6ca → TASK-d3085ce2 and IDEA-fab848e7 categorisation (F3, F16)

### Navigation Architecture (F17-F24)

- [ ] [TASK-43acbe04](TASK-43acbe04): Restructure main nav: Exploring → Delivery → Documentation → Process

### Artifact Link System (F6-F10, F27, F28)

- [ ] [TASK-4c973e88](TASK-4c973e88): Rework ArtifactLink component — status dot inside, ellipsis, hover popover
- [ ] [TASK-16fc8f8f](TASK-16fc8f8f): Artifact link settings — display mode (ID/title) and per-type colour coding

### Roadmap (F4, F5)

- [ ] [TASK-29cf1029](TASK-29cf1029): Roadmap columns fill available space
- [ ] [TASK-237ebc9f](TASK-237ebc9f): Roadmap hierarchical drill-down with breadcrumbs

### Schema/Data Fixes (F12, F14, F15)

- [ ] [TASK-cf872a80](TASK-cf872a80): Migrate rule/skill scope fields to relationship types
- [ ] [TASK-27360347](TASK-27360347): Fix lesson status vocabulary, viewer layout (actions + pipeline)

### Dashboard Polish (F30, F32, F33)

- [ ] [TASK-84b816cd](TASK-84b816cd): Dashboard components use shared library

### Documentation (F25, F26)

- [ ] [TASK-26ae8fff](TASK-26ae8fff): Documentation reorg research + redundant badge removal

### UI Polish + Settings (F29, F31)

- [ ] [TASK-bf7ba85a](TASK-bf7ba85a): Toast notifications + keyboard shortcuts config

## Verification

All findings re-tested after implementation. Each theme verified independently.

## Deferred with User Awareness

- F1 (dashboard holistic redesign) — research created ([RES-206222e3](RES-206222e3)), needs its own epic after research is complete
- F2 (agent artifact discipline) — addressed by EPIC-12fba656
- IDEA-cb35c573 (exploration whiteboard) — captured as idea, not in scope for implementation
