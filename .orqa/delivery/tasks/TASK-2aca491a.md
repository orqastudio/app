---
id: TASK-2aca491a
title: "Extract stores into SDK — session, project, artifact, conversation"
description: "Move the four primary data stores into @orqastudio/sdk. Fix conversationStore's DEFAULT_MODEL dependency by making it a config parameter."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 4
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - sessionStore extracted and exported from SDK
  - projectStore extracted and exported from SDK
  - artifactStore extracted and exported from SDK
  - "conversationStore extracted — DEFAULT_MODEL accepted as config, not imported from UI component"
  - "All stores import types from @orqastudio/types"
  - "All stores use SDK's invoke wrapper"
  - "Unit tests for each store's state transitions"
relationships:
  - target: EPIC-a210c825
    type: delivers
    rationale: Primary stores — session, project, artifact, conversation
  - target: TASK-3b56186e
    type: depends-on
  - target: TASK-b9cb39f8
    type: depended-on-by
---

## Scope

### From ui/src/lib/stores/
- `session.svelte.ts` → fully portable, no changes
- `project.svelte.ts` → fully portable, no changes
- `artifact.svelte.ts` → fully portable, depends on SDK graph
- `conversation.svelte.ts` → extract DEFAULT_MODEL to constructor/config parameter

### Modification needed
`conversationStore` imports `DEFAULT_MODEL` from `$lib/components/conversation/model-options`. This must become a config parameter so the store doesn't depend on a UI component.
