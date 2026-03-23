---
id: KNOW-e0dec720
title: Project Setup
description: |
  Universal project scaffolding: creates the governance directory structure,
  copies core rules and knowledge, initialises project configuration, and sets up
  CLI symlinks. This is the base setup — project type presets layer on top.
  Use when: initialising a new project with governance, or repairing a broken
  governance directory structure.
status: active
onboarding: true
created: 2026-03-01
updated: 2026-03-10
category: tool
version: 1.0.0
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
  project configuration         # Project metadata and artifact paths
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
    rules/                      #   Rule artifacts
    hooks/                      #   Event hooks
  team/                         # Team artifacts
    agents/                     #   Agent definitions
    knowledge/                  #   Knowledge directories
```

## Project Configuration Schema

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

## Core Content

These files are copied during setup (core layer — non-editable by project):

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
2. Generate project configuration with project name and default artifacts config
3. Copy core rules, agents, and knowledge
4. Create CLI symlinks (if Claude Code is detected)
5. Run `project-inference` to detect project characteristics
6. Run `epic-requirement-inference` to recommend `workflow.epics-required` setting
7. Set `workflow.epics-required` in project configuration based on recommendation
8. Apply appropriate project type preset (e.g., `project-type-software`)
9. Report what was created and what the user should review

## Critical Rules

- NEVER overwrite existing governance content — setup is for NEW projects
- If governance directory already exists, offer repair/update instead of overwrite
- Core content is read-only for the project — updates come from platform releases
- Project-added rules and knowledge layer ON TOP of core, never replace it
