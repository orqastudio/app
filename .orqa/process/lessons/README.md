---
role: artifacts
label: "Lessons"
description: "Implementation observations that feed the knowledge maturity pipeline."
icon: "book-open"
sort: 2
---

# Lessons

Lessons capture implementation observations — patterns discovered during code review, QA, or debugging that reveal something non-obvious about the codebase or process. They are the raw material that feeds the knowledge maturity pipeline.

## Pipeline Role

Lessons are **Observations** — the first stage of the knowledge maturity pipeline:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

A single lesson is a data point. When the same pattern recurs, it becomes **Understanding** — a decision or architectural insight. When it recurs enough, it hardens into **Practice** (a skill) or **Enforcement** (a rule).

## Lifecycle

```
active → recurring → promoted
```

- **Active**: Lesson documented; recurrence tracked
- **Recurring**: Recurrence >= 2; pending promotion review by orchestrator
- **Promoted**: Elevated to a rule, coding standard, or skill — `promoted-to` field set

## Promotion

At recurrence >= 2, the orchestrator is triggered to promote the lesson. Escalation path: lesson → rule → pre-commit hook → hard scanner block. The weaker the enforcement, the more likely recurrence continues.
