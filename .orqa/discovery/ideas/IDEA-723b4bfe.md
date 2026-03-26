---
id: "IDEA-723b4bfe"
type: idea
title: "Verification definition and scheduling system"
description: "Verification definitions on artifacts (mode, cadence, trigger, source, parameters, prompt) with a scheduling system that creates VER-NNN records from automated checks and TASK-NNN from failures or reflection prompts."
status: captured
created: "2026-03-12"
updated: "2026-03-13"
horizon: "later"
research-needed:
  - "Verification definition schema design (mode/cadence/trigger/source/parameters/prompt/required-fields)"
  - "Scheduling mechanism — cron-like, project-start hooks, manual triggers"
  - "VER-NNN storage strategy for different volume levels"
  - "Auto-creation of TASK-NNN from failed automated checks"
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-c4afd86b"
    type: "benefits"
---
## Motivation

[AD-430829f1](AD-430829f1) defines two verification paths: automated (plugin data snapshots compared against parameters) and human (tasks created on failure or for reflection). The scheduling system is the mechanism that triggers these at the configured cadence and creates the appropriate records.