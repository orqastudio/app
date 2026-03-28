---
id: IDEA-cbbe02f7
type: discovery-idea
title: Session tasklist — track pending artifact creation in working memory
description: "A session-level scratchpad that tracks things the agent knows it needs to create (tasks, ideas, observations) but hasn't done yet. Prevents knowledge from getting lost during extended conversations when context compacts."
status: archived
created: 2026-03-13
updated: 2026-03-13
horizon: next
research-needed:
  - "What belongs in the session tasklist — pending artifact creation, outstanding epic tasks, or both?"
  - "How should it be surfaced — system prompt injection, sidebar panel, persistent UI element?"
  - "Should items auto-clear when the artifact is created, or require manual dismissal?"
  - "How does this interact with context compaction — does it survive as injected context?"
  - "CLI equivalent — is .state/session-state.md sufficient or does it need structure?"
  - "Should this hook into the provider's built-in task/memory capability (e.g., Claude Code tasks) rather than reimplementing?"
  - "In-memory task lists as a capability — how does this fit the capability model?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

## Motivation

During extended sessions, the agent accumulates knowledge that "I need to create a task for X" or "that should be logged as an observation" but hasn't done it yet. When context compacts, this knowledge is lost. The user then has to remind the agent of things it already knew it needed to do.

A session tasklist would be a persistent, visible scratchpad of pending artifact creation work — ensuring nothing gets lost between the moment the agent recognises the need and the moment it acts on it. This is distinct from the epic's task list (which tracks planned work) — this tracks the *meta-work* of maintaining the artifact graph itself.
