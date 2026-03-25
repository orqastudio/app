---
id: TASK-d32da42f
type: task
name: "Verify P3 Generated Not Loaded"
status: active
description: "Verify that system prompts are generated programmatically from the pipeline — not hand-maintained files"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 4 — Principle Verification"
  - target: TASK-c9fa5dd0
    type: depends-on
    rationale: "Phase 3 (knowledge structure verified) must complete before principle verification"
acceptance:
  - "Audit report with PASS/FAIL verdict for P3"
  - "All .claude/agents/*.md files confirmed as pipeline-generated output"
  - "No manual prompt content found outside the pipeline"
---

## What

Verify Principle 3 from RES-d6e8ab11 section 2:

> **P3: Generated, Not Loaded** — System prompts are generated programmatically from plugin registries and workflow state -- not loaded wholesale from disk. The prompt pipeline assembles only what the agent needs for its current task. Full rule text is available on-demand via semantic search; compressed summaries are the default.

Confirm that `.claude/agents/` files are output of the prompt generation pipeline, not hand-maintained.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` section 2 (P3 definition)
- `libs/cli/src/lib/agent-file-generator.ts` — the prompt pipeline output stage
- `libs/cli/src/lib/prompt-registry.ts` — the registry that feeds the pipeline
- `.orqa/prompt-registry.json` — the compiled registry data
- `.claude/agents/` — the generated output files

## Agent Role

Researcher — read-only audit producing a PASS/FAIL verdict with evidence.

## Steps

1. Read RES-d6e8ab11 section 2 to confirm the exact P3 principle text
2. Read `libs/cli/src/lib/agent-file-generator.ts` to trace the generation pipeline
3. Read `libs/cli/src/lib/prompt-registry.ts` to understand registry assembly
4. Check `.orqa/prompt-registry.json` to see the compiled registry
5. Inspect `.claude/agents/*.md` files — verify they match expected pipeline output format
6. Search for any hand-maintained content in agent files that bypasses the pipeline (hardcoded sections, manual edits)
7. Verify the pipeline flow: plugin registries -> prompt-registry.json -> agent-file-generator -> .claude/agents/*.md
8. Document findings with evidence
9. Produce a PASS/FAIL verdict

## Verification

- `agent-file-generator.ts` reads from `.orqa/prompt-registry.json` (not directly from plugins at runtime)
- `.claude/agents/*.md` files match the template structure produced by the generator
- No `.claude/agents/*.md` file contains content not traceable to the pipeline
- `grep -rn "hand.*maintain\|manual\|hardcoded" .claude/agents/` — should return 0 meaningful matches
