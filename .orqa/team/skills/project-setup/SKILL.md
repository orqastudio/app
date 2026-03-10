---
id: SKILL-023
title: Project Setup
description: |
  Universal project scaffolding: creates the .orqa/ directory structure,
  copies canon rules and skills, initialises project.json, and sets up
  CLI symlinks. This is the base setup — project type presets layer on top.
  Use when: Initialising a new project with OrqaStudio governance, or
  repairing a broken .orqa/ structure.
status: active
created: "2026-03-01"
updated: "2026-03-10"
layer: canon
version: 1.0.0
user-invocable: true
---


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
        { "key": "ideas", "label": "Ideas", "path": ".orqa/planning/ideas" },
        { "key": "research", "label": "Research", "path": ".orqa/planning/research" },
        { "key": "milestones", "label": "Milestones", "path": ".orqa/planning/milestones" },
        { "key": "epics", "label": "Epics", "path": ".orqa/planning/epics" },
        { "key": "tasks", "label": "Tasks", "path": ".orqa/planning/tasks" }
      ]
    },
    { "key": "governance", "label": "Governance", "icon": "shield",
      "children": [
        { "key": "lessons", "label": "Lessons", "path": ".orqa/governance/lessons" },
        { "key": "decisions", "label": "Decisions", "path": ".orqa/governance/decisions" },
        { "key": "rules", "label": "Rules", "path": ".orqa/governance/rules" }
      ]
    },
    { "key": "team", "label": "Team", "icon": "users",
      "children": [
        { "key": "agents", "label": "Agents", "path": ".orqa/team/agents" },
        { "key": "skills", "label": "Skills", "path": ".orqa/team/skills" }
      ]
    }
  ]
}
```

## Canon Content

These files are copied during setup (canon layer — non-editable by project):

### Canon Rules
- `artifact-lifecycle.md` — Artifact status transitions and gates
- `documentation-first.md` — Documentation before code
- `honest-reporting.md` — No false completion claims
- `no-stubs.md` — Real implementations only
- `systems-thinking.md` — Think in systems, not patches

### Canon Agents (7 Universal Roles)
- `orchestrator.md`, `researcher.md`, `planner.md`, `implementer.md`
- `reviewer.md`, `writer.md`, `designer.md`

### Canon Skills
- `code-search`, `chunkhound` — Code search
- `orqa-composability` — Composability philosophy
- `planning` — Planning methodology
- `architecture` — ADR patterns
- `diagnostic-methodology`, `restructuring-methodology` — Process skills
- `code-quality-review`, `qa-verification`, `ux-compliance-review` — Review skills
- `test-engineering`, `security-audit`, `architectural-evaluation` — Specialisation skills
- `governance-maintenance`, `skills-maintenance` — Maintenance skills

## CLI Symlink Setup

For Claude Code compatibility, create symlinks in `.claude/`:

```
.claude/rules/    → .orqa/governance/rules/
.claude/agents/   → .orqa/team/agents/
.claude/skills/   → .orqa/team/skills/
.claude/hooks/    → .orqa/governance/hooks/
.claude/CLAUDE.md → .orqa/team/agents/orchestrator.md
```

## Setup Procedure

1. Create the `.orqa/` directory tree
2. Generate `project.json` with project name and default artifacts config
3. Copy canon rules, agents, and skills
4. Create CLI symlinks (if Claude Code is detected)
5. Run `project-inference` to detect project characteristics
6. Apply appropriate project type preset (e.g., `project-type-software`)
7. Report what was created and what the user should review

## Critical Rules

- NEVER overwrite existing `.orqa/` content — setup is for NEW projects
- If `.orqa/` already exists, offer repair/update instead of overwrite
- Canon content is read-only for the project — updates come from OrqaStudio releases
- Project-added rules and skills layer ON TOP of canon, never replace it
