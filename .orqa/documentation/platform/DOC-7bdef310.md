---
id: DOC-7bdef310
type: doc
status: active
title: Delegation Reference
domain: reference
category: reference
description: "Orchestrator's source of truth for delegation — maps work types to roles, skills, and grounding. If the orchestrator is writing anything other than coordination output, the system has failed."
created: 2026-03-14
updated: 2026-03-14
sort: 10
relationships:
  - target: RULE-87ba1b81
    type: documents
    rationale: This document operationalises the delegation rule
---

# Delegation Reference

## The Orchestrator's Job

The orchestrator has one job: coordination. It decides what needs to happen, delegates to the right role, and verifies results. That is the complete list.

If the orchestrator is writing code, writing documentation content, or editing `.orqa/` governance artifacts with complex frontmatter, the system has failed to delegate. The orchestrator does not accumulate implementation details — it routes work to agents with the right skills and verifies their output against acceptance criteria.

Delegation is a lookup against the table below, not a judgement call. For every piece of work, the table provides the role, the skills to load, and the grounding document that anchors that role's purpose. Loading the grounding document is mandatory — it gives the delegated agent the context it needs to make good decisions, not just follow instructions.

Every delegation includes three things: the role, the required skills, and the grounding document. No delegation is complete without all three.

## Delegation Table

| Work Type | Agent Role | Required Skills | Grounding Doc |
| ----------- | ----------- | ---------------- | --------------- |
| Rust backend code (`app/src-tauri/`) | Implementer | `backend-best-practices`, `tauri-v2`, `orqa-domain-services`, `orqa-error-composition` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Svelte frontend code (`ui/`) | Implementer | `frontend-best-practices`, `svelte5-best-practices`, `orqa-store-patterns` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Sidecar/streaming code (`sidecar/`) | Implementer | `orqa-streaming`, `backend-best-practices` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Database/repository code | Implementer | `orqa-repository-pattern`, `orqa-error-composition` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| IPC commands and types | Implementer | `orqa-ipc-patterns`, `tauri-v2` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Plugin hooks and scripts | Implementer | `orqa-governance` (for context), `backend-best-practices` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Refactoring | Implementer | `restructuring-methodology`, `diagnostic-methodology` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Debugging | Implementer | `diagnostic-methodology` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Test writing | Implementer | `test-engineering`, `orqa-testing` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Architecture assessment | Planner | `architecture`, `architectural-evaluation` | product-purpose ([DOC-a0490c49](DOC-a0490c49)) |
| Implementation planning | Planner | `planning`, `architecture` | product-purpose ([DOC-a0490c49](DOC-a0490c49)) |
| Code review | Reviewer | `code-quality-review`, `test-engineering` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| QA verification | Reviewer | `qa-verification` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| UX compliance review | Reviewer | `ux-compliance-review` | design-principles ([DOC-40b1498a](DOC-40b1498a)) |
| Security assessment | Reviewer | `security-audit` | code-principles ([DOC-ebf19a16](DOC-ebf19a16)) |
| Documentation content | Writer | `orqa-documentation` | artifact-principles ([DOC-0ea4c263](DOC-0ea4c263)) |
| Investigation and research | Researcher | `research-methodology` | research-principles ([DOC-bdb520ae](DOC-bdb520ae)) |
| UI/UX design | Designer | `component-extraction`, `svelte5-best-practices`, `tailwind-design-system` | design-principles ([DOC-40b1498a](DOC-40b1498a)) |
| Governance artifacts (`.orqa/`) | Governance Steward | `orqa-governance`, `orqa-schema-compliance`, `orqa-documentation` | artifact-principles ([DOC-0ea4c263](DOC-0ea4c263)) |
| Coordination only | Orchestrator | — | product-purpose ([DOC-a0490c49](DOC-a0490c49)) |

## Failure Signals

These signals indicate that delegation has broken down and must be corrected immediately:

- **The orchestrator is reading large code files.** Reading more than 3 files directly when a search or agent delegation would be more efficient means the orchestrator is accumulating implementation details it should be routing to a researcher or implementer.
- **The orchestrator is editing `.rs`, `.svelte`, or `.ts` files.** Any change to `app/src-tauri/`, `ui/`, or `sidecar/` must be delegated to an Implementer. The orchestrator does not write production code.
- **The orchestrator is creating `.orqa/` artifacts with complex frontmatter.** Governance artifacts with multi-field frontmatter, relationship arrays, or schema-constrained content belong to the Governance Steward role.
- **The orchestrator's context is filling with implementation details.** If the orchestrator is holding function signatures, error messages, or test output across multiple turns, it should be delegating and summarising — not accumulating.
- **The orchestrator is running `make check` or `cargo test`.** Quality verification is the Reviewer's job. The orchestrator delegates the check and receives a verdict.
