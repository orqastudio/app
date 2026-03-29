---
id: PD-c1e5a39e
type: principle-decision
title: "Three-Tier Skill Loading — Agent, Orchestrator, Wrapper"
description: "Skills are loaded at three tiers: portable skills on agent definitions, project-specific skills injected by the orchestrator at task creation, and wrapper skills that resolve context-dependent implementations at load time.\n"
status: completed
created: 2026-03-09
updated: 2026-03-13
relationships:
  - target: EPIC-42a5330b
    type: drives
---

## Decision

The skill loading system uses three tiers:

### Tier 1: Agent-Level (Portable)

Skills declared in the agent's YAML `skills:` frontmatter. These are domain and language skills that are true regardless of project or runtime context. They travel with the agent definition and work across any project.

**Examples:** `architecture`, `rust-async-patterns`, `svelte5-best-practices`, `typescript-advanced-types`, `tailwind-design-system`, `planning`

**Rule:** Agents MUST NOT declare project-specific or context-dependent skills in their frontmatter. If a skill mentions a specific project name, directory structure, or runtime context, it belongs in Tier 2 or Tier 3.

### Tier 2: Orchestrator-Injected (Project-Specific)

Skills injected by the orchestrator at task creation time based on the project and the task scope. The orchestrator knows which project patterns apply to the task and adds the relevant skills to the agent's context.

**Examples:** `orqa-ipc-patterns`, `orqa-store-patterns`, `orqa-domain-services`, `orqa-streaming`, `orqa-repository-pattern`, `orqa-error-composition`, `orqa-governance`, `orqa-testing`, `orqa-store-orchestration`

**Rule:** The orchestrator determines which project skills to inject based on the task's `scope` (which files/directories are being touched) and the agent's role. A frontend task gets store/component skills; a backend task gets IPC/domain/repo skills.

### Tier 3: Wrapper Skills (Context-Dependent)

Wrapper skills that resolve to a concrete implementation based on runtime context. The agent declares the wrapper; the wrapper detects the context and loads the right implementation.

**Examples:**

- `code-search` → resolves to `chunkhound` (CLI) or `orqa-native-search` (App)
- Future: `code-formatting` → could resolve to different formatters per context

**Rule:** Wrapper skills are declared on agent definitions (Tier 1 position) because they are portable — the agent just says "I need code search" without knowing the context. Resolution happens at load time.

## Skill Loading Sequence

```text
1. Agent starts task
2. Load Tier 1 skills (from agent YAML frontmatter) — portable skills + wrapper skills
3. Wrapper skills (Tier 3) detect context and load the correct implementation
4. Orchestrator injects Tier 2 skills based on project + task scope
5. Read Required Reading
6. Begin implementation
```

## Context Detection for Wrapper Skills

Wrapper skills detect their context using tool availability:

| Signal | Context | Resolution |
| -------- | --------- | ------------ |
| `mcp__chunkhound__*` tools available | CLI (Claude Code) | Load `chunkhound` |
| `search_regex` is a Tauri command | App (OrqaStudio) | Load `orqa-native-search` |
| Neither available | Unknown | Fall back to Grep/Glob, note in summary |

Future wrapper skills follow the same pattern: check what's available, load the matching implementation.

## Rationale

The previous approach put all skills on agent definitions, including project-specific and context-dependent ones. This caused:

1. **Bloated skill lists** — agents listed both `chunkhound` AND `orqa-native-search` even though only one applies at runtime
2. **Non-portable agents** — agent definitions were tied to OrqaStudio's specific patterns
3. **Context blindness** — agents couldn't know at definition time whether they'd run in CLI or App
4. **Orchestrator bypass** — project skills loaded without orchestrator awareness of task scope

The three-tier model separates concerns cleanly:

- Agent definitions are portable (Tier 1)
- Project knowledge is injected by the coordinator who has context (Tier 2)
- Runtime ambiguity is resolved by detection, not declaration (Tier 3)

## Consequences

- Agent YAML `skills:` lists become shorter — only portable + wrapper skills
- Orchestrator task creation must include a skill injection step
- Wrapper skills need a standard format for context detection
- Project skill injection tables live in the orchestrator's configuration, not in agent definitions
- The `skill-enforcement.md` rule must be updated to reflect the three tiers
- Agent definitions across projects become reusable — only Tier 2 skills change per project

## Migration

1. Create the `code-search` wrapper skill
2. Remove `chunkhound` and `orqa-native-search` from individual agent `skills:` lists
3. Add `code-search` to agent `skills:` lists
4. Move all `orqa-*` skills from agent frontmatter to the orchestrator's injection table
5. Update `skill-enforcement.md` to enforce the three-tier model
6. Update the orchestrator to inject Tier 2 skills at task creation

## Relationship to Three-Layer Architecture

The three skill tiers map to the three-layer governance architecture:

| Skill Tier | Governance Layer | Scope |
| ----------- | ----------------- | ------- |
| Tier 1 (Agent) | Canon | Ships with the agent, universal |
| Tier 2 (Orchestrator) | Project | Specific to this project's patterns |
| Tier 3 (Wrapper) | Plugin | Resolves based on runtime context |
