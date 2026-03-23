---
id: "TASK-fa777da9"
type: task
title: "Update streaming pipeline documentation"
description: "Update streaming-pipeline.md to reflect the new SystemPromptSent emission point added by TASK-58a9d218."
status: "completed"
created: "2026-03-07"
updated: "2026-03-07"
assignee: "AGENT-ec1b3785"
acceptance:
  - "SystemPromptSent emission point documented in event sequence"
  - "Emission location noted (after resolve_system_prompt"
  - "before sidecar.send)"
  - "custom_prompt documented as populated by EPIC-642234ba"
  - "ContextInjected documented as added by EPIC-1dcf5ffa"
relationships:
  - target: "EPIC-e045ab6d"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-b6aa7a3b"
    type: "depended-on-by"
---
## What

Update `.orqa/documentation/development/streaming-pipeline.md` to reflect the new `SystemPromptSent` emission point added by [TASK-58a9d218](TASK-58a9d218).

## Sections to Update

1. **Event sequence diagram** — Add `SystemPromptSent` between system prompt resolution and sidecar send
2. **StreamEvent variants table** — Ensure `SystemPromptSent` and `ContextInjected` are listed with their fields
3. **Emission points section** — Document where and when `SystemPromptSent` is emitted
4. **Future work notes** — Note that `custom_prompt` [EPIC-642234ba](EPIC-642234ba) and `ContextInjected` [EPIC-1dcf5ffa](EPIC-1dcf5ffa) extend this

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.