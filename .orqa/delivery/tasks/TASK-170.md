---
id: TASK-170
title: "Frontend: Spotlight-style AI search overlay"
description: Build a floating search overlay (Ctrl+Space) that sends queries to the AI provider with artifact graph context and renders structured results.
status: active
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Ctrl+Space opens the search overlay from anywhere in the app
  - Search input is auto-focused on open
  - Current panel content remains visible behind dimmed overlay
  - AI returns structured results with artifact IDs and explanations
  - Clicking a result navigates to the artifact
  - Escape dismisses the overlay
  - ActivityBar search icon opens the overlay
  - make typecheck passes
relationships:
  - target: EPIC-005
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-312
    type: depended-on-by
---


## What

A Spotlight-style floating search overlay that provides AI-driven cross-artifact search without losing the user's current browsing context.

## How

1. Create `ArtifactSearchOverlay.svelte` in `ui/src/lib/components/navigation/`
2. Use shadcn Dialog or a custom overlay with backdrop blur and dimming
3. Register global keyboard shortcut (Ctrl+Space) via a Svelte action or window event listener
4. On query submit, build a system prompt with artifact graph summary (types, counts, statuses, key references)
5. Send to AI provider via existing streaming infrastructure or a dedicated search command
6. Parse the AI response for artifact IDs and render as a results list
7. Add SearchIcon button to ActivityBar bottom section above Settings

## Verification

- [ ] `make typecheck` passes
- [ ] Ctrl+Space opens and closes the overlay
- [ ] Search input receives focus immediately on open
- [ ] AI search returns relevant artifacts for test queries
- [ ] Selecting a result navigates correctly
