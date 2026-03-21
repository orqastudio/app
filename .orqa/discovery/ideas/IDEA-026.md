---
id: "IDEA-adce6c86"
title: "Artifact platform audit — built-in vs external"
description: "Audit which artifact types are first-class platform citizens with enforced lifecycles versus display-only documents, mapped to the three-layer governance model."
status: "surpassed"
created: "2026-03-07"
updated: "2026-03-13"
horizon: "later"
research-needed:
  - "Audit which artifact types need to be built into the platform for it to function"
  - "Identify which artifacts are external/manual and whether they should be promoted to platform-managed"
  - "Define the boundary between platform-managed and user-managed artifacts"
  - "Consider the three-layer governance model (canon, project, plugin) and where each artifact type falls"
relationships:
  - type: "crystallises"
    target: "AD-c6abc8e6"
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-cda6edd6"
    type: "benefits"
---
n> **Surpassed 2026-03-16**: Core vs plugin artifact split fully captured in AD-c6abc8e6. Implementation details covered in EPIC-3f65c703 Phase 2.

## Problem

The platform scans and displays artifacts from `.orqa/` but treats them all as generic markdown files with frontmatter. Some artifact types (milestones, epics, tasks) are core to the platform's purpose and should have richer support — lifecycle enforcement, status transitions, cross-referencing validation. Others may remain as plain documents.

## Questions

1. Which artifact types are "first-class" platform citizens with enforced lifecycles?
2. Which are "display-only" documents the platform renders but doesn't manage?
3. Should the platform enforce status transitions, or just display them?
4. How does this map to the three-layer governance model (canon/project/plugin)?

## Origin

UAT Round 1 [EPIC-a2fa3068](EPIC-a2fa3068): User observation during artifact browsing
