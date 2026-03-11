---
id: RULE-038
title: User-Invocable Skill Semantics
description: The user-invocable field in skill frontmatter controls whether a skill can be triggered directly by users as a slash command or from the UI.
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: canon
scope: general
promoted-from: null
---

Every skill MUST have a `user-invocable` field in its YAML frontmatter. This field determines how the skill surfaces to users.

## Field Values

| Value | Meaning | System Behavior |
|-------|---------|-----------------|
| `true` | Users can trigger this skill directly | Appears in CLI slash command list, app skill picker, and skill browser |
| `false` | Only loaded into agent context by the orchestrator | Hidden from user-facing lists; only agents use it |

## When to Set `true`

A skill should be `user-invocable: true` when:

- It provides methodology a user might want to invoke on demand (e.g., "run a code quality review")
- It loads domain knowledge that enriches agent behavior when explicitly requested
- It has a clear trigger phrase or use case a user would recognise

## When to Set `false`

A skill should be `user-invocable: false` when:

- It is a process skill that only makes sense within a specific workflow (e.g., UAT phases that must follow a sequence)
- It is an internal audit skill that the orchestrator triggers automatically
- Direct user invocation would bypass necessary preconditions or sequencing

## FORBIDDEN

- Skills without a `user-invocable` field in frontmatter
- Setting `user-invocable: true` on workflow-sequenced skills that bypass preconditions when invoked directly
- Setting `user-invocable: false` on general methodology skills that users should be able to request

## Related Rules

- [RULE-026](RULE-026) (skill-enforcement) — skill loading model and tier system
- [RULE-035](RULE-035) (skill-portability) — skill layer and portability requirements
