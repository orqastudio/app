---
id: TASK-148
title: Replace OrqaStudio-specific examples with generic ones in composability skill
description: Replace OrqaStudio-specific file paths and code examples in the composability skill with generic equivalents that illustrate the same composability principles without tying them to this codebase.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - No OrqaStudio-specific file paths in examples
  - No OrqaStudio module/component names in examples
  - Composability principles unchanged
  - Examples still clearly illustrate each principle (small units, pure functions, swappable parts)
  - Skill remains layer canon
---

## What

The `composability` skill is `layer: canon` (portable across all projects) but currently contains OrqaStudio-specific file paths and code examples. The composability principles are universal and essential for plugin development — the examples need to be generic.

Replace OrqaStudio-specific references (file paths, module names, component names) with generic examples that illustrate the same composability principles without tying them to this codebase.

## How

1. Open `composability.md` in `.orqa/process/skills/`
2. Identify every code block that references OrqaStudio-specific paths (e.g., `backend/src-tauri/src/domain/enforcement_engine.rs`) or module names
3. For each such block: rewrite using generic names (e.g., `src/domain/engine.rs`, `MyService`, `UserStore`) that convey the same structural lesson
4. Verify every composability principle section (Pure Over Stateful, Small Composable Units, Trait Boundaries, etc.) still has a clear illustrative example after the rename
5. Confirm `layer: canon` is still set in frontmatter

## Verification

- [ ] No OrqaStudio-specific file paths in examples
- [ ] No OrqaStudio module/component names in examples
- [ ] Composability principles unchanged
- [ ] Examples still clearly illustrate each principle (small units, pure functions, swappable parts)
- [ ] Skill remains layer canon
