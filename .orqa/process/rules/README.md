---
role: artifacts
label: "Rules"
description: "Coding standards, process requirements, and project conventions."
icon: "shield"
sort: 3
---

# Rules

Rules enforce coding standards, process requirements, and project conventions. They are automatically loaded and applied based on file path context. Each rule defines what is required, what is forbidden, and how compliance is verified.

## Rule Categories

- **Process rules**: Enforce workflow requirements (documentation-first, plan compliance, honest reporting)
- **Coding rules**: Enforce technical standards (error handling, type safety, no stubs)
- **Architecture rules**: Enforce structural boundaries (IPC patterns, component purity, end-to-end completeness)
- **Governance rules**: Enforce the governance framework itself (artifact lifecycle, vision alignment)

## Enforcement

Rules are enforced at multiple levels: agent self-compliance, code reviewer verification, pre-commit hooks, and automated scanners. Repeated violations trigger escalation through the lesson promotion pipeline.
