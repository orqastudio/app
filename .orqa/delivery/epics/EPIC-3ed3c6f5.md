---
id: EPIC-3ed3c6f5
type: epic
title: "Plugin decomposition"
description: "Decompose the monolithic agile-governance plugin into the three-layer plugin architecture defined in RES-d6e8ab11: workflow-definition plugins (project domain), stage-definition plugins (methodology), and knowledge plugins (domain expertise). This was an acceptance criterion of EPIC-f6da17ed that was never completed."
status: active
priority: P0
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: EPIC-f6da17ed
    type: depends-on
    rationale: "Completes deferred acceptance criterion — three-layer plugin composition from RES-d6e8ab11 sections 3 and 4"
  - target: EPIC-aad17f28
    type: depends-on
    rationale: "Daemon enforcement must work before plugins are restructured"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Plugin composition is core to the dogfooding milestone"
---

## Context

RES-d6e8ab11 section 3 defines three plugin types:

1. **Workflow-definition plugins** — define the delivery skeleton with contribution points (stage slots)
2. **Stage-definition plugins** — fill specific slots with methodology (how a stage works)
3. **Knowledge plugins** — provide domain expertise composed into agents at delegation time

EPIC-f6da17ed's acceptance criteria included "workflow-definition plugin defines the skeleton, stage-definition plugins fill slots (composability)" — this was never done. The monolithic `agile-governance` plugin provides everything: artifact types, state machines, workflows, knowledge, rules, agent definitions, and relationship types.

The research gives concrete examples:
- `software-discovery plugin` fills planning
- `software-kanban plugin` fills implementation + review
- `governance plugin` fills learning pipeline

## Target Plugin Layout

| Plugin | Type | Provides |
|--------|------|----------|
| `software-project` | Workflow-definition | Delivery workflow skeleton with contribution points |
| `agile-methodology` | Stage-definition | `planning-methodology` contribution (scoping, estimation, prioritisation) |
| `software-kanban` | Stage-definition | `implementation-workflow` contribution (task lifecycle, branching) |
| *(new or existing)* | Stage-definition | `review-process` contribution |
| *(new or existing)* | Stage-definition | `discovery-artifacts` contribution (optional) |
| `core` | Stage-definition | `learning-pipeline` contribution (lessons, recurrence, promotion) |
| `rust` | Knowledge | Rust domain knowledge |
| `svelte` | Knowledge | Svelte/frontend domain knowledge |
| `tauri` | Knowledge | Tauri/desktop domain knowledge |
| `typescript` | Knowledge | TypeScript domain knowledge |
| `coding-standards` | Knowledge | Cross-cutting quality standards |
| `systems-thinking` | Knowledge | Process/methodology knowledge |

## Tasks

### Phase 1: Create New Plugins

**TASK-1: Create `software-project` workflow-definition plugin**
- New plugin with manifest, delivery workflow skeleton
- Move the delivery skeleton from agile-governance to here
- This plugin defines the project domain, not the methodology
- Acceptance criteria: plugin exists, delivery skeleton resolves, contribution points available

**TASK-2: Create `agile-methodology` stage-definition plugin**
- New plugin providing the `planning-methodology` contribution
- Move planning-related rules, knowledge from agile-governance
- Acceptance criteria: plugin exists, fills planning-methodology point, no warnings

**TASK-3: Rename `software` to `software-kanban` stage-definition plugin**
- Rename plugin, update manifest name
- Keep the `implementation-workflow` contribution (currently there, correctly placed)
- Remove the planning and review contributions (incorrectly placed there this session)
- Move software domain knowledge to appropriate knowledge plugins if mixed in
- Acceptance criteria: plugin renamed, fills only implementation-workflow

**TASK-4: Create or identify `review-process` stage-definition plugin**
- Determine which plugin provides the review methodology
- Could be a standalone `code-review` plugin or part of an existing plugin
- Must fill the `review-process` contribution point
- Acceptance criteria: review-process point filled

**TASK-5: Add `learning-pipeline` contribution to `core` plugin**
- Core plugin provides lesson creation, recurrence tracking, promotion pipeline
- This is a core governance capability, not domain-specific
- Acceptance criteria: core fills learning-pipeline point

### Phase 2: Decompose agile-governance

**TASK-6: Split agile-governance manifest**
- Move artifact type schemas to appropriate plugins (core for base types, software-project for delivery types)
- Move relationship definitions to the plugin that owns the source type
- Move rules to the plugin that owns the methodology they enforce
- Move knowledge to matching knowledge plugins
- Move agent definitions to core (universal roles are core)
- Acceptance criteria: agile-governance is eliminated or reduced to minimal scope

**TASK-7: Update all cross-references**
- Plugin names in workflow files, knowledge declarations, prompt registry
- Relationship targets that reference moved artifacts
- Update manifest `provides` arrays
- Acceptance criteria: `orqa check validate` passes, 0 errors

### Phase 3: Verify

**TASK-8: Full reinstall verification**
- Run `orqa plugin install` from scratch
- Verify all workflows resolve with correct contributions
- Verify prompt registry includes all knowledge from new plugin layout
- Verify agent files generate correctly
- Run full test suite
- Acceptance criteria: clean install, 0 errors, all tests pass

### Phase 4: Revert incorrect contributions

**TASK-9: Remove incorrect contributions from software plugin**
- Delete `planning.contribution.workflow.yaml` (belongs in agile-methodology)
- Delete `review.contribution.workflow.yaml` (belongs in review plugin)
- Keep `implementation.contribution.workflow.yaml` (correct location)
- Acceptance criteria: software plugin only fills implementation-workflow
