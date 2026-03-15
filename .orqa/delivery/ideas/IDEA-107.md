---
id: IDEA-107
title: "App-shipped system documentation — uneditable knowledge that teaches agents how the platform works"
description: "System knowledge (artifact types, state machines, how to work with the platform) should be shipped as uneditable app documentation, not as project-level skills or rules. This documentation is loaded into agent context automatically. Project-level skills/rules are reserved for project-specific conventions."
status: captured
created: 2026-03-15
updated: 2026-03-15
pillars:
  - PILLAR-001
  - PILLAR-003
milestone: null
horizon: next
research-needed:
  - "Which current skills/rules describe system behavior vs project conventions"
  - "How does the app inject system documentation into agent context"
  - "Where does app-shipped documentation live on disk vs project documentation"
promoted-to: null
spun-off-from: null
relationships:
  - type: informed-by
    target: AD-051
    rationale: Three-layer configurability establishes what is core vs project
  - type: informed-by
    target: RULE-044
    rationale: Core graph firmware principle applies to system documentation
---

## Motivation

Currently, knowledge about how OrqaStudio works is spread across project-level skills (SKILL-051 artifact-status-management, SKILL-011 orqa-governance) and rules. These can be edited or deleted by the project, which means the AI could lose understanding of how the platform itself works.

System knowledge should be:
- **Shipped with the app** as uneditable documentation
- **Loaded automatically** into agent context (like grounding but for platform mechanics)
- **Versioned with the app** — when the app updates, the system docs update
- **Separate from project knowledge** — project skills teach domain conventions, system docs teach platform behavior

### What becomes app-shipped system docs:
- How artifact types work (what a rule is, what a lesson is, what statuses mean)
- The state machine and how to work within it (when to change status, what transitions mean)
- How the artifact graph works (relationships, integrity, traversal)
- How delegation works (the orchestrator pattern)
- How the learning loop works (lesson → recurrence → promotion)

### What stays as project-level skills/rules:
- **Project state machine skill** — required skill, editable at project level. Teaches agents how THIS project uses the state machine (when to use each status, project-specific conventions like "we use hold for client-waiting states"). The app generates a default version from the state machine config; the project can customise it.
- Coding standards (project-specific)
- Architecture decisions (project-specific)
- Domain knowledge (project-specific)
- Custom workflows (project-specific)
