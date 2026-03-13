---
role: artifacts
label: "Rules"
description: "Coding standards, process requirements, and project conventions."
icon: "shield"
sort: 3
---

# Rules

Rules enforce coding standards, process requirements, and project conventions. Each rule defines what is required, what is forbidden, and how compliance is verified.

## Pipeline Role

Rules are **Enforcement** — the fifth stage of the knowledge maturity pipeline:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

A rule is what a decision becomes when the decision needs active policing, not just documentation. Rules are applied at multiple levels: agent self-compliance, code reviewer verification, pre-commit hooks, and automated scanners. When enforcement is insufficient, it escalates: rule → hook → scanner → hard block.

## Rule Categories

- **Process rules**: Enforce workflow requirements (documentation-first, plan compliance, honest reporting)
- **Coding rules**: Enforce technical standards (error handling, type safety, no stubs)
- **Architecture rules**: Enforce structural boundaries (IPC patterns, component purity, end-to-end completeness)
- **Governance rules**: Enforce the governance framework itself (artifact lifecycle, vision alignment)

## Rule Status

Rules carry `status: active` or `status: inactive`. Inactive rules are preserved as historical record but not enforced. Missing status is treated as active.
