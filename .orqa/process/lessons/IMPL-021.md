---
id: "IMPL-97e2788f"
type: "lesson"
title: "Open items discovered during implementation are not tracked as tasks"
description: "When work reveals open items (dead code, cleanup needed, research questions, follow-up fixes), these get reported in conversation but are not formalized as tasks in the artifact system. When the session ends, they exist only in conversation history and Claude memory — both lossy stores. The epic gets marked done while untracked work remains."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

During [EPIC-942c7678](EPIC-942c7678) implementation, several items were identified but never formalized as tasks:

1. **[RES-c4dd4c4d](RES-c4dd4c4d)** (draft research) — behavioral directives in the pipeline. Research doc exists but no task tracks its completion.
2. **Claude memory cleanup** — 6 of 10 memory files duplicate artifact knowledge. No task to clean them up.
3. **ArtifactType::Hook dead code** — Hook is no longer a governance artifact. No task to remove the enum variant.
4. **Broken forward-references** — 9 broken-link errors to artifacts that don't exist yet (KNOW-bcfeb64e, AGENT-1dab5ebe/004/005, VER-001). No task to create these artifacts or remove the references.
5. **Stale source paths** — 31 broken paths discovered and fixed, but the fix work was not tracked as a task.
6. **Three new observations** (IMPL-c306b136, 019, 020) — systemic issues identified but not scoped into the epic.

These items were reported in conversation summaries and Claude memory, but neither of those is part of the artifact system. When the context window compacts or a new session starts, the items are at risk of being lost.

The root cause: the process has a **structure-before-work** rule ([RULE-8035e176](RULE-8035e176)) but no **structure-during-work** discipline. Items discovered mid-implementation fall through because the agent's attention is on completing the current task, not on creating new tasks for emergent work.

## Fix

Three mechanisms (user-approved via RES-cd3d33bf):
1. Every open item becomes a task immediately — no threshold, no batching
2. Epic completion requires human approval — orchestrator presents completed tasks, remaining todos, observations, and asks user
3. Auto-surface epics where all tasks are done but epic not yet verified by user — prevents limbo

## Triage

Promoted — observation triage protocol in [RULE-7b770593](RULE-7b770593) ensures open items are tracked and triaged.
