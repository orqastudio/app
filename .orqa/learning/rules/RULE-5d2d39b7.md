---
id: RULE-5d2d39b7
type: rule
title: Completion Gate Before New Work
description: "The orchestrator MUST NOT start new epics, phases, or work batches when the current batch has unresolved follow-up items. A completion gate triggers before every new team or phase."
status: active
enforcement_type: mechanical
created: 2026-03-24
updated: 2026-03-24
enforcement:

  - engine: hook

    type: PostToolUse
    event: team_create
    action: check
    message: "Before creating a new team, verify all follow-up items from the current batch are resolved"

  - engine: behavioral

    message: "The orchestrator must run the completion gate checklist before starting any new epic, phase, or work batch"

  - engine: hook

    type: Stop
    event: session_end
    action: check
    message: "Flag any unresolved follow-up items in session state before session end"
summary: "Orchestrator must not start new epics/teams/phases with unresolved follow-ups from current batch. Completion gate checklist: task statuses, follow-up items from findings files, dead file cleanup, dependency cleanup, binary rebuild, stale agents, session state freshness. PostToolUse hook on TeamCreate enforces."
tier: always
roles: [orchestrator]
priority: P0
tags: [completion-gate, follow-ups, batch-transitions]
relationships:

  - target: RULE-8ee65d73

    type: extends

  - target: RULE-8aadfd6c

    type: related

  - target: RULE-d543d759

    type: extends

  - target: RULE-99abcea1

    type: constrains
---

The orchestrator MUST NOT create a new team, start a new epic, or begin a new phase of work while follow-up items from the current batch remain unresolved. This is a hard gate, not a suggestion.

## Why This Exists

Follow-up items are the most common source of silent scope loss. When an agent reports findings — dead files to remove, dependencies to clean, statuses to update, binaries to rebuild — those items are real work. But the orchestrator's natural momentum is forward: create the next team, start the next epic. Follow-ups get noted, not done. The user catches them hours later.

This rule makes the transition between work batches a checkpoint where outstanding items must be resolved before new work begins.

## The Completion Gate (NON-NEGOTIABLE)

Before ANY of the following actions, the orchestrator MUST run the completion gate checklist:

- Creating a new team (`TeamCreate`)
- Starting a new epic or phase
- Switching focus to a different body of work
- Ending a session

### Checklist

| Check | What to Verify | How |
| --- | --- | --- |
| **Task statuses** | All tasks from the current batch have accurate statuses in `.orqa/implementation/tasks/` | Read each task file, verify status matches reality |
| **Follow-up items** | All follow-ups reported in agent findings files (`.state/team/*/task-*.md`) are resolved | Read findings files, check each follow-up item |
| **Dead file cleanup** | Files marked for deletion during the batch are actually deleted | `git status` to verify no stale files remain |
| **Dependency cleanup** | Dependencies marked for removal are actually removed from manifests | Check `package.json` / `Cargo.toml` for stale entries |
| **Binary rebuild** | If code was changed, affected binaries are rebuilt and running the latest code | Verify build output timestamps or run rebuild |
| **Stale agents** | No agents from the previous batch are still running | `TeamDelete` the previous team before `TeamCreate` |
| **Session state** | `.state/session-state.md` reflects the current state, not the state before the batch | Read and verify freshness |

### Gate Protocol

1. **Read all findings files** from the current team: `.state/team/<team-name>/task-*.md`
2. **Extract follow-up items** — any item marked as remaining, deferred, or needing attention
3. **For each follow-up item**, either:
   - Resolve it immediately (delete file, remove dep, update status, rebuild)
   - Create a task for it in the current batch and complete it before moving on
   - Surface it to the user for an explicit scope decision (the user may approve deferral)
4. **Verify the checklist** — every item above must pass
5. **Only then** proceed with `TeamCreate` or the next phase

## Follow-Up Items Are First-Class Work

When an agent reports follow-ups in their findings, those are not afterthoughts. They are work items that must be completed before the batch is done.

