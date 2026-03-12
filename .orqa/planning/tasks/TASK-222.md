---
id: TASK-222
title: Audit existing architecture decisions against AD-038/039/040
description: Review AD-001 through AD-037 to identify which decisions are superseded, affected, or made defunct by the graph-based knowledge injection (AD-038), core graph firmware (AD-039), and task-first audit trail (AD-040) decisions.
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on: []
docs:
  - .orqa/documentation/product/artifact-framework.md
skills:
  - orqa-governance
  - orqa-artifact-audit
scope:
  - Read every AD from AD-001 through AD-037
  - For each, evaluate whether AD-038, AD-039, or AD-040 supersedes, modifies, or renders it irrelevant
  - Mark truly superseded decisions with status superseded and superseded-by field
  - Update decisions that are partially affected (add notes about what changed)
  - Leave decisions that are unaffected as-is
  - Ensure all supersession pairs are updated in the same commit (both old and new)
  - Document findings in a summary table
acceptance:
  - Every AD from AD-001 to AD-037 has been reviewed
  - Superseded decisions have status superseded and superseded-by set
  - New decisions (AD-038/039/040) have supersedes set where applicable
  - No one-sided supersessions exist (RULE-004 compliance)
  - Summary table of audit findings exists in this task's body
---
## What

[AD-038](AD-038) (graph-based knowledge injection), [AD-039](AD-039) (core graph firmware), and [AD-040](AD-040)
(task-first audit trail with configurable epic requirement) represent a significant
architectural shift. Several earlier decisions may now be:

- **Superseded**: The new decision replaces the old one entirely
- **Partially affected**: The old decision still holds but some aspects changed
- **Unaffected**: The old decision is independent of the new architecture

## Known Candidates for Review

These decisions are likely affected based on what [AD-038](AD-038)/039/040 change:

| Decision | Title | Likely Impact |
|----------|-------|---------------|
| [AD-028](AD-028) | Three-tier skill model | May be affected by graph-based skill discovery |
| [AD-029](AD-029) | Universal agent roles | Likely unaffected — roles are orthogonal to injection |
| [AD-032](AD-032) | SQLite for conversations only | Likely unaffected — governance stays file-based |
| [AD-033](AD-033) | Enforcement engine | May be affected by graph-based enforcement |
| [AD-034](AD-034) | Plugin architecture | May be affected by plugin reading graph |
| [AD-035](AD-035) | Companion plugin | Likely affected — plugin now reads graph |
| [AD-036](AD-036) | Rule enforcement entries | May be affected by self-enforcing rules |
| [AD-037](AD-037) | Capability-based agents | Likely unaffected — orthogonal to injection |

All other ADs should still be reviewed for completeness.

## How

1. Read each AD from [AD-001](AD-001) through [AD-037](AD-037)
2. Evaluate against [AD-038](AD-038)/039/040 changes
3. Mark superseded decisions with proper status and cross-references
4. Update both sides of any supersession in the same commit

## Verification

- All ADs reviewed and verdicts recorded in summary table below
- No one-sided supersessions
- All superseded decisions have correct status and cross-references

## Output

A summary table in this task's body (updated when complete) showing:

```
| AD | Title | Verdict | Notes |
|----|-------|---------|-------|
| AD-001 | ... | Unaffected | ... |
| AD-002 | ... | Partially affected | ... |
```
