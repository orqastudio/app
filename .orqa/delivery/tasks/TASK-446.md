---
id: TASK-446
title: "Wire auto-fix and confirmations to toast"
description: "Replace inline confirmation banners with toast notifications for auto-fix success and other confirmations."
status: done
priority: P2
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-069
depends-on:
  - TASK-445
assignee: null
skills: []
acceptance:
  - "Auto-fix success shows as concise toast"
  - "All inline confirmation banners replaced with toast"
relationships:
  - target: EPIC-069
    type: delivers
    rationale: "Consistent toast usage replaces scattered inline confirmations"
---

## Scope

Update IntegrityWidget and any other components with inline confirmation banners to use the toast notification system instead. Auto-fix success messages should appear as concise toasts.
