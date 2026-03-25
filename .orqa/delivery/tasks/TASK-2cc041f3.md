---
id: TASK-2cc041f3
type: task
title: "CRITICAL: Artifact IDs must be hashes of title — audit and migrate"
status: captured
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "Document the canonical ID generation rule: ID = first 8 hex chars of MD5 hash of title"
  - "Audit all existing artifacts — identify which IDs match their title hash and which don't"
  - "Create migration script to rename artifacts whose IDs don't match"
  - "Update all cross-references (relationships, body text) to point to new IDs"
  - "Record the ID generation rule in knowledge and documentation"
  - "Ensure orqa CLI commands that create artifacts use this rule"
  - "Pre-commit hook validates that ID matches title hash"
relationships:
  - target: EPIC-4304bdcc
    type: delivers
    rationale: "Governance foundation work for stabilisation epic"
---

## Context

Artifact IDs (e.g., TASK-d28b2909, RES-55bacef1) use 8-character hex strings. These SHOULD be deterministic hashes of the artifact title, but this has not been consistently enforced. Some IDs were generated from arbitrary input strings, timestamps, or other non-title sources.

## The Rule

```
ID = PREFIX-{first 8 hex chars of MD5(title)}
```

Where PREFIX is the artifact type prefix (TASK, RES, EPIC, AD, RULE, KNOW, IMPL, IDEA, MS, PILLAR, AGENT, DOC).

## Why

- Deterministic: same title always produces same ID
- Collision detection: two artifacts with the same title would have the same ID (caught immediately)
- Verifiable: anyone can check if an ID matches its title
- No external state: ID generation doesn't require a counter, database, or registry

## What Needs to Happen

1. **Audit**: Script to read all artifacts, compute MD5(title)[:8], compare to actual ID
2. **Report**: List artifacts where ID doesn't match title hash
3. **Migrate**: Rename files, update all relationship targets and body references
4. **Enforce**: Pre-commit hook validates ID = hash(title) for new/modified artifacts
5. **Document**: Record the rule in knowledge + documentation
6. **CLI**: Ensure `orqa` commands that create artifacts follow this rule
