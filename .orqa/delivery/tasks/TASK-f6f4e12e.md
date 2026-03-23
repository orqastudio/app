---
id: TASK-f6f4e12e
type: task
title: Orchestrator skill injection table
description: Adds a Tier 2 skill injection table to the orchestrator definition and updates the skill-enforcement rule to document how project-specific skills are matched to task scope at delegation time.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-1dab5ebe
acceptance:
  - Orchestrator agent definition includes a skill injection table mapping scope → skills
  - skill-enforcement.md updated to document the three-tier model
  - "Injection table covers all orqa-* project skills with clear scope triggers"
relationships:
  - target: EPIC-a1dd9e9f
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-80821c3e
    type: depended-on-by
---
## What

Add the Tier 2 skill injection table to the orchestrator's agent definition and update
the skill-enforcement rule to document the three-tier loading model.

## Implementation Notes

The injection table maps task scope (file paths/directories) to the project-specific
skills that should be loaded. The orchestrator includes these skills in the delegation
prompt when creating tasks for agents.

Table from [EPIC-a1dd9e9f](EPIC-a1dd9e9f) implementation design:

| Task Scope | Injected Skills |
|-----------|----------------|
| `backend/src-tauri/src/commands/` | `orqa-ipc-patterns`, `orqa-error-composition` |
| `backend/src-tauri/src/domain/` | `orqa-domain-services`, `orqa-error-composition` |
| `backend/src-tauri/src/repo/`, `backend/src-tauri/src/db.rs` | `orqa-repository-pattern` |
| `backend/src-tauri/src/search/` | `orqa-native-search` |
| `sidecars/claude-agentsdk-sidecar/src/` | `orqa-streaming` |
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