---
id: TASK-9f4a331e
type: task
name: "Fix content sync gaps — .md filter, injector config, blind overwrite"
status: archived
description: Fix three bugs in the current content sync — the hardcoded .md-only filter, injector config not regenerated on refresh, and blind overwrite on copy that clobbers user edits.
relationships:
  - target: EPIC-8b01ee51
    type: delivers
    rationale: Phase 1 — prerequisite fixes before three-way diff is useful
  - target: TASK-a3774911
    type: depends-on
    rationale: Blind overwrite fix depends on hash tracking being in place
acceptance:
  - "copyPluginContent() copies all files in content directories, not just .md"
  - "generateInjectorConfig() is called during orqa plugin refresh, not just orqa install"
  - "copyPluginContent() checks three-way state before overwriting — preserves user-modified files"
  - "make check passes"
---

## Scope

### Bug 1: .md-only filter

`content-lifecycle.ts` line ~120 filters on `.md` extension. Content directories may contain JSON schemas, config files, or other file types that plugins need to sync. Remove the hardcoded filter — copy all files in declared content directories.

### Bug 2: Injector config not regenerated on refresh

`generateInjectorConfig()` is called in `cmdBuildAll()` and `cmdLink()` but NOT in `cmdRefresh()`. A plugin refresh that adds new `behavioral_rules` or `session_reminders` won't update the injector config. Add the call to `cmdRefresh()`.

### Bug 3: Blind overwrite

`copyPluginContent()` always overwrites destinations. With hash tracking in place (TASK-a3774911), the copy should check three-way state first:

- Clean or plugin-updated: overwrite (safe)
- User-modified: skip and warn
- Conflict: skip and warn with both sides

### Key files

- `libs/cli/src/lib/content-lifecycle.ts` — `copyPluginContent()`
- `libs/cli/src/lib/injector-config.ts` — `generateInjectorConfig()`
- `libs/cli/src/commands/plugin.ts` — `cmdRefresh()`
