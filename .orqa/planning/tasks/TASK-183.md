---
id: TASK-183
title: Add enforcement field to rule schema and key rules
description: Add the enforcement array to the rule schema and add enforcement entries to key mechanically-enforceable rules.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on: []
assignee: AGENT-003
docs:
  - .orqa/documentation/product/artifact-framework.md
skills:
  - orqa-governance
  - orqa-schema-compliance
  - rule-enforcement
scope:
  - .orqa/governance/rules/schema.json
  - .orqa/governance/rules/
acceptance:
  - Rule schema includes enforcement field (array of objects with event, pattern, action, message)
  - At least 5 key rules have enforcement entries added
  - All modified rules pass schema validation
  - Enforcement entries are mechanically testable patterns
---

## What

Extend the rule schema with the `enforcement` field defined in [EPIC-050](EPIC-050)'s design.
Then add enforcement entries to rules that have clear, pattern-matchable violations.

## How

1. Add `enforcement` to `.orqa/governance/rules/schema.json` as an optional array
2. Each entry: `{ event, pattern, paths (optional), action, message }`
3. Identify rules with mechanically enforceable patterns:
   - [RULE-006](RULE-006): `unwrap()` in production code → file event
   - [RULE-013](RULE-013): `--no-verify` on commits → bash event
   - [RULE-007](RULE-007): raw cargo/npm commands → bash event
   - [RULE-020](RULE-020): TODO/FIXME comments → file event
   - [RULE-025](RULE-025): files created in project root → file event
4. Add enforcement entries to those rules

## Verification

- Schema validates with ajv
- Rules with enforcement entries pass schema validation
- Each enforcement entry has a testable regex pattern
