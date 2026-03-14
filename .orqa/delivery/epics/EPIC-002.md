---
id: EPIC-002
title: Settings UI for Thinking & Custom Prompt
description: Add UI controls for show_thinking toggle and custom system prompt fields that already exist in the backend.
status: draft
priority: P1
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
horizon: next
pillars:
  - PILLAR-001
research-refs: []
docs-required: []
docs-produced: []
scoring:
  pillar: 4
  impact: 4
  dependency: 2
  effort: 1
  score: 26
relationships:
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: DOC-036
    type: documented-by
    rationale: Referenced in documentation page Artifact Framework
---
## Why P1

Can't control reasoning behaviour without these toggles. The custom system prompt is how dogfooding context reaches the AI.

## Tasks

- [ ] Add `show_thinking` toggle in Settings > Model section
- [ ] Add `custom_system_prompt` textarea in Settings > Project section
- [ ] Add "View auto-generated prompt" collapsible preview (reuse `build_system_prompt()`)
- [ ] Separate dogfood context from base orchestrator — move dogfood behavioral rules (session lifecycle, sidecar self-edit warnings, restart protocol) out of the orchestrator prompt and into app-injected system prompt only. CLI sessions should NOT absorb dogfood-mode behavioral rules via `.claude/rules/` symlinks. The `[RULE-019](RULE-019)` (dogfood-mode) file should either be restructured or the app system prompt builder should be the sole injection point for dogfood behavioral context when `project.json` has `dogfood: true`.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
