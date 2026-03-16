---
id: TASK-133
title: Implement governance filesystem scanner
description: Built the filesystem walker that collects governance artifacts from the .orqa/ directory structure.
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - Scanner discovers all governance artifacts in the project
  - Frontmatter is parsed correctly for each artifact type
  - "Scan results include file paths, types, and metadata"
relationships:
  - target: EPIC-031
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-321
    type: depended-on-by
---
## What

Built the governance filesystem scanner that recursively walks `.orqa/` directories, parses YAML frontmatter, and classifies artifacts by type.

## How

Implemented the walker in the domain layer using `walkdir`, parsing frontmatter with `serde_yaml` and matching directory paths to artifact type classifications. Results are returned as structured `GovernanceScanResult` via IPC.

## Verification

Scanner discovers all governance artifacts, frontmatter is parsed correctly for each type, and results include file paths, types, and metadata.
