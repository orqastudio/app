---
role: artifacts
label: "Decisions"
description: "Architecture decisions that capture why key technical choices were made."
icon: "scroll-text"
sort: 1
---

# Decisions

Architecture decisions capture the reasoning behind key technical choices — what was decided, why, what alternatives were considered, and what trade-offs were accepted. They are permanent records; when circumstances change, a new decision supersedes the old one rather than overwriting it.

## Pipeline Role

Decisions are **Understanding** — the second stage of the knowledge maturity pipeline:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

A lesson (observation) that recurs enough produces a decision: a definitive answer to "how should we handle this?" Decisions then inform skills (practice) and rules (enforcement). When multiple related decisions converge, they may crystallise into a pillar (principle).

## When to Create a Decision

Create an `AD-NNN.md` when research produces an architectural choice, when a technology or design pattern is adopted project-wide, or whenever a future contributor might reasonably ask "why did they do it this way?"

## Key Fields

- **`status`**: `proposed` → `accepted` → `superseded` / `deprecated`
- **`layer`**: `core` (platform-level, ships with OrqaStudio) or `project` (project-specific)
