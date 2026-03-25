---
id: IDEA-d2b8077d
type: idea
title: Tool-Linked Skills — Documentation Per Tool with UI Integration
description: "Every tool in OrqaStudio should have a companion skill that documents how to use it effectively. The UI tool list should link directly to the tool's skill, making tool documentation discoverable and actionable."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: later
research-needed:
  - "How should tool-skill linkage be declared? (field in tool definition, field in skill, or both?)"
  - "Should skills be auto-generated from tool schemas or hand-authored?"
  - "How does the UI surface the link? (icon in tool list, expandable panel, separate tab?)"
  - "How does this interact with MCP tools that come from external servers?"
  - "What's the minimum viable skill for a tool? (parameters, examples, common patterns, anti-patterns)"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---
## Motivation

Tools are currently documented in scattered locations — some in skills, some in rules, some in architecture docs. When an agent encounters a tool, there's no direct path from "here's the tool" to "here's how to use it well." Users browsing the tool list in the app see tool names and parameter schemas but no usage guidance.

A tool-linked skill pattern solves both problems:

1. **For agents**: Every tool has a skill that loads usage patterns, common queries, anti-patterns, and examples. The skill IS the tool documentation.
2. **For users**: The app's tool list shows each tool with a link to its skill. Clicking opens the skill content — instant documentation.
3. **For the system**: Tool-skill linkage creates a feedback loop. When a tool's behavior changes, its skill is updated. When an agent misuses a tool, the skill is enhanced with the anti-pattern.

This extends the existing pattern (the `code-search` skill documents the search tools) into a universal convention for all tools.

## Sketch

- Each tool definition carries a `skill` field pointing to its companion skill
- Each skill's frontmatter carries a `tools` field listing the tools it documents
- The UI tool list renders a "docs" icon next to each tool that has a linked skill
- Clicking opens the skill content in the artifact panel
- For MCP tools from external servers, skills can be auto-scaffolded from the tool schema and then hand-enriched
- Minimum skill content: purpose, parameters explained, 3 example calls, common anti-patterns