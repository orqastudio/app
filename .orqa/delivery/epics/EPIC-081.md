---
id: EPIC-081
title: "Organisation-mode multi-project support"
status: exploring
created: 2026-03-16
relationships:
  - type: driven-by
    target: AD-055
  - type: fulfils
    target: MS-002
  - target: IDEA-006
    type: realised-by
  - target: IDEA-007
    type: realised-by
  - target: IDEA-085
    type: realised-by
---

# EPIC-081: Organisation-mode multi-project support

## Goal

Enable a project to operate as an organisation that aggregates artifacts from multiple child projects, providing cross-project visibility while maintaining each project's independence.

## Scope

### Phase 1: project.json organisation schema
- Add `organisation: boolean` field to project.json schema
- Add `projects: Array<{ name: string; path: string }>` field
- Path resolution: relative to the organisation project root, or absolute
- Validation: verify each child path exists and contains `.orqa/`

### Phase 2: Multi-project graph engine
- Graph builder scans organisation project's own `.orqa/` first
- Then scans each child project's `.orqa/` directory
- Each child's artifacts become a sub-graph with a `project` property on every node
- Organisation root node connects to each child's sub-graph root
- Cross-project relationships are resolved: `EPIC-001` in the SDK project can reference `TASK-042` in the app project via qualified IDs (`sdk:EPIC-001 → app:TASK-042`)

### Phase 3: SDK multi-project traversal
- `artifactGraphSDK.byType("epic")` returns epics from ALL projects (org view)
- `artifactGraphSDK.byType("epic", { project: "sdk" })` filters to one project
- `artifactGraphSDK.projects` returns list of child projects with metadata
- Navigation store supports project switching: org view ↔ single-project mode
- Breadcrumbs show project context when in org view

### Phase 4: Organisation UI
- Project switcher in toolbar or activity bar
- Organisation dashboard: aggregated metrics across all projects
- Filtered views: "show me all blocked tasks across all projects"
- Drill-down: click into a child project to enter single-project mode
- Return to org view via breadcrumb or project switcher

### Phase 5: Dev environment setup
- Create `orqastudio-dev` organisation project repo
- Add all library repos, app, plugins as git submodules (convenience, not requirement)
- Organisation `project.json` references all child projects by relative path
- Claude Code sessions from `orqastudio-dev/` have full context across the ecosystem
- Product-level governance (vision, milestones, decisions) lives in the org project's `.orqa/`

## Acceptance criteria
- [ ] Organisation project lists artifacts from all child projects
- [ ] Artifacts tagged with source project, filterable in all views
- [ ] Single-project mode shows only that project's artifacts
- [ ] Cross-project relationships resolve and display correctly
- [ ] Organisation dashboard aggregates health metrics across projects
- [ ] Adding/removing a child project updates the graph without restart
- [ ] `orqastudio-dev` organisation project works with all repos as submodules
