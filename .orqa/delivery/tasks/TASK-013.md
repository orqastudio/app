---
id: TASK-013
title: Code indexer and regex search
description: "Implements the code indexing pipeline using DuckDB to store file chunks, and exposes a regex search command for matching patterns across all indexed content."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - DuckDB database stores code chunks with file path
  - content
  - and metadata
  - Regex search finds patterns across indexed files
  - IPC command registered and callable from frontend
relationships:
  - target: EPIC-034
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-003
    type: informed-by
  - target: SKILL-027
    type: informed-by
  - target: SKILL-032
    type: informed-by
  - target: TASK-324
    type: depended-on-by
  - target: SKILL-003
    type: informed-by
  - target: SKILL-027
    type: informed-by
  - target: SKILL-032
    type: informed-by
---
## What

Implement the code indexing pipeline (file walking, semantic chunking, DuckDB storage)
and regex search command.

## Outcome

Implemented as `search_regex` Tauri command. DuckDB stores code chunks with file paths.
Regex patterns are matched across all indexed content. Git commit: `2313f80`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
