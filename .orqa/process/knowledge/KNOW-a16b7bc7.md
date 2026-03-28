---
id: KNOW-a16b7bc7
type: knowledge
title: Demoted Rule Stability Tracking
domain: methodology/governance
summary: "When a rule becomes redundant — because its enforcement is now handled by a linter, LSP, schema validator, or another rule — it should not be deleted immediately. A **demotion** period lets the team verify that the replacement actually catches violations before the original rule disappears."
description: |
  Explains the stability tracking model for demoted (inactive) rules. Agents learn
  how demotion works, what the stability counter means, and when a demoted rule is
  safe to delete. Use when: Demoting a rule, reviewing rule lifecycle, or responding
  to stability-check output at session start.
status: active
created: 2026-03-24
updated: 2026-03-24
category: methodology
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-a16b7bc7
    type: synchronised-with
---

## Purpose

When a rule becomes redundant — because its enforcement is now handled by a linter, LSP, schema validator, or another rule — it should not be deleted immediately. A **demotion** period lets the team verify that the replacement actually catches violations before the original rule disappears.

---

## Demotion Lifecycle

```text
active  ──demote──>  inactive (with demoted_date)
                         │
        stability_count increments each clean session
                         │
        ┌────────────────┼───────────────────┐
        │                │                   │
   violation found   threshold reached   manual review
   (count resets)    (safe to delete)    (user decides)
```

### Frontmatter Fields

| Field | Type | Default | Purpose |
| ------- | ------ | --------- | --------- |
| `demoted_date` | date | — | ISO date when the rule was set to `inactive` |
| `demoted_reason` | string | — | Why the rule was demoted (e.g. "enforced by LSP schema validation") |
| `replaced_by` | string | — | What now covers this rule's intent |
| `stability_threshold` | number | 10 | Consecutive clean sessions before the rule is safe to delete |
| `stability_count` | number | 0 | Current count of consecutive clean sessions since demotion |

### How to Demote a Rule

1. Set `status: inactive` in the rule's YAML frontmatter
2. Add `demoted_date`, `demoted_reason`, and `replaced_by`
3. Optionally set `stability_threshold` (default 10)
4. The stability tracker handles the rest automatically

---

## Stability Tracking

The **stability-check** script runs at every session start. For each inactive rule with a `demoted_date`, it:

1. Reads `.state/precommit-violations.jsonl` for violations in the rule's enforcement domain
2. If violations found since demotion: **resets** `stability_count` to 0
3. If no violations: **increments** `stability_count` by 1
4. If `stability_count >= stability_threshold`: **surfaces** the rule as safe to delete

### Domain Matching

The tracker matches violations to rules using multiple signals:

- Enforcement entry `domain` fields
- Enforcement entry `mechanism` fields
- The `replaced_by` field (normalized to lowercase kebab-case)
- A baseline `governance` domain (always checked)

### Violation Log

The pre-commit hook appends to `.state/precommit-violations.jsonl` whenever a validation check fails. Each line is a JSON object:

```json
{"timestamp":"2026-03-24T10:30:00Z","violation_type":"orqa-enforce","domain":"governance","detail":"3 error(s)","files":"..."}
```

---

## Agent Actions

| Situation | Action |
| ----------- | -------- |
| Session start shows "STABLE: RULE-xxx" | Confirm with user before deleting. Verify the replacement is genuinely covering the rule's intent. |
| Session start shows "RESET: RULE-xxx" | The domain still has violations. The replacement is not fully effective yet. Investigate. |
| Session start shows "TRACKING: RULE-xxx" | No action needed. Progress is being recorded. |
| Demoting a rule | Set all five frontmatter fields. Do NOT delete the rule file. |
| Promoting a lesson to a rule | If the lesson replaces an existing rule, demote the old rule in the same commit. |

---

## FORBIDDEN

- Deleting a demoted rule without reaching the stability threshold
- Deleting a demoted rule without user confirmation
- Manually incrementing `stability_count` (the tracker handles this)
- Skipping `demoted_date` when setting a rule to `inactive`
