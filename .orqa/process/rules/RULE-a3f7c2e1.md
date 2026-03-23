---
id: "RULE-a3f7c2e1"
title: "Architecture Decisions"
description: "Architecture decisions are first-class governance artifacts. Every significant technical choice must be recorded with status, rationale, alternatives considered, and relationships to the rules and delivery work it drives."
status: "active"
created: "2026-03-22"
updated: "2026-03-22"
enforcement:
  - mechanism: behavioral
    message: "Orchestrator reads architecture decisions before delegating; every significant technical choice must be recorded as a decision artifact; plans must include an Architectural Compliance section"
---
Architecture decisions are first-class governance artifacts. Every significant technical choice must be recorded as a decision artifact with status, rationale, alternatives considered, and relationships to the rules and delivery work it drives. Decisions are the bridge between discovery (research, ideas) and governance (rules, enforcement).

## What Belongs in a Decision Artifact

- The decision itself (what was chosen)
- The status (proposed, accepted, superseded)
- The rationale (why this choice over alternatives)
- Alternatives considered (what was rejected and why)
- Consequences (what this constrains, what it enables)
- Relationships to rules, epics, and other decisions

## Before Writing Code

1. Check if your change affects any existing decision — search the decisions directory for relevant artifacts
2. Read the relevant decision artifact(s) for full context
3. If proposing a new decision, create a decision artifact following the project's artifact schema

## Before Writing Plans

1. Start with user journeys and the desired outcome
2. Include an architectural compliance section verifying all relevant decisions
3. Flag any conflict between the proposed approach and existing decisions before proceeding

## Decision Lifecycle

- Decisions begin as `proposed`
- Accepted decisions move to `accepted`
- When a decision is superseded, the old artifact is updated to `superseded` and linked to the new one via `evolves-into` / `evolves-from`
- Superseded decisions are historical reference — they are not deleted

## FORBIDDEN

- Making a significant technical choice without a decision artifact
- Treating decisions as immutable — they evolve; use `evolves-into` to track that evolution
- Deleting superseded decisions instead of marking them superseded
