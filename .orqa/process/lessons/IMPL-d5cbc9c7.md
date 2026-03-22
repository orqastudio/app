---
id: "IMPL-d5cbc9c7"
title: "Do not refactor agentic structure while delegating to agents"
description: "When modifying rules, skills, or agent definitions, the orchestrator must handle changes directly. Delegating to agents that read the same artifacts being modified causes stale-context conflicts.\n"
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
maturity: "understanding"
recurrence: 1
relationships: []
---
## What Happened

During the schema alignment sprint, agents were delegated tasks that modified the artifact schema, rules, and agent definitions — the same artifacts those agents read for instructions. Three background agents ran concurrently, each with stale context about the structure they were modifying. One agent wrote directly to `.claude/` instead of `.orqa/`, another referenced a plan field that was being removed, and coordination became impossible.

## Why It Was Unexpected

Agent delegation normally works well — isolated context, clear scope, parallel execution. But when the agents' own governing documents are the thing being changed, the isolation becomes a liability. Each agent starts with a snapshot of the old structure and produces output that may conflict with the new structure being established concurrently.

## The Correct Approach

When refactoring the agentic structure itself (rules, skills, agent definitions, artifact schemas):

1. **Do the work directly** — the orchestrator performs all changes, no delegation
2. **Add a temporary no-delegation instruction** to the orchestrator agent definition
3. **Use session state files** to maintain context across context window boundaries
4. **Remove the temporary instruction** once the alignment is verified complete

This is the meta-lesson: the system that coordinates work must be stable before it can coordinate work on itself.

## Prevention

Added to [RULE-6083347d](RULE-6083347d) (`dogfood-mode`) awareness. When `dogfood: true` and the work involves changing governance structure, the orchestrator must handle it directly.

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
