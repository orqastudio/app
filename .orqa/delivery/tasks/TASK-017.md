---
id: TASK-017
title: First-run setup wizard
description: "Implements an onboarding wizard for first-time users covering Claude CLI detection, auth verification, and project name, icon, and model configuration."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - Claude CLI detection (binary on PATH)
  - Auth status verification
  - Project name
  - icon
  - model configuration
  - Custom project icon upload/removal
relationships:
  - target: EPIC-035
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-030
    type: grounded-by
  - target: SKILL-032
    type: grounded-by
  - target: TASK-325
    type: depended-on-by
---
## What

Implement the first-run onboarding wizard for project creation and AI provider setup.

## Outcome

Implemented as ProjectSetupWizard.svelte with Claude CLI detection, auth status,
and project configuration. Git commits: `1ccf304`, `5156a6e`, `34ec185`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
