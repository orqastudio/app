---
id: IDEA-d4c0f409
type: idea
title: App-shipped platform knowledge — immutable Layer 1 documentation and skills
description: "Platform knowledge (type definitions, relationship semantics, enforcement rules, how the system works) ships as immutable Layer 1 artifacts. Platform docs teach humans. Platform skills teach agents. Both are versioned with the app and cannot be edited by the project. Project-level docs and skills are separate, editable, and synchronised via relationships."
status: exploring
created: 2026-03-15
updated: 2026-03-15
horizon: next
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

Currently, knowledge about how OrqaStudio works is spread across project-level skills and rules. These can be edited or deleted by the project, which means the AI could lose understanding of how the platform itself works.

Platform knowledge is Layer 1 — immutable, app-shipped, protected by pre-commit hooks.

## Three Categories of Knowledge

### Platform (Layer 1 — Immutable)

| For Humans | For Agents |
|---|---|
| Platform documentation | Platform skills |
| How to use OrqaStudio | How agents interact with OrqaStudio |
| Versioned with the app | Versioned with the app |
| Read-only in the app | Loaded automatically into context |

What becomes platform knowledge:
- How artifact types work (what a rule is, what a lesson is, what statuses mean)
- The state machine and how to work within it
- How the artifact graph works (relationships, integrity, traversal)
- How the learning loop works (lesson → recurrence → promotion)
- Canonical relationship semantics

### Project (Layer 3 — Editable, Synchronised)

| For Humans | For Agents |
|---|---|
| Project documentation | Project skills |
| Architecture docs, how-to guides | How agents work in THIS project |
| Editable by the project | Editable by the project |
| `synchronised-with` project skills | `synchronised-with` project docs |

The integrity checker flags when one changes without the other — preventing drift.

### Domain (Layer 3 — Portable, No Doc Counterpart)

Domain skills (Rust patterns, Svelte 5, testing methodology) are portable expertise. They're project-scoped but not project-specific — they could be shared across projects. They have no documentation counterpart.

## Current Rules That Are Actually Platform Knowledge

Rules like RULE-b10fe6d1 (artifact lifecycle), RULE-23699df2 (schema validation), and RULE-130f1f63 (data integrity) describe how the platform works, not what this project requires. They should become platform documentation/skills (Layer 1), not project rules (Layer 3).