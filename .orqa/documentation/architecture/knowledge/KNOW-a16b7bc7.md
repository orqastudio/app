---
id: KNOW-a16b7bc7
type: knowledge
status: active
title: "Demoted Rule Stability Tracking"
description: "The lifecycle for demoted rules: demotion metadata, violation logging, stability counter, and auto-delete trigger after N clean sessions"
tier: stage-triggered
created: 2026-03-29
roles: [governance-steward, reviewer]
paths: [.orqa/learning/rules/, .state/]
tags: [governance, rules, demotion, stability, enforcement]
relationships:
  - type: synchronised-with
    target: DOC-a16b7bc7
---

# Demoted Rule Stability Tracking

## Overview

When a governance rule becomes redundant (its enforcement now handled by LSP, linter, schema validator, or another rule), it enters a **demotion period**. The system automatically tracks whether the rule's domain remains violation-free. Once enough clean sessions pass, the rule is surfaced as safe to delete.

This prevents two failure modes:

- **Premature deletion:** Removing a rule before its replacement actually works
- **Stale rules:** Keeping inactive rules indefinitely because nobody checks

## Demotion Frontmatter

Set the rule to `status: inactive` and add:

```yaml
status: inactive
demoted_date: "2026-03-24"
demoted_reason: "Enforced by orqa LSP schema validation"
replaced_by: "orqa LSP frontmatter validator"
stability_threshold: 10       # sessions without violations before safe to delete
stability_count: 0            # auto-managed — do NOT edit manually
```

## How It Works

### Violation Logging

The pre-commit hook logs every validation failure to `.state/precommit-violations.jsonl`. Each entry records: timestamp, violation_type, domain, detail, files involved. Append-only, persists across sessions.

### Session-Start Stability Check

At every session start, the stability tracker runs. For each demoted rule:

1. Check violation log for entries matching the rule's enforcement domain
2. Reset counter to 0 if any matching violations found
3. Increment counter by 1 if no matching violations found
4. Surface rule as deletion candidate when counter reaches threshold

**Messages:**

- `STABLE: RULE-xxx` — threshold reached, safe to delete (requires user confirmation)
- `RESET: RULE-xxx` — violations found, counter reset, replacement not fully effective
- `TRACKING: RULE-xxx` — no violations, counter incremented

### Deletion

When STABLE is reached:

1. Stability threshold met
2. User confirms deletion
3. Replacement mechanism verified as working
4. Rule file removed

## What Can Be Demoted

Rules enforcing constraints the schema and validation engine can check:

- Valid status values
- Valid relationship types
- Required frontmatter fields
- Artifact ID format
- File naming conventions

## What Cannot Be Demoted

Rules requiring judgement:

- Pillar alignment
- Documentation-before-code
- Honest reporting
- Delegation boundaries
- Process sequencing

These will always require behavioral enforcement (Layer 2) because they cannot be mechanically verified.

## File Locations

| File | Purpose |
| ------ | --------- |
| `connectors/claude-code/hooks/scripts/stability-check.mjs` | Session-start stability tracker |
| `plugins/githooks/hooks/pre-commit` | Violation logging |
| `.state/precommit-violations.jsonl` | Append-only violation log (gitignored) |
