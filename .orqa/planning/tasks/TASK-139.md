---
id: TASK-139
title: Audit shared component inventory and update RULE-024
description: Audit all Svelte components under ui/lib/components/ and update RULE-024 to reflect the accurate shared component inventory.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - Every .svelte file under ui/lib/components/ catalogued with purpose
  - RULE-024 inventory reflects actual disk state
  - Name mismatches resolved (rule uses actual component names)
  - Components in wrong locations identified with recommended moves
  - Follow-up tasks created for missing-but-useful components
---
## What

Audit all `.svelte` components under `ui/lib/components/` to produce a complete inventory of reusable/shared components. Map each against [RULE-024](RULE-024)'s current 12-item list. Identify:

1. Components that exist but under different names (e.g., StatusBadge = StatusIndicator)
2. Components that exist but in the wrong location (e.g., CodeBlock exists but not in `shared/`)
3. Components that don't exist but would be useful to create
4. Components that exist in shared/ but aren't in the rule

Update [RULE-024](RULE-024) with the accurate inventory. For components that should exist but don't, create follow-up tasks.

## How

1. Glob all `.svelte` files under `ui/lib/components/` and record their paths and purposes
2. Compare each against the 12 entries in [RULE-024](RULE-024)'s shared component table
3. Categorise findings: matches, name mismatches, wrong location, missing from rule, useful-but-missing
4. Edit [RULE-024](RULE-024) to replace the table with the accurate inventory
5. Create TASK-NNN follow-up artifacts for any missing components worth building

## Verification

- [ ] Every .svelte file under ui/lib/components/ catalogued with purpose
- [ ] [RULE-024](RULE-024) inventory reflects actual disk state
- [ ] Name mismatches resolved (rule uses actual component names)
- [ ] Components in wrong locations identified with recommended moves
- [ ] Follow-up tasks created for missing-but-useful components
