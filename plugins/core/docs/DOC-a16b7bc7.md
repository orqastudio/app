---
id: DOC-a16b7bc7
type: doc
title: Demoted Rule Stability Tracking
description: "How OrqaStudio tracks whether demoted rules are safe to delete: the demotion lifecycle, stability counter, violation logging, and auto-delete trigger."
category: governance
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-a16b7bc7
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
---

# Demoted Rule Stability Tracking

## Overview

When a governance rule becomes redundant — because its enforcement is now handled by a linter, an LSP, a schema validator, or another rule — it enters a **demotion period**. During this period, OrqaStudio automatically tracks whether the rule's domain remains violation-free. Once enough clean sessions pass, the rule is surfaced as safe to delete.

This prevents two failure modes:

- **Premature deletion**: Removing a rule before verifying its replacement actually works
- **Stale rules**: Keeping inactive rules indefinitely because nobody remembers to check

## How It Works

### 1. Demoting a Rule

Set the rule to `status: inactive` and add demotion metadata in its YAML frontmatter:

```yaml
status: inactive
demoted_date: "2026-03-24"
demoted_reason: "Enforced by orqa LSP schema validation"
replaced_by: "orqa LSP frontmatter validator"
stability_threshold: 10
stability_count: 0
```

| Field | Required | Default | Description |
| ----- | -------- | ------- | ----------- |
| `demoted_date` | Yes | — | The date the rule was demoted |
| `demoted_reason` | No | — | Why the rule was demoted |
| `replaced_by` | No | — | What now covers this rule's enforcement |
| `stability_threshold` | No | 10 | Sessions without violations before deletion is safe |
| `stability_count` | No | 0 | Auto-managed counter (do not edit manually) |

### 2. Violation Logging

The pre-commit hook (`plugins/githooks/hooks/pre-commit`) logs every validation failure to `.state/precommit-violations.jsonl`. Each entry records:

- **timestamp** — when the violation occurred
- **violation_type** — which check failed (e.g., `orqa-enforce`, `frontmatter-schema`)
- **domain** — the enforcement domain (e.g., `governance`)
- **detail** — human-readable description
- **files** — which staged files were involved

This log is append-only and persists across sessions (it lives in `.state/`, which is gitignored).

### 3. Session Start Stability Check

At every session start, the stability tracker (`connectors/claude-code/hooks/scripts/stability-check.mjs`) runs automatically. For each demoted rule, it:

1. **Checks** the violation log for entries matching the rule's enforcement domain
2. **Resets** the counter to 0 if any matching violations are found
3. **Increments** the counter by 1 if no matching violations are found
4. **Surfaces** the rule as a deletion candidate when the counter reaches the threshold

The tracker outputs one of three messages:

| Message | Meaning |
| ------- | ------- |
| `STABLE: RULE-xxx` | Threshold reached — safe to delete (with user confirmation) |
| `RESET: RULE-xxx` | Violations found — counter reset, replacement not fully effective |
| `TRACKING: RULE-xxx` | No violations, counter incremented — steady progress |

### 4. Deletion

When a rule is surfaced as STABLE, an agent or user can delete it. Deletion requires:

1. The stability threshold has been reached
2. The user confirms the deletion
3. The replacement mechanism is verified to be working

## Domain Matching

The stability tracker matches violations to rules using these signals from the rule's frontmatter:

- **Enforcement entry domains** — `enforcement[].domain`
- **Enforcement entry mechanisms** — `enforcement[].mechanism`
- **Replaced-by field** — normalized to lowercase kebab-case
- **Baseline domain** — `governance` is always included as a fallback

A violation matches if any of its fields (`violation_type`, `domain`, or `detail`) overlap with the rule's domain signals.

## Example Lifecycle

```text
Day 1:   Rule demoted (status: inactive, stability_count: 0)
Day 2:   Session start — no violations — count: 1
Day 3:   Session start — no violations — count: 2
Day 5:   Session start — violation found! — count: 0 (reset)
Day 6:   Session start — no violations — count: 1
...
Day 16:  Session start — count reaches 10 — STABLE message shown
Day 16:  User confirms deletion — rule file removed
```

## File Locations

| File | Purpose |
| ---- | ------- |
| `connectors/claude-code/hooks/scripts/stability-check.mjs` | Session-start stability tracker |
| `plugins/githooks/hooks/pre-commit` | Violation logging (appends to JSONL) |
| `.state/precommit-violations.jsonl` | Append-only violation log |
| `plugins/agile-methodology/orqa-plugin.json` | Rule schema with demotion fields |

## Related Documents

- [KNOW-a16b7bc7](KNOW-a16b7bc7) — Agent-facing knowledge pair for this documentation page
- [KNOW-51de8fb7](KNOW-51de8fb7) — Artifact Status Management (covers status transitions including active/inactive)
