---
id: TASK-aa9aec9e
type: task
title: "Reverse relationship checks with code actions"
description: "Surface missing inverse relationships as diagnostics and offer code actions to auto-insert the missing inverse in the target artifact."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3ecc76ff
    type: delivers
    rationale: "Reverse relationship checks enforce graph consistency from the editor"
  - target: TASK-061b5052
    type: depends-on
    rationale: "Needs plugin schemas to know which relationship types require inverses"
  - target: TASK-124d9841
    type: depends-on
    rationale: "Code action infrastructure must exist before adding reverse-relationship quick fixes"
---

# Reverse Relationship Checks with Code Actions

## What to Implement

The daemon already performs `MissingInverse` checks via `/validate`. The LSP receives these as diagnostics. This task adds:

1. Precise positioning of the diagnostic at the relationship entry that lacks an inverse.
2. A code action that inserts the inverse relationship into the target artifact's frontmatter.

### Steps

1. **Map MissingInverse diagnostics to source lines** — find the `- target: <id>` line in frontmatter where the relationship with a missing inverse is declared.

2. **Create a code action for each MissingInverse diagnostic** — the action edits the target file to add the inverse relationship entry. Use `WorkspaceEdit` with a `TextEdit` that inserts the YAML block.

3. **Determine inverse relationship type from plugin schemas** — plugin `RelationshipDef` entries define forward/inverse pairs (e.g., `delivers` / `delivered-by`). Use these to generate the correct inverse type.

4. **Handle the case where the target file is not open** — the code action must work even when the target artifact is not open in the editor (use `WorkspaceEdit` with document changes).

## Acceptance Criteria

- [ ] Missing inverse relationships produce Warning diagnostics at the correct frontmatter line
- [ ] Each diagnostic offers a code action to insert the inverse in the target artifact
- [ ] The inserted inverse uses the correct relationship type from plugin schemas
- [ ] The code action works even when the target file is not open in the editor
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
