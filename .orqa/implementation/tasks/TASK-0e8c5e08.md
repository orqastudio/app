---
id: "TASK-0e8c5e08"
type: "task"
title: "Fix RULE-83411442 scope field to use valid value"
description: "Change RULE-83411442's scope field from the undocumented value software-engineering to a valid value from the documented set."
status: archived
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "RULE-83411442 scope field uses a documented valid value"
  - "Value accurately reflects the rule's scope (likely project or domain)"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

[RULE-83411442](RULE-83411442) (Tooltip Usage) has `scope: software-engineering` which is not in the documented valid value set (`system | domain | project | role | artifact`). Fix to use the correct value.

## How

1. Read [RULE-83411442](RULE-83411442) to understand its scope — it enforces shadcn Tooltip usage over native `title` attributes
2. Determine the correct scope: likely `project` (specific to this codebase's UI conventions) or `domain` (applies to any project using shadcn)
3. Update the frontmatter `scope` field

## Verification

- [ ] [RULE-83411442](RULE-83411442) scope field uses a value from the valid set
- [ ] Value accurately reflects the rule's applicability
