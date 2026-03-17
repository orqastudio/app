---
id: IDEA-037
title: Artifact Node Graph
description: "Build a bidirectional graph of all governance artifacts with typed relationships, enabling backreference queries, broken link detection, orphan detection, and a plugin-ready SDK for unified artifact access."
status: completed
created: 2026-03-10
updated: 2026-03-13
research-needed:
  - Inter-artifact linking design (RES-033)
  - Graph SDK design (RES-034)
relationships:
  - target: RES-032
    type: informed-by
    rationale: "Research on consistent entity structure informed artifact graph design"
  - target: RES-033
    type: informed-by
    rationale: "Research on inter-artifact linking design informed graph relationships"
  - target: RES-034
    type: informed-by
    rationale: "Research on graph SDK design informed frontend SDK approach"
  - target: EPIC-048
    type: evolves-into
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-001
    type: grounded-by
---
## Motivation

The artifact system has hundreds of interconnected files — epics reference milestones, tasks reference epics, decisions supersede other decisions, lessons promote to rules. But there's no unified way to query these relationships. The frontend uses a hardcoded prefix map for navigation, the viewer reads raw files, and backreferences ("what links to this?") are impossible without scanning every file.

A node graph makes all relationships queryable in both directions, enables broken link detection, provides a typed SDK for consistent access, and lays the foundation for plugins that need to traverse artifact relationships.

## Sketch

- Backend: Rust `ArtifactGraph` with `HashMap<String, ArtifactNode>`, bidirectional `ArtifactRef` edges computed during scan
- Frontend: Typed Svelte 5 rune SDK (`artifactGraph`) with synchronous resolution, relationship queries, and plugin subscription API
- Live refresh via `.orqa/` file watcher with debounced graph rebuild
- Replaces all ad-hoc artifact access patterns (prefix map, label matching, raw file reads for metadata)

Research: [RES-033](RES-033) (linking design), [RES-034](RES-034) (graph SDK design). Implementation: [EPIC-048](EPIC-048).
