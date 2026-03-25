---
id: RULE-3c2da849
type: rule
title: Core Graph Firmware Protection
description: "Core graph artifacts (schemas, orchestrator, core knowledge, role definitions) are firmware — non-editable by agents or users except through the update system or in dogfood mode."
status: active
created: 2026-03-12
updated: 2026-03-12
enforcement:
  - mechanism: behavioral
    message: "Core graph artifacts are firmware — non-editable by agents or users except through the update system or in dogfood mode"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: warn
    pattern: ".orqa/delivery/*/schema.json|.orqa/process/*/schema.json|.orqa/process/agents/schema.json|.orqa/process/knowledge/schema.json"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: warn
    pattern: ".orqa/process/knowledge/composability/KNOW.md|.orqa/process/knowledge/research-methodology/KNOW.md|.orqa/process/knowledge/planning/KNOW.md|.orqa/process/knowledge/orqa-code-search/KNOW.md"
summary: "Core graph artifacts (schemas, orchestrator prompt, core knowledge, role definitions) are firmware — non-editable except via update system or dogfood mode. PostToolUse hooks warn on writes to schema.json and core knowledge files. Dogfood exception: project.json dogfood:true flag bypasses protection."
tier: always
roles: [orchestrator, implementer, writer]
priority: P0
tags: [safety, core-protection, firmware, dogfood]
relationships:
  - target: AD-f079c196
    type: enforces
  - target: AD-45f32bab
    type: enforces
  - target: AD-e8a0f910
    type: enforces
---
Core graph artifacts define how the artifact graph works, how agents traverse it, and how the structured thinking process operates. They are **firmware** — they ship with OrqaStudio and are not modified during normal project use.

## What Is Core (Firmware)

| Artifact | Path | Why It's Protected |
|----------|------|--------------------|
| **Artifact schemas** | `schema.json` in every artifact directory | Define what fields exist, what edges connect artifact types |
| **Core knowledge** | `composability`, `planning`, `research-methodology`, `orqa-code-search` | Define universal methodology all agents use |
| **Orchestrator prompt** | `.orqa/process/agents/orchestrator.md` | Defines graph traversal and process model |
| **Role definitions** | `.orqa/process/agents/*.md` + `schema.json` | Define the 7 universal roles and boundaries |

## What Is NOT Core (User-Editable)

Everything else in `.orqa/` is project-specific and freely editable:

- Project rules, project knowledge, documentation
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
- Modifying core knowledge content outside of an OrqaStudio release
- Modifying the orchestrator prompt to add project-specific content (use project-layer artifacts instead)
- Weakening or removing this rule's enforcement entries without explicit user approval
- Using `ORQA_CORE_OVERRIDE=1` as a routine workaround instead of fixing the actual need

## Related Rules

- [RULE-63cc16ad](RULE-63cc16ad) (artifact-config-integrity) — config paths must match disk; this rule protects the schemas that config relies on
- [RULE-23699df2](RULE-23699df2) (schema-validation) — schemas validate frontmatter; this rule protects the schemas themselves
- [RULE-205d9c91](RULE-205d9c91) (knowledge-portability) — core knowledge must be portable; this rule prevents project-specific contamination
- [RULE-998da8ea](RULE-998da8ea) (dogfood-mode) — dogfood exception to this rule's protection
