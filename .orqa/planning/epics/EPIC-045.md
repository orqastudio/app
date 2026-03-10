---
id: EPIC-045
title: Portable Governance Framework
status: done
priority: P1
milestone: MS-002
pillars: [PILLAR-001]
description: Restructure agents from 16 software-specific roles to 7 universal roles, extract domain knowledge into skills, create project setup skills, update product documentation to reflect the clarity engine identity and governance hub capability. Implements AD-029 and AD-030.
created: 2026-03-09
updated: 2026-03-09
research-refs: [.orqa/planning/research/universal-roles-ownership.md]
docs-required: [.orqa/governance/decisions/AD-029.md, .orqa/governance/decisions/AD-030.md, .orqa/documentation/product/vision.md]
docs-produced: [.orqa/documentation/product/governance.md, .orqa/documentation/product/governance-hub.md, .orqa/team/agents/researcher.md, .orqa/team/agents/planner.md, .orqa/team/agents/implementer.md, .orqa/team/agents/reviewer.md, .orqa/team/agents/writer.md]
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 5
  effort: 4
  risk: 3
  score: 18
---

## Context

AD-029 established that agents should represent universal roles (Orchestrator, Researcher,
Planner, Implementer, Reviewer, Writer, Designer) that work across any project type.
Domain-specific knowledge lives in skills loaded at runtime.

AD-030 established that project setup is skill-driven — no templates directory, no hardcoded
scaffolding. Setup skills contain the knowledge of what a project needs.

Both decisions support OrqaStudio's identity as a clarity engine for structured thinking,
not an AI development tool.

## Implementation Scope

### 1. Agent Restructuring (AD-029)

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

### 2. Domain Skill Extraction (AD-029)

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

### 3. Project Setup Skills (AD-030)

Create the four setup skills that replace templates:

- `project-setup` — Universal scaffolding (creates .orqa/, installs canon)
- `project-inference` — Reads folder, produces project profile
- `project-migration` — Reads existing agentic config, maps to OrqaStudio
- `project-type-software` — Software development governance preset

### 4. Product Documentation Update

- Update `governance.md` with concept taxonomy from AD-029
- Create `governance-hub.md` for the distribution/coexistence model from AD-030
- Verify `artifact-framework.md` alignment with AD-029 concepts

## Constraints

- **Orchestrator-only work** — This restructuring affects agent definitions directly.
  Launching agents that reference old/restructured definitions risks confusion.
  All tasks are executed by the orchestrator, not delegated.
- **No code changes** — This epic is entirely governance artifacts (.orqa/ files).
  No changes to src-tauri/, ui/, or sidecar/.
- **Backward compatible** — The orchestrator.md (which IS CLAUDE.md via symlink)
  must continue to work after restructuring. Agent references in rules must be
  updated to use new role names.
- **Update all cross-references** — Rules, skills, and other artifacts that reference
  old agent names must be updated in the same commit as the agent changes.

## Tasks

| Task | Title | Depends On |
|------|-------|------------|
| TASK-057 | Task Dependency Mechanism | — |
| TASK-051 | Create universal agent definitions | TASK-057 |
| TASK-053 | Extract domain skills from old agents | TASK-057, TASK-051 |
| TASK-052 | Remove old software-specific agents | TASK-051, TASK-053 |
| TASK-054 | Create project setup skills | TASK-057 |
| TASK-056 | Update rules for universal roles | TASK-052, TASK-053 |
| TASK-055 | Update product documentation | TASK-052, TASK-056 |

## Dependency Chain

```
TASK-057 (dependency mechanism)
  ├── TASK-051 (create new agents)
  │     └── TASK-053 (extract skills from old agents)
  │           └── TASK-052 (remove old agents)
  │                 └── TASK-056 (update rules)
  │                       └── TASK-055 (update product docs)
  └── TASK-054 (setup skills — independent track)
```
