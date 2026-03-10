---
id: TASK-017
title: First-run setup wizard
status: done
epic: EPIC-035
description: Implements an onboarding wizard for first-time users covering Claude CLI detection, auth verification, and project name, icon, and model configuration.
created: 2026-03-04
updated: 2026-03-09
assignee: frontend-engineer
skills: [svelte5-best-practices, tauri-v2]
scope: [ui/lib/components/settings/ProjectSetupWizard.svelte, ui/lib/components/settings/]
acceptance: [Claude CLI detection (binary on PATH), Auth status verification, Project name, icon, model configuration, Custom project icon upload/removal]
---

## What

Implement the first-run onboarding wizard for project creation and AI provider setup.

## Outcome

Implemented as ProjectSetupWizard.svelte with Claude CLI detection, auth status,
and project configuration. Git commits: `1ccf304`, `5156a6e`, `34ec185`.
