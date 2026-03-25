---
id: "TASK-d7346ee9"
type: "task"
title: "Tighten RULE-87ba1b81 orchestrator content boundary"
description: "Clarify in RULE-87ba1b81 that the orchestrator creates artifact structure but delegates content writing to Writer."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-4c94fe14"
acceptance:
  - "RULE-87ba1b81 exception list distinguishes structure (orchestrator) from content (Writer)"
  - "Research artifacts listed as Writer-delegated content"
  - "No new rule created — existing RULE-87ba1b81 tightened"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

The orchestrator exception list says `.orqa/delivery/` is orchestrator territory, but writing research findings is content creation (Writer role). Tighten the boundary.

## How

1. Update [RULE-87ba1b81](RULE-87ba1b81) exception list to clarify:
   - Creating task/epic/idea structure = orchestrator
   - Writing research content, documentation pages = delegate to Writer
2. Keep it concise — add one clarifying sentence, not a new rule

## Verification

[RULE-87ba1b81](RULE-87ba1b81) clearly distinguishes structure creation from content authoring.