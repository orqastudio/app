---
id: TASK-141
title: Document orphaned skills as forward-looking in their SKILL.md files
description: Add a forward-looking status note to each of five skills that have no current loading mechanism because their parent features are not yet built.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - Each of the 5 skills has a clear note that it is forward-looking
  - Each note references the parent epic/idea it will be activated by
  - No changes to the skill content itself — just status clarity
---
## What

Five skills have no current loading mechanism because the features they support are not built yet. They are already referenced from the correct epics/tasks/ideas:

| Skill | Linked From |
|-------|------------|
| `project-inference` | [EPIC-045](EPIC-045), [TASK-054](TASK-054) |
| `project-migration` | [EPIC-045](EPIC-045), [TASK-054](TASK-054) |
| `project-setup` | [EPIC-045](EPIC-045), [TASK-054](TASK-054) |
| `project-type-software` | [EPIC-045](EPIC-045), [TASK-054](TASK-054), [TASK-067](TASK-067), [EPIC-047](EPIC-047) |
| `orqa-plugin-development` | [IDEA-038](IDEA-038), [TASK-081](TASK-081) |

Add a note to each skill's SKILL.md frontmatter or body indicating it is forward-looking and which epic/idea it supports. This prevents them from appearing to be active skills that should be loaded.

## How

1. Open each of the five skill files in `.orqa/process/skills/`
2. Add a `status: forward-looking` field or a note block at the top of the body (e.g., `> **Forward-looking:** This skill will be activated by [EPIC-045](EPIC-045) when project initialisation is implemented.`)
3. Reference the relevant epic or idea ID in the note
4. Leave all skill content unchanged

## Verification

- [ ] Each of the 5 skills has a clear note that it is forward-looking
- [ ] Each note references the parent epic/idea it will be activated by
- [ ] No changes to the skill content itself — just status clarity
