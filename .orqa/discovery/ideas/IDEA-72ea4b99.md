---
id: IDEA-72ea4b99
type: discovery-idea
title: VSCode workspace plugin — IDE integration for OrqaStudio projects
description: "A VSCode plugin that opens code for editing, manages workspace settings, and drives plugin/extension recommendations based on project requirements. AI-powered project configuration."
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: later
research-needed: []
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

OrqaStudio manages the planning and governance layer of a project but has no integration with the IDE where code is actually written. Developers context-switch between OrqaStudio and VSCode with no connection between them. A VSCode plugin could read `project.json` to configure workspace settings, recommend extensions matching the detected stack, and surface relevant OrqaStudio artifacts (current task, related epics) alongside the file being edited. AI-powered configuration could auto-apply settings based on project requirements without manual workspace setup.

## Sketch

A VSCode extension that reads `.orqa/project.json` on workspace open. Recommends extensions based on detected stack (Rust → rust-analyzer, Svelte → Svelte for VS Code, etc.). Surfaces the active task in the status bar. Could eventually support inline `@orqa` reference navigation (linked with IDEA-b37248bf). AI component suggests workspace settings based on project coding standards from `.orqa/documentation/development/coding-standards.md`.