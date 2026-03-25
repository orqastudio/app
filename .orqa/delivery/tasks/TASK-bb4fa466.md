---
id: "TASK-bb4fa466"
type: "task"
title: "Write graph-based orchestrator prompt"
description: "Rewrite the orchestrator prompt from ~2000 lines of inlined rules/tables to ~200 lines of graph navigation instructions. The prompt teaches agents how to read and extend the artifact graph."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
docs:
  - "DOC-28344cd7"
acceptance:
  - "Orchestrator prompt is <=250 lines"
  - "Agents can navigate the graph to find rules, skills, docs"
  - "Core process and safety constraints are preserved"
  - "A dogfood session works correctly with the new prompt"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-77bfa7e1"
    type: "depends-on"
  - target: "TASK-608d1faa"
    type: "depends-on"
  - target: "TASK-7a8690e2"
    type: "depends-on"
---
## What

The orchestrator prompt is the most critical artifact in the system. Currently it's ~2000 lines encoding everything inline. The target is ~200 lines that teach graph navigation.

## Structure

```markdown
# OrqaStudio Orchestrator

## Your Role
[~20 lines: coordinate, delegate, don't implement]

## The Artifact Graph
[~30 lines: how to read nodes, follow edges, discover context]

## Process
[~20 lines: understand → plan → document → implement → review → learn]

## Delegation
[~20 lines: roles, how to resolve capabilities, how to inject context from graph]

## Safety
[~30 lines: NON-NEGOTIABLE constraints that must always be in prompt]

## Project Context
[~30 lines: tech stack, key paths, minimal project-specific info]

## Graph Maintenance
[~20 lines: how to extend the graph when creating/modifying artifacts]
```

## Critical Preservation

These MUST remain in the prompt (not delegated to graph):
- Role definitions and ownership boundaries
- NON-NEGOTIABLE safety rules (no unwrap, no --no-verify, no force push)
- The documentation-first process sequence
- How to read the graph (bootstrap problem — can't read graph instructions from graph)

## How

1. Read the current orchestrator.md to understand everything it contains
2. Categorise content into: must-keep-inline vs move-to-graph
3. Write the new prompt following the Structure above
4. Verify the prompt bootstraps correctly (agents can read graph from the prompt)

## Verification

- Prompt is <=250 lines
- All NON-NEGOTIABLE constraints preserved inline
- Graph navigation instructions are clear and actionable
- A test delegation finds the right skills and docs via graph traversal