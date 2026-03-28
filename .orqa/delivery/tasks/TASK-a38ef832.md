---
id: TASK-a38ef832
type: task
title: "Verify workflow template override mechanism"
status: active
description: "Verify project-level workflow overrides resolve correctly over plugin-provided workflows per RES-d6e8ab11 section 3"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 3 — Agent Prompt Generation Verification"
  - target: TASK-c298a900
    type: depends-on
    rationale: "Phase 1 complete — workflows restructured before verifying override mechanism"
acceptance:
  - "A project-level override of a plugin workflow resolves correctly at install time"
  - "Override merging tested with at least one real workflow — plugin base + project override = correct resolved output"
  - "npx tsc --noEmit passes for libs/cli"
---

# TASK-a38ef832: Verify workflow template override mechanism

## What to do

Verify that project-level workflow overrides work per RES-d6e8ab11 section 3:

1. **Read the workflow resolver:**
   - Read `libs/cli/src/lib/workflow-resolver.ts`
   - Find the merge/override logic
   - Understand how plugin-provided workflows and project-level overrides are combined

2. **Verify the merge pattern:**
   - Plugins provide workflow templates (base definitions)
   - Project-level workflow files in `.orqa/workflows/` can override specific sections
   - The YAML merge pattern should: take plugin base, apply project overrides, produce resolved output
   - Project overrides inherit unmodified sections from the plugin template

3. **Test with a real workflow:**
   - Identify a plugin-provided workflow (e.g., `task.workflow.yaml` from software-kanban)
   - Check if a project-level override exists in `.orqa/workflows/`
   - Verify the resolved output in `.orqa/workflows/*.resolved.yaml` correctly merges both
   - If no override exists, create a test scenario:
     - Add a minimal project override for one workflow
     - Run the resolver and verify the output

4. **Check edge cases:**
   - What happens when project override adds a new field?
   - What happens when project override removes a field?
   - What happens when no project override exists (pure plugin template)?

5. **If mechanism does not exist:**
   - Document what needs to be implemented in workflow-resolver.ts
   - Specify the merge algorithm (deep merge, shallow merge, field-level override)

## Knowledge needed

- `libs/cli/src/lib/workflow-resolver.ts` — workflow resolution logic
- `plugins/*/workflows/` — plugin-provided workflow templates
- `.orqa/workflows/` — project-level overrides and resolved output
- RES-d6e8ab11 section 3 — P7 composition model

## Agent role

Researcher — reads code, tests resolution, produces findings. May create a test override file for verification.

## Verification

- Workflow resolver code read and merge logic identified
- At least one workflow tested: plugin base + override = correct resolved output
- Edge cases documented (add/remove/absent override)
- `npx tsc --noEmit` in `libs/cli/`
