---
id: TASK-017
title: First-run setup wizard
description: Implements an onboarding wizard for first-time users covering Claude CLI detection, auth verification, and project name, icon, and model configuration.
status: done
created: "2026-03-04"
updated: "2026-03-09"
epic: EPIC-035
assignee: AGENT-002
skills:
  - SKILL-030
  - SKILL-032
scope:
  - ui/lib/components/settings/ProjectSetupWizard.svelte
  - ui/lib/components/settings/
acceptance:
  - Claude CLI detection (binary on PATH)
  - Auth status verification
  - Project name
  - icon
  - model configuration
  - Custom project icon upload/removal
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
