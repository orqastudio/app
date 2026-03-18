---
id: IMPL-018
title: Hardcoded .orqa/ paths in source code should be project-configurable
description: "Source code references to .orqa/ subdirectories are hardcoded constants. If a project requires a different directory structure, the code breaks. These required paths should come from project config so the system adapts to different project layouts."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships: []
---

## Pattern

`paths.rs` defines constants like `PILLARS_DIR = ".orqa/process/pillars"`. `project_scanner.rs` hardcodes `.orqa/process/lessons`. `artifact_fs.rs` hardcodes `governance_dir()` mappings. None of these read from `project.json`.

If a project needs a different directory structure (e.g., no `process/` level, or different subdirectory names), every one of these constants must be changed in source code and recompiled. The system should discover where required artifact types live from project configuration, not from compiled-in paths.

[RULE-003](RULE-003) already says "no hardcoded paths in Rust or TypeScript — all artifact paths come from config" but the enforcement gap is that several Rust modules still use constants instead of reading `project.json`.

## Fix

Option C from [RES-052](RES-052): Runtime config cache. Load `project.json` once at startup, build a `ProjectPaths` struct, pass through the call chain. Remove `paths.rs` constants (keep only `ORQA_DIR` and `SETTINGS_FILE` as bootstrap constants needed to find the config file itself). User-approved decision.

## Triage

Resolved by [TASK-306](TASK-306) — ProjectPaths runtime config cache replaced all hardcoded path constants.
