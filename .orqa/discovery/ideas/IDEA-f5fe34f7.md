---
id: IDEA-f5fe34f7
type: discovery-idea
title: Unified Search Tool with Flexible Parameters
description: "Combine the app's three search tools (search_regex, search_semantic, code_research) into a single flexible Search tool that accepts different parameters to control search mode, scope, and output format."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: someday
research-needed:
  - "What parameter design gives the best agent ergonomics? (mode enum vs separate fields)"
  - "Should the unified tool auto-select mode based on query characteristics?"
  - "How does this affect the CLI vs App context resolution (currently two separate skill paths)?"
  - "What backward compatibility is needed for existing tool calls in skills and rules?"
  - "How does this relate to MCP tool naming conventions?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

Currently agents must choose between three separate tools (`search_regex`, `search_semantic`, `code_research`) and two context-dependent implementations (ChunkHound MCP for CLI, native ONNX+DuckDB for app). This creates friction:

1. Agents must decide which tool to use before searching — the `code-search` skill exists partly to teach this
2. Three tool names across two contexts means six possible tool identifiers
3. Simple searches that could benefit from both regex and semantic results require two separate calls

A unified `search` tool with a `mode` parameter (or auto-detection) simplifies the interface. One tool, one skill, flexible parameters. The tool internally routes to the appropriate engine based on mode and context.

## Sketch

```
search({
  query: "error handling in commands",
  mode: "semantic" | "regex" | "research" | "auto",
  scope: "backend/src-tauri/" | "ui/" | ".orqa/" | null,
  file_types: ["rs", "ts"] | null,
  limit: 20
})
```

- `mode: "auto"` detects whether the query looks like a pattern (regex) or a question (semantic/research)
- Single skill documents one tool instead of three
- Context resolution (CLI vs App) happens inside the tool, invisible to the agent
- Backward compatibility: keep the three individual tools as aliases during transition