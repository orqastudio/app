---
id: TASK-0b584382
title: "Implement rule engine core (loader, parser, pattern matcher)"
description: "Build the core engine that loads .orqa/process/rules/, parses frontmatter, and evaluates enforcement patterns."
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-cc255bc8
docs: []
acceptance:
  - Engine loads all .md files from .orqa/process/rules/
  - Engine parses YAML frontmatter including enforcement array
  - Engine filters by status (only active rules)
  - Engine filters by scope (agent ID matching)
  - "Pattern matcher supports regex, glob, and contains matching"
  - Engine is testable in isolation
relationships:
  - target: EPIC-3a8ad459
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-869c27b5
    type: depends-on
  - target: TASK-b4c3c05d
    type: depends-on
  - target: TASK-73f7e0fa
    type: depended-on-by
  - target: TASK-bd0e805b
    type: depended-on-by
  - target: TASK-11cf4c1d
    type: depended-on-by
  - target: TASK-fa39671d
    type: depended-on-by
---

## What

The rule engine is the core of the plugin. It reads rule files, parses their
frontmatter (including the `enforcement` array), and evaluates patterns against
tool call context.

## How

1. Implement rule loader: scan `.orqa/process/rules/*.md`, parse YAML frontmatter
2. Implement filters: status=active, layer matching, scope (agent ID) matching
3. Implement pattern matcher: regex patterns against event context fields
4. Implement enforcement evaluation: for each rule with enforcement entries, match
   event type + pattern against the current tool call context
5. Return action (block with message, warn with additionalContext, or allow)

## Verification

- Unit tests for rule loading, parsing, filtering
- Unit tests for pattern matching (regex, glob, contains)
- Integration test against OrqaStudio's actual rule files
