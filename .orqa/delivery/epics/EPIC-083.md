---
id: EPIC-083
title: Dev tool integrations as installable plugins
description: >
  Convert development tool integrations (Claude Code plugin, git hooks,
  verification scripts) into installable OrqaStudio plugins so they can
  be managed through the plugin system rather than as submodules or
  hardcoded scripts.
status: captured
priority: P2
scoring:
  impact: 3
  urgency: 1
  complexity: 3
  dependencies: 1
created: 2026-03-16
updated: 2026-03-16
horizon: next
relationships:
  - target: EPIC-080
    type: depends-on
    rationale: Requires plugin discovery and runtime loading from EPIC-080
  - target: EPIC-082
    type: depends-on
    rationale: Requires schema-driven enforcement infrastructure
  - target: MS-002
    type: delivers
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-001
    type: grounded-by
---

## Objective

All development tool integrations should be packaged as installable plugins
managed through the OrqaStudio plugin system. This includes:

- **Claude Code integration** (currently `.orqa/plugins/orqastudio-claude-plugin` submodule)
  - Hooks for pre-commit validation
  - Skills for AI-assisted governance
  - Commands for artifact management
- **Git hooks** (currently `.githooks/` scripts)
  - Schema validation
  - Status transition validation
  - Artifact autolinking
- **Verification tools** (currently `tools/` scripts)
  - Link verification
  - Pipeline integrity checks
  - Enforcement rule audits

## Blocked By

- EPIC-080 Phase 2+: plugin discovery, runtime loading, enable/disable
- EPIC-082 Phase 3: schema-driven enforcement (so plugins can declare validation rules)

## Research Needed

- How do CLI-only plugins (hooks, scripts) differ from app plugins (views, widgets)?
- What's the plugin manifest structure for a tool integration vs a UI plugin?
- Can git hooks be managed through the plugin system or do they need a separate mechanism?
- How does a Claude Code plugin declare its capabilities to the OrqaStudio plugin registry?
