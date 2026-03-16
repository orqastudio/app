---
id: TASK-213
title: Extract project-specific requirements to graph-readable artifacts
description: "Move project-specific sections from the orchestrator prompt to standalone artifacts discoverable via graph traversal — tech stack, naming conventions, verification gates."
status: completed
created: 2026-03-12
updated: 2026-03-12
docs: []
acceptance:
  - Project-specific content removed from orchestrator prompt
  - Content exists in appropriate artifacts
  - "Graph edges make content discoverable (e.g., task.docs points to coding-standards.md)"
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-011
    type: grounded-by
  - target: SKILL-037
    type: grounded-by
  - target: TASK-343
    type: depended-on-by
---
## What

The orchestrator prompt's "Section 2: Project-Specific Requirements" is ~500 lines of content that should live in the artifact graph, not be inlined into every session.

## How

1. Identify all project-specific content blocks in orchestrator.md
2. For each block, determine the right destination artifact (project.json, coding-standards.md, skill, etc.)
3. Move content to destination, ensuring it's complete and self-contained
4. Update graph edges so the content is discoverable

## Verification

- Orchestrator prompt no longer contains project-specific tech stack details
- Content exists in appropriate artifacts with proper graph relationships
- Coding standards, naming conventions, and verification gates are all findable via graph
