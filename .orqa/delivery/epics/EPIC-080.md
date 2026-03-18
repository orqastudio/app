---
id: EPIC-080
title: "Plugin infrastructure and delivery plugin extraction"
status: exploring
created: 2026-03-16
relationships:
  - type: driven-by
    target: AD-055
  - target: AD-056
    type: driven-by
  - type: fulfils
    target: MS-002
  - target: IDEA-030
    type: realised-by
  - target: IDEA-038
    type: realised-by
  - target: IDEA-055
    type: realised-by
  - target: IDEA-071
    type: realised-by
  - target: IDEA-090
    type: realised-by
  - target: IDEA-100
    type: realised-by
  - target: IDEA-105
    type: realised-by
---

# EPIC-080: Plugin infrastructure and delivery plugin extraction

## Goal

Build the runtime plugin loading system and extract the "software project" delivery artifacts (milestones, epics, tasks, ideas, research) and views (roadmap, kanban, dashboard widgets) into `@orqastudio/plugin-delivery`.

## Scope

### Phase 1: Plugin runtime infrastructure
- Plugin discovery: scan `plugins/` directory for `orqa-plugin.json` manifests
- Plugin registration API in the SDK: `registerPlugin(manifest)` → registers artifact types, views, icons, relationship types
- Type system becomes plugin-aware: core types are always present, plugin types are additive
- Navigation system renders plugin-registered views in ActivityBar
- Icon registry accepts plugin icon registrations
- Dashboard widget registration: plugins contribute widgets with data contracts (read-only graph access)
- Widget configuration persistence: per-project layout and visibility settings (from IDEA-090)

### Phase 2: Core artifact isolation
- Identify and lock down core artifact types that ship with the app: pillars, vision, personas, grounding, decisions, rules, lessons, skills, agents
- Core types defined in platform config, not removable
- App views for core artifacts remain in the app repo
- Core artifact views use only `@orqastudio/svelte-components` (already done via component library rewiring)

### Phase 3: Delivery plugin extraction
- Create `@orqastudio/plugin-delivery` repo
- Move artifact type definitions: milestone, epic, task, idea, research
- Move views: RoadmapView, HorizonBoard, StatusKanban, KanbanCard, MilestoneCard, CollapsibleColumn, DrilldownBreadcrumbs
- Move dashboard widgets: PipelineWidget, DecisionQueueWidget, MilestoneContextCard
- Plugin manifest registers all types, views, icons, and relationship types (delivers, delivered-by, depends-on, depended-on-by)
- Plugin contributes status transition rules for its artifact types (from IDEA-105)
- App discovers and loads the plugin at startup

### Phase 4: Plugin bundling
- Build process bundles the delivery plugin into the app distribution
- Users get delivery functionality out of the box
- Plugin can be disabled in project settings
- Third-party plugins follow the same loading path

## Acceptance criteria
- [ ] App starts with zero delivery artifact types → plugin registers them
- [ ] Disabling the plugin removes delivery types from navigation
- [ ] All delivery views render identically to pre-extraction
- [ ] Plugin manifest validates against the JSON schema in `@orqastudio/types`
- [ ] Third-party plugin with custom artifact types can be loaded
