---
id: TASK-170
title: "Frontend: Spotlight-style AI search overlay"
description: Build a floating search overlay (Ctrl+Space) that sends queries to the AI provider with artifact graph context and renders structured results.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on: []
scope:
  - Create ArtifactSearchOverlay.svelte — centred floating modal with search input and results area
  - Auto-focus search input on open
  - Keyboard shortcut Ctrl+Space to toggle overlay
  - Add Search icon to ActivityBar above Settings as alternative trigger
  - Build search query prompt with artifact graph summary context
  - Send query to AI provider and stream response
  - Parse structured results (artifact IDs + relevance explanations)
  - Render results as navigable list with ArtifactLink chips
  - Selecting a result navigates to the artifact and closes the overlay
  - Escape or click-outside dismisses the overlay
acceptance:
  - Ctrl+Space opens the search overlay from anywhere in the app
  - Search input is auto-focused on open
  - Current panel content remains visible behind dimmed overlay
  - AI returns structured results with artifact IDs and explanations
  - Clicking a result navigates to the artifact
  - Escape dismisses the overlay
  - ActivityBar search icon opens the overlay
  - make check-frontend passes
---

## What

A Spotlight-style floating search overlay that provides AI-driven cross-artifact search without losing the user's current browsing context.

## How

1. Create `ArtifactSearchOverlay.svelte` in `ui/lib/components/navigation/`
2. Use shadcn Dialog or a custom overlay with backdrop blur and dimming
3. Register global keyboard shortcut (Ctrl+Space) via a Svelte action or window event listener
4. On query submit, build a system prompt with artifact graph summary (types, counts, statuses, key references)
5. Send to AI provider via existing streaming infrastructure or a dedicated search command
6. Parse the AI response for artifact IDs and render as a results list
7. Add SearchIcon button to ActivityBar bottom section above Settings

## Verification

- [ ] `make check-frontend` passes
- [ ] Ctrl+Space opens and closes the overlay
- [ ] Search input receives focus immediately on open
- [ ] AI search returns relevant artifacts for test queries
- [ ] Selecting a result navigates correctly
