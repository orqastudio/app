---
id: "AD-74a2cb7a"
type: decision
title: "Pillars as First-Class Planning Artifacts"
description: "Product pillars become structured artifacts in .orqa/principles/pillars/ with frontmatter schema, referenced by ID from other artifacts, and injected into AI system prompts. Replaces hardcoded pillar strings scattered across rules and documentation."
status: "completed"
created: "2026-03-09"
updated: "2026-03-13"
relationships: []
---
## Decision

Product pillars — the guiding principles that every feature must serve — become
first-class structured artifacts stored in `.orqa/principles/pillars/`.

### Current State

Pillars are prose embedded in multiple files:
- `vision.md` defines them narratively
- `governance.md` repeats the definitions
- `vision-alignment.md` rule hardcodes "Clarity Through Structure" and "Learning Through Reflection"
- `pillar-alignment-docs.md` rule hardcodes the same strings
- Every epic's scoring block references pillar alignment as a number but not by ID
- The system prompt has no awareness of what pillars exist

This means:
- Changing a pillar requires updating 5+ files manually
- Non-software projects cannot define their own pillars
- The AI has no structured knowledge of the project's guiding principles
- Pillar alignment is a human-readable convention, not machine-queryable data

### Proposed State

Each pillar is a markdown artifact with YAML frontmatter:

```yaml
---
id: PILLAR-c9e0a695
title: "Clarity Through Structure"
description: >
  Making thinking, standards, and decisions visible and structured.
gate:
  - Does this make governance artifacts visible and manageable?
  - Does it produce structured knowledge (plans, decisions, rules)?
  - Does it surface what would otherwise be hidden?
status: active
---

## What This Pillar Means

[Narrative description — what it means in practice, examples, anti-patterns]
```

### Why Planning, Not Governance

Pillars sit under `.orqa/principles/pillars/` because they represent product vision —
the strategic direction that planning artifacts (epics, ideas, milestones) are
evaluated against. While pillars also drive the learning loop (governance),
their primary role is shaping what gets built and why. Planning is the natural
home for "what matters to this project."

### System Prompt Injection

The system prompt builder reads active pillars from `.orqa/principles/pillars/`
and injects them into every AI conversation:

```
Project Pillars:

1. Clarity Through Structure (PILLAR-c9e0a695)
   Making thinking, standards, and decisions visible and structured.
   Test: Does this make governance artifacts visible? Does it produce
   structured knowledge? Does it surface what would otherwise be hidden?

2. Learning Through Reflection (PILLAR-2acd86c1)
   The system and its users improve over time through structured retrospection.
   Test: Does this capture lessons? Does it track outcomes? Does it feed
   information back into the governance loop?
```

This means the AI always knows the project's guiding principles and can
evaluate suggestions against them — without hardcoding specific pillars
into rules or agent definitions.

### Artifact References

Other artifacts reference pillars by ID:

```yaml
# In an epic
pillars: [PILLAR-c9e0a695, PILLAR-2acd86c1]

# In a rule
pillars: [PILLAR-c9e0a695]
```

The `pillar-alignment` scoring dimension reads these references. Rules like
`vision-alignment.md` become generic: "every feature must serve at least one
active pillar" — without naming which pillars exist.

### Conflict Resolution

The `priority` field on each pillar determines precedence when pillars
conflict. Lower number = higher priority. Currently "Clarity Through
Structure" has priority 1, meaning it wins conflicts. This is configurable
per project.

## Rationale

Same principle as [AD-48b310f9](AD-48b310f9) (universal roles): separate the mechanism from the
content. The mechanism (pillar alignment enforcement) is core. The content
(which pillars exist) is project-configurable. A consulting project's pillars
are different from a software project's pillars, but the enforcement pattern
is identical.

## Consequences

1. New artifact type `pillar` in `.orqa/principles/pillars/`
2. `project.json` artifacts array updated with pillars path
3. System prompt builder reads pillars and injects them
4. Rules referencing specific pillar names become generic
5. Scoring dimensions reference pillar IDs instead of hardcoded strings
6. Every project can define its own guiding principles
7. Pillar alignment metrics become queryable (count artifacts per pillar)

## Open Questions

1. Should core include default pillars that new projects start with?
   Probably yes — "Clarity Through Structure" and "Learning Through
   Reflection" as defaults that users can modify or replace.
2. How many pillars should a project have? Recommend 2-4. More than 4
   dilutes focus. Fewer than 2 doesn't create enough tension for
   meaningful prioritisation decisions.