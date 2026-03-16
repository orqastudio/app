---
id: TASK-045
title: Hooks display in governance section
description: Fixes the hooks section of the governance panel showing empty by updating the scanner to surface hook files even when they are not markdown documents.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - Hooks section in governance displays existing hook files from .orqa/process/hooks/
  - If hooks directory contains shell scripts (not .md files)
  - they are still listed with their filename as label
  - Consider whether Claude hooks from .claude/settings.json should also surface here (may defer to IDEA-027)
relationships:
  - target: EPIC-043
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-005
    type: grounded-by
  - target: SKILL-008
    type: grounded-by
  - target: TASK-333
    type: depended-on-by
---
## Findings Addressed

- **F28**: Hooks section shows empty despite hook files existing

## Investigation Needed

The scanner in `artifact_reader.rs` scans for `.md` files. If `.orqa/process/hooks/` contains `.sh` files (shell scripts), they won't be found. Need to either:
1. Scan for all file types in hooks directory, or
2. Create `.md` wrapper files for each hook with frontmatter describing the hook, or
3. Defer hooks display to [IDEA-027](IDEA-027) (hooks system research)

Check what files actually exist in `.orqa/process/hooks/` first.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
