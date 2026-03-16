---
id: TASK-150
title: Create task dependency validator hook
description: Pre-task hook that checks depends-on tasks have status done before allowing a task to start.
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Starting a task with unfinished dependencies produces a clear warning
  - Lists which dependencies are not done
  - Does not block if depends-on is empty or all dependencies are done
relationships:
  - target: EPIC-049
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-339
    type: depended-on-by
---
## What

[RULE-004](RULE-004) defines the task dependency gate as NON-NEGOTIABLE — tasks with `depends-on` entries must have all dependencies at `status: done` before starting. Currently this is orchestrator-enforced via manual checking. Automate it.

## How

1. Create a validation script that reads a task's `depends-on` frontmatter
2. For each dependency, read the referenced task file and check its status
3. If any dependency is not `done`, report which ones and exit non-zero
4. Can be integrated as a pre-commit check (when task status changes to in-progress) or as a standalone validation script called by the orchestrator

## Verification

- [ ] Task with unfinished depends-on produces clear error listing blocked dependencies
- [ ] Task with all depends-on done passes validation
- [ ] Task with empty depends-on passes validation
- [ ] Script handles missing task files gracefully
