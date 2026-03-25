---
id: AD-bb125c2c
type: decision
title: "Skills→Knowledge rename and connector architecture"
status: active
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: EPIC-fdcdb958
    type: drives
---

# AD-060: Skills→Knowledge Rename and Connector Architecture

## Context

OrqaStudio uses the term "skill" to mean domain knowledge injected into agents (coding patterns, architecture rules, testing standards). Claude Code uses "skill" to mean a user-invocable slash command. When OrqaStudio's connector plugin registered all 48 agent knowledge files as Claude Code skills, every one became a `/orqastudio:*` slash command — overwhelming the user with commands they'd never type.

This naming collision revealed a deeper architectural question: what IS the connector, and how does it map OrqaStudio's governance model to Claude Code's plugin system without forking or duplicating?

## Decision

### 1. Rename "skills" to "knowledge" in the OrqaStudio platform

OrqaStudio's agent domain files are renamed from "skills" to "knowledge" everywhere:

| Before | After |
|--------|-------|
| `.orqa/process/skills/` | `.orqa/process/knowledge/` |
| `SKILL-XXXXXXXX` IDs | `KNOW-XXXXXXXX` IDs |
| Artifact type `skill` | Artifact type `knowledge` |
| Plugin `provides.schemas` skill entries | Plugin `provides.schemas` knowledge entries |
| `core.json` skill artifact type | `core.json` knowledge artifact type |
| UI navigation "Skills" | UI navigation "Knowledge" |

The term "skill" is reserved exclusively for Claude Code slash commands — user-invocable actions in the connector plugin's `skills/` directory.

### 2. Semantic distinction (documented in core artifacts)

| Term | Context | What It Is | Who Uses It |
|------|---------|-----------|-------------|
| **Knowledge** | OrqaStudio platform | Domain expertise files injected into agents for implementation context | Agents (via orchestrator delegation) |
| **Skill** | Claude Code plugin | User-invocable slash command (`/orqastudio:search`) | Humans using Claude Code |

Knowledge files teach agents HOW to do something. Skills let humans ASK for something. Knowledge is context; skills are commands.

### 3. Connector is a bridge, not a fork

The Claude Code connector maps OrqaStudio concepts to Claude Code concepts:

| OrqaStudio | Claude Code | Connector Maps Via |
|-----------|-------------|-------------------|
| Agents (`.orqa/process/agents/`) | `.claude/agents/` | Symlinks |
| Rules (`.orqa/process/rules/`) | `.claude/rules/` | Symlinks |
| Orchestrator agent | `.claude/CLAUDE.md` | Symlink |
| Knowledge files | Agent prompts | Orchestrator reads + injects at delegation time |
| User-facing commands | `skills/` directory | 5 curated slash commands |
| Governance hooks | `hooks/hooks.json` | Plugin hooks |

**The connector NEVER maintains its own copies of agents, rules, or knowledge.** It reads from canonical sources:
- Agents: from the app's `.orqa/process/agents/` (symlinked)
- Knowledge: from installed plugins' directories (resolved via `project.json` plugin paths)
- Rules: from `.orqa/process/rules/` (symlinked)

### 4. Knowledge resolution path

When the orchestrator delegates a task, it resolves knowledge from installed plugin directories:

```
project.json plugins.<name>.path → <path>/knowledge/<file>.md → injected into agent prompt
```

No copies. No sync scripts. The plugin owns its knowledge files. The orchestrator reads them at delegation time.

### 5. Connector-specific agent adaptations

The connector's `agents/orchestrator.md` is the ONE file that IS connector-specific (not a symlink to the app). It contains Claude Code-specific instructions: how to use the Agent tool, how to inject knowledge into subagents, how to use MCP tools. This is not a fork — it's a CLI-specific adaptation of the orchestrator role.

All other agents (implementer, planner, reviewer, etc.) are symlinked from the app's canonical definitions.

## Impact

- **Rust**: `ProjectSettings`, `ArtifactEntry`, scanner, `core.json` references
- **TypeScript**: types lib, SDK, UI navigation, all `skill` references
- **Plugins**: all 8 plugins' `orqa-plugin.json` and knowledge file paths
- **CLI**: `orqa enforce`, graph commands, skill references
- **Governance**: all `SKILL-*` artifact IDs, relationship targets, rule references
- **.orqa/**: directory rename `process/skills/` → `process/knowledge/`

## Risks

- ID migration (`SKILL-*` → `KNOW-*`) affects 1000+ relationship targets across the artifact graph
- Must be atomic — partial rename leaves the graph in an inconsistent state
- `orqa enforce --fix` should handle the ID migration

## Alternatives Considered

1. **Keep "skills" everywhere, qualify in docs** — rejected: the naming collision is the root cause, not the symptom
2. **Rename only in the connector** — rejected: creates two vocabularies for the same concept
3. **Use "expertise" instead of "knowledge"** — considered but "knowledge" is shorter and clearer