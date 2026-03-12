---
id: TASK-034
title: Orchestrator skill injection table
description: Adds a Tier 2 skill injection table to the orchestrator definition and updates the skill-enforcement rule to document how project-specific skills are matched to task scope at delegation time.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-042
assignee: AGENT-003
skills:
  - SKILL-029
  - SKILL-011
acceptance:
  - Orchestrator agent definition includes a skill injection table mapping scope → skills
  - skill-enforcement.md updated to document the three-tier model
  - Injection table covers all orqa-* project skills with clear scope triggers
---
## What

Add the Tier 2 skill injection table to the orchestrator's agent definition and update
the skill-enforcement rule to document the three-tier loading model.

## Implementation Notes

The injection table maps task scope (file paths/directories) to the project-specific
skills that should be loaded. The orchestrator includes these skills in the delegation
prompt when creating tasks for agents.

Table from [EPIC-042](EPIC-042) implementation design:

| Task Scope | Injected Skills |
|-----------|----------------|
| `backend/src-tauri/src/commands/` | `orqa-ipc-patterns`, `orqa-error-composition` |
| `backend/src-tauri/src/domain/` | `orqa-domain-services`, `orqa-error-composition` |
| `backend/src-tauri/src/repo/`, `backend/src-tauri/src/db.rs` | `orqa-repository-pattern` |
| `backend/src-tauri/src/search/` | `orqa-native-search` |
| `sidecars/orqa-sidecar/src/` | `orqa-streaming` |
| `ui/src/lib/stores/` | `orqa-store-patterns`, `orqa-store-orchestration` |
| `ui/src/lib/components/` | `orqa-store-patterns` |
| `.orqa/` | `orqa-governance` |
| Any streaming work | `orqa-streaming` |
| Any cross-boundary work | `composability` (always injected) |
| Any test work | `orqa-testing` |

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
