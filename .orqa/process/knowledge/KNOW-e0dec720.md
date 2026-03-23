---
id: KNOW-e0dec720
type: knowledge
title: Project Setup
description: |
  Universal project scaffolding: creates the governance directory structure,
  installs core plugins, initialises project configuration, and sets up
  CLI symlinks. This is the base setup — project type presets layer on top.
  Use when: initialising a new project with governance, or repairing a broken
  governance directory structure.
status: active
onboarding: true
created: 2026-03-01
updated: 2026-03-23
category: tool
version: 2.0.0
user-invocable: true
relationships:
  - target: DOC-a1b2c3d4
    type: synchronised-with
---

Creates the base governance structure for any project. This skill is domain-agnostic — it sets up the infrastructure that all project types share. Domain-specific rules and knowledge are added by project type presets (e.g., `project-type-software`).

## Governance Directory Structure

The base structure created by project setup:

```
<governance-dir>/
  project.json                  # Project metadata and artifact paths
  manifest.json                 # Plugin content ownership tracking
  icon.svg                      # Project icon (default provided)
  documentation/                # Documentation tree
    architecture/               #   Architecture decisions and docs
    development/                #   Development guides
    process/                    #   Process documentation
    product/                    #   Product vision, roadmap
  delivery/                     # Planning artifacts
    ideas/                      #   Idea artifacts
    research/                   #   Investigation documents
    milestones/                 #   Milestone artifacts
    epics/                      #   Epic artifacts
    tasks/                      #   Task artifacts
  process/                      # Governance artifacts
    lessons/                    #   Lesson artifacts
    decisions/                  #   Architecture decision records
    rules/                      #   Rule artifacts (populated by plugins)
    hooks/                      #   Event hooks
    agents/                     #   Agent definitions (populated by plugins)
    knowledge/                  #   Knowledge artifacts (populated by plugins)
```

## How Core Content Is Installed

Core rules, agents, and knowledge are NOT copied manually. They come from the `@orqastudio/plugin-core-framework` plugin, installed automatically during project setup.

The plugin lifecycle:
1. `orqa plugin install @orqastudio/plugin-core-framework` — copies agents, rules, and knowledge to `.orqa/`
2. The installed files are recorded in `manifest.json` and owned by the plugin
3. Plugin-owned files cannot be edited directly — edit in the plugin source and run `orqa plugin refresh`

Additional plugins (software, coding-standards, etc.) are installed the same way and layer content on top.

## Project Configuration Schema (`project.json`)

```json
{
  "name": "<project-name>",
  "description": "<project-description>",
  "default_model": "sonnet",
  "artifacts": [
    { "key": "docs", "label": "Documentation", "icon": "file-text", "path": "<governance-dir>/documentation" },
    { "key": "planning", "label": "Planning", "icon": "target",
      "children": [
        { "key": "ideas", "label": "Ideas", "path": "<governance-dir>/delivery/ideas" },
        { "key": "research", "label": "Research", "path": "<governance-dir>/delivery/research" },
        { "key": "milestones", "label": "Milestones", "path": "<governance-dir>/delivery/milestones" },
        { "key": "epics", "label": "Epics", "path": "<governance-dir>/delivery/epics" },
        { "key": "tasks", "label": "Tasks", "path": "<governance-dir>/delivery/tasks" }
      ]
    },
    { "key": "governance", "label": "Governance", "icon": "shield",
      "children": [
        { "key": "lessons", "label": "Lessons", "path": "<governance-dir>/process/lessons" },
        { "key": "decisions", "label": "Decisions", "path": "<governance-dir>/process/decisions" },
        { "key": "rules", "label": "Rules", "path": "<governance-dir>/process/rules" }
      ]
    },
    { "key": "team", "label": "Team", "icon": "users",
      "children": [
        { "key": "agents", "label": "Agents", "path": "<governance-dir>/process/agents" },
        { "key": "knowledge", "label": "Knowledge", "path": "<governance-dir>/process/knowledge" }
      ]
    }
  ]
}
```

## Manifest File (`manifest.json`)

Created alongside `project.json`. Tracks which files in `.orqa/` are owned by which plugin:

```json
{
  "plugins": {
    "@orqastudio/plugin-core-framework": {
      "version": "0.1.0-dev",
      "installed_at": "2026-03-23T00:35:00.000Z",
      "files": [
        ".orqa/process/agents/AGENT-1dab5ebe.md",
        ".orqa/process/knowledge/KNOW-b453410f.md",
        ".orqa/process/rules/RULE-029db175.md"
      ]
    }
  }
}
```

The engine uses `manifest.json` to enforce edit protection on plugin-owned files and to know which files to remove on uninstall or disable.

## Core Content (from `@orqastudio/plugin-core-framework`)

The core plugin provides:

### Core Rules
- `artifact-lifecycle.md` — Artifact status transitions and gates
- `documentation-first.md` — Documentation before code
- `honest-reporting.md` — No false completion claims
- `no-stubs.md` — Real implementations only
- `systems-thinking.md` — Think in systems, not patches

### Core Agents (7 Universal Roles)
- `orchestrator.md`, `researcher.md`, `planner.md`, `implementer.md`
- `reviewer.md`, `writer.md`, `designer.md`

### Core Knowledge
- `search` — Search methodology
- `composability` — Composability philosophy
- `planning` — Planning methodology
- `architecture` — ADR patterns
- `diagnostic-methodology`, `restructuring-methodology` — Process knowledge
- `code-quality-review`, `qa-verification`, `ux-compliance-review` — Review knowledge
- `test-engineering`, `security-audit`, `architectural-evaluation` — Specialisation knowledge
- `governance-maintenance`, `skills-maintenance` — Maintenance knowledge

## CLI Symlink Setup

For Claude Code compatibility, create symlinks in the agent infrastructure directory:

```
<agent-dir>/rules/     → <governance-dir>/process/rules/
<agent-dir>/agents/    → <governance-dir>/process/agents/
<agent-dir>/knowledge/ → <governance-dir>/process/knowledge/
<agent-dir>/hooks/     → <governance-dir>/process/hooks/
<agent-dir>/CLAUDE.md  → <governance-dir>/process/agents/orchestrator.md
```

## Setup Procedure

1. Create the governance directory tree
2. Generate `project.json` with project name and default artifacts config
3. Create an empty `manifest.json`
4. Run `orqa plugin install @orqastudio/plugin-core-framework` — populates agents, rules, knowledge
5. Create CLI symlinks (if Claude Code is detected)
6. Run `project-inference` to detect project characteristics
7. Run `epic-requirement-inference` to recommend `workflow.epics-required` setting
8. Set `workflow.epics-required` in `project.json` based on recommendation
9. Apply appropriate project type preset (e.g., `project-type-software`)
10. Report what was created and what the user should review

## Critical Rules

- NEVER overwrite existing governance content — setup is for NEW projects
- If governance directory already exists, offer repair/update instead of overwrite
- Plugin-owned files in `.orqa/` are read-only for the project — update via `orqa plugin refresh`
- Project-added rules and knowledge layer ON TOP of plugin content, never replace it