---
id: "TASK-33fce918"
type: task
title: "Document orphaned skills as forward-looking in their SKILL.md files"
description: "Add a forward-looking status note to each of five skills that have no current loading mechanism because their parent features are not yet built."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "Each of the 5 skills has a clear note that it is forward-looking"
  - "Each note references the parent epic/idea it will be activated by"
  - "No changes to the skill content itself — just status clarity"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-698afd4c"
    type: "depended-on-by"
---
## What

Five skills have no current loading mechanism because the features they support are not built yet. They are already referenced from the correct epics/tasks/ideas:

| Skill | Linked From |
|-------|------------|
| `project-inference` | [EPIC-7394ba2a](EPIC-7394ba2a), [TASK-bc351dc1](TASK-bc351dc1) |
| `project-migration` | [EPIC-7394ba2a](EPIC-7394ba2a), [TASK-bc351dc1](TASK-bc351dc1) |
| `project-setup` | [EPIC-7394ba2a](EPIC-7394ba2a), [TASK-bc351dc1](TASK-bc351dc1) |
| `project-type-software` | [EPIC-7394ba2a](EPIC-7394ba2a), [TASK-bc351dc1](TASK-bc351dc1), [TASK-427a541f](TASK-427a541f), [EPIC-31a26b85](EPIC-31a26b85) |
| `orqa-plugin-development` | [IDEA-5113eeae](IDEA-5113eeae), [TASK-38b82d46](TASK-38b82d46) |

Add a note to each skill's SKILL.md frontmatter or body indicating it is forward-looking and which epic/idea it supports. This prevents them from appearing to be active skills that should be loaded.

## How

1. Open each of the five skill files in `.orqa/process/skills/`
2. Add a `status: forward-looking` field or a note block at the top of the body (e.g., `> **Forward-looking:** This skill will be activated by [EPIC-7394ba2a](EPIC-7394ba2a) when project initialisation is implemented.`)
3. Reference the relevant epic or idea ID in the note
4. Leave all skill content unchanged

## Verification

- [ ] Each of the 5 skills has a clear note that it is forward-looking
- [ ] Each note references the parent epic/idea it will be activated by
- [ ] No changes to the skill content itself — just status clarity