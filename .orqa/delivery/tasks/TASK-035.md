---
id: TASK-035
title: Orchestrator delegation with skill injection
description: "Updates the orchestrator's delegation protocol to automatically include the relevant Tier 2 project skills in every delegation prompt, matched from the task's scope against the injection table."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - "Orchestrator's delegation protocol includes skill injection step"
  - Delegation prompts to agents include project skill injection instructions
  - "The injection is based on the task's scope field matching the injection table"
relationships:
  - target: EPIC-042
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-029
    type: grounded-by
  - target: SKILL-011
    type: grounded-by
  - target: TASK-332
    type: depended-on-by
---
## What

Update the orchestrator's delegation protocol so that when delegating a task to an
agent, the delegation prompt includes the Tier 2 project skills that match the task's
scope.

## Implementation Notes

In the orchestrator's "Delegation Protocol" section, add a step between "Scope the task"
and "Provide context":

```
3. **Inject project skills** — Match the task's scope against the injection table.
   Include in the delegation prompt: "Load these project skills before starting: [list]"
```

The agent then loads these alongside its own Tier 1 skills and any Tier 3 wrapper
resolutions.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
