---
id: "TASK-b4c3c05d"
title: "Add enforcement field to rule schema and key rules"
description: "Add the enforcement array to the rule schema and add enforcement entries to key mechanically-enforceable rules."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-1dab5ebe"
docs:
  - "DOC-01ddd8aa"
acceptance:
  - "Rule schema includes enforcement field (array of objects with event, pattern, action, message)"
  - "At least 5 key rules have enforcement entries added"
  - "All modified rules pass schema validation"
  - "Enforcement entries are mechanically testable patterns"
relationships:
  - target: "EPIC-3a8ad459"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-0b584382"
    type: "depended-on-by"
  - target: "TASK-fa39671d"
    type: "depended-on-by"
---
## What

Extend the rule schema with the `enforcement` field defined in [EPIC-3a8ad459](EPIC-3a8ad459)'s design.
Then add enforcement entries to rules that have clear, pattern-matchable violations.

## How

1. Add `enforcement` to `.orqa/process/rules/schema.json` as an optional array
2. Each entry: `{ event, pattern, paths (optional), action, message }`
3. Identify rules with mechanically enforceable patterns:
   - [RULE-b49142be](RULE-b49142be): `unwrap()` in production code → file event
   - [RULE-633e636d](RULE-633e636d): `--no-verify` on commits → bash event
   - [RULE-c71f1c3f](RULE-c71f1c3f): raw cargo/npm commands → bash event
   - [RULE-e9c54567](RULE-e9c54567): TODO/FIXME comments → file event
   - [RULE-1f30904a](RULE-1f30904a): files created in project root → file event
4. Add enforcement entries to those rules

## Verification

- Schema validates with ajv
- Rules with enforcement entries pass schema validation
- Each enforcement entry has a testable regex pattern
