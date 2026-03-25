---
id: TASK-40cd25d8
type: task
name: "Verify P5 Token Efficiency as Architecture"
status: active
description: "Verify that per-agent prompts meet token budget targets from the architecture research"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 4 — Principle Verification"
  - target: TASK-c9fa5dd0
    type: depends-on
    rationale: "Phase 3 (knowledge structure verified) must complete before principle verification"
acceptance:
  - "Audit report with PASS/FAIL verdict for P5"
  - "Measured token counts for each agent role documented"
  - "Comparison against budget table from RES-d6e8ab11 section 6 with variance analysis"
---

## What

Verify Principle 5 from RES-d6e8ab11 section 2:

> **P5: Token Efficiency as Architecture** — Token efficiency is not an optimization pass applied after the architecture is designed. It is a first-class architectural constraint that shapes every decision: prompt structure, knowledge loading strategy, agent lifecycle, model selection, and cache behavior. The target is a 2-4x overhead ratio, down from the current 13.4x.

Confirm that per-agent prompts meet the token budget targets from the research.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` section 2 (P5 definition) and section 6 (token budget table)
- Token budget table from RES-d6e8ab11 section 6:
  - Orchestrator: 2,500 total (1,500 static + 500 stage + 500 on-demand)
  - Implementer: 2,800 total (800 static + 500 stage + 1,500 on-demand)
  - Reviewer: 1,900 total (600 static + 300 stage + 1,000 on-demand)
  - Researcher: 2,100 total (400 static + 200 stage + 1,500 on-demand)
  - Writer: 1,800 total (500 static + 300 stage + 1,000 on-demand)
  - Designer: 1,800 total (500 static + 300 stage + 1,000 on-demand)
- `.claude/agents/` — generated agent files to measure
- `libs/cli/src/lib/prompt-pipeline.ts` — token budget enforcement code

## Agent Role

Researcher — read-only audit producing a PASS/FAIL verdict with evidence.

## Steps

1. Read RES-d6e8ab11 sections 2 and 6 to confirm the principle and budget table
2. List all generated agent files in `.claude/agents/`
3. Measure the token count of each agent file (use `wc -w` as a rough proxy: ~0.75 tokens per word for English text, or count characters / 4)
4. Compare measured counts against the budget table
5. Verify all agent prompts fall within the 1,500-4,000 token budget range
6. For any over-budget agents, identify which P3/P2 sections could be trimmed
7. Check `libs/cli/src/lib/prompt-pipeline.ts` for the `applyTokenBudget` function — verify it enforces the budget
8. Document the measurement methodology and results
9. Produce a PASS/FAIL verdict with variance analysis

## Verification

- Every agent file word count measured: `wc -w .claude/agents/*.md`
- All agent prompts within 1,500-4,000 token range (approximately 2,000-5,300 words)
- Budget enforcement code exists in prompt-pipeline.ts
- Over-budget agents (if any) have identified trim candidates
