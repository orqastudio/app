---
id: TASK-0a8c26f6
type: task
name: "Verify P6 Hub-Spoke Orchestration"
status: active
description: "Verify that the orchestrator coordinates without implementing — workers write findings, orchestrator reads summaries"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 4 — Principle Verification"
  - target: TASK-c9fa5dd0
    type: depends-on
    rationale: "Phase 3 (knowledge structure verified) must complete before principle verification"
acceptance:
  - "Audit report with PASS/FAIL verdict for P6"
  - "CLAUDE.md hub-spoke directives verified present and correct"
  - "Agent role constraints verified — workers have bounded tool access"
---

## What

Verify Principle 6 from RES-d6e8ab11 section 2:

> **P6: Hub-Spoke Orchestration** — A persistent orchestrator coordinates ephemeral task-scoped workers. This is validated by Google's ADK research and Microsoft's AutoGen patterns. The orchestrator maintains high-level plan memory; workers maintain granular sub-task memory. The orchestrator never sees worker-level implementation details -- it reads structured summaries from findings files.

Confirm that the system enforces the hub-spoke pattern: orchestrator coordinates, workers implement, findings flow through files.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` section 2 (P6 definition)
- `CLAUDE.md` — orchestrator behavioral directives
- `.orqa/process/agents/` — agent definitions with role constraints
- `.claude/agents/` — generated agent files
- `libs/cli/src/lib/agent-spawner.ts` — agent spawning with role-based constraints
- `libs/daemon/src/` — any daemon enforcement of role boundaries

## Agent Role

Researcher — read-only audit producing a PASS/FAIL verdict with evidence.

## Steps

1. Read RES-d6e8ab11 section 2 to confirm the exact P6 principle text
2. Read `CLAUDE.md` and verify it contains hub-spoke directives:
   - Orchestrator coordinates, does not implement
   - Workers write findings to disk
   - Orchestrator reads structured summaries
   - Team infrastructure required (TeamCreate, TaskCreate, Agent, TaskUpdate, TeamDelete)
3. Review `.orqa/process/agents/` for role-based tool constraints (the table: Implementer can edit, Reviewer cannot, etc.)
4. Review `.claude/agents/` generated files for role boundary enforcement
5. Check if daemon enforces role boundaries (e.g., blocking an orchestrator from direct file edits)
6. Verify the findings file pattern: workers write to `.state/team/*/task-*.md`, orchestrator reads these
7. Document evidence for each aspect of P6
8. Produce a PASS/FAIL verdict

## Verification

- `grep -n "orchestrator.*coordinate\|hub-spoke\|does NOT implement\|delegate.*background" CLAUDE.md` — directives present
- `grep -rn "findings\|\.state/team" .orqa/process/agents/` — findings file pattern referenced
- Role-based tool constraints documented in agent definitions
- Workers write findings files, orchestrator reads summaries — pattern verified
