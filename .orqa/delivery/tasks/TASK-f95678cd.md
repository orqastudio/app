---
id: "TASK-f95678cd"
type: "task"
title: "Audit shared component inventory and update RULE-eb269afb"
description: "Audit all Svelte components under ui/src/lib/components/ and update RULE-eb269afb to reflect the accurate shared component inventory."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "Every .svelte file under ui/src/lib/components/ catalogued with purpose"
  - "RULE-eb269afb inventory reflects actual disk state"
  - "Name mismatches resolved (rule uses actual component names)"
  - "Components in wrong locations identified with recommended moves"
  - "Follow-up tasks created for missing-but-useful components"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

Audit all `.svelte` components under `ui/src/lib/components/` to produce a complete inventory of reusable/shared components. Map each against [RULE-eb269afb](RULE-eb269afb)'s current 12-item list. Identify:

1. Components that exist but under different names (e.g., StatusBadge = StatusIndicator)
2. Components that exist but in the wrong location (e.g., CodeBlock exists but not in `shared/`)
3. Components that don't exist but would be useful to create
4. Components that exist in shared/ but aren't in the rule

Update [RULE-eb269afb](RULE-eb269afb) with the accurate inventory. For components that should exist but don't, create follow-up tasks.

## How

1. Glob all `.svelte` files under `ui/src/lib/components/` and record their paths and purposes
2. Compare each against the 12 entries in [RULE-eb269afb](RULE-eb269afb)'s shared component table
3. Categorise findings: matches, name mismatches, wrong location, missing from rule, useful-but-missing
4. Edit [RULE-eb269afb](RULE-eb269afb) to replace the table with the accurate inventory
5. Create TASK-NNN follow-up artifacts for any missing components worth building

## Verification

- [ ] Every .svelte file under ui/src/lib/components/ catalogued with purpose
- [ ] [RULE-eb269afb](RULE-eb269afb) inventory reflects actual disk state
- [ ] Name mismatches resolved (rule uses actual component names)
- [ ] Components in wrong locations identified with recommended moves
- [ ] Follow-up tasks created for missing-but-useful components