---
id: "TASK-abba7331"
type: "task"
title: "Security hardening: CSP and permissions"
description: "Audits and restricts Tauri capabilities to the minimum required permissions, removing broad defaults and enabling a Content Security Policy."
status: "completed"
created: 2026-03-06T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
assignee: "AGENT-8e58cd87"
acceptance:
  - "fs:default, shell:default, dialog:default, notification:default removed"
  - "Only dialog:allow-open retained"
  - "Content Security Policy enabled"
relationships:
  - target: "EPIC-c1833545"
    type: "delivers"
    rationale: "Task belongs to this epic"
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