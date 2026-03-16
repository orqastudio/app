---
id: TASK-400
title: "Integrity check: delivered ideas must have all research-needed items resolved"
description: "Add an integrity check that flags ideas transitioning to delivered/partially-delivered when their research-needed items haven't been answered in the body or tracked as tasks. Prevents research questions from falling through the cracks when ideas are absorbed organically without formal promotion."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Integrity check exists in artifact_graph.rs that inspects idea artifacts
  - Check fires when an idea has status delivered/partially-delivered and research-needed items
  - Each research-needed item must be either answered in the idea body or tracked as a task
  - Unanswered items produce IntegrityCheck warnings with category ResearchGap
  - IPC returns the warnings via run_integrity_scan
  - IntegrityWidget surfaces the warnings in the dashboard
relationships:
  - target: EPIC-060
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-390
    type: depends-on
  - target: TASK-401
    type: depended-on-by
---

## What

When an idea is delivered without going through formal promotion (absorbed organically), its `research-needed` items have no home. The promotion process normally turns these into tasks — skip the promotion, skip the tasks.

## How

1. Add `ResearchGap` to the `IntegrityCategory` enum in `artifact_graph.rs`
2. During integrity scan, find all ideas with `status: delivered` or `status: partially-delivered`
3. For each, check `research-needed` items against:
   - Is the question answered in the idea body text? (heuristic: body contains relevant keywords)
   - Is there a task that references this idea?
4. Flag unanswered items as warnings

## Verification

Run integrity scan on current `.orqa/` — should flag IDEA-065's remaining questions (now tracked as TASK-398/399) as resolved since tasks exist.

## Lessons

(To be filled during implementation)
