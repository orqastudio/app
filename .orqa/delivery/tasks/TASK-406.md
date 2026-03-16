---
id: TASK-406
title: "Enhance artifact viewer with unified relationships, actions needed, and pipeline position"
description: Move all relationships to the relationships viewer across all artifact types. Add inferred Actions Needed box. Add pipeline position stepper. Auto-embed child artifacts. Display horizon field. Implement acceptance criteria checkboxes.
status: completed
priority: P1
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - "ALL relationship data across ALL artifact types renders in the relationships viewer, not the metadata box"
  - Relationships viewer organizes relationships by category/type
  - "Actions Needed box appears after metadata when actions exist, hidden when empty"
  - Actions Needed content is inferred from artifact status and pipeline position
  - "Pipeline position stepper shows the artifact's current stage highlighted within its full lifecycle"
  - Milestone views auto-embed child epics
  - Epic views auto-embed child tasks
  - Ideas horizon field displayed in artifact view
  - Task acceptance criteria render as checkboxes
  - All acceptance checkboxes must be checked before task status can transition to done
relationships:
  - target: EPIC-065
    type: delivers
    rationale: Artifact viewer enhancements — PipelineStepper, ActionsNeeded, AcceptanceCriteria, grouped relationships, horizon display
  - target: IDEA-025
    type: informs
    rationale: IDEA-025 (agent task ownership) relates to auto-embedded child artifact display
  - target: SKILL-030
    type: grounded-by
  - target: SKILL-042
    type: grounded-by
  - target: SKILL-011
    type: grounded-by
---

## Scope

### Unified Relationships Viewer (Findings #11, #12)
- **Files**: `FrontmatterHeader.svelte`, `RelationshipsList.svelte`, `ReferencesPanel.svelte`
- **Current**: Some relationships appear in metadata box despite `SKIP_FIELDS` including `relationships`
- **Fix**: Ensure ALL relationship data routes to the RelationshipsList/ReferencesPanel components. Organize by relationship type category.

### Actions Needed Box (Finding #13)
- **New component**: Infer next actions from artifact status and pipeline rules
- **Examples**: Task `in-progress` → "Complete acceptance criteria, request reviewer verification". Epic `draft` → "Satisfy docs-required gate: [list missing docs]"
- **Hidden when empty** (no actions to suggest)

### Pipeline Position Stepper (Finding #14)
- **New component**: Visual stepper showing all lifecycle stages for the artifact type with current stage highlighted
- **Example**: Task: `todo → [in-progress] → done` with `in-progress` highlighted

### Auto-Embed Children (Finding #23)
- **Logic**: Query artifact graph for child relationships (epics with `milestone: MS-NNN`, tasks with `epic: EPIC-NNN`)
- **Render**: Embedded list below artifact body showing child artifacts with status indicators
- **Link to existing idea if one exists**

### Horizon Display (Finding #24)
- **Current**: Horizon field exists on ideas schema, populated on some artifacts, not displayed
- **Fix**: Add horizon indicator to artifact detail views

### Acceptance Criteria Checkboxes (Findings #26, #27)
- **Schema**: Update task schema `acceptance` to support structured items with checked state
- **UI**: Render as checkboxes. All must be checked before done transition.
- **Note**: Do NOT retroactively mark any as checked — audit and verify each first
