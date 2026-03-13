---
id: TASK-171
title: Create capability vocabulary rule (RULE-040)
description: Create a rule defining the abstract capability vocabulary, provider mapping tables, and delegation resolution protocol.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-051
depends-on: []
assignee: AGENT-003
docs:
  - DOC-036
skills:
  - SKILL-011
  - SKILL-037
acceptance:
  - Rule defines the canonical capability vocabulary (file_read, file_write, etc.)
  - Rule contains provider mapping tables for Claude Code CLI and OrqaStudio App
  - Rule specifies the orchestrator delegation protocol for capability resolution
  - Rule includes detection mechanism for determining current context
  - Rule passes schema validation
---

## What

Create [RULE-040](RULE-040) that governs how agent capabilities map to provider-specific tools.
This is the foundation that [EPIC-050](EPIC-050) (companion plugin) and all future provider
integrations will consume.

## How

1. Define the abstract capability vocabulary as a table
2. Define the Claude Code CLI mapping table
3. Define the OrqaStudio App mapping table
4. Specify the delegation protocol: before spawning any subagent, resolve capabilities
   to the current context's tool names
5. Specify context detection: CLI = Claude Code native tools available; App = Tauri
   commands available

## Verification

- Rule file exists at `.orqa/process/rules/[RULE-040](RULE-040).md`
- Frontmatter passes schema validation
- All current agent tool lists can be expressed using the capability vocabulary
- No capability is ambiguous or overlapping
