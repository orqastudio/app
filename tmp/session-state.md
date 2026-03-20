## Session: 2026-03-20T19:10:00Z

### Scope
- Completed: EPIC-663d52ac (Skills→Knowledge Rename) — ALL 14 tasks done
- Created: EPIC-8d2e4f6a (Connector Architecture v2) — ready, tasks not yet populated
- AD-047d2e07 status: accepted
- AD-3f9a1c7b status: accepted
- Persona: Alex (Lead)
- Pillars served: PILLAR-001 (Clarity), PILLAR-003 (Continuity)

### What Was Done

**EPIC-098 (complete):**
- Phase 1: skill→knowledge rename across core.json, Rust, TypeScript, 1000+ artifact relationships, 8 plugin manifests, CLI, project.json, UI
- Phase 2: Connector stripped to pure symlinks + manifest + hooks, install-time setup via runPostInstallSetup(), delegation chain verified (9/9 domains pass)
- Phase 3: Docs updated (vision, artifact-framework, CLAUDE.md, 5 rules, plugin docs, connector README)
- Debug: MCP/LSP timing root cause identified and fixed
- Verification: 4 bugs caught and fixed (broken symlink, wrong source path, unreachable env var, stale loadSkillContent path)

**Post-epic work:**
- Renamed loadSkillContent→loadKnowledgeContent in connector source
- Deep architectural review of connector (agent defs, knowledge injection, enforcement, platform tools)
- ChunkHound reference audit: 117 references in 49 files catalogued (38 active files need cleanup)
- Created AD-061 and EPIC-099 for connector architecture v2

### EPIC-099 Task Plan (not yet created as .orqa/ artifacts)

**Phase 1 — Dev process infrastructure:**
1. Purge all ChunkHound references from active files (~38 files)
2. Add MCP server as managed process in dev controller
3. Add ONNX search engine initialization as dev controller concern
4. Add LSP server as managed process in dev controller
5. Verify MCP tools surface in Claude Code
6. Hook telemetry logging to dev controller output

**Phase 2 — Leverage existing capabilities:**
7. Replace validate-artifact.mjs with LSP/MCP calls
8. Expand INTENT_MAP to cover all 26+ knowledge files
9. Implement knowledge bundles
10. Add bash safety PostToolUse hook
11. Plugin specialist agent pattern (provides.agents)

**Phase 3 — Graph-first enforcement:**
12. Enforce graph_query before artifact actions
13. Add search_semantic to delegation flow
14. Capability-to-tool mapping knowledge artifact
15. Document hook execution semantics

### Key Architectural Decisions This Session
- Connector owns ZERO content files (manifest + hooks + skills only)
- Install-time setup replaces session-start generation
- CLAUDE.md is Claude Code artifact, not derived from orchestrator.md
- Core has generic agents, plugins provide specialists
- ChunkHound is dead — native ONNX search only
- MCP/ONNX/LSP should run as independent dev processes
- Knowledge bundles for coherent injection
- Hook telemetry to dev controller output

### Blockers
- None

### Next Session Priority
- Populate EPIC-099 tasks as .orqa/ artifacts
- Start Phase 1: ChunkHound purge + dev process separation
