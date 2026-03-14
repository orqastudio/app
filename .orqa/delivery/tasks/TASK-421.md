---
id: TASK-421
title: "Extract stores into SDK — session, project, artifact, conversation"
description: "Move the four primary data stores into @orqastudio/sdk. Fix conversationStore's DEFAULT_MODEL dependency by making it a config parameter."
status: done
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-066
depends-on:
  - TASK-420
assignee: null
skills:
  - SKILL-030
  - SKILL-016
acceptance:
  - "sessionStore extracted and exported from SDK"
  - "projectStore extracted and exported from SDK"
  - "artifactStore extracted and exported from SDK"
  - "conversationStore extracted — DEFAULT_MODEL accepted as config, not imported from UI component"
  - "All stores import types from @orqastudio/types"
  - "All stores use SDK's invoke wrapper"
  - "Unit tests for each store's state transitions"
relationships:
  - target: EPIC-066
    type: delivers
    rationale: "Primary stores — session, project, artifact, conversation"
  - target: RES-058
    type: informed-by
    rationale: "Research confirmed all four are portable with one trivial fix"
---

## Scope

### From ui/src/lib/stores/
- `session.svelte.ts` → fully portable, no changes
- `project.svelte.ts` → fully portable, no changes
- `artifact.svelte.ts` → fully portable, depends on SDK graph
- `conversation.svelte.ts` → extract DEFAULT_MODEL to constructor/config parameter

### Modification needed
`conversationStore` imports `DEFAULT_MODEL` from `$lib/components/conversation/model-options`. This must become a config parameter so the store doesn't depend on a UI component.
