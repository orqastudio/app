---
id: TASK-dea6a012
type: task
name: "Verify token budgets"
status: active
description: "Measure actual token counts in generated agent prompts against RES-d6e8ab11 section 6 budget targets"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 3 — Agent Prompt Generation Verification"
  - target: TASK-c716f4a7
    type: depends-on
    rationale: "Three-layer composition must be verified before measuring token budgets"
acceptance:
  - "Token counts measured for each generated agent role using a documented methodology"
  - "All agent prompts within the 1,500-4,000 token budget range"
  - "Any over-budget agents have P3/P2 sections identified for trimming"
  - "Measurement methodology documented in findings"
---

# TASK-dea6a012: Verify token budgets

## What to do

Measure actual token counts in generated agent prompts against the targets from RES-d6e8ab11 section 6:

1. **Establish measurement methodology:**
   - Use a consistent tokenizer (tiktoken cl100k_base or equivalent word-count approximation: tokens ~= words * 1.3)
   - Document the method used so results are reproducible

2. **Measure each agent role:**
   - Orchestrator: target 2,500 total (1,500 static + 500 stage + 500 on-demand)
   - Implementer: target 2,800 total (800 static + 500 stage + 1,500 on-demand)
   - Reviewer: target 1,900 total (600 static + 300 stage + 1,000 on-demand)
   - Researcher: target 2,100 total (400 static + 200 stage + 1,500 on-demand)
   - Writer: target 1,800 total (500 static + 300 stage + 1,000 on-demand)
   - Designer: target 1,800 total (500 static + 300 stage + 1,000 on-demand)

3. **Read generated files:**
   - Read each `.claude/agents/*.md` file
   - Count tokens using the chosen methodology
   - Compare against the 1,500-4,000 range

4. **Identify trimming opportunities:**
   - For any agent over 4,000 tokens: identify P3 and P2 sections that can be trimmed
   - For any agent under 1,500 tokens: note if content is missing
   - Document the priority level of each section

## Knowledge needed

- `.claude/agents/*.md` — generated agent prompt files
- RES-d6e8ab11 section 6 — token budget targets per role
- `libs/cli/src/lib/prompt-pipeline.ts` — token budget enforcement code (if exists)

## Agent role

Researcher — reads files, measures token counts, produces findings. No code changes.

## Verification

- Every agent role has a measured token count in findings
- Counts compared against 1,500-4,000 range with pass/fail per agent
- Over-budget agents have specific sections flagged for trimming
- Measurement methodology is reproducible
