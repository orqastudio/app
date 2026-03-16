---
id: SKILL-024
title: "Project Type: Software"
description: |
  Software development governance preset: adds worktree workflow rules,
  code quality standards, testing requirements, CI/CD patterns, and
  development-specific agent skills to the base OrqaStudio setup.
  Use when: The project being set up is a software development project.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: setup
category: tool
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Software preset layers worktree rules, testing standards, and CI patterns onto base governance, making dev workflow expectations explicit
---

> **Forward-looking:** This skill will be activated when project initialisation and software project type support are implemented. See [EPIC-045](EPIC-045) and [EPIC-047](EPIC-047) for context.

Adds software development governance to a base OrqaStudio setup. This is a project type preset — it layers development-specific rules, skills, and documentation templates on top of the core governance foundation.

## What This Preset Adds

### Additional Rules

| Rule | Purpose |
|------|---------|
| `coding-standards.md` | Language-specific coding standards (populated from inference) |
| `testing-standards.md` | Test organisation, coverage requirements, mock boundaries |
| `git-workflow.md` | Worktree workflow, branch naming, merge protocol |
| `development-commands.md` | Build/test/lint command standardisation |
| `end-to-end-completeness.md` | All layers in the same commit |
| `error-ownership.md` | All errors are your responsibility |
| `no-aliases-or-hacks.md` | Fix root causes, not symptoms |
| `reusable-components.md` | Shared component library usage |

### Additional Skills (loaded based on detected stack)

| Detected Stack | Skills Added |
|---------------|-------------|
| Rust | `rust-async-patterns` |
| TypeScript | `typescript-advanced-types` |
| Svelte | `svelte5-best-practices` |
| Tailwind | `tailwind-design-system` |
| Tauri | `tauri-v2` |

### Documentation Templates

| Template | Path | Purpose |
|----------|------|---------|
| Coding Standards | `.orqa/documentation/development/coding-standards.md` | Populated from inference + user input |
| Architecture Decisions | `.orqa/documentation/development/decisions.md` | Index of architecture decisions |
| Development Commands | `.orqa/documentation/development/commands.md` | Build/test/lint command reference |

### Agent Skill Injection Rules

For software projects, the orchestrator's Tier 2 injection table is populated:

| Task Touches | Inject Skills |
|-------------|---------------|
| Backend code | `orqa-domain-services`, `orqa-error-composition`, `orqa-ipc-patterns` (if Tauri) |
| Frontend code | `orqa-store-patterns`, `orqa-store-orchestration` |
| Database code | `orqa-repository-pattern` |
| Streaming code | `orqa-streaming` |
| Test code | `orqa-testing` |
| Governance files | `orqa-governance` |

(The `orqa-*` skills are project-specific and only created for OrqaStudio itself. For other software projects, equivalent project skills would be created during setup based on their architecture.)

## Setup Procedure

1. Read the project profile from `project-inference`
2. Copy software-specific rules to `.orqa/process/rules/`
3. Generate coding standards from detected languages/frameworks
4. Create documentation templates
5. Configure the orchestrator's skill injection table based on detected stack
6. Set up pre-commit hooks if the project uses git
7. Create a development commands reference from detected build tools

## Stack-Specific Configuration

### Rust Projects

- Enable clippy pedantic lint group
- Configure rustfmt
- Set up cargo test as the test command
- Add `thiserror` error handling patterns to coding standards

### TypeScript/JavaScript Projects

- Enable strict TypeScript (if tsconfig exists)
- Configure ESLint
- Set up Vitest or Jest as the test framework
- Add import ordering and naming convention rules

### Full-Stack Projects (Frontend + Backend)

- Add end-to-end completeness rules
- Configure both test runners
- Add IPC boundary rules (if applicable)
- Set up type consistency checking across boundaries

## Commit Discipline

Software projects require regular commits to prevent work loss and maintain clean history.

### Commit Boundaries

| Work Type | When to Commit |
|-----------|---------------|
| Feature implementation (worktree) | At each sub-task completion |
| Bug fixes | After each fix, before starting the next |
| Governance-only work (rules, docs, artifacts) | At each logical milestone or every ~20 files |
| Refactoring | After each safe, verified step |

### Governance Work on Main

Governance-only changes (`.orqa/` files) are often done directly on main without a worktree. This is acceptable because they don't affect build state, but commit discipline still applies:

- Commit at logical milestones (e.g., "rules updated", "epic planned")
- The session-start hook warns when uncommitted files exceed 20 on main
- Never end a session with uncommitted changes

### Session Boundaries

Every session that produces changes MUST commit before ending. The session-end hook reminds about this, but agents should commit proactively rather than waiting for the reminder.

## Critical Rules

- NEVER assume a one-size-fits-all approach — adapt to the detected stack
- ALWAYS ask the user to review generated coding standards before finalising
- Stack detection drives skill selection — don't load irrelevant skills
- Generated rules should be starting points, not final — the user customises them
- ALWAYS commit at natural boundaries — never accumulate large uncommitted batches
