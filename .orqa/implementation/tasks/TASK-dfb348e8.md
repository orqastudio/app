---
id: TASK-dfb348e8
type: task
title: "Test delegation: verify knowledge injection end-to-end"
description: "Write and run an end-to-end test that verifies the connector correctly injects knowledge files from canonical plugin paths into subagent prompts during delegation."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - Test verifies knowledge content from a plugin knowledge/ file appears in delegated subagent context
  - Test covers the case where a knowledge file is missing (graceful degradation)
  - Test covers the case where project.json plugin path is invalid
  - All tests pass with make test
  - No mocks of internal domain logic (only adapter/boundary mocking per RULE-029)
relationships:
  - target: EPIC-fdcdb958
    type: delivers
  - target: TASK-904a7533
    type: depends-on
---

## What

Write integration tests that exercise the full knowledge injection path:

1. Connector reads `project.json` and discovers plugin paths
2. Knowledge file is found at canonical location
3. Content is included in the delegation prompt sent to the subagent
4. Edge cases: missing file, invalid path, empty knowledge set

## How

Create test fixtures:

- A minimal `project.json` pointing to a test plugin directory
- A test plugin directory with `knowledge/test-domain/SKILL.md`

Write integration tests (in `connectors/claude-code/` test directory or equivalent):

- Happy path: knowledge injection produces prompt containing knowledge content
- Missing file: delegation proceeds without knowledge, no error thrown
- Invalid plugin path: delegation proceeds without knowledge, warning logged

Follow RULE-029: mock only at the adapter/boundary layer. Do not mock the file system directly — use test fixture directories instead.

## Verification

1. `make test` passes with all new tests included
2. Test output confirms knowledge content was injected in the happy path
3. Error path tests confirm no crash on missing/invalid paths
4. Coverage for the knowledge resolution module meets the 80% threshold
