---
id: RULE-4d4f540d
type: rule
title: UAT Process
description: User acceptance testing follows a collect-then-systematize approach. Findings are grouped by root cause before tasks are created.
status: active
created: 2026-03-07
updated: 2026-03-07
enforcement: "agent system prompt — uat-process knowledge injected during review/testing phases; orchestrator rejects UAT completion reports without encoded process improvements"
relationships:
  - target: AD-e156310d
    type: enforces
---
User acceptance testing follows a structured process that prevents premature fixing and ensures systemic solutions.

## UAT Phases (MANDATORY)

See the `uat-process` skill for full phase methodology.

### Phase 1: Collection — Listen and record findings without fixing or diagnosing.

### Phase 2: Systemic Analysis — Group findings by root cause and investigate architecture before proposing solutions.

### Phase 3: Task Creation — Create tasks scoped to systemic themes, not individual findings.

### Phase 4: Fix and Verify — Implement fixes, re-test, and encode process improvements.

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

- [RULE-d90112d9](RULE-d90112d9) (systems-thinking) — systemic analysis is systems thinking applied to QA
- [RULE-551bde31](RULE-551bde31) (lessons-learned) — lesson creation and promotion pipeline
- [RULE-878e5422](RULE-878e5422) (honest-reporting) — findings must be reported accurately
- [RULE-8035e176](RULE-8035e176) (structure-before-work) — tasks require artifacts before implementation
