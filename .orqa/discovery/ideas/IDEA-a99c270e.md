---
id: "IDEA-a99c270e"
type: idea
title: "Artifact Node Graph"
description: "Build a bidirectional graph of all governance artifacts with typed relationships, enabling backreference queries, broken link detection, orphan detection, and a plugin-ready SDK for unified artifact access."
status: "completed"
created: "2026-03-10"
updated: "2026-03-13"
research-needed:
  - "Inter-artifact linking design (RES-c387773d)"
  - "Graph SDK design (RES-06ba5474)"
relationships:
  - target: "EPIC-0a8a5e72"
    type: "realises"
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-cda6edd6"
    type: "benefits"
---
## Motivation

The artifact system has hundreds of interconnected files — epics reference milestones, tasks reference epics, decisions supersede other decisions, lessons promote to rules. But there's no unified way to query these relationships. The frontend uses a hardcoded prefix map for navigation, the viewer reads raw files, and backreferences ("what links to this?") are impossible without scanning every file.

A node graph makes all relationships queryable in both directions, enables broken link detection, provides a typed SDK for consistent access, and lays the foundation for plugins that need to traverse artifact relationships.

## Sketch

- Backend: Rust `ArtifactGraph` with `HashMap<String, ArtifactNode>`, bidirectional `ArtifactRef` edges computed during scan
- Frontend: Typed Svelte 5 rune SDK (`artifactGraph`) with synchronous resolution, relationship queries, and plugin subscription API
- Live refresh via `.orqa/` file watcher with debounced graph rebuild
- Replaces all ad-hoc artifact access patterns (prefix map, label matching, raw file reads for metadata)

Research: [RES-c387773d](RES-c387773d) (linking design), [RES-06ba5474](RES-06ba5474) (graph SDK design). Implementation: [EPIC-0a8a5e72](EPIC-0a8a5e72).