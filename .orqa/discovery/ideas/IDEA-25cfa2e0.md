---
id: IDEA-25cfa2e0
type: idea
title: Preserve Navigation State Across Refresh and HMR
description: "Browser refresh, Vite HMR, and app rebuilds should restore the user to the same active screen (artifact, panel, scroll position) rather than resetting to the default view."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: next
research-needed:
  - "What navigation state needs persisting (active artifact, panel, scroll position, search overlay state)?"
  - "Storage mechanism — sessionStorage vs URL hash vs SQLite?"
  - "How does Vite HMR interact with Svelte store state — does $state survive HMR or reset?"
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

During dogfooding, a browser refresh or HMR rebuild drops the user back to the default screen. This is disorienting — you lose your place mid-task. The app should feel like it remembers where you were, especially during development when refreshes are frequent.