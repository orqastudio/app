---
id: IDEA-056
title: Preserve Navigation State Across Refresh and HMR
description: Browser refresh, Vite HMR, and app rebuilds should restore the user to the same active screen (artifact, panel, scroll position) rather than resetting to the default view.
status: captured
created: "2026-03-11"
updated: "2026-03-11"
pillars:
  - PILLAR-001
research-needed:
  - "What navigation state needs persisting (active artifact, panel, scroll position, search overlay state)?"
  - "Storage mechanism — sessionStorage vs URL hash vs SQLite?"
  - "How does Vite HMR interact with Svelte store state — does $state survive HMR or reset?"
promoted-to: null
---

## Motivation

During dogfooding, a browser refresh or HMR rebuild drops the user back to the default screen. This is disorienting — you lose your place mid-task. The app should feel like it remembers where you were, especially during development when refreshes are frequent.
