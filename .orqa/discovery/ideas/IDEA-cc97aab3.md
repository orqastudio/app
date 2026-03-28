---
id: "IDEA-cc97aab3"
type: discovery-idea
title: "Document Browser Sorting, Filtering & Navigation Config"
description: "Enhance the artifact navigation pane with sorting and filtering controls. Include manual sort order that persists to a _navigation.json config file alongside the README in each artifact group directory. Filter and sort state also persists to this file so it is properly retained across sessions."
status: archived
created: "2026-03-10"
updated: "2026-03-12"
research-needed:
  - "Sort options (alphabetical, by status, by date, manual drag-and-drop)"
  - "Filter options (by status, by priority, by pillar, free-text search)"
  - "_navigation.json schema design (sort order array, active filters, view preferences)"
  - "Interaction between _navigation.json and README.md frontmatter (sort field)"
  - "Scanner changes to read/write _navigation.json alongside README.md"
  - "Performance of re-sorting large artifact directories"
relationships:
  - target: "EPIC-9ddef7f9"
    type: "realises"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-c4afd86b"
    type: "benefits"
---

## Archived

Document browser sorting, filtering, and navigation config was absorbed by [EPIC-9ddef7f9](EPIC-9ddef7f9) (Artifact Browser -- Sort, Filter, Search), which is the active epic implementing this functionality. The research in [RES-8d203b37](RES-8d203b37) provides the detailed design.

## Motivation

The artifact navigation pane currently displays items in a fixed order (scanner default, typically alphabetical or by file modification time). Users working with large artifact directories (50+ epics, 70+ tasks) need the ability to sort and filter to find what they need quickly.

Manual sort order is especially important for artifact groups where the user wants to control presentation order (e.g., prioritised epics, ordered milestones). This should persist to a `_navigation.json` file in the artifact directory so it survives across sessions and is version-controlled alongside the artifacts.

## Sketch

- Sort controls in the navigation pane toolbar (dropdown or toggle buttons)
- Sort modes: alphabetical (A-Z, Z-A), by status, by date (created/updated), by priority, manual
- Manual sort: drag-and-drop reordering that saves to `_navigation.json`
- Filter controls: status chips (click to toggle), priority filter, free-text search
- `_navigation.json` sits alongside `README.md` in each artifact directory
- Schema includes: `sortOrder` (array of artifact IDs), `filters` (active filter state), `viewPreferences` (collapsed groups, etc.)
- Scanner reads `_navigation.json` to apply sort order and persisted view state
- Hidden from artifact browsing (prefixed with `_`, same as other hidden entries)
