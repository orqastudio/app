---
id: IMPL-063
title: Duplicate controls for the same operation confuse users
description: Multiple buttons that trigger the same underlying function (Refresh on dashboard and Re-index on statusbar both call artifactGraphSDK.refresh()) create confusion about whether they do different things.
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: observation
relationships:
  - target: EPIC-068
    type: cautions
    rationale: "TASK-440 removes the duplicate Refresh button"
---

## Pattern

The dashboard had a Refresh button and the statusbar had a Re-index button. Both called `artifactGraphSDK.refresh()`. Users naturally assumed they did different things, and investigated the difference (which didn't exist). This erodes trust in the UI — if two buttons look different but do the same thing, what else might be misleading?

## Fix

One operation, one control, one location. If the same operation is needed in multiple contexts, use the same label and icon so the user recognises it as the same action.
