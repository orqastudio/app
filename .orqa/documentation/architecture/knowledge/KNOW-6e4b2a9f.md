---
id: KNOW-6e4b2a9f
type: knowledge
status: active
title: "State Machine Design and Validation Timing"
description: "Workflow plugin state machine rules, state categories, guard design, and validation timing (write-time vs commit-time)"
tier: on-demand
created: 2026-03-29
roles: [implementer, reviewer, planner]
paths: [engine/workflow/, plugins/]
tags: [architecture, state-machine, guards, validation-timing, workflow]
relationships:
  - type: synchronised-with
    target: DOC-70063f55
---

# State Machine Design and Validation Timing

## State Machine Rules

Each workflow plugin owns its **complete** state machine for its artifact types. There is no inheritance — each plugin's state machine is self-contained.

- State machines are YAML, validated by the composed JSON schema
- The engine provides the evaluation engine and primitives; plugins provide the declarations
- No code is written for state machine logic — everything is declarative YAML (P4)

## State Categories

Every state must have a category:

| Category | Meaning |
| ---------- | --------- |
| `planning` | Artifact being created or defined, not yet in active work |
| `active` | Currently being worked on |
| `review` | Awaiting review or verification |
| `completed` | Work complete, all acceptance criteria met |
| `terminal` | Final state — archived, rejected, or cancelled |

## Guard Design

Guards are declarative prerequisites on state transitions. The engine evaluates them mechanically.

**Guard types:**

- **Field checks** — required frontmatter fields present and valid
- **Relationship checks** — artifact has required relationships established
- **Graph queries** — related artifacts are in the required states
- **AC verification** — acceptance criteria have been met (linked test exists and passes)

Guards should enforce acceptance criteria mechanically — not just check for green CI.

**Example:** A task transitioning to `completed` should verify:

- A test exists that defines the task's purpose
- That test passes
- The deliverable relationship is established (e.g., `delivers: EPIC-xxx`)

## Human Gates

Human gates are five-phase sub-workflows: GATHER, PRESENT, COLLECT, EXECUTE, LEARN

They represent decision points where a human must be involved. The engine manages the gate lifecycle; the workflow plugin declares where gates occur.

## Validation Timing

### Write Time (LSP)

The LSP server validates artifacts in real-time as you edit. It provides immediate feedback:

- Frontmatter correctness (required fields, valid types)
- Knowledge size constraints (500-2000 tokens)
- Schema compliance (valid statuses, valid relationship types, broken references)

Write-time validation catches issues before they are ever committed.

### Commit Time (Pre-Commit)

The pre-commit hook runs `orqa enforce --staged` — all registered enforcement engines against staged files only.

Pre-commit catches:

- Anything that slipped through write-time validation
- Cross-artifact consistency (relationship targets exist, inverses consistent)
- Lint and type checks (from installed enforcement plugins)
- Tests affected by staged changes (scoped, not full suite)
- Knowledge size constraints (500-2000 tokens)
- Status value validity

Pre-commit runs tests **scoped to what's changed** — not the full test suite.

## The Learning Loop Feeds Enforcement

```text
Lesson captured → "can we mechanically prevent this?" → if yes: create guard/test/validation rule/plugin-generated config
```

Every lesson is a candidate for a new mechanical guard. The enforcement tooling grows organically from real failures. The more mechanical enforcement exists, the more autonomy agents can have.

## Purpose-Specific Guards vs Generic CI

Guards should mechanically enforce acceptance criteria — not just "CI is green":

- A task implementing a function should verify **a test exists AND passes that defines the function's purpose**
- AC enforcement must trace from the task's deliverable to its verification
- Generic "all tests pass" guards are insufficient — they don't prove the right thing was done
