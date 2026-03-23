---
id: "TASK-7cbdca2a"
type: task
title: "Create rule for tool access restrictions per role"
description: "Formalize the tool access restrictions defined in agent YAML tools lists into a dedicated rule that enforces which tools each universal role may use."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "New rule created mapping roles to permitted tool sets"
  - "FORBIDDEN section lists role-tool violations"
  - "Rule references agent YAML as the detailed source"
  - "Ownership boundaries from RULE-532100d9 reinforced through tool restrictions"
relationships:
  - target: "EPIC-4a7aeacb"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ec136ce9"
    type: "depended-on-by"
---
## What

Tool access restrictions per role are currently defined only in each agent's YAML `tools:` list. There is no rule that enforces these restrictions or makes them visible as a constraint. This means an agent could use tools outside its role's scope without any governance artifact flagging it.

## How

1. Read each agent YAML file in `.orqa/process/agents/` and extract the `tools:` lists
2. Determine the next available RULE number
3. Create the rule with: a table mapping each role to its permitted tools, the rationale for each restriction (e.g., Reviewer cannot Edit because it doesn't implement fixes), FORBIDDEN patterns, related rules (RULE-532100d9 ownership boundaries)
4. Agent YAML remains the detailed source — the rule provides the enforceable constraint and overview

## Verification

- [ ] New rule created with role-to-tool mapping table
- [ ] Every universal role's tool restrictions documented
- [ ] FORBIDDEN section with concrete violations
- [ ] Consistent with agent YAML tools lists
- [ ] References [RULE-532100d9](RULE-532100d9) (agent delegation) for ownership boundary context