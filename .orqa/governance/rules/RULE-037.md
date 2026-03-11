---
id: RULE-037
title: Tool Access Restrictions
description: Each universal role has a defined set of permitted tools. Using tools outside a role's scope violates ownership boundaries.
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: canon
scope: general
promoted-from: null
---

Tool access per role enforces ownership boundaries defined in [RULE-001](RULE-001). A Reviewer that can Edit would be tempted to fix issues instead of reporting them. A Researcher that can Write would be tempted to create artifacts instead of investigating.

## Role-to-Tool Matrix

| Tool | Orchestrator | Implementer | Researcher | Planner | Reviewer | Writer | Designer |
|------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Read | Y | Y | Y | Y | Y | Y | Y |
| Grep | Y | Y | Y | Y | Y | Y | Y |
| Glob | Y | Y | Y | Y | Y | Y | Y |
| Search* | Y | Y | Y | Y | Y | Y | Y |
| Edit | Y | Y | — | — | — | Y | Y |
| Write | Y | Y | — | — | — | Y | Y |
| Bash | Y | Y | — | — | Y | — | — |

*Search = search_regex, search_semantic, code_research (all contexts)

## Key Restrictions

- **Researcher and Planner** are read-only. They investigate and plan but do not modify files or run commands.
- **Reviewer** can run Bash (for checks like `make check`, `cargo test`) but cannot Edit or Write. It diagnoses but does not fix.
- **Writer and Designer** can Edit and Write but cannot run Bash. They produce artifacts and UI but do not run system commands.
- **Orchestrator and Implementer** have full access. The orchestrator is restricted by [RULE-001](RULE-001) to governance files only.

## FORBIDDEN

- Reviewer using Edit or Write to fix issues it found (send findings back to Implementer)
- Researcher using Edit or Write to create artifacts (report findings to orchestrator)
- Planner using Edit or Write to implement plans (plans are approved then delegated)
- Writer or Designer using Bash to run build/test commands (delegate verification to Reviewer)

## Related Rules

- [RULE-001](RULE-001) (agent-delegation) — ownership boundaries that tool restrictions enforce
- [RULE-026](RULE-026) (skill-enforcement) — agent YAML defines the detailed tool lists
