---
id: IMPL-3fbabea8
title: "Frontmatter reference fields should be relationship types, not standalone fields"
description: "Fields like epic, milestone, scope, depends-on are semantically relationships but stored as standalone frontmatter fields. This creates two classes of graph edges — relationship array edges (scannable, typed, invertible) and field-based edges (invisible to relationship tooling)."
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: understanding
relationships:
  - target: EPIC-58ba6d53
    type: cautions
    rationale: "TASK-0905e285 (scope migration) and TASK-c6514b53 (epic/milestone migration) address this"
---

## Pattern

The artifact graph has two kinds of edges:
1. **Relationship array entries** — typed, invertible, scannable, displayed in the relationships panel
2. **Standalone frontmatter fields** — `epic:`, `milestone:`, `scope:`, `depends-on:` — parsed by the graph builder but not treated as first-class relationships

This split means relationship tooling (display, validation, graph visualization) only sees half the graph. The integrity validator checks field-based refs separately from relationship refs. The relationships panel doesn't show them.

## Fix

Migrate standalone reference fields to relationship types. Keep the frontmatter fields during transition for backwards compatibility, but the graph builder should emit relationship edges from both sources. Long term, all inter-artifact references should be relationship entries with a type.
