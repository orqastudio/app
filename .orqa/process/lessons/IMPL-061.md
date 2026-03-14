---
id: IMPL-061
title: "Confirmations and status changes should use toast notifications, not inline banners"
description: "Inline success banners in widgets take permanent space and clutter the UI. Transient events (auto-fix applied, scan complete) should use toast notifications that auto-dismiss."
status: active
recurrence: 1
created: "2026-03-14"
updated: "2026-03-14"
maturity: observation
relationships:
  - target: EPIC-069
    type: informs
    rationale: "Notification system epic addresses the missing toast infrastructure"
---

## Pattern

The pipeline health widget shows auto-fix results as a persistent green banner inside the card. This takes space, doesn't auto-dismiss, and clutters the "all clear" state. The confirmation is a transient event — it happened, the user saw it, it should go away.

## Fix

Implement a toast notification system. Transient confirmations (fix applied, scan complete, settings saved) go to toast. Persistent state (errors found, action needed) stays inline in the relevant widget.
