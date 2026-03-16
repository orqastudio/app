---
id: TASK-162
title: Remove outer wrapping parentheses from artifact links
description: "Strip surrounding parentheses/brackets from artifact links across all .orqa/ markdown files — change ([EPIC-001](EPIC-001)) to [EPIC-001](EPIC-001). The markdown links themselves stay as-is."
status: ready
created: 2026-03-11
updated: 2026-03-11
docs: []
acceptance:
  - "No artifact links are wrapped in outer parentheses like ([ID](ID))"
  - "All markdown links [ID](ID) remain intact and functional"
  - All .orqa/ markdown files updated consistently
  - Research files (status surpassed) are exempt per RULE-014
relationships:
  - target: EPIC-010
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-037
    type: grounded-by
---
## What

Many artifact links across `.orqa/` files are wrapped in unnecessary outer parentheses: `([EPIC-001](EPIC-001))`. The markdown link format `[EPIC-001](EPIC-001)` is correct and should stay, but the surrounding parentheses add visual noise in both raw markdown and rendered output.

## How

1. Search all `.orqa/` markdown files for the pattern `([ID](ID))` where ID matches artifact prefixes
2. Replace with `[ID](ID)` — removing only the outer wrapping parentheses
3. Skip research files with `status: surpassed` per [RULE-014](RULE-014) (historical records are immutable)
4. Verify no links were broken by the replacement

## Verification

- [ ] No `([ID](ID))` patterns remain in active .orqa/ files
- [ ] All `[ID](ID)` markdown links still resolve correctly
- [ ] Surpassed research files are unchanged
- [ ] Consistent format across all artifact types
