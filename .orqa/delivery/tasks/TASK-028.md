---
id: TASK-028
title: Provider interface and Claude adapter
description: Defines a provider abstraction interface in the sidecar and refactors the Claude Agent SDK integration into a concrete adapter, enabling future providers to be added via a factory.
status: done
created: "2026-03-07"
updated: "2026-03-09"
epic: EPIC-040
assignee: AGENT-002
skills:
  - SKILL-017
  - SKILL-032
acceptance:
  - Provider interface defined with query/resume/cancel/health methods
  - ClaudeAgentProvider implements the interface
  - Factory function creates providers by type
  - Existing behavior unchanged
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
