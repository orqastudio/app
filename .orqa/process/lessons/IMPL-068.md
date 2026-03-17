---
id: IMPL-068
title: Duplicate relationship types on same artifact — contains AND delivered-by for same target
description: EPIC-073 had both contains and delivered-by relationships for 38 task targets. These are the same relationship from different directions — only one should exist on each artifact.
status: captured
created: 2026-03-15
updated: 2026-03-15
maturity: observation
recurrence: 1
relationships:
  - target: RULE-045
    type: informs
    rationale: Bidirectional relationship system created duplicates instead of proper inverses
---

## Pattern

The auto-inverse relationship system added BOTH the forward (contains) and inverse (delivered-by) to the same artifact, instead of placing the inverse on the target artifact. This inflates relationship arrays and creates semantic confusion.

## Fix

The inverse relationship generator should verify it's adding the inverse to the TARGET artifact, not the SOURCE. Graph-guardian should detect and warn about duplicate relationship types for the same target on a single artifact.
