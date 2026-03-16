---
id: IDEA-073
title: "UI polish: favicons, titlebar icon, and visual rough edges"
description: "The app's favicons and titlebar icon need updating from defaults, and there are visual rough edges like spacing around the close button hover state that make the app feel unfinished."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: next
research-needed:
  - "Inventory all placeholder/default icons (favicon, titlebar, taskbar)"
  - Audit close/minimize/maximize button hover states for spacing issues
  - "What other visual rough edges exist in the current dogfood build?"
relationships:
  - target: MS-001
    type: delivers
---

## Motivation

During dogfooding, visual rough edges are noticeable — the favicon and titlebar icon are still defaults, and there is visible spacing above and below the close button hover state that looks messy. These small details undermine the sense of a polished product even when the underlying functionality works well.

This should be scoped as a focused UI polish pass for the Dogfood milestone.
