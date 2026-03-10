---
id: IDEA-026
title: Artifact platform audit — built-in vs external
description: |
  Audit which artifact types are first-class platform citizens with enforced lifecycles versus display-only documents, mapped to the three-layer governance model.
status: captured
created: 2026-03-07
updated: 2026-03-07
pillars:
  - PILLAR-001
research-needed:
  - Audit which artifact types need to be built into the platform for it to function
  - Identify which artifacts are external/manual and whether they should be promoted to platform-managed
  - Define the boundary between platform-managed and user-managed artifacts
  - Consider the three-layer governance model (canon, project, plugin) and where each artifact type falls
promoted-to: null
---## Problem

The platform scans and displays artifacts from `.orqa/` but treats them all as generic markdown files with frontmatter. Some artifact types (milestones, epics, tasks) are core to the platform's purpose and should have richer support — lifecycle enforcement, status transitions, cross-referencing validation. Others may remain as plain documents.

## Questions

1. Which artifact types are "first-class" platform citizens with enforced lifecycles?
2. Which are "display-only" documents the platform renders but doesn't manage?
3. Should the platform enforce status transitions, or just display them?
4. How does this map to the three-layer governance model (canon/project/plugin)?

## Origin

UAT Round 1 (EPIC-043): User observation during artifact browsing
