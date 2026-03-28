---
id: "TASK-6a335dc3"
type: "task"
title: "Create code-search wrapper skill"
description: "Creates a context-detecting wrapper skill that instructs agents to load the appropriate concrete search skill depending on whether they are running in CLI or App context."
status: archived
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "Wrapper skill exists at .orqa/process/skills/code-search/SKILL.md"
  - "Documents context detection logic (check for mcp__chunkhound__* vs search_regex availability)"
  - "Instructs agent to load chunkhound (CLI) or orqa-native-search (App) based on detection"
  - "Shared query patterns documented (same as both underlying skills)"
relationships:
  - target: "EPIC-42a5330b"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Create the `code-search` wrapper skill that detects runtime context (CLI vs App) and
instructs the agent to load the appropriate concrete skill.

## Implementation Notes

The wrapper skill SKILL.md should:

1. Explain the two contexts and how to detect them
2. Provide the shared query patterns (identical across both implementations)
3. Instruct: "If `mcp__chunkhound__*` tools are available, you are in CLI context — follow

   ChunkHound patterns. If `search_regex`/`search_semantic` are Tauri commands, you are in
   App context — follow native search patterns."

4. Link to both concrete skills for implementation-specific details

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
