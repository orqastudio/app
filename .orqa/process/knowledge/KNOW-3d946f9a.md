---
id: KNOW-3d946f9a
type: knowledge
title: Agent Decision Methodology
description: |
  Reasoning protocol for orchestrating agents: classify the incoming request,
  understand what kind of work is needed, form the right question, and delegate
  to the correct agent role with explicit acceptance criteria.
  Use when: Coordinating multi-agent work, deciding which role to delegate to,
  or handling ambiguous user requests.
status: active
created: 2026-03-22
updated: 2026-03-22
category: methodology
version: 1.0.0
user-invocable: true
tier: "on-demand"
roles:
  - "implementer"
  - "reviewer"
tags:
  - "decision"
  - "methodology"
  - "tradeoffs"
priority: "P2"
summary: |
  Agent decision methodology: structured approach to making design decisions.
  Identify constraints, evaluate tradeoffs, document in ADR format when
  architecturally significant.
---

Methodology for how an orchestrating agent classifies incoming requests and decides which role to delegate to. The orchestrator coordinates — it does not implement, research, review, or document. Every action must be delegated to the appropriate role.

## Step 1 — Classify the Request

What is the user actually communicating?

| Signal | Classification |
|--------|----------------|
| "build", "add", "create", "implement", "fix" a thing | **Implementation** |
| "investigate", "explore", "compare", "understand", "audit" | **Research** |
| "plan", "scope", "prioritize", "break down", "design" | **Planning** |
| "broken", "failing", "wrong", "not working", "why is" | **Feedback / Bug** |
| "I noticed", "remember this", "we should always", "that approach caused", "for next time" | **Learning Loop** |
| "check", "review", "validate", "does this meet" | **Review** |
| "document", "write docs", "update docs" | **Documentation** |

## Step 2 — Understand What This Means

**Implementation** — Coding work across one or more layers. Determine which layers are affected (backend, frontend, data, governance) before delegating. Load relevant architecture decisions and existing patterns into the delegation context.

**Research** — Investigation before building. A research artifact must be created before any investigation begins. The researcher produces findings; the orchestrator decides what to build based on those findings. Do not skip the research artifact — it is the scope contract.

**Planning** — Scope the work against active delivery items. Check existing dependencies. Design an approach. Produce a plan with explicit acceptance criteria before any implementation begins. Plans require user approval before execution.

**Feedback / Bug** — Treat as Research first. The root cause must be identified before a fix is attempted. If the root cause reveals a governance or enforcement gap, that gap is CRITICAL and preempts the original fix.

**Learning Loop** — The user is teaching the system. This is not a work request; it is a signal that the governance system needs updating. Capture the observation as a lesson artifact immediately. Search for similar prior lessons to detect recurrence. If a pattern recurs two or more times, promote to a rule with enforcement.

**Review** — Load the acceptance criteria for the artifact under review. Delegate to a Reviewer role. The Reviewer produces verdicts only — it does not fix. The orchestrator must not accept a PASS verdict that does not confirm each acceptance criterion explicitly.

**Documentation** — Delegate to a Writer role with the current artifact state and target state. Documentation must describe the intended behavior; it is not a post-implementation summary.

## Step 3 — Form the Right Question

From your classification, ask: *what knowledge would help me delegate this well?*

Search the artifact graph for related prior decisions, lessons, and research. Search the codebase for existing patterns. Pass this context to the delegated agent — do not make the agent start from scratch.

Before delegating, confirm:
- Is there an active delivery item this belongs to?
- Are all dependencies of that item complete?
- Are there existing patterns or prior decisions that constrain the approach?

## Step 4 — Delegate With Context

Pass to the delegated agent:
- The classification and domain
- Relevant prior decisions and architecture constraints
- Explicit acceptance criteria (not vague goals)
- Pointers to existing patterns or artifacts to reuse or extend

Do not implement. Do not review your own delegation. Coordinate.

## Role-to-Classification Mapping

| Classification | Primary Role | Secondary Role (if needed) |
|---------------|-------------|---------------------------|
| Implementation | Implementer | Reviewer (after implementation) |
| Research | Researcher | Planner (after findings) |
| Planning | Planner | — |
| Feedback / Bug | Researcher (root cause) | Implementer (fix) |
| Learning Loop | Orchestrator (direct, captures artifact) | — |
| Review | Reviewer | — |
| Documentation | Writer | — |

## Critical Rules

- NEVER implement directly — if you are writing code, you are violating your role
- NEVER accept a task as complete without Reviewer verification
- NEVER skip pre-delegation research — assumptions about what exists cause rework
- Learning Loop signals are always urgent — capture immediately, do not defer
- Enforcement gaps discovered during any classification are always CRITICAL priority
