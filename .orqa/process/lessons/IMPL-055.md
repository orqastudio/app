---





id: IMPL-055
title: "Orchestrator creates artifacts without maintaining bidirectional relationships"
description: "When creating or modifying artifacts (AD-048, RULE-001, RULE-017), the orchestrator added content links in body text but failed to add bidirectional relationship entries in frontmatter. This happened repeatedly across multiple artifacts in the same session despite RULE-045 requiring bidirectional inverses."
status: review
recurrence: 3
created: "2026-03-14"
updated: "2026-03-14"
maturity: observation
relationships:
  - target: RULE-045
    type: observes
    rationale: "RULE-045 requires bidirectional inverses but enforcement only fires at commit time — too late to catch the pattern during multi-artifact creation"
  - target: IMPL-054
    type: informed-by
    rationale: "Same session, same class of problem — orchestrator not following the system it maintains"
  - target: AD-048
    type: informed-by
    rationale: "The artifacts created without proper relationships were AD-048 and its targets"
  - target: RES-056
    type: informs
    rationale: "Auto-generated inverse of informs relationship from RES-056"
  - target: EPIC-064
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-064"
  - target: TASK-413
    type: informs
    rationale: "Auto-generated inverse of informs relationship from TASK-413"
  - target: TASK-465
    type: informs
    rationale: "Auto-generated inverse of informs relationship from TASK-465"
  - target: IDEA-095
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IDEA-095"
---
## Pattern

When creating or modifying multiple artifacts in a session, the orchestrator:

1. Writes content that references other artifacts by ID in body text
2. Adds relationship entries to the source artifact's frontmatter
3. **Fails to add the inverse relationship on the target artifact**

This happened with [AD-048](AD-048) (5 relationships declared, 0 inverses added to targets), [RULE-001](RULE-001) (promotion from [IMPL-052](IMPL-052) without inverse), and [RULE-017](RULE-017) (enforced-by [AD-048](AD-048) not added).

## Root Cause

[RULE-045](RULE-045)'s enforcement only fires at commit time via the pre-commit hook. The orchestrator creates/modifies artifacts across multiple turns before committing. By the time the pre-commit hook catches missing inverses, the orchestrator has lost context about which inverses were needed.

## Fix

Write-time enforcement: [RULE-045](RULE-045) now has `event: file` enforcement entries on `.orqa/**/*.md` that inject graph integrity reminders when artifacts are written. This catches the gap between "artifact modified" and "commit attempted."

The deeper issue is that 40+ rules in context become passive text that the orchestrator deprioritizes under task pressure. Mechanical enforcement at the moment of action is more reliable than rules that depend on the orchestrator remembering.

## Principle

Enforcement at write-time catches integrity violations when they're cheapest to fix. Enforcement at commit-time catches them when they're expensive (multiple files to backfill). The enforcement system should provide both layers.
