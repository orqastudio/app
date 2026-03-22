---
id: EPIC-663d52ac
title: "Skills→Knowledge rename and connector architecture alignment"
description: Rename OrqaStudio's "skills" concept to "knowledge" to resolve naming collision with Claude Code's "skills" (slash commands). Establish the connector as a bridge that reads from canonical sources — no copies, no forks, no staleness. Document the semantic distinction in core artifacts.
status: ready
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: MS-654badde
    type: fulfils
  - target: AD-047d2e07
    type: driven-by
  - target: TASK-a1b2c3d4
    type: delivered-by
  - target: TASK-e5f6a7b8
    type: delivered-by
  - target: TASK-c9d0e1f2
    type: delivered-by
  - target: TASK-3a4b5c6d
    type: delivered-by
  - target: TASK-7e8f9a0b
    type: delivered-by
  - target: TASK-1c2d3e4f
    type: delivered-by
  - target: TASK-5a6b7c8d
    type: delivered-by
  - target: TASK-9e0fa1b2
    type: delivered-by
  - target: TASK-c3d4e5f6
    type: delivered-by
  - target: TASK-a7b8c9d0
    type: delivered-by
  - target: TASK-e1f2a3b4
    type: delivered-by
  - target: TASK-c5d6e7f8
    type: delivered-by
  - target: TASK-a9b0c1d2
    type: delivered-by
  - target: TASK-e3f4a5b6
    type: delivered-by
---

# EPIC-098: Skills→Knowledge Rename and Connector Architecture Alignment

## Problem

OrqaStudio and Claude Code both use the term "skill" to mean different things. OrqaStudio skills are domain knowledge files injected into agents. Claude Code skills are user-invocable slash commands. When the connector registered all 48 knowledge files as Claude Code skills, the UX became unusable. This naming collision also revealed that the connector was diverging from the app's canonical agent definitions — copying files instead of reading from source.

## Design

See AD-060 for the full architecture decision.

### Phase 1: Knowledge rename (app-wide)

1. **Directory rename**: `.orqa/process/skills/` → `.orqa/process/knowledge/` in all projects
2. **ID migration**: `SKILL-XXXXXXXX` → `KNOW-XXXXXXXX` across all artifacts and relationships
3. **Type system**: update `core.json`, Rust types, TypeScript types, plugin manifests
4. **UI**: update navigation labels, artifact browser, icons
5. **CLI**: update `orqa validate`, `orqa graph`, ID generation
6. **Documentation**: update vision.md, CLAUDE.md, artifact-framework docs to explain the semantic distinction

### Phase 2: Connector architecture

1. **Remove knowledge copies** from connector — read from plugin paths at delegation time
2. **Verify symlink model** — agents and rules symlinked, orchestrator.md is connector-specific
3. **Knowledge resolution** — orchestrator reads from `project.json` plugin paths
4. **Test delegation** — verify knowledge injection into subagents works end-to-end
5. **Document** — core docs explain knowledge vs skills distinction

### Phase 3: Documentation

1. Update `vision.md` — explain knowledge (domain context) vs skills (commands)
2. Update `artifact-framework.md` — document knowledge artifact type
3. Update plugin development docs — plugins provide `knowledge/` not `skills/`
4. Update connector README — explain bridge model

## Acceptance Criteria

- [ ] No directory named `skills/` exists in `.orqa/process/` anywhere
- [ ] All `SKILL-*` IDs migrated to `KNOW-*` with `orqa validate` passing
- [ ] `core.json` has `knowledge` artifact type, not `skill`
- [ ] Connector reads knowledge from plugin paths — no copies in connector directory
- [ ] Vision.md and core docs explain the semantic distinction
- [ ] `orqa validate schema` passes on all project.json and plugin manifests
- [ ] Only 5 user-facing slash commands registered in Claude Code
- [ ] Delegation injects correct knowledge files into subagent prompts

## Risks

- ID migration touches 1000+ relationships across the artifact graph
- Must coordinate across app, types lib, SDK, all plugins, CLI, and connector
- Breaking change for any external tooling that references `SKILL-*` IDs
