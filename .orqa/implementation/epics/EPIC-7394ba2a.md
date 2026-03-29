---
id: "EPIC-7394ba2a"
type: "epic"
title: "Portable Governance Framework"
description: "Restructure agents from 16 software-specific roles to 7 universal roles, extract domain knowledge into skills, create project setup skills, update product documentation to reflect the PILLAR-c9e0a695 engine identity and governance hub capability. Implements PD-48b310f9 and PD-26b0eb9f."
status: archived
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
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

[PD-48b310f9](PD-48b310f9) established that agents should represent universal roles (Orchestrator, Researcher,
Planner, Implementer, Reviewer, Writer, Designer) that work across any project type.
Domain-specific knowledge lives in skills loaded at runtime.

[PD-26b0eb9f](PD-26b0eb9f) established that project setup is skill-driven — no templates directory, no hardcoded
scaffolding. Setup skills contain the knowledge of what a project needs.

Both decisions support OrqaStudio's identity as a clarity engine for structured thinking,
not an AI development tool.

## Implementation Scope

### 1. Agent Restructuring [PD-48b310f9](PD-48b310f9)

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

### 2. Domain Skill Extraction [PD-48b310f9](PD-48b310f9)

Extract domain knowledge from old agents into loadable skills:

| Source Agent | New Skill | Content |
| ------------- | ----------- | --------- |
| debugger | `diagnostic-methodology` | Root cause analysis, stack tracing |
| refactor-agent | `restructuring-methodology` | Safe refactoring steps, verification |
| security-engineer | `security-audit` | Audit checklist, threat model |
| agent-maintainer | `governance-maintenance` | Lesson promotion, rule updates |
| code-reviewer | `code-quality-review` | Zero-error policy, check sequence |
| qa-tester | `qa-verification` | E2E test approach, acceptance criteria |
| ux-reviewer | `ux-compliance-review` | UI spec comparison, accessibility |
| test-engineer | `test-engineering` | TDD workflow, coverage, mock boundaries |
| systems-architect | `architectural-evaluation` | Compliance checks, boundary verification |

### 3. Project Setup Skills [PD-26b0eb9f](PD-26b0eb9f)

Create the four setup skills that replace templates:

- `project-setup` — Universal scaffolding (creates .orqa/, installs canon)
- `project-inference` — Reads folder, produces project profile
- `project-migration` — Reads existing agentic config, maps to OrqaStudio
- `project-type-software` — Software development governance preset

### 4. Product Documentation Update

- Update `governance.md` with concept taxonomy from [PD-48b310f9](PD-48b310f9)
- Create `governance-hub.md` for the distribution/coexistence model from [PD-26b0eb9f](PD-26b0eb9f)
- Verify `artifact-framework.md` alignment with [PD-48b310f9](PD-48b310f9) concepts

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
| ------ | ------- | ------------ |
| [TASK-8c0c77b0](TASK-8c0c77b0) | Task Dependency Mechanism | — |
| [TASK-0a4a9172](TASK-0a4a9172) | Create universal agent definitions | [TASK-8c0c77b0](TASK-8c0c77b0) |
| [TASK-4023ac04](TASK-4023ac04) | Extract domain skills from old agents | [TASK-8c0c77b0](TASK-8c0c77b0), [TASK-0a4a9172](TASK-0a4a9172) |
| [TASK-8c4ca6b8](TASK-8c4ca6b8) | Remove old software-specific agents | [TASK-0a4a9172](TASK-0a4a9172), [TASK-4023ac04](TASK-4023ac04) |
| [TASK-bc351dc1](TASK-bc351dc1) | Create project setup skills | [TASK-8c0c77b0](TASK-8c0c77b0) |
| [TASK-39c5ac3f](TASK-39c5ac3f) | Update rules for universal roles | [TASK-8c4ca6b8](TASK-8c4ca6b8), [TASK-4023ac04](TASK-4023ac04) |
| [TASK-8a26a79b](TASK-8a26a79b) | Update product documentation | [TASK-8c4ca6b8](TASK-8c4ca6b8), [TASK-39c5ac3f](TASK-39c5ac3f) |

## Dependency Chain

```text
TASK-8c0c77b0 (dependency mechanism)
  ├── TASK-0a4a9172 (create new agents)
  │     └── TASK-4023ac04 (extract skills from old agents)
  │           └── TASK-8c4ca6b8 (remove old agents)
  │                 └── TASK-39c5ac3f (update rules)
  │                       └── TASK-8a26a79b (update product docs)
  └── TASK-bc351dc1 (setup skills — independent track)
```

## Implementation Design

Implementation approach to be defined during planning.
