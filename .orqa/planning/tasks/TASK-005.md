---
id: TASK-005
title: Architecture Docs Vision Audit
description: Audit architecture, process, and development docs for alignment with the updated vision. These define how the app is built and how work happens — they must reflect .orqa/ as source of truth and provider-agnostic framing.
status: done
scope:
  - .orqa/documentation/architecture/decisions.md
  - .orqa/documentation/architecture/project-configuration.md
  - .orqa/documentation/architecture/rust-modules.md
  - .orqa/documentation/architecture/streaming-pipeline.md
  - .orqa/documentation/architecture/governance-bootstrap.md
  - .orqa/documentation/architecture/ipc-commands.md
  - .orqa/documentation/architecture/svelte-components.md
  - .orqa/documentation/process/orchestration.md
  - .orqa/documentation/process/artifact-workflow.md
  - .orqa/documentation/process/content-governance.md
  - .orqa/documentation/process/workflow.md
  - .orqa/documentation/process/team.md
  - .orqa/documentation/development/architecture-overview.md
  - .orqa/documentation/development/getting-started.md
  - .orqa/documentation/development/contributing.md
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-033
assignee: AGENT-007
skills:
  - SKILL-003
  - SKILL-011
  - SKILL-002
acceptance:
  - Every architecture
  - process
  - and development doc checked against 5 audit criteria
  - No .claude/ referenced as source of truth
  - No Claude-as-product-identity language
  - Three-layer architecture referenced where governance is discussed
  - Architecture decisions doc updated to reflect .orqa/ as canonical path
---
## What

Read each doc listed in scope. Apply the 5 audit criteria from the plan.
Fix misalignment in-place.

## Priority Order

1. `architecture/decisions.md` — foundational, referenced by many rules
2. `architecture/project-configuration.md` — directly affected by config changes
3. `process/orchestration.md` — defines how the orchestrator works
4. `process/artifact-workflow.md` — defines artifact lifecycle
5. Everything else

## Deliverable

Updated markdown files with a summary of changes made per file.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
