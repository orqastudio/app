---
id: TASK-006
title: Governance Vision Alignment Audit
description: Audit governance rules (.claude/rules/) and agent definitions (.claude/agents/) for references that contradict the updated vision. These are the enforcement layer — they must accurately reflect .orqa/ as source of truth.
status: completed
created: 2026-03-08
updated: 2026-03-08
assignee: AGENT-003
acceptance:
  - Every rule file checked for .claude/ as source-of-truth references
  - Every agent definition checked for .claude/ path references
  - Paths updated to .orqa/ equivalents where appropriate
  - UI docs checked for Claude-specific branding
  - No rule or agent definition implies Claude is the product identity
relationships:
  - target: EPIC-033
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-323
    type: depended-on-by
---
## What

The rules and agent definitions currently reference `.claude/` paths as if
they are the source of truth. Update these to reference `.orqa/` paths.

Note: The rules and agents themselves still live in `.claude/` for CLI
compatibility during the bootstrap phase. The content within them should
reference `.orqa/` as the canonical location.

## Also Audit

- `ui/brand-identity.md` — check for Claude-specific branding
- `ui/design-system.md` — check for Claude-specific language

## Deliverable

Updated rule and agent files with a summary of changes.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
