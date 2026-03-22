---
id: TASK-db618792
title: Write Artifact Graph SDK documentation
description: "Create a development guide for the Artifact Graph SDK covering API reference, usage patterns, and plugin integration examples."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-ec1b3785
acceptance:
  - API reference for all SDK methods with TypeScript signatures
  - "Usage examples for resolution, relationships, content reading, and subscriptions"
  - Plugin integration examples showing how plugins consume the SDK
  - "Migration guide from old patterns (prefix map, raw invoke) to SDK"
  - Architecture diagram showing backend graph → Tauri commands → SDK → components
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-64ceb043
    type: depends-on
  - target: TASK-30307a19
    type: depended-on-by
  - target: TASK-12eec0f3
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
