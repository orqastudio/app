---
id: EPIC-ecef93a8
type: epic
title: "Human Gates and Review Sub-Workflows"
description: "Implement human gate infrastructure with gather/present/collect/execute/learn pipeline, five gate patterns, learning integration, workflow variant selection, and ad-hoc workflow variants."
status: captured
priority: P1
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Human gates ensure quality during dogfooding"
  - target: EPIC-f6da17ed
    type: depends-on
    rationale: "Gates are part of the state machine system"
---

## Scope

From RES-55bacef1 section 7 (State Machine Design — gates and variants):

- Gate infrastructure — gather/present/collect/execute/learn pipeline
- Simple approval pattern — single reviewer, approve/reject
- Structured review pattern (Maker-Checker) — AI review first, then human
- Scope decision pattern — multiple outcome paths (proceed, descope, expand, cancel)
- Learning integration — lesson creation on FAIL, recurrence tracking, promotion threshold
- Workflow variant selection rules — plugin manifest assigns variant based on artifact properties
- Ad-hoc variants: quickfix, security, docs-only, hotfix

## Note

This epic can run in parallel with EPIC-a63fde02 (Prompt Generation) and EPIC-281f7857 (Agent Lifecycle) since it depends only on the workflow engine, not on the prompt pipeline.
