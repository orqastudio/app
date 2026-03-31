---
id: IDEA-7e8f58c9
type: discovery-idea
status: captured
title: Move agent generation into app core framework
created: 2026-03-28T14:00:00.000Z
relationships:
  - type: realises
    target: EPIC-2451d1a9
  - type: benefits
    target: PERSONA-c4afd86b
---

# Move Agent Generation into App Core Framework

## Context

The connector's `agent-file-generator.ts` (~438 lines) contains hardcoded agent role definitions, tool constraints, and file scope rules. `generator.ts` reads `.orqa/` paths directly (rules, workflows) instead of calling the daemon. Both are business logic that belongs in the app core, not the connector translation layer.

## Proposal

After the migration stabilizes and the UI/connector process is solid, move agent file generation from the connector into the app framework:

1. **Role definitions** — come from the methodology plugin's role YAMLs (already exist in `plugins/agile-methodology/roles/`), not hardcoded templates
2. **Tool constraints and file permissions** — come from the engine's enforcement configuration, not inline strings
3. **Generator reads from daemon** — `generator.ts` calls daemon endpoints for rules, workflows, and schema instead of reading `.orqa/` paths directly
4. **App framework owns generation** — the app core provides the agent assembly pipeline; the connector is purely a format translator (engine output → Claude Code plugin format)

## Relationship to agent framework

This is part of the broader work to build out the in-app agent framework. How agent generation links into the connector is part of that design. The connector should consume what the app provides, not duplicate it.

## Priority

Post-migration. After Phases 7-11 complete and the UI/connector process is stable.
