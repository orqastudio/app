---
id: IMPL-a3b5c7d9
type: lesson
title: Agent team findings must be written to disk before task completion
category: process
status: promoted
recurrence: 1
promoted-to: RULE-d2e4f6a8
created: 2026-03-23
tags: [dogfooding, agent-teams, honest-reporting]
---

## Observation

Agent team message delivery is unreliable when the team lead is mid-turn. A teammate can mark a task complete but the lead never receives the findings report. We don't control Claude agent teams internals, so we can't fix the delivery mechanism.

## Pattern: Findings to Disk

Every teammate MUST write their findings to `tmp/team/<team-name>/<task-id>.md` BEFORE marking the task complete via TaskUpdate. The team lead reads the file to verify completion. Message delivery becomes a notification, not the evidence.

```
tmp/team/post-migration-fixes/task-5.md   ← findings written by teammate
```

The file IS the evidence. TaskUpdate(completed) is just the signal that the file is ready. If the signal is lost, the lead can poll the directory.

## Delegation Template Addition

When spawning teammates or assigning tasks, include:
> Write your findings to `tmp/team/<team-name>/task-<id>.md` before marking the task complete.

## Affected Rules

- RULE-878e5422 (honest-reporting) — completion without evidence
- RULE-9cd980b1 (honest-status-reporting) — status must be verifiable
