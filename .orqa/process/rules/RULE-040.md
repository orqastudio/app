---
id: RULE-040
title: Provider-Agnostic Tool Capabilities
description: Agent definitions declare abstract capabilities. A provider mapping table resolves capabilities to concrete tool names per context.
status: active
created: 2026-03-11
updated: 2026-03-13
layer: core
scope:
  - AGENT-001
  - AGENT-002
  - AGENT-003
  - AGENT-004
  - AGENT-005
  - AGENT-006
  - AGENT-007
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Provider-agnostic capabilities create a structured abstraction for tool resolution
  - target: RULE-026
    type: informs
    rationale: Skill loading complements capability resolution — both define what an agent brings to a task
  - target: RULE-001
    type: informs
    rationale: The delegation protocol includes resolving capabilities to concrete tools for each provider context
  - target: RULE-005
    type: informs
    rationale: Code search capabilities resolve to either ChunkHound MCP tools or native search tools depending on context
  - type: informed-by
    target: RULE-001
    rationale: Agent delegation defines the roles whose capabilities this rule maps to provider-specific tools
  - type: informed-by
    target: RULE-026
    rationale: The skill tier model defines which skills load per role, complementing the capability-to-tool mapping
  - type: informed-by
    target: RULE-037
    rationale: Role-based tool restrictions constrain which resolved capabilities each role may actually use
  - type: enforces
    target: AD-025
    rationale: Provider-agnostic capability declarations generalise the AI provider-agnosticism decision to all agent tooling — no agent binds to provider-specific tool names
  - target: IMPL-030
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-030
  - target: IMPL-032
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-032
  - target: IDEA-057
    type: informs
    rationale: All 4 research questions from IDEA-057 resolved directly into this rule
  - type: scoped-to
    target: AGENT-001
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-004
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-005
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-007
    rationale: Migrated from scope field
  - target: DOC-030
    type: documented-by
    rationale: Referenced in documentation page Orchestration
---
Agent definitions declare **capabilities** (what they can do), not **tools** (how they
do it). The mapping from capabilities to provider-specific tool names is defined in this
rule and resolved at delegation time by the orchestrator, companion plugin, or app.

## Capability Vocabulary

| Capability | Description |
|-----------|-------------|
| `file_read` | Read file contents |
| `file_write` | Create new files |
| `file_edit` | Edit existing files |
| `file_search` | Find files by name/pattern |
| `content_search` | Search file contents by pattern |
| `code_search_regex` | Exact pattern code search (semantic search engine) |
| `code_search_semantic` | Meaning-based code search (semantic search engine) |
| `code_research` | Architectural analysis (semantic search engine) |
| `shell_execute` | Run shell commands |
| `skill_load` | Load a skill into agent context |
| `web_fetch` | Fetch content from URLs |
| `web_search` | Search the web |
| `notebook_edit` | Edit Jupyter notebooks |

This vocabulary is extensible. New capabilities are added to this table when a new
tool type is introduced to any provider.

## Provider Mapping: Claude Code CLI

| Capability | Tool Name | Source |
|-----------|-----------|--------|
| `file_read` | `Read` | Built-in |
| `file_write` | `Write` | Built-in |
| `file_edit` | `Edit` | Built-in |
| `file_search` | `Glob` | Built-in |
| `content_search` | `Grep` | Built-in |
| `code_search_regex` | `mcp__chunkhound__search_regex` | MCP server |
| `code_search_semantic` | `mcp__chunkhound__search_semantic` | MCP server |
| `code_research` | `mcp__chunkhound__code_research` | MCP server |
| `shell_execute` | `Bash` | Built-in |
| `skill_load` | `Skill` | Built-in |
| `web_fetch` | `WebFetch` | Built-in |
| `web_search` | `WebSearch` | Built-in |
| `notebook_edit` | `NotebookEdit` | Built-in |

## Provider Mapping: OrqaStudio App

| Capability | Tool Name | Source |
|-----------|-----------|--------|
| `file_read` | `read` | Tauri command |
| `file_write` | `write` | Tauri command |
| `file_edit` | `edit` | Tauri command |
| `file_search` | `glob` | Tauri command |
| `content_search` | `grep` | Tauri command |
| `code_search_regex` | `search_regex` | Native ONNX+DuckDB |
| `code_search_semantic` | `search_semantic` | Native ONNX+DuckDB |
| `code_research` | `code_research` | Native ONNX+DuckDB |
| `shell_execute` | `bash` | Tauri command |
| `skill_load` | `load_skill` | Tauri command |
| `web_fetch` | `web_fetch` | Tauri command |
| `web_search` | `web_search` | Tauri command |
| `notebook_edit` | `notebook_edit` | Tauri command |

## Context Detection

| Signal | Context |
|--------|---------|
| `Read`, `Edit`, `Bash` tools available (PascalCase) | Claude Code CLI |
| `read`, `edit`, `bash` tools available (lowercase Tauri commands) | OrqaStudio App |

## Delegation Protocol

When the orchestrator delegates to an agent:

1. Read the agent's `capabilities` field from its definition
2. Determine the current context (CLI or App) using the detection signals above
3. Resolve each capability to the concrete tool name from the matching provider table
4. Include the resolved tool names in the delegation prompt
5. If a capability has no mapping in the current context (e.g., semantic search
   unavailable), note the gap explicitly in the delegation prompt

Until the companion plugin (EPIC-050) automates this resolution, the orchestrator
performs it manually in every delegation.

## Agent Definition Format

Agent definitions declare capabilities, not tools:

```yaml
capabilities:
  - file_read
  - file_edit
  - file_search
  - content_search
  - code_search_regex
  - code_search_semantic
  - code_research
  - shell_execute
```

The `tools` field is removed from agent definitions. All tool resolution goes
through this rule's mapping tables.

## Adding a New Provider

To support a new AI provider:

1. Add a new "Provider Mapping: [Provider Name]" section to this rule
2. Map each capability to the provider's tool name
3. Add a context detection signal
4. Update the orchestrator, plugin, or app to consume the new mapping

No agent definitions change. The capability vocabulary stays the same.

## FORBIDDEN

- Concrete tool names in agent `tools` fields (use `capabilities` instead)
- Hardcoding provider-specific tool names in delegation prompts without resolving
  from this mapping
- Adding a new tool to a provider without adding the corresponding capability to
  the vocabulary
- Assuming tool availability without checking the current context

## Related Rules

- [RULE-026](RULE-026) (skill-enforcement) — skill loading complements capability resolution
- [RULE-001](RULE-001) (agent-delegation) — delegation protocol includes capability resolution
- [RULE-005](RULE-005) (code-search-usage) — code search capabilities and context detection
