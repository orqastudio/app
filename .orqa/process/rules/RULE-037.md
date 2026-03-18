---
id: RULE-037
title: Tool Access Restrictions
description: "Each universal role has a defined set of permitted tools. Using tools outside a role's scope violates ownership boundaries."
status: active
created: 2026-03-11
updated: 2026-03-11
relationships:
  - target: AD-029
    type: enforces
---
Tool access per role enforces ownership boundaries defined in [RULE-001](RULE-001). A Reviewer that can Edit would be tempted to fix issues instead of reporting them. A Researcher that can Write would be tempted to create artifacts instead of investigating.

## Role-to-Capability Matrix

| Capability | Orchestrator | Implementer | Researcher | Planner | Reviewer | Writer | Designer |
|-----------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| file_read | Y | Y | Y | Y | Y | Y | Y |
| content_search | Y | Y | Y | Y | Y | Y | Y |
| file_search | Y | Y | Y | Y | Y | Y | Y |
| code_search_* | Y | Y | Y | Y | Y | Y | Y |
| code_research | Y | Y | Y | Y | Y | Y | Y |
| file_edit | Y | Y | — | — | — | Y | Y |
| file_write | Y | Y | — | — | — | Y | Y |
| shell_execute | Y | Y | — | — | Y | — | — |
| web_fetch | Y | Y | Y | Y | — | Y | — |
| web_search | Y | Y | Y | Y | — | Y | — |

Capabilities resolve to provider-specific tools via [RULE-040](RULE-040).

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
- [RULE-026](RULE-026) (skill-enforcement) — skill loading complements capability resolution
- [RULE-040](RULE-040) (provider-agnostic-capabilities) — resolves capabilities to concrete tool names per provider
