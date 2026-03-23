---
id: "IMPL-e848fb92"
type: lesson
title: "Orchestrator creates artifacts without maintaining bidirectional relationships"
description: "When creating or modifying artifacts (AD-f9034c99, RULE-532100d9, RULE-551bde31), the orchestrator added content links in body text but failed to add bidirectional relationship entries in frontmatter. This happened repeatedly across multiple artifacts in the same session despite RULE-130f1f63 requiring bidirectional inverses."
status: "review"
created: "2026-03-14"
updated: "2026-03-14"
recurrence: 3
maturity: "observation"
relationships:
  - target: "EPIC-915291e7"
    type: "cautions"
    rationale: "Auto-generated inverse of informs relationship from EPIC-915291e7"
---
## Pattern

When creating or modifying multiple artifacts in a session, the orchestrator:

1. Writes content that references other artifacts by ID in body text
2. Adds relationship entries to the source artifact's frontmatter
3. **Fails to add the inverse relationship on the target artifact**

This happened with [AD-f9034c99](AD-f9034c99) (5 relationships declared, 0 inverses added to targets), [RULE-532100d9](RULE-532100d9) (promotion from [IMPL-85add0f1](IMPL-85add0f1) without inverse), and [RULE-551bde31](RULE-551bde31) (enforced-by [AD-f9034c99](AD-f9034c99) not added).

## Root Cause

[RULE-130f1f63](RULE-130f1f63)'s enforcement only fires at commit time via the pre-commit hook. The orchestrator creates/modifies artifacts across multiple turns before committing. By the time the pre-commit hook catches missing inverses, the orchestrator has lost context about which inverses were needed.

## Fix

Write-time enforcement: [RULE-130f1f63](RULE-130f1f63) now has `event: file` enforcement entries on `.orqa/**/*.md` that inject graph integrity reminders when artifacts are written. This catches the gap between "artifact modified" and "commit attempted."

The deeper issue is that 40+ rules in context become passive text that the orchestrator deprioritizes under task pressure. Mechanical enforcement at the moment of action is more reliable than rules that depend on the orchestrator remembering.

## Principle

Enforcement at write-time catches integrity violations when they're cheapest to fix. Enforcement at commit-time catches them when they're expensive (multiple files to backfill). The enforcement system should provide both layers.