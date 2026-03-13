---
id: "SKILL-023"
title: "Project Setup"
description: "Universal project scaffolding: creates the .orqa/ directory structure,

  copies core rules and skills, initialises project.json, and sets up

  CLI symlinks. This is the base setup — project type presets layer on top.

  Use when: Initialising a new project with OrqaStudio governance, or

  repairing a broken .orqa/ structure.\n"
status: "active"
created: "2026-03-01"
updated: "2026-03-10"
layer: "setup"
scope:
  - "AGENT-003"
category: "tool"
version: "1.0.0"
user-invocable: true
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Project setup creates structured scaffolding"
---

> **Forward-looking:** This skill will be activated when project initialisation is implemented. See [EPIC-045](EPIC-045) for context.

Creates the base governance structure for any project. This skill is domain-agnostic — it sets up the infrastructure that all project types share. Domain-specific rules and skills are added by project type presets (e.g., `project-type-software`).

## .orqa/ Directory Structure

The base structure created by project setup:

```
.orqa/
  project.json              # Project configuration
  icon.svg                  # Project icon (default provided)
  documentation/            # Documentation tree
    architecture/           #   Architecture decisions and docs
    development/            #   Development guides
    process/                #   Process documentation
    product/                #   Product vision, roadmap
  planning/                 # Planning artifacts
    ideas/                  #   IDEA-NNN.md
    research/               #   Investigation documents
    milestones/             #   MS-NNN.md
    epics/                  #   EPIC-NNN.md
    tasks/                  #   TASK-NNN.md
  governance/               # Governance artifacts
    lessons/                #   IMPL-NNN.md
    decisions/              #   AD-NNN.md
    rules/                  #   Rule markdown files
    hooks/                  #   Event hooks
  team/                     # Team artifacts
    agents/                 #   Agent definitions
    skills/                 #   Skill directories
```

## project.json Schema

```json
{
  "name": "<project-name>",
  "description": "<project-description>",
  "dogfood": false,
  "default_model": "sonnet",
  "artifacts": [
    { "key": "docs", "label": "Documentation", "icon": "file-text", "path": ".orqa/documentation" },
    { "key": "planning", "label": "Planning", "icon": "target",
      "children": [
        { "key": "ideas", "label": "Ideas", "path": ".orqa/delivery/ideas" },
        { "key": "research", "label": "Research", "path": ".orqa/delivery/research" },
        { "key": "milestones", "label": "Milestones", "path": ".orqa/delivery/milestones" },
        { "key": "epics", "label": "Epics", "path": ".orqa/delivery/epics" },
        { "key": "tasks", "label": "Tasks", "path": ".orqa/delivery/tasks" }
      ]
    },
    { "key": "governance", "label": "Governance", "icon": "shield",
      "children": [
        { "key": "lessons", "label": "Lessons", "path": ".orqa/process/lessons" },
        { "key": "decisions", "label": "Decisions", "path": ".orqa/process/decisions" },
        { "key": "rules", "label": "Rules", "path": ".orqa/process/rules" }
      ]
    },
    { "key": "team", "label": "Team", "icon": "users",
      "children": [
        { "key": "agents", "label": "Agents", "path": ".orqa/process/agents" },
        { "key": "skills", "label": "Skills", "path": ".orqa/process/skills" }
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

### Core Skills
- `orqa-code-search`, `chunkhound` — Code search
- `composability` — Composability philosophy
- `planning` — Planning methodology
- `architecture` — ADR patterns
- `diagnostic-methodology`, `restructuring-methodology` — Process skills
- `code-quality-review`, `qa-verification`, `ux-compliance-review` — Review skills
- `test-engineering`, `security-audit`, `architectural-evaluation` — Specialisation skills
- `governance-maintenance`, `skills-maintenance` — Maintenance skills

## CLI Symlink Setup

For Claude Code compatibility, create symlinks in `.claude/`:

```
.claude/rules/    → .orqa/process/rules/
.claude/agents/   → .orqa/process/agents/
.claude/skills/   → .orqa/process/skills/
.claude/hooks/    → .orqa/process/hooks/
.claude/CLAUDE.md → .orqa/process/agents/orchestrator.md
```

## Setup Procedure

1. Create the `.orqa/` directory tree
2. Generate `project.json` with project name and default artifacts config
3. Copy core rules, agents, and skills
4. Create CLI symlinks (if Claude Code is detected)
5. Run `project-inference` to detect project characteristics
6. Run `epic-requirement-inference` to recommend `workflow.epics-required` setting
7. Set `workflow.epics-required` in `project.json` based on recommendation
8. Apply appropriate project type preset (e.g., `project-type-software`)
9. Report what was created and what the user should review

## Critical Rules

- NEVER overwrite existing `.orqa/` content — setup is for NEW projects
- If `.orqa/` already exists, offer repair/update instead of overwrite
- Core content is read-only for the project — updates come from OrqaStudio releases
- Project-added rules and skills layer ON TOP of core, never replace it
