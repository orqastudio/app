---
id: "IDEA-95654649"
title: "Verification definition and scheduling system"
description: "Verification definitions on artifacts (mode, cadence, trigger, source, parameters, prompt) with a scheduling system that creates VER-NNN records from automated checks and TASK-NNN from failures or reflection prompts."
status: "captured"
created: "2026-03-12"
updated: "2026-03-13"
horizon: "later"
research-needed:
  - "Verification definition schema design (mode/cadence/trigger/source/parameters/prompt/required-fields)"
  - "Scheduling mechanism — cron-like, project-start hooks, manual triggers"
  - "VER-NNN storage strategy for different volume levels"
  - "Auto-creation of TASK-NNN from failed automated checks"
relationships:
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-cda6edd6"
    type: "benefits"
---
## Motivation

[AD-a76663db](AD-a76663db) defines two verification paths: automated (plugin data snapshots compared against parameters) and human (tasks created on failure or for reflection). The scheduling system is the mechanism that triggers these at the configured cadence and creates the appropriate records.
