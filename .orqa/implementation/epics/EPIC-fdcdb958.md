---
id: "EPIC-fdcdb958"
type: epic
title: "Skills→Knowledge rename and connector architecture alignment"
description: "Rename OrqaStudio's \"skills\" concept to \"knowledge\" to resolve naming collision with Claude Code's \"skills\" (slash commands). Establish the connector as a bridge that reads from canonical sources — no copies, no forks, no staleness. Document the semantic distinction in core artifacts."
status: ready
created: 2026-03-20T00:00:00.000Z
updated: 2026-03-20T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
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
5. **CLI**: update `orqa enforce`, `orqa graph`, ID generation
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
- [ ] All `SKILL-*` IDs migrated to `KNOW-*` with `orqa enforce` passing
- [ ] `core.json` has `knowledge` artifact type, not `skill`
- [ ] Connector reads knowledge from plugin paths — no copies in connector directory
- [ ] Vision.md and core docs explain the semantic distinction
- [ ] `orqa enforce schema` passes on all project.json and plugin manifests
- [ ] Only 5 user-facing slash commands registered in Claude Code
- [ ] Delegation injects correct knowledge files into subagent prompts

## Risks

- ID migration touches 1000+ relationships across the artifact graph
- Must coordinate across app, types lib, SDK, all plugins, CLI, and connector
- Breaking change for any external tooling that references `SKILL-*` IDs
