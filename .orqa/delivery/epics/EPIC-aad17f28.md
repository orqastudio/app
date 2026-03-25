---
id: EPIC-aad17f28
type: epic
title: "Rule enforcement generation"
description: "Implement automatic hook generation from rule enforcement specifications. Rules declare enforcement mode (mechanical/advisory/validation/workflow-guard/observational) and constraints in YAML frontmatter. The install pipeline generates Claude Code hooks from mechanical rules using templates. Closes the gap where 20 rules declare hook specifications but only 3-4 have corresponding implementations."
status: active
priority: P0
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: RES-2c959f47
    type: informed-by
    rationale: "Research document designed the enforcement architecture"
  - target: EPIC-a5501c18
    type: depends-on
    rationale: "Connector rebuild must be complete before generating hooks into it"
  - target: AD-1ef9f57c
    type: implements
    rationale: "AD resolved against DSL in favor of declarative templates"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Mechanical enforcement is required for dogfooding milestone"
---

## Context

RES-2c959f47 found that 20 rules declare 44 hook specifications in YAML frontmatter, but only 3-4 have corresponding hand-written hooks. The remaining rules' enforcement declarations are aspirational — they describe enforcement that does not exist.

Six enforcement archetypes were identified. Archetypes 1-3 (bash patterns, field checks, file guards) cover 58% of entries and are trivially templateable. The research recommends template-based generation at `orqa install` time.

34 of 59 active rules are appropriately advisory-only and should NOT be mechanized.

## Tasks

### Phase 1: Schema + Migration

**TASK-1: Define enforcement entry JSON schema**
- Add enforcement entry schema to plugin type definitions
- Five modes: mechanical, advisory, validation, workflow-guard, observational
- Six templates: bash-pattern, field-check, file-guard, session-state, knowledge-inject, session-lifecycle
- Each entry: mode, event, matcher, template, parameters, action (block/warn), message
- Acceptance criteria: schema in libs/types, TypeScript types exported, JSON Schema for validation

**TASK-2: Migrate 59 active rules to standardized enforcement schema**
- Audit all 59 rules' existing enforcement entries
- Standardize to the new schema format
- 20 rules already declare hook entries — normalize their format
- 34 advisory-only rules — add explicit `mode: advisory` entries
- Remaining rules — classify and add appropriate entries
- Acceptance criteria: all 59 rules have valid enforcement entries, schema validates

### Phase 2: Template Engine

**TASK-3: Build bash-pattern template**
- Template that generates a hook script matching bash commands against regex patterns
- Reads patterns from rule enforcement entries at install time
- Generates a single `generated/bash-guard.mjs` that checks all patterns
- Replaces the 4 hardcoded patterns in rule-engine.ts with all 17 declared patterns
- Acceptance criteria: template generates working hook, covers all bash-pattern entries, tests pass

**TASK-4: Build field-check template**
- Template for checking tool_input fields (e.g. Agent must have run_in_background: true)
- Replaces enforce-background-agents.mjs with generated version
- Acceptance criteria: template generates working hook, replaces hand-written script

**TASK-5: Build file-guard template**
- Template for glob-matching file paths (e.g. block writes to plugin-owned files)
- Integrates with manifest.json for file ownership
- Acceptance criteria: template generates working hook, covers all file-guard entries

### Phase 3: Generation Pipeline

**TASK-6: Build the generation pipeline**
- Scanner: reads all rules, parses enforcement entries
- Template engine: maps entries to templates, generates scripts
- Merger: combines generated + hand-written hooks into hooks.json
- Writer: outputs to connectors/claude-code/hooks/generated/
- Verification: validates generated hooks can be loaded
- Acceptance criteria: `orqa install` generates hooks from rules, hand-written hooks preserved

**TASK-7: Wire into install pipeline**
- Call hook generation after prompt registry build and agent file generation
- Add `orqa hooks generate` CLI command for standalone use
- Acceptance criteria: `orqa install` produces complete hooks.json, `orqa hooks generate` works standalone

### Phase 4: Verification

**TASK-8: End-to-end verification**
- Verify all 44 hook entries produce working enforcement
- Test: bare Agent spawn is blocked (RULE-99abcea1)
- Test: --no-verify is blocked (RULE-00700241)
- Test: foreground agents are blocked (RULE-99abcea1)
- Acceptance criteria: all mechanical rules produce working hooks, advisory rules still inject via pipeline

**TASK-9: Enforcement coverage reporting**
- Add `orqa audit enforcement` command
- Reports: which rules have enforcement, which modes, coverage gaps
- Acceptance criteria: command produces accurate report matching research findings
