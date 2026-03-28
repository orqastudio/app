---
id: DOC-fc36aeec
type: doc
status: active
title: "Software Project Setup Guide"
domain: onboarding
description: "How to set up OrqaStudio for a software development project — what the software preset adds, how stack detection works, and what to customise."
category: onboarding
created: 2026-03-18
updated: 2026-03-18
relationships:
  - target: KNOW-d03337ac
    type: synchronised-with
---

# Software Project Setup Guide

When you initialise a project as a "software development" type, OrqaStudio layers development-specific governance on top of the core framework. This guide explains what the software preset adds and how to customise it.

## What the Preset Adds

### The Software Plugin

The software plugin (`@orqastudio/plugin-software-kanban`) is installed automatically. It provides:

- **5 artifact types:** milestone, epic, task, research, wireframe
- **9 relationships:** delivers, fulfils, depends-on, realises, produces, yields, reports, fixes, affects
- **Views:** roadmap (kanban board of delivery status)
- **Widgets:** delivery pipeline, milestone context

### Development Rules

The preset copies development-specific rules to your project:

| Rule | Purpose |
| ------ | --------- |
| Coding Standards | Language-specific standards populated from stack detection |
| Testing Standards | Test organisation, coverage requirements, mock boundaries |
| Git Workflow | Worktree workflow, branch naming, merge protocol |
| Development Commands | Build/test/lint command standardisation |
| End-to-End Completeness | All layers in the same commit |
| Error Ownership | All errors are your responsibility |

### Stack-Specific Skills

Based on detected languages and frameworks, relevant skills are loaded:

| Detected Stack | Skills Added |
| --------------- | ------------- |
| Rust | `rust-async-patterns` |
| TypeScript | `typescript-advanced-types` |
| Svelte | `svelte5-best-practices` |
| Tailwind | `tailwind-design-system` |
| Tauri | `tauri-v2` |

## Stack Detection

The setup process analyses your project directory to detect:

- **Languages:** presence of `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`
- **Frameworks:** Svelte, React, Vue (from package.json), Tauri (from tauri.conf.json)
- **Build tools:** Make, npm scripts, cargo
- **Test frameworks:** cargo test, Vitest, Jest

Detection drives skill selection and coding standard generation.

## Customisation

Generated rules are starting points. After setup:

1. Review generated coding standards and adjust to your team's preferences
2. Configure the orchestrator's skill injection table for your architecture
3. Add project-specific rules for patterns unique to your codebase
4. Set up pre-commit hooks for automated enforcement

## Epic Workflow

For software projects, `workflow.epics-required` is set to `true` by default. This means tasks should be linked to epics for traceability. Research-focused software projects can set this to `false`.
