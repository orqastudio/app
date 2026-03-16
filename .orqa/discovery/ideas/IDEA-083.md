---
id: IDEA-083
title: Auto-rendered task tables from relationship edges
description: "Epic body task tables and milestone epic breakdowns should be automatically rendered from relationship edges rather than manually maintained. When a task has epic: EPIC-NNN, the epic's task table is generated from the graph. When an epic has milestone: MS-NNN, the milestone's breakdown table is generated similarly. Makes these views self-maintaining as edges are created."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: next
research-needed:
  - "Should auto-rendered tables replace or supplement manually authored tables in epic/milestone bodies?"
  - "How does this interact with the body template system? Does the Tasks section become a rendered view rather than authored content?"
  - "What columns should the auto-rendered table include? (status, depends-on, phase, assignee?)"
  - "Should the app render these dynamically (query graph at display time) or should a tool generate them into the markdown?"
  - "How does ordering work? By dependency graph topological sort? By phase? By status?"
relationships:
  - target: MS-001
    type: delivers
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
---

## Motivation

Task tables in epics and epic breakdowns in milestones are manually maintained markdown tables that duplicate information already expressed in the artifact graph edges. When a task is created with `epic: EPIC-NNN`, that relationship exists as structured data — but the epic's body table must be manually updated to reflect it. This creates drift: tasks get added or completed but the table isn't updated.

Auto-rendering these tables from the graph makes them self-maintaining. The table becomes a view of the graph, not a separate data source.

## Sketch

**Epic task table**: Query all tasks where `epic: EPIC-NNN`, render as table sorted by dependency order or phase, showing status/title/depends-on.

**Milestone epic table**: Query all epics where `milestone: MS-NNN`, render as table showing status/priority/title.

Both could be rendered:
- **In the app**: dynamically at display time (always current)
- **In markdown**: via a tool that regenerates the table section (for CLI/static viewing)
- **Both**: app renders live, tool regenerates for git-committed markdown
