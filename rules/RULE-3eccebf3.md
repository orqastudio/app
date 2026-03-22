---
id: RULE-3eccebf3
title: Enforcement Before Code
description: "Enforcement artifacts (rules, knowledge, lessons) must be created before the implementation code that establishes the pattern."
status: active
created: 2026-03-07
updated: 2026-03-07
enforcement: "agent system prompt — orchestrator verifies enforcement artifacts exist before delegating implementation; code-reviewer rejects tasks where enforcement artifacts are missing"
relationships:
  - target: AD-f9034c99
    type: enforces
---
# Enforcement Before Code (NON-NEGOTIABLE)

When making changes to the app that introduce new patterns, fix recurring bugs, or establish conventions, the corresponding enforcement artifacts MUST be created BEFORE the implementation code is written.

## Why

Code without enforcement drifts. If a pattern is important enough to implement, it's important enough to enforce. Creating enforcement artifacts after the code means they get forgotten, deprioritized, or written with incomplete understanding of the pattern they're supposed to enforce.

## What Counts as an Enforcement Artifact

| Artifact | When to Create | Location |
|----------|---------------|----------|
| **Rule** | When establishing a new convention, fixing a recurring mistake, or introducing a constraint | `.orqa/process/rules/` |
| **Knowledge** | When capturing reusable domain knowledge that agents need to implement correctly | `.orqa/process/knowledge/` |
| **Knowledge update** | When existing knowledge is incomplete or incorrect | Update existing knowledge in `.orqa/process/knowledge/` |
| **Lesson** | When a bug or mistake reveals a non-obvious pattern | `.orqa/process/lessons/` |
| **Agent update** | When an agent's knowledge list or required reading needs to change | `.orqa/process/agents/` |

## The Sequence (MANDATORY)

```
1. Identify the pattern/convention/fix
2. Create or update the enforcement artifact (rule, knowledge, lesson)
3. Get user approval if the artifact changes process
4. THEN write the implementation code
```

## When This Applies

- Fixing a bug caused by a missing convention (e.g., "paths in config must match disk")
- Adding a new architectural pattern (e.g., "recursive directory scanning")
- Discovering a non-obvious behavior (e.g., "$derived vs $derived.by in Svelte 5")
- Establishing a new coding standard
- Any change where you think "agents should know this going forward"

## When This Does NOT Apply

- Straightforward bug fixes where the convention already exists and was simply violated
- Changes that are already covered by existing rules or knowledge
- Trivial fixes (typos, formatting)

## FORBIDDEN

- Writing implementation code that establishes a new pattern without a corresponding enforcement artifact
- Creating enforcement artifacts after the code is merged ("we'll add the rule later")
- Marking a task as complete when the enforcement artifact is missing

## Related Rules

- [RULE-9daf29c0](RULE-9daf29c0) (documentation-first) — documentation before code; this rule extends that to enforcement artifacts
- [RULE-551bde31](RULE-551bde31) (lessons-learned) — lessons are one type of enforcement artifact
- [RULE-deab6ea7](RULE-deab6ea7) (knowledge-enforcement) — knowledge must be loaded before implementation
