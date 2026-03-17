---
id: TASK-011
title: Backfill Historical Artifact Metadata
description: "Backfill historical artifact data so the decision history, lesson pipeline, and surpassed artifact workflows have real data to test against when we build the navigation and visualization tooling. Some reconstruction from session history is expected — the goal is representative data, not perfect accuracy. This ensures we're dogfooding the full artifact lifecycle, not just the happy path of \"create and leave as accepted."
status: completed
created: 2026-03-08
updated: 2026-03-08
assignee: AGENT-007
acceptance:
  - At least 3 decisions have surpassed predecessors (showing evolution chains)
  - At least 2 research docs marked as surpassed with surpassed-by references
  - At least 3 additional lessons exist capturing real implementation learnings from session history
  - At least 2 tasks marked as surpassed showing work that was overtaken
  - "All surpassed artifacts have status: surpassed and surpassed-by field set"
  - Decision chains are traceable (AD-NNN surpassed-by AD-MMM)
  - "Enough variety to test filtering, sorting, and chain navigation in future UI"
  - "Reference integrity verified: every task has a valid epic, every epic has a valid milestone"
  - "No orphaned references (plan:, promoted-to, depends-on all point to existing artifacts)"
relationships:
  - target: EPIC-033
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-003
    type: informed-by
  - target: SKILL-011
    type: informed-by
  - target: TASK-323
    type: depended-on-by
  - target: SKILL-003
    type: informed-by
  - target: SKILL-011
    type: informed-by
---
## What

The artifact system currently has 20 decisions, 3 lessons, and 0 surpassed artifacts. When we build tooling to navigate decision history and the lesson pipeline, we need real data that exercises:

1. **Decision evolution chains** — Decisions that were superseded by later thinking. E.g., early architecture decisions that were refined, technology choices that shifted, process approaches that evolved.

2. **Surpassed research** — Research that led to conclusions which were later overtaken. E.g., the claude-integration research is now surpassed by provider-architecture research.

3. **Lesson history** — Real implementation lessons from session history that capture the kinds of mistakes and discoveries that the lesson pipeline is designed to surface.

4. **Surpassed tasks** — Tasks from this session that were overtaken by the vision alignment refactor (e.g., tasks that assumed `.claude/` as source of truth).

## Approach

Review git history and session transcripts to reconstruct real decisions and pivots. Where exact details are unclear, reconstruct the spirit of what happened — the goal is representative data, not forensic accuracy.

### Decision Chains to Create

Examples of real evolution that happened:

- **Scanner approach**: Started with folder-guessing heuristics → moved to config-driven scanning (AD-NNN surpassed by new AD)
- **Source of truth location**: `.claude/` as canonical → `.orqa/` as canonical with `.claude/` symlinks (existing AD may need a predecessor)
- **Navigation architecture**: Hardcoded constants → config-driven derivation

### Research to Mark Surpassed

- `claude-integration.md` → surpassed by `provider-architecture.md` (provider-agnostic framing)

### Lessons to Add

Real lessons from recent sessions:
- `$derived(() => ...)` vs `$derived.by(() => ...)` — caused app hang
- Config paths must match disk paths — caused empty artifact views
- Symlinks vs copies for `.claude/` — caused governance divergence

### Tasks to Mark Surpassed

- Any tasks from earlier plans that assumed `.claude/` as source of truth

## Why This Matters

Without historical data, the decision history UI will only show a flat list of accepted decisions with no evolution story. The lesson pipeline will have 3 entries. The surpassed status will be untested. We need enough variety to validate that the tooling works for the real use case: tracing how thinking evolved over time.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
