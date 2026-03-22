---
id: IDEA-2dfe18ae
title: "Tools as plugins — runtime tool registration"
status: captured
horizon: someday
created: 2026-03-16
relationships:
  - type: merged-from
    target: IDEA-68d4e688
  - target: PILLAR-94b281db
    type: grounded
  - target: PERSONA-a4b15450
    type: benefits
---

# IDEA-2dfe18ae: Tools as plugins — runtime tool registration

Extracted from IDEA-68d4e688 (Plugin-Based Tool Architecture).

Tools registered at runtime (not compile-time Rust functions). Support for WASM, MCP, and script-based tool formats. Capability-based permissions per tool (fs read, fs write, network, process). MCP protocol alignment so plugin tools work like external MCP servers. Each tool ships with companion skill documentation.

Architecturally distinct from artifact/view plugins. Represents the extensibility layer where plugins contribute executable capabilities, not just data types and UI.
