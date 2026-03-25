---
id: IMPL-6b8ab467
type: lesson
title: Orchestrator repeatedly uses invalid relationship types in artifact frontmatter
category: governance
status: promoted
recurrence: 2
promoted-to: RULE-pending-relationship-validation
created: 2026-03-23
tags: [dogfooding, relationship-types, schema-compliance]
---

## Observation

The orchestrator has used invalid relationship types (`supersedes`, `supports`, `follows`, `related`, `evolves`, `dependency-of`) in artifact frontmatter on two separate occasions in one session. The validation daemon logs WARN and silently skips them, breaking graph traceability.

## Root Cause

The orchestrator invents relationship types that sound correct but aren't in the plugin-defined vocabulary. The valid types are defined in `plugins/agile-workflow/orqa-plugin.json` and `plugins/software-kanban/orqa-plugin.json` but the orchestrator doesn't check these before writing relationships.

## Recurrence

- First occurrence: EPIC-8b01ee51 used `dependency-of`, IDEA-f8f8dc69 used `follows` and `related`, AD-9ab3e0a4 used `evolves`
- Second occurrence: AD-9ab3e0a4 used `supersedes` (after the first batch was "fixed"), RULE-04684a16 used `supports`

## Recommendation — Promote to Enforcement

At recurrence >= 2, this should be promoted to mechanical enforcement:
1. The pre-commit hook should validate relationship types against the plugin schemas
2. The connector's PreToolUse:Write hook should check relationship types when writing to .orqa/
3. The relationship type vocabulary should be injected into the orchestrator's context at session start
