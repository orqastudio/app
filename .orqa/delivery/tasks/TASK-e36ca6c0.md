---
id: TASK-e36ca6c0
type: task
title: Mermaid and PlantUML rendering in markdown
description: "Render mermaid and plantuml code blocks as diagrams in the markdown renderer, styled to match the app theme."
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 1
  complexity: 3
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Code blocks with language mermaid or plantuml render as diagrams
  - "Styled to match app theme (dark/light mode, fonts, colours)"
relationships:
  - target: EPIC-7d587280
    type: delivers
    rationale: Diagram rendering makes documentation more visual and self-contained
---

## Scope

Integrate a Mermaid and/or PlantUML rendering library into MarkdownRenderer.svelte. Detect code blocks with language identifiers mermaid or plantuml and render them as diagrams. Apply theme-aware styling for dark/light mode compatibility.