---
id: RULE-035
title: Skill Portability
description: Skills must be portable within their declared layer. Core skills must work on any project unchanged. Project skills must declare their scope.
status: active
created: 2026-03-11
updated: 2026-03-11
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Skill portability ensures clean separation between core and project content
  - target: RULE-026
    type: informs
    rationale: Skill loading model requires skills to be correctly layered and declared as portable or project-specific
  - target: RULE-005
    type: informs
    rationale: Search skills are core-layer portable and demonstrate the context-resolved portability pattern
  - type: informed-by
    target: RULE-038
    rationale: User-invocable field is part of the portability contract each skill must declare
  - type: informed-by
    target: RULE-044
    rationale: Core skills are firmware — they are protected from project-specific contamination
  - type: enforces
    target: AD-001
    rationale: Skill portability ensures skills correctly separate core from project concerns, supporting the thick backend architecture boundary
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
---
A skill's `layer` field declares its portability scope. Content within the skill MUST be portable within that scope. A core skill that contains project-specific paths is broken — it will give wrong guidance on other projects.

## Layer Requirements

| Layer | Portability Test | Allowed Content |
|-------|-----------------|-----------------|
| `core` | Would this skill be useful on a different project unchanged? | General principles, language/framework patterns, methodology |
| `project` | Does this skill describe THIS project's specific patterns? | Project-specific paths, conventions, architecture patterns |
| `plugin` | Can this skill be installed from an external source? | Same as core — must be self-contained and portable |

## Core Layer Constraints

Core skills (`layer: core`) MUST NOT contain:

- Project-specific file paths (e.g., `backend/src-tauri/src/domain/sessions.rs`)
- Architecture decision references from this project (e.g., [AD-001](AD-001), [AD-005](AD-005))
- Project-specific config values (hardcoded URLs, service names, environment variables)
- Enforcement rules that belong in `.orqa/process/rules/`
- Product decisions that belong in `.orqa/documentation/product/`
- Implementation patterns specific to this codebase's conventions

## Project Layer Constraints

Project skills (`layer: project`) MUST:

- Declare their project scope in the skill description
- Reference project-specific paths, patterns, and conventions as appropriate
- Be clearly marked as non-portable

## FORBIDDEN

- Core skills with project-specific file paths or artifact IDs
- Project skills without a clear scope declaration
- Core skills that reference project rules or decisions by ID
- Mixing core and project content in a single skill — split into two skills instead

## Related Rules

- [RULE-026](RULE-026) (skill-enforcement) — skill loading and tier model
- [RULE-005](RULE-005) (code-search-usage) — search skills as an example of context-resolved portability
