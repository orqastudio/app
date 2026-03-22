---
id: "TASK-77f6948d"
title: "Fix RULE-89155a7f scope field to use valid value"
description: "Change RULE-89155a7f's scope field from the undocumented value software-engineering to a valid value from the documented set."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "RULE-89155a7f scope field uses a documented valid value"
  - "Value accurately reflects the rule's scope (likely project or domain)"
relationships:
  - target: "EPIC-4a7aeacb"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ec136ce9"
    type: "depended-on-by"
---
## What

[RULE-89155a7f](RULE-89155a7f) (Tooltip Usage) has `scope: software-engineering` which is not in the documented valid value set (`system | domain | project | role | artifact`). Fix to use the correct value.

## How

1. Read [RULE-89155a7f](RULE-89155a7f) to understand its scope — it enforces shadcn Tooltip usage over native `title` attributes
2. Determine the correct scope: likely `project` (specific to this codebase's UI conventions) or `domain` (applies to any project using shadcn)
3. Update the frontmatter `scope` field

## Verification

- [ ] [RULE-89155a7f](RULE-89155a7f) scope field uses a value from the valid set
- [ ] Value accurately reflects the rule's applicability
