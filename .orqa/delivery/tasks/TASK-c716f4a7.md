---
id: TASK-c716f4a7
type: task
name: "Verify three-layer agent composition"
status: active
description: "Verify agent-file-generator.ts produces three-layer composition: role + stage context + domain knowledge"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 3 — Agent Prompt Generation Verification"
  - target: TASK-96128eb2
    type: depends-on
    rationale: "Pipeline source verification must pass before checking composition output"
acceptance:
  - "At least 3 generated agent files inspected and confirmed to contain all three layers"
  - "Layer 1 contains role definition from core plugin (universal role, behavioral boundaries, tool constraints)"
  - "Layer 2 contains stage context references (workflow-stage-specific instructions)"
  - "Layer 3 contains domain knowledge references with titles — not bare IDs"
  - "npx tsc --noEmit passes for libs/cli"
---

# TASK-c716f4a7: Verify three-layer agent composition

## What to do

Verify that `agent-file-generator.ts` produces the three-layer composition model from RES-d6e8ab11 section 4:

1. **Read the generator:**
   - Read `libs/cli/src/lib/agent-file-generator.ts`
   - Identify how it composes agent prompt content
   - Confirm it assembles three distinct layers

2. **Layer 1 — Universal Role:**
   - From core plugin agent definitions in `.orqa/process/agents/`
   - Must include: role name, behavioral boundaries, tool constraints
   - Verify the generator reads role definitions from the registry

3. **Layer 2 — Stage Context:**
   - From stage-definition plugins for the current workflow stage
   - Must include: stage-specific instructions, workflow context
   - Verify the generator injects stage context when a workflow stage is active

4. **Layer 3 — Domain Knowledge:**
   - From knowledge plugins matching the task scope
   - Must include: knowledge references WITH titles (not bare artifact IDs)
   - Verify the generator resolves knowledge IDs to titles

5. **Inspect generated output:**
   - Read at least 3 files from `.claude/agents/*.md`
   - Confirm each contains all three layers
   - Flag any file missing a layer

## Knowledge needed

- `libs/cli/src/lib/agent-file-generator.ts` — generator implementation
- `.orqa/prompt-registry.json` — registry data structure
- `.claude/agents/*.md` — generated output files
- `.orqa/process/agents/` — core agent role definitions
- RES-d6e8ab11 section 4 — three-layer composition model

## Agent role

Researcher — reads code and generated output, produces findings report. No code changes.

## Verification

- Read 3+ agent files in `.claude/agents/` and confirm three layers present
- Confirm generator code has distinct sections for role, stage, and knowledge
- Confirm knowledge references include titles, not bare IDs
- `npx tsc --noEmit` in `libs/cli/`
