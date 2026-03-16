---
id: IDEA-085
title: Multi-repo product overview via git submodules
description: "A parent OrqaStudio project that consumes .orqa/ artifacts from multiple git submodules, creating an overarching view of cross-project status. Includes a permissions hierarchy controlling whether parent projects can write to child .orqa/ directories or access code outside them. Potentially the implementation mechanism for organisations."
status: surpassed
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "How do git submodules map to .orqa/ artifact consumption? Can the artifact scanner recurse into submodules?"
  - "What permissions hierarchy is needed? Read-only .orqa/ vs read-write .orqa/ vs code access outside .orqa/"
  - "How does this relate to IDEA-007 (Spaces & Organisational Structure)? Is this the implementation mechanism for organisations?"
  - "Security model: how do we enforce that an orqa agent is scoped to files within its project boundary?"
  - "Can a parent project's integrity checks span submodule artifacts (e.g. cross-project dependency tracking)?"
  - "How does this interact with IDEA-038 (Plugin Distribution via Git Submodules) — are plugins and project submodules the same mechanism?"
  - "What does the UX look like for a product owner viewing aggregated status across 5 team repos?"
relationships:
  - type: evolves-into
    target: IDEA-112
  - type: merged-into
    target: EPIC-081
  - target: IDEA-007
    type: informs
    rationale: "Multi-repo management may be the implementation mechanism for the Spaces concept"
  - target: IDEA-038
    type: informs
    rationale: "Both use git submodules but for different purposes — plugins vs project aggregation"
  - target: PILLAR-001
    type: grounded-by
  - target: AD-055
    type: merged-into
---

> **Surpassed 2026-03-16**: Multi-repo product overview merged into EPIC-081 via AD-055 (organisation mode with path-based project references, no nesting required). Security and integrity concepts (permissions hierarchy, agent scope boundaries, cross-project validation) split into IDEA-112.

## Motivation

OrqaStudio currently scopes an agent to a single project directory. Real products often span multiple repos or monorepos managed by different teams. A product owner needs visibility across all of them without granting agents cross-repo code access.

Git submodules could create a "product overview" project that pulls in `.orqa/` directories from child projects. The parent sees planning artifacts, health snapshots, and integrity status across the product — without needing access to the actual codebases.

This raises a permissions question: should a parent project be able to *write* to child `.orqa/` directories (e.g. setting priorities, creating cross-team epics), or only *read*? And should it ever access code outside `.orqa/`?

This may be how "organisations" get implemented — not as a separate concept, but as a natural consequence of project hierarchy through submodules.
