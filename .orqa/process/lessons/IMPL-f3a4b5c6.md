---
id: IMPL-f3a4b5c6
type: lesson
title: Orchestrator must use TeamCreate + background agents for ALL delegated work
category: process
status: promoted
recurrence: 4
promoted-to: RULE-00a8c660
created: 2026-03-24
tags: [delegation, teams, orchestrator, agent-teams]
---

## Observation

The user has corrected the orchestrator at least 3 times for performing work inline (in the foreground) instead of delegating via TeamCreate + background Agent spawning. The orchestrator would read files, write governance artifacts, or run implementation tasks directly in the main conversation thread, blocking itself from responding to the user.

## Pattern

The orchestrator defaults to doing work inline because it is faster for small tasks. But this violates the core delegation principle: the orchestrator's primary job is conversation with the user, not waiting for file reads or artifact writes to complete. Even "quick" tasks add up and block the user from steering.

Specific recurrences:
1. Orchestrator wrote governance artifacts inline instead of spawning a Writer agent
2. Orchestrator performed research by reading files directly instead of spawning a Researcher agent in background
3. Orchestrator ran implementation-adjacent work (updating configs, creating artifacts) without using TeamCreate

## Root Cause

The Agent tool can run in foreground or background. Running in foreground is the path of least resistance — no team setup, no findings files, no async coordination. But foreground execution blocks the orchestrator from responding to user messages until the agent completes.

## Correct Behavior

1. **ALL delegated work** (implementation, review, research, documentation, governance artifact creation) MUST use `TeamCreate` to create a named team
2. **Agents MUST run with `run_in_background: true`** so the orchestrator stays available for conversation
3. **Findings MUST be written to disk** at `tmp/team/<team-name>/task-<id>.md` per RULE-d2e4f6a8
4. The orchestrator reads findings files when agents complete, then reports to the user

The only exceptions where the orchestrator may act directly:
- Quick file reads for planning decisions
- Writing `tmp/session-state.md`
- Artifact status transitions (frontmatter updates)
- Coordination messages to the user

## Promotion

Promoted to [RULE-00a8c660](RULE-00a8c660) which codifies the Agent Teams requirement. Also added to CLAUDE.md Safety section as a NON-NEGOTIABLE constraint. Memory entry `feedback_use_agent_teams.md` captures this for cross-session persistence.

## Mechanical Enforcement (2026-03-24)

Despite promotion to a rule, behavioral enforcement, AND a memory entry, the orchestrator continued to use bare Agent calls without TeamCreate (recurrence 4). Added mechanical enforcement via PreToolUse hook (`connectors/claude-code/hooks/scripts/enforce-background-agents.mjs`):

- Hook now checks for BOTH `run_in_background: true` AND `team_name` on every Agent tool call
- Missing `team_name` triggers a systemMessage warning citing RULE-00a8c660
- Warn-only (not blocking) to allow edge cases, but ensures the orchestrator sees the violation every time
