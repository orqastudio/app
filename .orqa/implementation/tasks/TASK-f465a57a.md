---
id: "TASK-f465a57a"
type: "task"
title: "Provider interface and Claude adapter"
description: "Defines a provider abstraction interface in the sidecar and refactors the Claude Agent SDK integration into a concrete adapter, enabling future providers to be added via a factory."
status: archived
created: 2026-03-07T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "Provider interface defined with query/resume/cancel/health methods"
  - "ClaudeAgentProvider implements the interface"
  - "Factory function creates providers by type"
  - "Existing behavior unchanged"
relationships:
  - target: "EPIC-2f1648f5"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Create the Provider interface abstraction and refactor the Claude Agent SDK
integration into a ClaudeAgentProvider class implementing that interface.

## Outcome

Provider interface extracted, Claude-specific code encapsulated. Factory pattern
for future provider addition. Git commit: `34cc4b6`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
