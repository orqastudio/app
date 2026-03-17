---
id: TASK-027
title: "Security hardening: CSP and permissions"
description: "Audits and restricts Tauri capabilities to the minimum required permissions, removing broad defaults and enabling a Content Security Policy."
status: completed
created: 2026-03-06
updated: 2026-03-09
assignee: AGENT-006
acceptance:
  - "fs:default, shell:default, dialog:default, notification:default removed"
  - "Only dialog:allow-open retained"
  - Content Security Policy enabled
relationships:
  - target: EPIC-039
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-032
    type: informed-by
  - target: TASK-329
    type: depended-on-by
  - target: SKILL-032
    type: informed-by
---
## What

Audit and restrict Tauri capabilities to minimum required permissions. Enable CSP.

## Outcome

Broad permissions removed, replaced with minimum required (`dialog:allow-open`).
CSP enabled. Git commit: `71838b4`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
