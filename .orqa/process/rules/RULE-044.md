---
id: RULE-044
title: Core Graph Firmware Protection
description: Core graph artifacts (schemas, orchestrator, core skills, role definitions) are firmware — non-editable by agents or users except through the update system or in dogfood mode.
status: active
created: 2026-03-12
updated: 2026-03-12
layer: core
scope: []
enforcement:
  - event: file
    pattern: .*
    paths:
      - .orqa/delivery/*/schema.json
      - .orqa/process/*/schema.json
      - .orqa/process/agents/schema.json
      - .orqa/process/skills/schema.json
    action: block
    message: "BLOCKED: schema.json files are core graph firmware. They define artifact structure for all projects. Do not modify unless you are updating OrqaStudio itself (dogfood mode). See RULE-044."
  - event: file
    pattern: .*
    paths:
      - .orqa/process/skills/composability/SKILL.md
      - .orqa/process/skills/research-methodology/SKILL.md
      - .orqa/process/skills/planning/SKILL.md
      - .orqa/process/skills/orqa-code-search/SKILL.md
    action: block
    message: "BLOCKED: Core skills are firmware. They define universal methodology for all projects. Do not modify unless you are updating OrqaStudio itself (dogfood mode). See RULE-044."
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Core graph protection preserves the fundamental structural integrity of the system
  - target: RULE-003
    type: informs
    rationale: Config paths rely on stable schema.json files — firmware protection ensures config-driven scanning remains consistent
  - target: RULE-032
    type: informs
    rationale: Schema validation depends on schema.json files that this rule protects from accidental modification
  - target: RULE-035
    type: informs
    rationale: Core skills are firmware — this protection prevents project-specific contamination of portable skills
  - target: RULE-009
    type: informs
    rationale: Dogfood mode is the only exception where core graph artifacts become editable
  - type: enforces
    target: AD-038
    rationale: AD-038 established the five-layer architecture whose core layer this rule protects as firmware
  - type: enforces
    target: AD-039
    rationale: AD-039 defined the core graph firmware concept that this rule implements through file-level blocking
---
Core graph artifacts define how the artifact graph works, how agents traverse it, and how the structured thinking process operates. They are **firmware** — they ship with OrqaStudio and are not modified during normal project use.

## What Is Core (Firmware)

| Artifact | Path | Why It's Protected |
|----------|------|--------------------|
| **Artifact schemas** | `schema.json` in every artifact directory | Define what fields exist, what edges connect artifact types |
| **Core skills** | `composability`, `planning`, `research-methodology`, `orqa-code-search` | Define universal methodology all agents use |
| **Orchestrator prompt** | `.orqa/process/agents/orchestrator.md` | Defines graph traversal and process model |
| **Role definitions** | `.orqa/process/agents/*.md` + `schema.json` | Define the 7 universal roles and boundaries |

## What Is NOT Core (User-Editable)

Everything else in `.orqa/` is project-specific and freely editable:

- Project rules, project skills, documentation
- Planning artifacts (tasks, epics, ideas, research, milestones)
- Governance artifacts (decisions, lessons)
- Project configuration (`project.json`)

## Why This Matters

The core graph is the foundation the entire system builds on. If a schema is changed incorrectly:
- Existing artifacts may fail validation
- Graph traversal instructions become wrong
- The plugin injects incorrect context
- The thinking process breaks down

These are the same class of risk as modifying a database schema without a migration — downstream systems depend on the contract.

## Dogfood Exception

When `project.json` contains `"dogfood": true`, core artifacts ARE editable because the developer is building OrqaStudio itself. The enforcement engine and pre-commit hook both check this flag and skip the protection.

In all other projects, core artifacts are read-only at three levels:
1. **This rule** — agents are blocked from writing to core files
2. **Pre-commit hook** — git commits touching core files are blocked (override: `ORQA_CORE_OVERRIDE=1`)
3. **App UI** (future) — core files render as read-only in the artifact editor

## FORBIDDEN

- Modifying `schema.json` files during normal project work
- Modifying core skill content outside of an OrqaStudio release
- Modifying the orchestrator prompt to add project-specific content (use project-layer artifacts instead)
- Weakening or removing this rule's enforcement entries without explicit user approval
- Using `ORQA_CORE_OVERRIDE=1` as a routine workaround instead of fixing the actual need

## Related Rules

- [RULE-003](RULE-003) (artifact-config-integrity) — config paths must match disk; this rule protects the schemas that config relies on
- [RULE-032](RULE-032) (schema-validation) — schemas validate frontmatter; this rule protects the schemas themselves
- [RULE-035](RULE-035) (skill-portability) — core skills must be portable; this rule prevents project-specific contamination
- [RULE-009](RULE-009) (dogfood-mode) — dogfood exception to this rule's protection
