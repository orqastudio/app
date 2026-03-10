---
id: TASK-003
title: Update streaming pipeline documentation
description: Update streaming-pipeline.md to reflect the new SystemPromptSent emission point added by TASK-001.
status: done
created: "2026-03-07"
updated: "2026-03-07"
epic: EPIC-001
assignee: AGENT-007
scope:
  - docs/architecture/streaming-pipeline.md
acceptance:
  - SystemPromptSent emission point documented in event sequence
  - Emission location noted (after resolve_system_prompt
  - before sidecar.send)
  - custom_prompt documented as populated by EPIC-002
  - ContextInjected documented as added by EPIC-003
---
## What

Update `docs/architecture/streaming-pipeline.md` to reflect the new `SystemPromptSent` emission point added by TASK-001.

## Sections to Update

1. **Event sequence diagram** — Add `SystemPromptSent` between system prompt resolution and sidecar send
2. **StreamEvent variants table** — Ensure `SystemPromptSent` and `ContextInjected` are listed with their fields
3. **Emission points section** — Document where and when `SystemPromptSent` is emitted
4. **Future work notes** — Note that `custom_prompt` (EPIC-002) and `ContextInjected` (EPIC-003) extend this

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
