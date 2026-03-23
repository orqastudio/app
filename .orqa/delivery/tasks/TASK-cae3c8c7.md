---
id: TASK-cae3c8c7
type: task
title: "Agent skill + docs for artifact ID format and generation"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-0b703fd2
    type: depends-on
---

# TASK-cae3c8c7: Agent Skill for Artifact IDs

## Acceptance Criteria

1. Skill created documenting the `TYPE-XXXXXXXX` hex ID format
2. Covers how to generate IDs (CLI command, programmatic)
3. Covers ID format validation rules
4. Covers migration context (old sequential IDs still accepted)
5. Human-facing documentation artifact created and `synchronised-with` linked
6. Skill registered in appropriate plugin/core manifest