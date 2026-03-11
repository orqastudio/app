---
id: TASK-144
title: Create rule for artifact link format constraints
description: Extract the artifact link format constraint from the orqa-documentation skill into a new dedicated rule with enforcement path.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Create new RULE-NNN for artifact link format enforcement
  - Remove constraint from orqa-documentation skill FORBIDDEN section
  - Update skill to reference the new rule
  - Include valid/invalid examples and FORBIDDEN section in the new rule
acceptance:
  - New RULE-NNN created for artifact link format enforcement
  - Constraint removed from orqa-documentation skill FORBIDDEN section
  - Skill references the new rule for the constraint
  - Rule includes valid and invalid examples
  - Rule has a FORBIDDEN section with concrete violations
---
## What

The artifact link format constraint (`[EPIC-001](EPIC-001)` style) is currently hidden in the `orqa-documentation` skill's FORBIDDEN section. This is a binary constraint that should be a rule with enforcement path.

Extract the link format constraints from `orqa-documentation` skill into a new rule. The skill keeps the "how to write good docs" knowledge; the rule enforces the link format requirement.

## How

1. Read the `orqa-documentation` skill and locate the artifact link format constraint in the FORBIDDEN section
2. Determine the next available RULE number by scanning `.orqa/governance/rules/`
3. Create the new rule file with: description of the constraint, valid examples, invalid examples, FORBIDDEN section, and related rules references
4. Remove the extracted constraint from the `orqa-documentation` skill's FORBIDDEN section
5. Add a "See RULE-NNN for link format requirements" reference in the skill

## Verification

- [ ] New RULE-NNN created for artifact link format enforcement
- [ ] Constraint removed from orqa-documentation skill FORBIDDEN section
- [ ] Skill references the new rule for the constraint
- [ ] Rule includes valid and invalid examples
- [ ] Rule has a FORBIDDEN section with concrete violations
