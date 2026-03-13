---
role: artifacts
label: "Pillars"
description: "Guiding principles that every feature is evaluated against."
icon: "compass"
sort: 0
---

# Pillars

Pillars are the guiding principles that define what this project cares about. Every feature, epic, and idea must serve at least one active pillar — if a proposed feature cannot trace to a pillar, it is out of scope.

## Pipeline Role

Pillars are the **guiding principles that ground the entire pipeline**. They are not a stage in the pipeline — they are the foundation that every stage serves:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
                                 ↑
                           Pillars ground this
```

Pillars answer "what kind of product do we want to be?" while decisions (principles) answer specific "what should we do?" questions. Every lesson, decision, skill, and rule traces back to at least one pillar. If work cannot connect to a pillar, it is out of scope.

## How Pillars Work

Each pillar defines a `title`, `description`, and `gate` — a set of test questions used to evaluate whether any piece of work serves this pillar. If a feature can answer "yes" to at least one gate question from at least one active pillar, it passes. Pillars are equal in importance; conflicts are escalated to the user, not resolved automatically.

## Lifecycle

```
active ←→ inactive
```

Active pillars are enforced against all new work. Inactive pillars are preserved as historical record but not evaluated against.
