---
id: "EPIC-be023ed2"
type: epic
title: "Portable Governance Framework"
description: "Restructure agents from 16 software-specific roles to 7 universal roles, extract domain knowledge into skills, create project setup skills, update product documentation to reflect the PILLAR-569581e0 engine identity and governance hub capability. Implements AD-774cc3d0 and AD-6f0dea5e."
status: "completed"
priority: "P1"
created: "2026-03-09"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: "RES-658ab0b7"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-658ab0b7"
  - target: "TASK-7d550875"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4a2936f9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f6850c71"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f1ada1f5"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-58887d38"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2ac2d88f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-3cb01b3f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0d8b4a06"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-b153fabd"
    type: "realised-by"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Context

[AD-774cc3d0](AD-774cc3d0) established that agents should represent universal roles (Orchestrator, Researcher,
Planner, Implementer, Reviewer, Writer, Designer) that work across any project type.
Domain-specific knowledge lives in skills loaded at runtime.

[AD-6f0dea5e](AD-6f0dea5e) established that project setup is skill-driven — no templates directory, no hardcoded
scaffolding. Setup skills contain the knowledge of what a project needs.

Both decisions support OrqaStudio's identity as a clarity engine for structured thinking,
not an AI development tool.

## Implementation Scope

### 1. Agent Restructuring [AD-774cc3d0](AD-774cc3d0)

Replace 16 software-specific agent files with 7 universal role definitions:

**Create:**
- `researcher.md` — Investigation, information gathering, analysis
- `planner.md` — Approach design, architectural evaluation
- `implementer.md` — Building things (code, deliverables, artifacts)
- `reviewer.md` — Quality verification, compliance checking

**Rename:**
- `documentation-writer.md` → `writer.md`

**Update:**
- `orchestrator.md` — Already restructured (Section 1 + 2)
- `designer.md` — Broaden from UI-only to experience/interface/structure design

**Remove (merge into universal roles):**
- backend-engineer.md, frontend-engineer.md, data-engineer.md → Implementer
- devops-engineer.md → Implementer
- systems-architect.md → Planner
- test-engineer.md, code-reviewer.md, qa-tester.md → Reviewer
- ux-reviewer.md, security-engineer.md → Reviewer
- debugger.md, refactor-agent.md → become skills only
- agent-maintainer.md → becomes skill for Orchestrator

### 2. Domain Skill Extraction [AD-774cc3d0](AD-774cc3d0)

Extract domain knowledge from old agents into loadable skills:

| Source Agent | New Skill | Content |
|-------------|-----------|---------|
| debugger | `diagnostic-methodology` | Root cause analysis, stack tracing |
| refactor-agent | `restructuring-methodology` | Safe refactoring steps, verification |
| security-engineer | `security-audit` | Audit checklist, threat model |
| agent-maintainer | `governance-maintenance` | Lesson promotion, rule updates |
| code-reviewer | `code-quality-review` | Zero-error policy, check sequence |
| qa-tester | `qa-verification` | E2E test approach, acceptance criteria |
| ux-reviewer | `ux-compliance-review` | UI spec comparison, accessibility |
| test-engineer | `test-engineering` | TDD workflow, coverage, mock boundaries |
| systems-architect | `architectural-evaluation` | Compliance checks, boundary verification |

### 3. Project Setup Skills [AD-6f0dea5e](AD-6f0dea5e)

Create the four setup skills that replace templates:

- `project-setup` — Universal scaffolding (creates .orqa/, installs canon)
- `project-inference` — Reads folder, produces project profile
- `project-migration` — Reads existing agentic config, maps to OrqaStudio
- `project-type-software` — Software development governance preset

### 4. Product Documentation Update

- Update `governance.md` with concept taxonomy from [AD-774cc3d0](AD-774cc3d0)
- Create `governance-hub.md` for the distribution/coexistence model from [AD-6f0dea5e](AD-6f0dea5e)
- Verify `artifact-framework.md` alignment with [AD-774cc3d0](AD-774cc3d0) concepts

## Constraints

- **Orchestrator-only work** — This restructuring affects agent definitions directly.
  Launching agents that reference old/restructured definitions risks confusion.
  All tasks are executed by the orchestrator, not delegated.
- **No code changes** — This epic is entirely governance artifacts (.orqa/ files).
  No changes to backend/src-tauri/, ui/, or sidecars/claude-agentsdk-sidecar/.
- **Backward compatible** — The orchestrator.md (which IS CLAUDE.md via symlink)
  must continue to work after restructuring. Agent references in rules must be
  updated to use new role names.
- **Update all cross-references** — Rules, skills, and other artifacts that reference
  old agent names must be updated in the same commit as the agent changes.

## Tasks

| Task | Title | Depends On |
|------|-------|------------|
| [TASK-3cb01b3f](TASK-3cb01b3f) | Task Dependency Mechanism | — |
| [TASK-7d550875](TASK-7d550875) | Create universal agent definitions | [TASK-3cb01b3f](TASK-3cb01b3f) |
| [TASK-f6850c71](TASK-f6850c71) | Extract domain skills from old agents | [TASK-3cb01b3f](TASK-3cb01b3f), [TASK-7d550875](TASK-7d550875) |
| [TASK-4a2936f9](TASK-4a2936f9) | Remove old software-specific agents | [TASK-7d550875](TASK-7d550875), [TASK-f6850c71](TASK-f6850c71) |
| [TASK-f1ada1f5](TASK-f1ada1f5) | Create project setup skills | [TASK-3cb01b3f](TASK-3cb01b3f) |
| [TASK-2ac2d88f](TASK-2ac2d88f) | Update rules for universal roles | [TASK-4a2936f9](TASK-4a2936f9), [TASK-f6850c71](TASK-f6850c71) |
| [TASK-58887d38](TASK-58887d38) | Update product documentation | [TASK-4a2936f9](TASK-4a2936f9), [TASK-2ac2d88f](TASK-2ac2d88f) |

## Dependency Chain

```
TASK-3cb01b3f (dependency mechanism)
  ├── TASK-7d550875 (create new agents)
  │     └── TASK-f6850c71 (extract skills from old agents)
  │           └── TASK-4a2936f9 (remove old agents)
  │                 └── TASK-2ac2d88f (update rules)
  │                       └── TASK-58887d38 (update product docs)
  └── TASK-f1ada1f5 (setup skills — independent track)
```

## Implementation Design

Implementation approach to be defined during planning.