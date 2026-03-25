---
id: "TASK-1da659da"
type: "task"
title: "Create grounding documents for all agent roles"
description: "Create 5 concise grounding documents distilled from restructured documentation. Each answers: why this role exists, what good looks like, what goes wrong under pressure. Designed for agent injection, not human browsing."
status: "completed"
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 3
  dependencies: 4
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
assignee: null
acceptance:
  - "grounding/product-purpose.md created — mission, pillars, identity (30-50 lines)"
  - "grounding/code-principles.md created — what good code means, architecture boundaries (30-50 lines)"
  - "grounding/artifact-principles.md created — what good artifacts look like, graph discipline (30-50 lines)"
  - "grounding/design-principles.md created — UX principles, what good design means (30-50 lines)"
  - "grounding/research-principles.md created — evidence standards, investigation quality (30-50 lines)"
  - "Each doc has frontmatter with ID, relationships to source docs, and pillar alignment"
  - "Content is distilled from restructured docs, not duplicated"
relationships:
  - target: "EPIC-12fba656"
    type: "delivers"
    rationale: "Phase 2 — grounding docs are the foundation for agent purpose injection"
  - target: "TASK-97d5ed5f"
    type: "depends-on"
---

## Scope

Create `.orqa/documentation/grounding/` directory with 5 role-area documents.

Each document answers three questions:
1. **Why does this role exist?** — Connection to mission and pillars
2. **What does "good" look like?** — The principles that define quality for this role
3. **What goes wrong under pressure?** — Specific failure modes and how to recognize them

### Documents

| File | Grounds | Distilled From |
|------|---------|---------------|
| product-purpose.md | Orchestrator, Planner, Writer | VISION-4893db55 (vision), DOC-06224bf6 (governance), pillars |
| code-principles.md | Implementer, Reviewer | DOC-9814ec3c (coding-standards), architecture decisions |
| artifact-principles.md | Orchestrator, Writer, Researcher, Governance Steward | DOC-28344cd7 (artifact-framework), DOC-06224bf6 (governance) |
| design-principles.md | Designer | DOC-31bcfa5c (design-system), DOC-712f8c56 (interaction-patterns), DOC-1ff7a9ba (personas) |
| research-principles.md | Researcher | Research methodology skill content |

### Constraints

- Maximum 50 lines per document — these are injected into agent context, not browsed
- No procedural content — grounding is about purpose and principles, not how-to
- Content must be distilled, not duplicated — reference the source docs for detail