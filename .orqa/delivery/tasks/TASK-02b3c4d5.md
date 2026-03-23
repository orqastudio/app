---
id: TASK-02b3c4d5
type: task
name: "Execute monorepo merge — import all 30 repos"
status: todo
description: Run the validated merge script to import all 30 submodule repos into the monorepo with full history preserved. Remove .gitmodules and submodule references.
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 1 — monorepo consolidation
  - target: TASK-01a2b3c4
    type: depends-on
    rationale: Must validate the process first
acceptance:
  - "All 30 repos imported as directories with full commit history"
  - ".gitmodules removed"
  - "All submodule entries removed from .git/config"
  - "Root-level files (.orqa/, Makefile, scripts/, etc.) preserved"
  - "git log shows merged history from all repos"
---

## Scope

### Import order (dependencies first)

**Tier 1 — Leaf libraries (no internal deps):**
libs/types, libs/logger, libs/brand, libs/validation, libs/search

**Tier 2 — Libraries with deps on Tier 1:**
plugins/typescript, libs/cli, libs/sdk, libs/mcp-server, libs/lsp-server

**Tier 3 — Libraries with deps on Tier 2:**
libs/svelte-components, libs/graph-visualiser

**Tier 4 — App:**
app

**Tier 5 — Plugins, connectors, integrations (independent):**
All plugins/*, connectors/claude-code, integrations/claude-agent-sdk

**Tier 6 — Meta:**
registry/official, registry/community, templates, tools/debug, .github-org

### After import

- Remove .gitmodules
- Remove submodule entries from .git/config
- Clean up .git/modules/
- Verify no stale submodule references