| Agent Reports | Orchestrator Action |
| --- | --- |
| "Dead file X still on disk" | Delete the file before moving on |
| "Dependency Y should be removed" | Remove it from the manifest and rebuild |
| "Task status not updated" | Update the task status in `.orqa/implementation/tasks/` |
| "Binary needs rebuild" | Run the rebuild command |
| "Documentation needs update" | Delegate to a Writer or do it if trivial |

"Will do later" is not a valid resolution. Either do it now, or get the user's explicit approval to defer it.

## Enforcement

Three enforcement layers:

1. **PostToolUse hook on TeamCreate** — when the orchestrator calls `TeamCreate`, the hook checks `.state/session-state.md` for a `### Completion Gate` section. If the section is missing or contains unchecked items (`[ ]`), the hook emits a `systemMessage` warning: "Completion gate not satisfied — resolve outstanding items before creating a new team." The hook is non-blocking (warn) to avoid breaking emergency workflows, but the warning is directive.

2. **Session state convention** — the orchestrator MUST maintain a `### Completion Gate` section in `.state/session-state.md` during active work. This section lists all follow-up items with checkbox status. The section must be empty (all items checked) before new work starts.

3. **Stop hook integration** — the existing stop-checklist hook checks for unresolved follow-up items in session state and flags them as warnings before session end. Unresolved items at session end are logged to `.state/hook-metrics.json`.

### Session State Format

```markdown
### Completion Gate
- [x] Task statuses updated (TASK-abc, TASK-def)
- [x] ChainTrace.svelte deleted
- [x] gray-matter removed from package.json
- [ ] Daemon rebuilt with latest code    <-- BLOCKS new work
```text

## FORBIDDEN

- Creating a new team while the previous team's follow-up items are unresolved
- Starting a new epic or phase without running the completion gate checklist
- Treating follow-up items as optional or "nice to have"
- Marking follow-up items as resolved without actually resolving them
- Moving to new work and saying "I'll clean up the previous batch later"
- Silently dropping follow-up items by not reading agent findings files
- Ending a session with unresolved follow-up items without flagging them to the user
- Using `TeamDelete` to discard unread findings — always read findings before deleting the team

## Examples

### Correct Flow

```text
1. Team "feature-batch" completes
2. Orchestrator reads all findings files in .state/team/feature-batch/
3. Findings report: "gray-matter still in deps, ChainTrace.svelte not deleted"
4. Orchestrator resolves both items (removes dep, deletes file)
5. Orchestrator updates session state: all completion gate items checked
6. Orchestrator runs TeamDelete on "feature-batch"
7. Orchestrator runs TeamCreate for "next-batch"
```text

### Incorrect Flow (FORBIDDEN)

```text
1. Team "feature-batch" completes
2. Orchestrator skips reading findings files
3. Orchestrator runs TeamDelete on "feature-batch"
4. Orchestrator runs TeamCreate for "next-batch"  <-- VIOLATION
5. gray-matter and ChainTrace.svelte silently remain
```text

## Related Rules

- [RULE-8ee65d73](RULE-8ee65d73) (no-deferred-deliverables) — this is the deliverable-level version; RULE-5d2d39b7 is the batch-level version that catches operational follow-ups between work batches
- [RULE-8aadfd6c](RULE-8aadfd6c) (session-state-management) — session state tracks the completion gate checklist; the gate depends on session state being current
- [RULE-d543d759](RULE-d543d759) (honest-status-reporting) — moving to new work with unresolved follow-ups is a form of dishonest status reporting
- [RULE-99abcea1](RULE-99abcea1) (agent-teams) — TeamCreate is the trigger point for the completion gate; this rule constrains when TeamCreate may be called
- [RULE-04684a16](RULE-04684a16) (findings-to-disk) — agent findings files are the source of follow-up items that the completion gate checks
- [RULE-f609242f](RULE-f609242f) (git-workflow) — dead file cleanup and dependency removal are git-tracked operations governed by this rule
