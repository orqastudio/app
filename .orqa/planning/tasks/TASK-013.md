---
id: TASK-013
title: Code indexer and regex search
description: Implements the code indexing pipeline using DuckDB to store file chunks, and exposes a regex search command for matching patterns across all indexed content.
status: done
created: 2026-03-04
updated: 2026-03-09
epic: EPIC-034
assignee: backend-engineer
skills:
  - chunkhound
  - rust-async-patterns
  - tauri-v2
scope:
  - src-tauri/src/search/store.rs
  - src-tauri/src/search/chunker.rs
  - src-tauri/src/search/types.rs
  - src-tauri/src/search/mod.rs
  - src-tauri/src/commands/search_commands.rs
acceptance:
  - DuckDB database stores code chunks with file path
  - content
  - and metadata
  - Regex search finds patterns across indexed files
  - IPC command registered and callable from frontend
---
## What

Implement the code indexing pipeline (file walking, semantic chunking, DuckDB storage)
and regex search command.

## Outcome

Implemented as `search_regex` Tauri command. DuckDB stores code chunks with file paths.
Regex patterns are matched across all indexed content. Git commit: `2313f80`.
