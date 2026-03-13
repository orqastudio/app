---
description: "Reusable knowledge packages that agents load before working."
role: artifacts
label: "Skills"
icon: "zap"
sort: 4
---

# Skills

Skills are reusable knowledge packages that agents load before starting work. Each skill encodes patterns, anti-patterns, and domain-specific conventions — the accumulated knowledge of what works in this codebase.

## Pipeline Role

Skills are **Practice** — the fourth stage of the knowledge maturity pipeline:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

A decision (understanding) answers "what should we do?" A skill answers "how do we do it correctly?" Skills turn architectural decisions into actionable guidance that agents can load at runtime. When a skill is insufficient to prevent violations, it escalates into a rule (enforcement).

## Skill Layers

- **`core`**: Portable across all projects — general methodology, language patterns, search usage
- **`project`**: Specific to this codebase — IPC patterns, store patterns, streaming pipeline
- **`plugin`**: Installed from external source; portable like core skills

Skills are loaded in two tiers: Tier 1 (declared in agent YAML frontmatter) and Tier 2 (injected by orchestrator based on task scope). The `orqa-code-search` and `composability` skills are universal — required by every agent.
