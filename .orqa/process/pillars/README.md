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

Pillars are **Principles** — the third stage of the knowledge maturity pipeline:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

They distil accumulated understanding (decisions) into durable commitments about what matters. A pillar answers: "what kind of product do we want to be?" Decisions answer specific "what should we do?" questions. Rules enforce the day-to-day consequences of both.

## How Pillars Work

Each pillar defines a `title`, `description`, and `gate` — a set of test questions used to evaluate whether any piece of work serves this pillar. If a feature can answer "yes" to at least one gate question from at least one active pillar, it passes. Pillars are equal in importance; conflicts are escalated to the user, not resolved automatically.

## Lifecycle

```
active ←→ inactive
```

Active pillars are enforced against all new work. Inactive pillars are preserved as historical record but not evaluated against.
