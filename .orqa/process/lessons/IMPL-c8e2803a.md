---

id: IMPL-c8e2803a
type: lesson
title: Research artifact not triggered before investigation — structure-before-work violated
description: "When asked to scan for principle enforcement gaps, the orchestrator jumped straight to running the audit without creating a research artifact first. RULE-d5d28fba (structure before work) and RULE-ec9462d8 (documentation first) both require the artifact structure to exist before work begins. The orchestrator should recognise investigation requests as research tasks and create RES-NNN artifacts before delegating."
status: archived
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships: []
---


## Pattern

When the user requests an investigation or audit, the orchestrator delegates directly to a research agent without first creating a RES-NNN artifact. This skips the structure-before-work requirement and means the research has no traceability — no artifact to reference from the epic, no place for findings to live.

## Fix

The orchestrator should recognise investigation-class requests ("scan for gaps", "audit X", "identify Y") and:
1. Create a RES-NNN research artifact with the investigation scope
2. Reference it from the epic's `research-refs` field
3. Then delegate the investigation to a research agent
4. Agent writes findings into the research artifact