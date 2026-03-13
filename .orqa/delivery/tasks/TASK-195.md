---
id: TASK-195
title: Design WorkflowTracker domain type
description: |
  Create a Rust struct that tracks session-level events to enable process gate
  evaluation. Tracks files read/written, searches performed, docs consulted,
  skills loaded, and commands run.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-193
---


## What

A `WorkflowTracker` struct that accumulates session events relevant to process
gate evaluation. Process gates query this tracker to determine if prerequisites
are met before allowing transitions (e.g., "has the agent read any docs before
writing code?").

## How

1. Create `backend/src-tauri/src/domain/workflow_tracker.rs`
2. Define `WorkflowTracker` with: files_read, files_written, searches_performed,
   docs_consulted, skills_loaded, commands_run, turn_count
3. Add methods: `record_read()`, `record_write()`, `record_search()`, etc.
4. Add query methods: `has_read_docs()`, `has_searched()`, `has_loaded_skills()`
5. Wire into the domain module

## Verification

- Struct compiles and passes unit tests
- Query methods return correct results after recording events
- Tracker resets per session (not per turn)
