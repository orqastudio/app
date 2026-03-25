---
id: "AD-45cfe1d1"
type: decision
title: "Config-Driven Artifact Scanning"
description: "The artifact scanner reads paths from project.json config, not hardcoded constants. Directories are walked recursively like a file explorer. Frontmatter title is used for display labels.\n"
status: "completed"
created: "2026-03-08"
updated: "2026-03-13"
relationships:
  - target: "RULE-63cc16ad"
    type: "enforced-by"
    rationale: "RULE-63cc16ad mandates config-driven paths, forbids hardcoded artifact paths in Rust or TypeScript, and requires config to match disk structure"
  - target: "EPIC-2f1efbd5"
    type: "drives"
  - target: "RULE-25baac14"
    type: "enforced-by"
---
## Decision

The artifact scanner in `artifact_reader.rs` is config-driven. The `artifacts` array in `project.json` defines exactly what gets scanned — the scanner does not guess or hardcode paths. Directories are walked recursively like a file explorer, building a tree of `DocNode` entries. Every `.md` file has its YAML frontmatter extracted for display labels (title field, falling back to humanized filename).

## Rationale

The original scanner used hardcoded navigation constants in the frontend and flat directory scanning in the backend. This failed when the `.orqa/` directory structure was reorganized — paths in code didn't match paths on disk, causing empty artifact views with no error (see [IMPL-91d951b6](IMPL-91d951b6)). Config-driven scanning eliminates this class of bug by making the path mapping explicit and auditable.

## Consequences

- Adding a new artifact type requires adding it to `project.json` first
- Moving directories on disk requires updating config paths
- The scanner handles both flat (milestones, epics) and tree (documentation) structures
- README.md files are skipped as browsable artifacts
- Hidden entries (`.` or `_` prefix) are skipped
- Empty directories are omitted from the tree