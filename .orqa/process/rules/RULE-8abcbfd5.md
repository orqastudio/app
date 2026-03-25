---
id: RULE-8abcbfd5
type: rule
title: Provider-Agnostic Tool Capabilities
description: Agent definitions declare abstract capabilities. A provider mapping table resolves capabilities to concrete tool names per context.
status: active
created: 2026-03-11
updated: 2026-03-13
enforcement:
  - mechanism: behavioral
    message: "Agent definitions must declare capabilities not tool names; orchestrator resolves capabilities to concrete tool names per provider mapping at delegation time"
  - mechanism: json-schema
    description: "Governance steward verifies agent YAML uses capabilities field not tool names"
summary: "Agent definitions declare abstract capabilities (file_read, file_edit, shell_execute, etc.), not concrete tool names. Provider mapping tables resolve capabilities to tools per context: Claude Code CLI (PascalCase built-ins + MCP) or OrqaStudio App (Tauri commands + native ONNX). Orchestrator resolves at delegation time."
tier: always
roles: [orchestrator, implementer, researcher, planner, reviewer, writer, designer]
priority: P1
tags: [capabilities, provider-agnostic, tool-mapping]
relationships:
  - target: AD-02a2a97b
    type: enforces
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
| `code_search_regex` | `search_regex` | orqastudio MCP server |
| `code_search_semantic` | `search_semantic` | orqastudio MCP server |
| `code_research` | `search_research` | orqastudio MCP server |
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

Until the companion plugin (EPIC-9a1eba3f) automates this resolution, the orchestrator
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

- [RULE-dd5b69e6](RULE-dd5b69e6) (skill-enforcement) — skill loading complements capability resolution
- [RULE-87ba1b81](RULE-87ba1b81) (agent-delegation) — delegation protocol includes capability resolution
- [RULE-0d29fc91](RULE-0d29fc91) (code-search-usage) — code search capabilities and context detection
