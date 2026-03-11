---
id: RULE-035
title: Skill Portability
description: Skills must be portable within their declared layer. Core skills must work on any project unchanged. Project skills must declare their scope.
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: core
scope: [AGENT-003]
promoted-from: null
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

- Project-specific file paths (e.g., `src-tauri/src/domain/sessions.rs`)
- Architecture decision references from this project (e.g., [AD-001](AD-001), [AD-005](AD-005))
- Project-specific config values (hardcoded URLs, service names, environment variables)
- Enforcement rules that belong in `.orqa/governance/rules/`
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
- [RULE-005](RULE-005) (chunkhound-usage) — search skills as an example of context-resolved portability
