---
id: TASK-027
title: "Security hardening: CSP and permissions"
status: done
epic: EPIC-039
description: Audits and restricts Tauri capabilities to the minimum required permissions, removing broad defaults and enabling a Content Security Policy.
created: 2026-03-06
updated: 2026-03-09
assignee: security-engineer
skills: [tauri-v2]
scope: [src-tauri/capabilities/default.json]
acceptance: ["fs:default, shell:default, dialog:default, notification:default removed", "Only dialog:allow-open retained", Content Security Policy enabled]
---

## What

Audit and restrict Tauri capabilities to minimum required permissions. Enable CSP.

## Outcome

Broad permissions removed, replaced with minimum required (`dialog:allow-open`).
CSP enabled. Git commit: `71838b4`.
