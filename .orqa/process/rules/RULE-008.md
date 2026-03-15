---
id: RULE-008
title: Documentation-First Implementation
description: Documentation is the source of truth. Verify docs exist before writing code; update docs before changing code.
status: active
created: 2026-03-07
updated: 2026-03-12
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Documentation-first ensures knowledge is captured before implementation
  - target: RULE-004
    type: informs
    rationale: Documentation gates on epics enforce the docs-required and docs-produced workflow
  - target: RULE-022
    type: informs
    rationale: Plans must read and comply with existing documentation before implementation
  - target: RULE-005
    type: informs
    rationale: Use code_research to discover existing documentation before implementing
  - target: RULE-031
    type: informs
    rationale: Documentation must be checked for pillar alignment before new features are implemented
  - target: RULE-002
    type: informs
    rationale: Architecture decisions are documentation and must be consulted before writing code
  - target: RULE-006
    type: informs
    rationale: Coding standards document is the source of truth that must be read before writing code
  - type: informed-by
    target: RULE-002
    rationale: Architecture decisions define the specific documentation agents must check before coding
  - type: informed-by
    target: RULE-003
    rationale: Config integrity requires documenting the config schema before implementing the scanner
  - type: informed-by
    target: RULE-004
    rationale: Artifact lifecycle gates require documentation to exist at specific transition points
  - type: informed-by
    target: RULE-011
    rationale: Enforcement before code extends documentation-first to include enforcement artifacts
  - type: informed-by
    target: RULE-014
    rationale: Historical preservation defines which docs are deleted (active state) vs preserved (research)
  - type: informed-by
    target: RULE-021
    rationale: Pillar alignment in docs requires every feature documentation page to serve a pillar
  - type: informed-by
    target: RULE-023
    rationale: Required reading formalises which documentation agents must load before starting work
  - type: informed-by
    target: RULE-027
    rationale: Structure before work requires artifact documentation to exist before implementation begins
  - target: IMPL-047
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-047
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
  - target: AGENT-008
    type: scoped-to
    rationale: "Auto-generated inverse of scoped-to relationship from AGENT-008"
---
## Principle

Documentation is the source of truth. Before writing ANY implementation code, verify that documentation exists for the feature area and read it. If documentation is missing or outdated, update the docs FIRST, get user approval, then implement.

## Before ANY Code Changes (MANDATORY)

1. Check `.orqa/documentation/` for existing designs related to the task
2. Check `.orqa/documentation/development/` for relevant architecture decisions
3. Check the relevant `EPIC-NNN.md` in `.orqa/delivery/epics/` for phase requirements and constraints
4. Verify the epic's `docs-required` gate is satisfied before starting implementation
5. Check `.orqa/documentation/about/artifact-framework.md` if working with `.orqa/` artifacts

**Documentation priorities ALWAYS come before implementation priorities.** Never reorder to put code changes ahead of documentation corrections.

**When an audit reveals gaps between docs and code:** Update the docs FIRST to define the correct target state. Code is then changed to match the docs. Never fix code without first verifying the docs describe the intended behavior.

## For New Features

1. Read `.orqa/documentation/about/vision.md` — verify feature serves at least one active pillar
2. Read `.orqa/documentation/about/governance.md` — verify feature passes governance criteria
3. Verify the feature has an artifact trail — an `IDEA-NNN` that was shaped and promoted to an `EPIC-NNN`, or an `EPIC-NNN` created directly with user approval
4. Verify the epic's `docs-required` gate is satisfied before starting implementation

## During Implementation

- Re-read governing docs at the START of every phase (even across sessions)
- If code diverges from docs, fix the code (not the docs)
- If an improvement is found, update docs first with justification, then code
- No silent deviations from approved documentation

## Documentation File Placement

- New documentation files MUST be placed in the folder matching their section
- Moving a page between sections requires `git mv` to the new folder

## No Deprecated Documentation (NON-NEGOTIABLE)

Documentation describes the **active target state only**. There is no audience for migration guides, deprecation notices, or historical redirects.

**Rules:**

- When a feature or concept is consolidated or removed, **delete its documentation page entirely**. Do not leave redirect stubs, "moved to" notices, or migration tables.
- Remove the deleted page from any sidebar or index in the same commit.
- Update any cross-references that linked to the deleted page.
- If content was merged into another page, only the destination page should exist.

**FORBIDDEN:**

- Pages whose only content is "This has moved to X" or "See Y instead"
- Migration reference tables showing old-to-new mappings
- Keeping deprecated pages "for reference" — git history serves that purpose

## Bug Investigation Protocol (MANDATORY)

When investigating bugs, UI mismatches, or differences between user expectations and implementation:

1. **Reproduce** — Understand what the user expects vs. what they see
2. **Find governing docs** — Locate documentation that defines the expected behavior
3. **Compare docs to code** — Determine whether docs match user expectations and whether code matches docs
4. **Report alignment** — Tell the user whether docs are right, code is right, or both need updating
5. **Ask for direction** — Present options: fix code to match docs, update docs + code, or keep current behavior

**NEVER jump straight to fixing code without checking documentation first.** The fix direction depends on whether the documentation matches user intent.

## Epic Documentation Gates

The artifact framework enforces documentation at two points:

- **`docs-required`** — documentation that must exist BEFORE implementation starts (epic `draft → ready` transition)
- **`docs-produced`** — documentation that this work MUST create or update on completion (verified at `review → done` transition)

These fields are defined on every epic. The orchestrator checks `docs-required` before starting work, and the code-reviewer verifies `docs-produced` during the review gate.

See [RULE-004](RULE-004) (artifact-lifecycle) for enforcement details and `.orqa/documentation/guide/artifact-workflow.md` for the full workflow.

## Related Rules

- [RULE-004](RULE-004) (artifact-lifecycle) — artifact creation, status transitions, documentation gates
- [RULE-022](RULE-022) (plan-mode-compliance) — plan structure requirements
- [RULE-005](RULE-005) (code-search-usage) — using code_research for documentation discovery
- [RULE-031](RULE-031) (vision-alignment) — pillar alignment and governance
- [RULE-002](RULE-002) (architecture-decisions) — architecture decision compliance
- [RULE-006](RULE-006) (coding-standards) — function size, typing, coverage requirements
