---
id: RULE-031
title: Vision Alignment
description: Every feature must serve at least one active pillar defined in .orqa/process/pillars/.
status: active
created: 2026-03-07
updated: 2026-03-13
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Vision alignment ensures all features serve the structural pillars
  - target: RULE-004
    type: informs
    rationale: Pillar alignment is required at artifact creation and promotion gates
  - target: RULE-021
    type: informs
    rationale: Vision alignment for features extends to requiring pillar alignment sections in documentation
  - target: RULE-002
    type: informs
    rationale: Architecture decisions must implement foundational principles defined by the pillars
  - target: RULE-020
    type: informs
    rationale: Real implementations are required — fake demos do not serve any pillar
  - type: informed-by
    target: RULE-002
    rationale: Architecture decisions implement the foundational principles that pillars define
  - type: informed-by
    target: RULE-004
    rationale: Artifact lifecycle enforces pillar alignment at creation and promotion gates
  - type: informed-by
    target: RULE-008
    rationale: Documentation must verify pillar alignment before features are implemented
  - type: informed-by
    target: RULE-016
    rationale: Pillar alignment drives priority — ID sequence must not be mistaken for priority order
  - type: informed-by
    target: RULE-021
    rationale: Pillar alignment in documentation extends vision alignment to all feature docs
  - type: informed-by
    target: RULE-022
    rationale: Plans must serve active pillars — the architectural compliance section verifies this
  - target: IMPL-041
    type: observes
    rationale: Rule updated from lesson IMPL-041 (orchestrator should think critically about user suggestions)
  - type: grounded
    target: IMPL-041
    rationale: Lesson IMPL-041 extended the questioning requirement from principle violations to all design suggestions
  - type: enforces
    target: AD-027
    rationale: Requiring every feature to trace to active pillars enforces that the vision remains consistent — foundational principles are immutable without explicit user approval
  - type: enforces
    target: AD-031
    rationale: Reading pillars by artifact ID depends on pillars being first-class artifacts — this rule implements the pillar alignment requirement that AD-031 enables
  - target: IMPL-041
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-041
  - type: scoped-to
    target: AGENT-001
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-004
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-005
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-007
    rationale: Migrated from scope field
  - target: DOC-039
    type: documented-by
    rationale: governance.md defines the feature governance framework and pillar alignment criteria this rule enforces
---
Every feature, command, and UI element must serve the project's guiding principles. These principles are defined as **pillar artifacts** in `.orqa/process/pillars/`. Read the active pillars before implementing any new capability.

## Pillar Alignment Test

Every feature MUST trace to at least one active pillar. Pillars are structured artifacts with:

- **`title`** — The principle name
- **`description`** — What the pillar means
- **`gate`** — Questions to evaluate whether work serves this pillar

**Source of truth:** `.orqa/process/pillars/PILLAR-NNN.md` files. Do not hardcode pillar names or descriptions in rules, documentation, or agent instructions — always reference the pillar artifacts.

To evaluate a feature, read each active pillar's `gate` questions and check if the feature can answer "yes" to at least one question from at least one pillar.

## Feature Rejection Criteria

Reject any feature that:

- Does not serve any active pillar (cannot answer "yes" to any pillar's gate questions)
- Adds complexity without serving a pillar's intent
- Cannot articulate which pillar(s) it serves and how
- Is a generic tool feature with no connection to any pillar

## Questions Every Agent Should Ask

Before implementing any feature:

1. **Read active pillars** from `.orqa/process/pillars/`
2. **For each pillar**, evaluate the feature against its gate questions
3. **If no pillar is served**, the feature is out of scope — flag to the user and suggest an alternative that aligns

## Pillar Conflict Resolution

Pillars are equal in importance — there is no numeric priority ranking between them. When a task or feature appears to conflict with one pillar while serving another, the agent MUST flag the conflict to the user and ask for direction. The user decides how to resolve pillar tensions — agents do not prioritise one pillar over another unilaterally. Each pillar's body may describe its relationship to other pillars, but this is context for understanding, not a precedence rule.

## UX-First Design

**Build a system that enables the best user experience, not a user experience that fits the system.**

Every feature plan starts with user journeys and UI design. The backend is derived from what the frontend needs, not the other way around. Implementation success is measured by what the user can see and do.

This means:

- Define user journeys before backend architecture
- Design the ideal UI unconstrained by current backend capabilities
- Every component has complete state handling (loading, error, empty, loaded, saving) defined upfront
- User-facing language drives naming — no framework names, no technical jargon in the UI

UX-first does NOT mean ignoring architectural constraints. It means the UI defines the *requirements* that the architecture must satisfy.

## Foundational Principles Are Immutable (NON-NEGOTIABLE)

The following are **foundational principles** that can ONLY be changed with explicit user direction and approval:

- The pillar framework (active pillars defined in `.orqa/process/pillars/`)
- The project's technology stack as defined in architecture decisions
- The service boundary design (defined in architecture decisions)
- The UX-first design principle
- The documentation-first workflow
- Error propagation via result types (no silent failures in production)

**No agent may modify, weaken, or work around these principles without the user explicitly directing the change.** If an implementation seems to require violating a foundational principle, STOP and ask the user before proceeding.

## Questioning Misaligned Instructions (MANDATORY)

If the user gives an instruction that appears to conflict with a foundational principle, the agent MUST:

1. **Flag the conflict** — Clearly explain which principle the instruction would violate and why
2. **Ask for clarification** — The user may have a valid reason, or the instruction may be a misunderstanding
3. **Document the outcome** — If the user confirms a change to a foundational principle:
   - Update the relevant documentation
   - Update `.orqa/documentation/product/vision.md` and/or `.orqa/documentation/product/governance.md` if the pillars or governance rules change
   - Update this rule file ([RULE-031](RULE-031) (vision-alignment)) to reflect the new principle
   - Update all affected agent definitions in `.orqa/process/agents/`
4. **Never silently comply** — If an instruction contradicts a principle, do NOT just implement it without flagging the conflict first

## Related Rules

- [RULE-004](RULE-004) (artifact-lifecycle) — artifact creation, status transitions, promotion gates, documentation gates
- [RULE-021](RULE-021) (pillar-alignment-docs) — pillar alignment for *documentation* pages
- [RULE-002](RULE-002) (architecture-decisions) — architecture decisions that implement the vision
- [RULE-020](RULE-020) (no-stubs) — real implementations required, not fake demos

## Governance References

- Vision: `.orqa/documentation/product/vision.md`
- Governance: `.orqa/documentation/product/governance.md`
- Artifact Framework: `.orqa/documentation/product/artifact-framework.md`
- Artifact Workflow: `.orqa/documentation/process/artifact-workflow.md`
