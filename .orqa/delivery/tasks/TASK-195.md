---
id: TASK-195
title: Design WorkflowTracker domain type
description: |
  Create a Rust struct that tracks session-level events to enable process gate
  evaluation. Tracks files read/written, searches performed, docs consulted,
  skills loaded, and commands run.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - WorkflowTracker struct exists in backend domain module
  - "Record methods for read, write, search, and command events"
  - "Query methods for has_read_docs, has_searched, has_loaded_skills"
  - Unit tests pass for event recording and querying
relationships:
  - target: EPIC-052
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-193
    type: depends-on
  - target: TASK-196
    type: depended-on-by
  - target: TASK-197
    type: depended-on-by
  - target: TASK-198
    type: depended-on-by
  - target: TASK-200
    type: depended-on-by
  - target: TASK-342
    type: depended-on-by
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
