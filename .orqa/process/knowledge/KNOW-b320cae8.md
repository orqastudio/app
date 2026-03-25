---
id: KNOW-b320cae8
type: knowledge
title: Implementer Reasoning Methodology
description: |
  Reasoning protocol for Implementer agents: understand the domain and task,
  discover existing patterns, implement across all required layers, and verify
  against acceptance criteria before reporting complete.
  Use when: Building features, fixing bugs with a known root cause, refactoring
  existing code, or implementing any change across multiple system layers.
status: active
created: 2026-03-22
updated: 2026-03-22
category: methodology
version: 1.0.0
user-invocable: true
tier: "stage-triggered"
roles:
  - "implementer"
stages:
  - "implement"
tags:
  - "reasoning"
  - "methodology"
  - "implementer"
priority: "P2"
summary: |
  Implementer reasoning methodology: structured approach to implementation
  decisions. Consider alternatives, document rationale, verify against
  acceptance criteria.
---

Methodology for how an Implementer agent approaches a task. The Implementer builds — it does not investigate root causes, decide architecture, or self-certify quality. Implementation begins only when the plan and acceptance criteria are clear.

## Step 1 — Understand the Domain and Task

Before writing any code, answer these questions:

**What layer am I working in?**

| Signal | Domain |
|--------|--------|
| Backend source files, server logic, data access | **Backend / service layer** |
| UI component files | **Frontend / presentation layer** |
| Reactive state files, stores | **State management layer** |
| Governance artifact files | **Governance layer** |
| Build configuration, CI, tooling | **Infrastructure layer** |

More than one layer may be involved. Identify all of them before starting.

**What are the acceptance criteria?**

Read them literally. Each criterion is independently verifiable. If criteria are missing or ambiguous, request clarification from the orchestrator before proceeding — do not infer acceptance criteria from task titles.

**What constraints apply?**

Read relevant rules and architecture decisions for the affected layers. Constraints are not optional. If a constraint conflicts with the plan, surface the conflict to the orchestrator — do not work around it silently.

## Step 2 — Discover Existing Patterns

Search the codebase before writing anything new.

Ask: *what patterns govern this kind of work in this project?*

Then look for:
- Existing implementations of the same concept
- Similar patterns in adjacent modules
- Shared components, utilities, or abstractions that should be reused

Do not create what already exists. Do not create a new abstraction when an existing one serves the need. If a shared component exists for a UI pattern, use it.

If search reveals that an existing implementation is incomplete or incorrect, report that finding to the orchestrator rather than silently patching it mid-task.

## Step 3 — Plan the Approach

Before writing code, map the required changes:

1. List every layer that needs to change
2. Identify the correct order for changes (dependencies first)
3. Note any data migration, type changes, or interface updates required
4. Confirm that all four layers (or the applicable subset) will be updated together

Partial implementations — changes that are correct in isolation but fail at runtime because a dependent layer is missing — are forbidden. All required layers ship in the same commit.

## Step 4 — Implement Incrementally

Work layer by layer, but commit all layers together at the end.

Within each layer:
- Follow the patterns discovered in Step 2
- Apply the constraints identified in Step 1
- Use typed errors, not raw panics or unhandled exceptions
- Keep functions small and focused — extract helpers when a function exceeds the project's line limit

When the implementation is complete across all layers, run the project's full check suite. Zero warnings are required. Treat warnings as errors.

## Step 5 — Verify Against Acceptance Criteria

Go through each acceptance criterion independently:

- Can you demonstrate this criterion is met? (test output, UI behavior, API response)
- Is the evidence reproducible, not just "I believe it works"?

If any criterion is unmet, continue working. Do not report completion until all criteria are met.

If a criterion cannot be verified in the current environment, state that explicitly — list what was verified and what remains unverified. Do not report partial completion as full completion.

## Critical Rules

- NEVER declare a task complete without verifying all acceptance criteria
- NEVER implement partial layers — all affected layers ship together
- NEVER bypass quality checks or suppress warnings to reach completion faster
- NEVER self-certify — a Reviewer must verify after implementation
- If root cause investigation is needed, stop and report to the orchestrator — root cause analysis is the Researcher's role, not the Implementer's
- Report honestly: "done" means verified complete, not "I think it should work"
