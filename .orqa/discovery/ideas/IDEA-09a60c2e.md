---
id: IDEA-09a60c2e
title: "Fix app UI breakage from nested artifact discovery removal"
type: discovery-idea
status: captured
description: "The OrqaStudio app UI is broken — the status bar still finds and displays artifacts correctly, but the main UI panels are broken. This was caused by removal of nested artifact discovery in a previous session. Critical errors visible in the app."
pillars:
  - PILLAR-c9e0a695
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: PILLAR-c9e0a695
    type: grounds
    rationale: "Broken UI prevents users from navigating the structured artifact graph, directly undermining Clarity Through Structure"
  - target: PERSONA-c4afd86b
    type: benefits
    rationale: "Alex (The Lead) uses the app UI to navigate governance artifacts and manage structured work — broken UI blocks their primary workflow"
---

## Context

The OrqaStudio app UI is currently broken. The status bar at the bottom of the app still correctly finds and displays artifact counts (indicating the artifact scanning backend is functional), but the main UI panels — artifact navigation, content rendering, and detail views — are broken and showing critical errors.

## Root Cause

This breakage was caused by the removal of nested artifact discovery logic in a previous session. The change affected how artifacts are resolved and rendered in the main UI panels, while the status bar uses a different code path that remained intact.

## What Works

- Status bar artifact scanning and display (artifact counts are correct)
- Backend artifact reader (the scanner finds artifacts on disk)

## What Is Broken

- Main UI panels (artifact navigation, content display)
- Critical errors visible in the app console
- Users cannot browse or interact with the artifact graph through the UI

## Research Needed

1. What specific changes were made during the "nested artifact discovery removal"?
2. Which UI components depend on the removed discovery logic?
3. What is the correct fix — restore the removed logic, or update the UI components to work with the new discovery approach?

## Pillar Alignment

| Pillar | Alignment |
| -------- | ----------- |
| Clarity Through Structure | The app UI is the primary interface for navigating structured governance artifacts. Broken panels prevent users from seeing and managing the artifact graph, directly violating this pillar's purpose. |
