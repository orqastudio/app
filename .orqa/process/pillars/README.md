---
role: artifacts
label: "Pillars"
description: "Guiding principles that every feature is evaluated against."
icon: "compass"
sort: 0
---

# Pillars

Pillars are the guiding principles that define what this project cares about. Every feature, epic, and idea must serve at least one active pillar. They are the top-level filter for all work — if a proposed feature cannot trace to a pillar, it is out of scope.

## How Pillars Work

Each pillar defines:

- **Title and description** — What the principle means
- **Test questions** — Concrete questions to evaluate whether work serves this pillar

When evaluating a feature, read each active pillar's test questions. If the feature can answer "yes" to at least one question from at least one pillar, it passes.

## Lifecycle

```
active ←→ inactive
```

- **Active**: The pillar is enforced — all new work is evaluated against it
- **Inactive**: The pillar is preserved but not evaluated against (historical record)

## Conflict Resolution

Pillars are equal in importance. When pillars appear to conflict, the conflict is flagged to the user for resolution — no pillar automatically takes precedence over another.

## Creating a Pillar

Pillars are project-level artifacts. To add a new pillar:

1. Create `PILLAR-NNN.md` with the next available ID
2. Define `title`, `description`, and `gate`
3. Write a body explaining what the pillar means in practice, with examples and anti-patterns
4. Update any rules or documentation that reference pillars generically

## Related

- Rules in **Governance > Rules** reference pillars for feature evaluation
- Epics and ideas carry a `pillars` field listing which pillars they serve
- The artifact framework in **Documentation > Product** defines the full pillar schema
