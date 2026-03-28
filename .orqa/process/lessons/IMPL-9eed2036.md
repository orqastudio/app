---
id: IMPL-9eed2036
type: lesson
title: Agent team findings must be written to disk before task completion
category: process
status: promoted
recurrence: 1
promoted-to: RULE-04684a16
created: 2026-03-23
tags: [dogfooding, agent-teams, honest-reporting]
---

## Observation

Agent team message delivery is unreliable when the team lead is mid-turn. A teammate can mark a task complete but the lead never receives the findings report. We don't control Claude agent teams internals, so we can't fix the delivery mechanism.

## Pattern: Findings to Disk

Every teammate MUST write their findings to `.state/team/<team-name>/<task-id>.md` BEFORE marking the task complete via TaskUpdate. The team lead reads the file to verify completion. Message delivery becomes a notification, not the evidence.

```text
.state/team/post-migration-fixes/task-5.md   ← findings written by teammate
```text

The file IS the evidence. TaskUpdate(completed) is just the signal that the file is ready. If the signal is lost, the lead can poll the directory.

## Delegation Template Addition

When spawning teammates or assigning tasks, include:
> Write your findings to `.state/team/<team-name>/task-<id>.md` before marking the task complete.

## Affected Rules

- RULE-5dd9decd (honest-reporting) — completion without evidence
- RULE-d543d759 (honest-status-reporting) — status must be verifiable
