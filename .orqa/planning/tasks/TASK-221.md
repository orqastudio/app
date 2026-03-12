---
id: TASK-221
title: Pre-commit hook blocks core graph artifact modifications
description: Add a pre-commit check that warns and blocks commits modifying core graph artifacts (schemas, orchestrator, core skills, role definitions) unless force-overridden. Dogfood mode bypasses.
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on:
  - TASK-211
docs: []
skills:
  - SKILL-011
acceptance:
  - Committing changes to schema.json in any artifact directory is blocked with warning
  - Committing changes to orchestrator.md is blocked with warning
  - Committing changes to core skill SKILL.md files is blocked with warning
  - Setting ORQA_CORE_OVERRIDE=1 allows the commit
  - Dogfood projects skip the check entirely
  - Warning message explains that changing core artifacts risks breaking the system thinking structure
---
## What

Core graph artifacts are firmware — they define how the artifact graph works, how
agents traverse it, and how the thinking process is structured. Modifying them in a
non-dogfood project risks breaking the system for that project.

The pre-commit hook is the last line of defence. The app UI will also enforce
read-only on these files, but the git hook catches changes made outside the app
(direct file edits, CLI agents, etc.).

## Warning Message

```
⚠ CORE GRAPH ARTIFACTS MODIFIED

The following files define OrqaStudio's thinking structure:
  [list of modified core files]

Changing these risks breaking graph traversal and context injection
for this project. Core artifacts are updated through OrqaStudio releases.

To proceed anyway: ORQA_CORE_OVERRIDE=1 git commit ...
```

## Core Artifact Paths

```
.orqa/planning/*/schema.json
.orqa/governance/*/schema.json
.orqa/team/agents/schema.json
.orqa/team/skills/schema.json
.orqa/team/agents/orchestrator.md
.orqa/team/skills/composability/SKILL.md
.orqa/team/skills/research-methodology/SKILL.md
.orqa/team/skills/planning/SKILL.md
.orqa/team/skills/orqa-code-search/SKILL.md
```

This list is maintained in the pre-commit hook itself. Adding new core artifacts
requires updating this list.

## How

1. Add a core artifact check section to `.githooks/pre-commit`
2. List the core artifact paths in the script
3. On commit, check if any staged files match core paths
4. If matched and `project.json` has `dogfood: true`, skip the check
5. If matched and `ORQA_CORE_OVERRIDE=1` is set, skip the check
6. Otherwise, block with the warning message

## Verification

- Staging a schema.json change in a non-dogfood project triggers the warning
- Setting `ORQA_CORE_OVERRIDE=1` allows the commit
- Dogfood projects bypass the check entirely
- Non-core files commit normally
