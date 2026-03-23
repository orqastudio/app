---
id: IMPL-b4c6d8ea
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

The orchestrator invents relationship types that sound correct but aren't in the plugin-defined vocabulary. The valid types are defined in `plugins/agile-governance/orqa-plugin.json` and `plugins/software/orqa-plugin.json` but the orchestrator doesn't check these before writing relationships.

## Recurrence

- First occurrence: EPIC-d4a8c1e5 used `dependency-of`, IDEA-d8f2a4c6 used `follows` and `related`, AD-b7e3f1a2 used `evolves`
- Second occurrence: AD-b7e3f1a2 used `supersedes` (after the first batch was "fixed"), RULE-d2e4f6a8 used `supports`

## Recommendation — Promote to Enforcement

At recurrence >= 2, this should be promoted to mechanical enforcement:
1. The pre-commit hook should validate relationship types against the plugin schemas
2. The connector's PreToolUse:Write hook should check relationship types when writing to .orqa/
3. The relationship type vocabulary should be injected into the orchestrator's context at session start
