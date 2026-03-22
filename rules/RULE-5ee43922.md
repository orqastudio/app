---
id: RULE-5ee43922
type: rule
title: User-Invocable Knowledge Semantics
description: The user-invocable field in knowledge frontmatter controls whether a knowledge artifact can be triggered directly by users as a slash command or from the UI.
status: active
created: 2026-03-11
updated: 2026-03-11
enforcement: "orqa validate — schema validation checks for user-invocable field presence in knowledge artifact frontmatter; missing field is a schema validation failure that blocks commits"
relationships:
  - target: AD-53e80192
    type: enforces
---
Every knowledge artifact MUST have a `user-invocable` field in its YAML frontmatter. This field determines how the knowledge artifact surfaces to users.

## Field Values

| Value | Meaning | System Behavior |
|-------|---------|-----------------|
| `true` | Users can trigger this knowledge directly | Appears in CLI slash command list, app knowledge picker, and knowledge browser |
| `false` | Only loaded into agent context by the orchestrator | Hidden from user-facing lists; only agents use it |

## When to Set `true`

A knowledge artifact should be `user-invocable: true` when:

- It provides methodology a user might want to invoke on demand (e.g., "run a code quality review")
- It loads domain knowledge that enriches agent behavior when explicitly requested
- It has a clear trigger phrase or use case a user would recognise

## When to Set `false`

A knowledge artifact should be `user-invocable: false` when:

- It is a process knowledge artifact that only makes sense within a specific workflow (e.g., UAT phases that must follow a sequence)
- It is an internal audit knowledge artifact that the orchestrator triggers automatically
- Direct user invocation would bypass necessary preconditions or sequencing

## FORBIDDEN

- Knowledge artifacts without a `user-invocable` field in frontmatter
- Setting `user-invocable: true` on workflow-sequenced knowledge that bypasses preconditions when invoked directly
- Setting `user-invocable: false` on general methodology knowledge that users should be able to request

## Related Rules

- [RULE-deab6ea7](RULE-deab6ea7) (knowledge-enforcement) — knowledge loading model and tier system
- [RULE-11c29c9e](RULE-11c29c9e) (knowledge-portability) — knowledge layer and portability requirements
