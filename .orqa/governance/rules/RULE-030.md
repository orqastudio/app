---
id: RULE-030
title: UAT Process
description: User acceptance testing follows a collect-then-systematize approach. Findings are grouped by root cause before tasks are created.
status: active
promoted-from:
  - IMPL-011
  - IMPL-012
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: system
---

User acceptance testing follows a structured process that prevents premature fixing and ensures systemic solutions.

## UAT Phases (MANDATORY)

### Phase 1: Collection

The user exercises the app end-to-end. The orchestrator:

1. **Listens and records** — note each finding without attempting fixes
2. **Does not interrupt** — let the user complete their testing pass
3. **Captures faithfully** — record exactly what the user reports, including their language
4. **Asks "next?"** — keep the user in flow, don't derail into diagnosis

Findings are recorded in the epic body as a numbered list with type classification (`bug`, `ux`, `data`, `missing`).

### Phase 2: Systemic Analysis

After collection is complete, the orchestrator:

1. **Groups findings by root cause** — multiple symptoms often share one architectural gap
2. **Identifies systemic themes** — e.g., "6 findings all stem from null value handling in the renderer"
3. **Investigates the architecture** — delegate to Explore agents to understand current component tree, data flow, and patterns before proposing solutions
4. **Documents themes** in the epic body alongside findings

### Phase 3: Task Creation

Only after systemic analysis:

1. **Create tasks scoped to systemic solutions** — one task that fixes 6 findings is better than 6 tasks
2. **Separate data quality fixes from code fixes** — governance file edits don't need the same pipeline as component redesigns
3. **Prioritize** — stability bugs (memory leaks, crashes) before UX improvements before polish
4. **Cross-reference** — each task references the finding numbers it addresses

### Phase 4: Fix and Verify

Implementation follows normal process (structure-before-work, delegation, verification gates). After fixes:

1. The user re-tests the specific areas
2. New findings go into the next UAT round (new epic)
3. Process improvements are encoded (see below)

## Process Improvement Encoding (NON-NEGOTIABLE)

Every UAT round produces process learnings alongside bug findings. These MUST be encoded:

1. **Lessons** — create IMPL-NNN for each process insight discovered
2. **Rule/skill updates** — if a process worked well, encode it in the relevant governance artifact
3. **Audit trail** — the chain from finding → lesson → enforcement artifact must be traceable

A UAT round that only produces bug fixes without process improvements has failed to exercise Pillar 2 (Learning Through Reflection).

## FORBIDDEN

- Fixing findings one by one as they are reported (breaks systemic analysis)
- Creating tasks before grouping findings by root cause
- Skipping the architectural investigation step
- Completing a UAT epic without encoding process improvements as lessons
- Treating UAT as "just testing" — it is testing AND learning

## Related Rules

- [RULE-028](RULE-028) (systems-thinking) — systemic analysis is systems thinking applied to QA
- [RULE-017](RULE-017) (lessons-learned) — lesson creation and promotion pipeline
- [RULE-015](RULE-015) (honest-reporting) — findings must be reported accurately
- [RULE-027](RULE-027) (structure-before-work) — tasks require artifacts before implementation
