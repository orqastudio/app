---
id: IMPL-021
title: Open items discovered during implementation are not tracked as tasks
description: "When work reveals open items (dead code, cleanup needed, research questions, follow-up fixes), these get reported in conversation but are not formalized as tasks in the artifact system. When the session ends, they exist only in conversation history and Claude memory — both lossy stores. The epic gets marked done while untracked work remains."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: RULE-027
    type: informs
    rationale: "Structure-before-work says artifacts must exist before implementation — this lesson reveals the inverse gap: work products that emerge DURING implementation also need structure"
  - target: RULE-015
    type: informs
    rationale: "Honest reporting — marking an epic done while open items exist in conversation but not in the task system is a form of incomplete reporting"
  - target: RULE-004
    type: informs
    rationale: "Artifact lifecycle — the epic completion gate should verify no untracked open items remain"
  - target: RULE-004
    type: informed-by
    rationale: "Lesson promoted to RULE-004 — observation triage protocol added to artifact lifecycle"
  - target: RULE-004
    type: informed-by
    rationale: "RULE-004 codified the open-item tracking and triage protocol first observed in this lesson"
  - target: IMPL-022
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-022"
  - target: IMPL-023
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-023"
---
## Pattern

During [EPIC-059](EPIC-059) implementation, several items were identified but never formalized as tasks:

1. **[RES-051](RES-051)** (draft research) — behavioral directives in the pipeline. Research doc exists but no task tracks its completion.
2. **Claude memory cleanup** — 6 of 10 memory files duplicate artifact knowledge. No task to clean them up.
3. **ArtifactType::Hook dead code** — Hook is no longer a governance artifact. No task to remove the enum variant.
4. **Broken forward-references** — 9 broken-link errors to artifacts that don't exist yet (SKILL-045, AGENT-003/004/005, VER-001). No task to create these artifacts or remove the references.
5. **Stale source paths** — 31 broken paths discovered and fixed, but the fix work was not tracked as a task.
6. **Three new observations** (IMPL-018, 019, 020) — systemic issues identified but not scoped into the epic.

These items were reported in conversation summaries and Claude memory, but neither of those is part of the artifact system. When the context window compacts or a new session starts, the items are at risk of being lost.

The root cause: the process has a **structure-before-work** rule ([RULE-027](RULE-027)) but no **structure-during-work** discipline. Items discovered mid-implementation fall through because the agent's attention is on completing the current task, not on creating new tasks for emergent work.

## Fix

Three mechanisms (user-approved via RES-052):
1. Every open item becomes a task immediately — no threshold, no batching
2. Epic completion requires human approval — orchestrator presents completed tasks, remaining todos, observations, and asks user
3. Auto-surface epics where all tasks are done but epic not yet verified by user — prevents limbo

## Triage

Promoted — observation triage protocol in [RULE-004](RULE-004) ensures open items are tracked and triaged.
