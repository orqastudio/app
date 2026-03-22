---
id: TASK-9e0fa1b2
title: "Update UI navigation: Skills → Knowledge labels, icons"
description: Update the OrqaStudio frontend navigation to display 'Knowledge' instead of 'Skills', update any icons or visual indicators, and ensure the artifact browser routes correctly to knowledge artifacts.
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - No UI label reads 'Skills' or 'Skill' in the navigation or artifact browser
  - Knowledge artifacts appear under a 'Knowledge' navigation node
  - Icons for knowledge artifacts are appropriate and consistent
  - make typecheck passes
  - make lint-frontend passes
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-e5f6a7b8
    type: depends-on
  - target: TASK-c9d0e1f2
    type: depends-on
  - target: TASK-a1b2c3d4
    type: depends-on
---

## What

Update the Svelte 5 frontend components responsible for navigation and artifact browsing to:
1. Replace all "Skills" / "Skill" labels with "Knowledge"
2. Update any icon assignments for the knowledge artifact type
3. Update route paths if `/skills` routes exist → `/knowledge`
4. Update store references that use skill type strings

## How

Search `app/ui/src/` for:
- String literals `"Skills"`, `"Skill"`, `"skill"` in Svelte components and stores
- Route definitions: `/skills` → `/knowledge`
- Icon mappings for the skill/knowledge artifact type
- Navigation tree configuration that lists artifact types

Follow RULE-024: use existing shared components (`ArtifactListItem`, `StatusIndicator`) — do not inline new patterns.
Follow RULE-033: use shadcn Tooltip, not native title attributes.
Follow RULE-006: Svelte 5 runes only, no Svelte 4 patterns.

## Verification

1. `make typecheck` passes
2. `make lint-frontend` passes
3. App artifact browser shows "Knowledge" navigation node
4. Clicking a knowledge artifact navigates correctly
5. No "Skills" text visible in the UI (search `"Skills"` in Svelte files returns zero)
