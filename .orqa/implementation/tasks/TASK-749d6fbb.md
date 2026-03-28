---
id: "TASK-749d6fbb"
type: "task"
title: "Implement rule-overrides in CLI plugin prompt injection"
description: "When loading rules into agent context for a task, read the task's rule-overrides (falling back to epic's if task has none). Suspended rules are loaded but annotated with the suspension reason."
status: archived
created: "2026-03-12"
updated: "2026-03-12"
assignee: null
docs: []
acceptance:
  - "Plugin reads rule-overrides from active task frontmatter"
  - "Falls back to epic rule-overrides if task has none"
  - "Suspended rules are included in context but annotated as suspended with reason"
  - "Non-suspended rules load normally"
rule-overrides: []
relationships:
  - target: "EPIC-3e6cad90"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Extend the CLI plugin's rule loading to check the active task/epic for `rule-overrides`. Suspended rules are still injected into agent context but wrapped with an annotation explaining the suspension and its reason.

## How

1. Read the active task's frontmatter for `rule-overrides`
2. If absent, read the parent epic's `rule-overrides`
3. When injecting rules, check each against the override list
4. Suspended rules get prefixed: "NOTE: This rule is suspended for this task because: [reason]. It remains visible for awareness but is not enforced."

## Verification

- Create a test task with `rule-overrides` suspending [RULE-23699df2](RULE-23699df2)
- Verify that [RULE-23699df2](RULE-23699df2) appears in agent context with suspension annotation
- Verify other rules load normally
