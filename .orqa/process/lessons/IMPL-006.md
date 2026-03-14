---
id: IMPL-006
title: Symlinks Prevent Governance Divergence
description: |
  When .claude/ contains copies instead of symlinks to .orqa/, agents writing to .claude/ bypass the source of truth and the two directories silently diverge.
status: promoted
created: 2026-03-07
updated: 2026-03-07
maturity: understanding
recurrence: 1
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Single source of truth prevents structural divergence
  - target: IMPL-005
    type: informs
    rationale: Both address source-of-truth alignment
  - target: RULE-003
    type: grounded-by
    rationale: Lesson promoted to RULE-003
  - type: observed-by
    target: RULE-003
    rationale: RULE-003 codified the symlink architecture pattern first observed in this lesson
  - target: DOC-055
    type: documented-by
    rationale: Referenced in documentation page Lesson Dashboard UI Spec
---
## What Happened

Agents were writing governance artifacts (rules, agents, skills) directly to `.claude/` directories, which were separate copies of the `.orqa/` source of truth. Over multiple sessions, the two directories diverged — `.claude/` had newer rule content while `.orqa/` had the original versions. The CLI loaded from `.claude/` and the app scanned `.orqa/`, resulting in inconsistent governance enforcement.

## Why It Was Unexpected

The initial setup created `.claude/` as copies of `.orqa/` files. When agent delegation rules said "write to `.claude/rules/`", agents complied — but this bypassed the `.orqa/` source of truth. The divergence was invisible until a full audit revealed different file contents.

## The Correct Approach

`.claude/` should contain ONLY symlinks to `.orqa/` directories, plus `settings.json` and `worktrees/` as real files:

| Symlink | Target |
|---------|--------|
| `.claude/rules/` | → `.orqa/process/rules/` |
| `.claude/agents/` | → `.orqa/process/agents/` |
| `.claude/skills/` | → `.orqa/process/skills/` |
| `.claude/hooks/` | → `.orqa/process/hooks/` |
| `.claude/CLAUDE.md` | → `.orqa/process/agents/orchestrator.md` |

All writes go to `.orqa/`; symlinks ensure CLI reads the same content.

## Prevention

This lesson was promoted to [RULE-003](RULE-003) (`artifact-config-integrity`), specifically the `.claude/ Symlink Architecture` section, which forbids writing directly to `.claude/` and requires symlinks.

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
