---
id: TASK-145
title: Create rule for skill portability constraints
description: Extract skill portability constraints from the skills-maintenance skill into a new dedicated rule with rule-level enforcement.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - New RULE-NNN created for skill portability enforcement
  - Constraints removed from skills-maintenance NON-NEGOTIABLE section
  - Skill references the new rule
  - Rule covers canon vs project vs plugin layer requirements
  - Rule has FORBIDDEN section with concrete violations
---
## What

Skill portability constraints (canon skills must not contain project-specific paths, project skills must declare their scope, etc.) are currently in the `skills-maintenance` skill's NON-NEGOTIABLE section. These are hard constraints that need rule-level enforcement.

Extract portability constraints from `skills-maintenance` skill into a new rule. The skill keeps the lifecycle management methodology; the rule enforces the portability requirements.

## How

1. Read the `skills-maintenance` skill and locate the portability constraints in the NON-NEGOTIABLE section
2. Determine the next available RULE number by scanning `.orqa/process/rules/`
3. Create the new rule file with: the portability requirements for each layer (canon, project, plugin), valid and invalid examples, FORBIDDEN section, and related rules references
4. Remove the portability constraints from the `skills-maintenance` skill's NON-NEGOTIABLE section
5. Add a "See RULE-NNN for skill portability requirements" reference in the skill

## Verification

- [ ] New RULE-NNN created for skill portability enforcement
- [ ] Constraints removed from skills-maintenance NON-NEGOTIABLE section
- [ ] Skill references the new rule
- [ ] Rule covers canon vs project vs plugin layer requirements
- [ ] Rule has FORBIDDEN section with concrete violations
