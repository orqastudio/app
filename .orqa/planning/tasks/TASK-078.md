---
id: TASK-078
title: Markdown cross-linking in MarkdownRenderer
description: "Auto-detect artifact ID patterns in rendered markdown and wrap them as clickable links that navigate to the referenced artifact."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-083
assignee: frontend-engineer
skills:
  - svelte5-best-practices
  - typescript-advanced-types
scope:
  - ui/lib/components/shared/MarkdownRenderer.svelte
acceptance:
  - "Regex matches all known artifact ID patterns: EPIC-NNN, TASK-NNN, AD-NNN, MS-NNN, IDEA-NNN, IMPL-NNN, RES-NNN, PILLAR-NNN, RULE-NNN"
  - "Matched IDs wrapped in clickable elements calling navigateToArtifact"
  - "IDs inside code blocks and pre elements are NOT linked"
  - "Always-on — no configuration toggle"
---
