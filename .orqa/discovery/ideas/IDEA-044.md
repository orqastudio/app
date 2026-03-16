---
id: IDEA-044
title: Multi-Tab Artifact Viewer
description: "Allow users to open artifacts in separate tabs within the app window, so multiple artifacts can be viewed and compared without losing context. The chat session remains pinned and global — tabs only affect the navigation and viewer panes."
status: captured
created: 2026-03-10
updated: 2026-03-13
horizon: later
research-needed:
  - "Tab management UX patterns in desktop apps (VS Code, Figma, Notion)"
  - "How tab state interacts with the navigation sub-panel (does selecting a nav item open a new tab or navigate the current one?)"
  - "Tab persistence across sessions (reopen last tabs on app restart?)"
  - Memory and performance implications of multiple artifact viewers
  - Split-view support (side-by-side artifact comparison)
relationships:
  - target: PILLAR-001
    type: grounded-by
---
## Motivation

Currently the artifact viewer shows one artifact at a time. Navigating to a different artifact replaces the current view, losing the user's scroll position and reading context. Users working across related artifacts (e.g., reading an epic while referencing its research docs, or comparing two decisions) are forced to navigate back and forth repeatedly.

Tabs solve this by letting users keep multiple artifacts open simultaneously, similar to browser tabs or editor tabs. The chat panel remains pinned and always visible — tabs only control what appears in the navigation + viewer area.

## Sketch

- Tab bar sits above the viewer pane, below the menu bar
- Each tab shows the artifact's display label (title or ID)
- Active tab is visually distinct; inactive tabs are dimmed
- Close button on each tab; middle-click to close
- Clicking an artifact link (ArtifactLink chip) opens in a new tab (or focuses existing tab if already open)
- Navigation panel click behavior: configurable — either opens in current tab or always opens new tab
- Chat session is NOT tabbed — it is always visible in its own panel regardless of which artifact tab is active
- Tab state could persist to SQLite or localStorage for session restore
