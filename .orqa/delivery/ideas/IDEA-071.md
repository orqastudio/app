---
id: IDEA-071
title: "Plugin ecosystem: type system, provider integration, capability routing"
description: "Implement the plugin type taxonomy, AI provider schema, capability fulfilment model, and plugin installation wiring designed in RES-052. Covers plugin.json schema extension, .orqa/providers/ definitions, per-project capability routing, and load-time plugin filtering."
status: captured
created: "2026-03-13"
updated: "2026-03-13"
pillars: [PILLAR-001, PILLAR-002]
research-needed:
  - "Detailed plugin.json schema with type array, requires shape per type, default-capabilities"
  - "AI provider schema for .orqa/providers/<name>.json"
  - "Capability routing resolution implementation (project > plugin defaults > app baseline)"
  - "Plugin installation process: wiring capabilities, skills, agent updates"
  - "Load-time filtering implementation for non-matching plugins"
  - "App MCP server baseline capabilities manifest"
promoted-to: null
---

## Motivation

[RES-052](RES-052) established the design decisions for plugin-provider pairing, capability fulfilment, and plugin type taxonomy. These decisions need implementation:

- Plugin type system (array of types, extendable enum, type-specific requires shape)
- AI provider definitions in `.orqa/providers/` (identity, detection, required plugins)
- Capability routing per-project with plugin-provided defaults and app baseline
- Plugin installation that wires capabilities, skills, and agent updates as a complete package
- Load-time filtering of non-matching plugins based on active AI provider
- Bidirectional provider-plugin relationship (providers declare required plugins, plugins declare supported providers)

This is the implementation companion to [IDEA-069](IDEA-069) (sidecar-as-plugin), which would eventually merge provider definitions into plugins themselves.
