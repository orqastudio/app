---
id: "EPIC-b2ca1ea3"
type: epic
title: "UAT round 3 — navigation architecture, artifact links, roadmap drill-down, doc reorg"
description: "30 UAT findings grouped into 10 systemic themes. Largest themes: navigation architecture overhaul (Exploring/Delivery/Documentation/Process), artifact link component rework with settings, roadmap hierarchical drill-down, documentation reorganisation into book chapters, and schema migrations for scope → relationship."
status: "active"
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
  - target: "RES-9c90ef3d"
    type: "guided-by"
    rationale: "Documentation reorganisation research informs the doc reorg tasks"
  - target: "TASK-6d896b8d"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-6d896b8d"
  - target: "TASK-44721f88"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-44721f88"
  - target: "TASK-d9c5be64"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-d9c5be64"
  - target: "TASK-07e661cd"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-07e661cd"
  - target: "TASK-8d601baf"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-8d601baf"
  - target: "TASK-697497a9"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-697497a9"
  - target: "TASK-d7ef4c9d"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-d7ef4c9d"
  - target: "TASK-88ed71e0"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-88ed71e0"
  - target: "TASK-a3f97edc"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-a3f97edc"
  - target: "TASK-6018c3ed"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-6018c3ed"
  - target: "TASK-e71504ed"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-e71504ed"
  - target: "TASK-d8a69e21"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-d8a69e21"
  - target: "TASK-a4f1bec7"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-a4f1bec7"
  - target: "TASK-27eca2aa"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-27eca2aa"
  - target: "TASK-b1dc0073"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-b1dc0073"
  - target: "TASK-ebc48b97"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-ebc48b97"
  - target: "TASK-b8d36c3f"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-b8d36c3f"
  - target: "TASK-71a3c1f7"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-71a3c1f7"
  - target: "TASK-e40d6994"
    type: "delivered-by"
    rationale: "Auto-generated inverse of contains relationship from TASK-e40d6994"
  - target: "TASK-730add34"
    type: "delivered-by"
  - target: "TASK-3fceb34c"
    type: "delivered-by"
  - target: "IDEA-a5f04c5c"
    type: "realised-by"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Context

UAT round 3 collected 30 findings across the app after implementing 31 tasks in 6 epics. F2 (agent artifact discipline — 57 integrity warnings) was addressed by EPIC-915291e7 (enforcement bootstrapping). The remaining 29 findings group into 10 systemic themes.

## UAT Findings

| # | Finding | Type | Area | Theme |
|---|---------|------|------|-------|
| F1 | Dashboard needs holistic redesign of what data is shown | ux | dashboard | Dashboard |
| F2 | ~~57 integrity warnings~~ | ~~bug~~ | ~~process~~ | **Addressed by EPIC-915291e7** |
| F3 | Broken link: RES-63578440 → TASK-3ce9c6dd | bug | data | Quick fix |
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
| F16 | IDEA-93949f43 appears under "Other" and can't be viewed | bug | artifact-browser | Quick fix |
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
**Root cause**: Docs grew organically without structure. Needs research ([RES-9c90ef3d](RES-9c90ef3d)) then its own epic.

**Fix**: Research doc-chapter structure, plan migration, create epic. Remove redundant category badge.

### Theme 8: UI Polish (F31)
**Root cause**: Toast notification defaults not customised.

**Fix**: Configure toast position, animation, min-width.

### Theme 9: Settings (F29)
**Root cause**: Keyboard shortcut exists but isn't registered in config UI.

**Fix**: Add Ctrl+Space to keyboard shortcuts section.

### Theme 10: Quick Fixes (F3, F16)
**Root cause**: Individual bugs.

**Fix**: Fix broken link. Fix IDEA-93949f43 categorisation.

## Tasks

### Quick Fixes
- [ ] [TASK-b8d36c3f](TASK-b8d36c3f): Fix broken link RES-63578440 → TASK-3ce9c6dd and IDEA-93949f43 categorisation (F3, F16)

### Navigation Architecture (F17-F24)
- [ ] [TASK-d8a69e21](TASK-d8a69e21): Restructure main nav: Exploring → Delivery → Documentation → Process

### Artifact Link System (F6-F10, F27, F28)
- [ ] [TASK-ebc48b97](TASK-ebc48b97): Rework ArtifactLink component — status dot inside, ellipsis, hover popover
- [ ] [TASK-88ed71e0](TASK-88ed71e0): Artifact link settings — display mode (ID/title) and per-type colour coding

### Roadmap (F4, F5)
- [ ] [TASK-27eca2aa](TASK-27eca2aa): Roadmap columns fill available space
- [ ] [TASK-e40d6994](TASK-e40d6994): Roadmap hierarchical drill-down with breadcrumbs

### Schema/Data Fixes (F12, F14, F15)
- [ ] [TASK-b1dc0073](TASK-b1dc0073): Migrate rule/skill scope fields to relationship types
- [ ] [TASK-6018c3ed](TASK-6018c3ed): Fix lesson status vocabulary, viewer layout (actions + pipeline)

### Dashboard Polish (F30, F32, F33)
- [ ] [TASK-07e661cd](TASK-07e661cd): Dashboard components use shared library

### Documentation (F25, F26)
- [ ] [TASK-a3f97edc](TASK-a3f97edc): Documentation reorg research + redundant badge removal

### UI Polish + Settings (F29, F31)
- [ ] [TASK-d9c5be64](TASK-d9c5be64): Toast notifications + keyboard shortcuts config

## Verification

All findings re-tested after implementation. Each theme verified independently.

## Deferred with User Awareness

- F1 (dashboard holistic redesign) — research created ([RES-627d3b37](RES-627d3b37)), needs its own epic after research is complete
- F2 (agent artifact discipline) — addressed by EPIC-915291e7
- IDEA-a5f04c5c (exploration whiteboard) — captured as idea, not in scope for implementation