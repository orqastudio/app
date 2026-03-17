---
id: TASK-464
title: Create delegation reference document
description: "Create the orchestrator's single source of truth for delegation — a lookup table mapping every work type to agent role, required skills, and grounding document. Makes delegation a lookup, not a judgement call."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - documentation/reference/delegation.md created with frontmatter and graph relationships
  - "Every work type in the project mapped to agent role, skills, and grounding doc"
  - "Orchestrator Direct\" column explicitly marks what the orchestrator may do (coordination only)"
  - Document states clearly that the orchestrator writing anything other than coordination output is a system failure
  - Connected to orchestrator via grounded-by relationship
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 2 — delegation reference grounds the orchestrator's coordination behavior
  - target: TASK-463
    type: informed-by
    rationale: Grounding docs must exist before the delegation reference can point to them
  - target: TASK-463
    type: depends-on
  - target: SKILL-037
    type: informed-by
  - target: SKILL-011
    type: informed-by
  - target: SKILL-011
    type: informed-by
  - target: SKILL-037
    type: informed-by
---

## Scope

Create `documentation/reference/delegation.md` as the orchestrator's delegation source of truth.

### Content

The document maps every type of work in the project:

| Work Type | Agent Role | Required Skills | Grounding Doc | Orchestrator Direct? |
|-----------|-----------|----------------|---------------|---------------------|
| Rust backend code | Implementer | backend-best-practices, tauri-v2, orqa-domain-services | code-principles | **NEVER** |
| Svelte frontend code | Implementer | frontend-best-practices, svelte5, orqa-store-patterns | code-principles | **NEVER** |
| Sidecar/streaming code | Implementer | orqa-streaming, backend-best-practices | code-principles | **NEVER** |
| Architecture assessment | Planner | architecture, architectural-evaluation | product-purpose | **NEVER** |
| Code review | Reviewer | code-quality-review, test-engineering | code-principles | **NEVER** |
| QA verification | Reviewer | qa-verification | code-principles | **NEVER** |
| UX review | Reviewer | ux-compliance-review | design-principles | **NEVER** |
| Documentation content | Writer | orqa-documentation | artifact-principles | **NEVER** |
| UI/UX design | Designer | component-extraction, svelte5 | design-principles | **NEVER** |
| Investigation/research | Researcher | research-methodology | research-principles | **NEVER** |
| Governance artifacts | Governance Steward | orqa-governance, orqa-schema-compliance | artifact-principles | **NEVER** |
| Coordination | Orchestrator | — | product-purpose | **ALWAYS** — this is all it does |

### Principles Section

The document opens with:
- The orchestrator's only job is coordination: deciding what needs to happen, delegating to the right role, verifying results
- If the orchestrator is writing code, docs, or artifacts, the system has failed to delegate
- Delegation is a lookup against this table, not a judgement call
- Every delegation includes the role's grounding document
