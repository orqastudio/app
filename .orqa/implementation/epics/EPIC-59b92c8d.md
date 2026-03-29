---
id: "EPIC-59b92c8d"
type: "epic"
title: "Content Migration to Plugin-Composed Architecture"
description: "Migrate CLAUDE.md, rule files, agent definitions, and knowledge artifacts to the plugin-composed architecture. One epic, sequential tasks with validation between each step."
status: captured
priority: "P1"
created: 2026-03-25T00:00:00.000Z
updated: 2026-03-25T00:00:00.000Z
horizon: "active"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Migration completes the transition to the target architecture for dogfooding"
  - target: "EPIC-281f7857"
    type: "depends-on"
    rationale: "Migration needs the agent lifecycle infrastructure to be in place"
---

## Scope

From RES-d6e8ab11 section 10 (Recommended Path Forward):

Sequential migration with validation between each step:

1. CLAUDE.md → plugin-composed generated prompts (decompose into role + stage + safety)
2. 58 rule files → knowledge plugin artifacts + compressed summaries (100-150 token summaries)
3. Agent definitions → universal role templates + knowledge composition declarations
4. Knowledge artifacts → plugin manifests with injection tier metadata (always/stage-triggered/on-demand)
5. Validation gate between each migration step
6. Remove CLAUDE.md fallback after migration complete

## Design Constraints (from PD-1ef9f57c)

- Short fallback period only — CLAUDE.md loading as safety net while LLM performs migration
- No backwards compatibility — once migrated, old format is dead
- Authors (including agents) write compressed summaries
