---
id: TASK-269
title: "Tighten RULE-001 orchestrator content boundary"
description: "Clarify in RULE-001 that the orchestrator creates artifact structure but delegates content writing to Writer."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-057
depends-on: []
assignee: AGENT-003
acceptance:
  - "RULE-001 exception list distinguishes structure (orchestrator) from content (Writer)"
  - "Research artifacts listed as Writer-delegated content"
  - "No new rule created — existing RULE-001 tightened"
---

## What

The orchestrator exception list says `.orqa/delivery/` is orchestrator territory, but writing research findings is content creation (Writer role). Tighten the boundary.

## How

1. Update RULE-001 exception list to clarify:
   - Creating task/epic/idea structure = orchestrator
   - Writing research content, documentation pages = delegate to Writer
2. Keep it concise — add one clarifying sentence, not a new rule

## Verification

RULE-001 clearly distinguishes structure creation from content authoring.
