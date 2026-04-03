---
id: TASK-test001
type: task
title: "Test Task"
description: "A minimal test task used in daemon integration test fixtures."
status: todo
created: 2026-04-01
updated: 2026-04-01
relationships:
  - target: EPIC-test001
    type: delivers
    rationale: "Fixture relationship: task delivers its parent epic."
---

This task delivers EPIC-test001, creating a single directed edge in the fixture graph. Tests can assert that the graph has at least one edge and that traceability from this task leads to the epic.
