---
id: "IDEA-1287dd52"
title: "Plugin ecosystem: type system, provider integration, capability routing"
description: "Implement the plugin type taxonomy, AI provider schema, capability fulfilment model, and plugin installation wiring designed in RES-cd3d33bf. Covers plugin.json schema extension, .orqa/providers/ definitions, per-project capability routing, and load-time plugin filtering."
status: "surpassed"
created: "2026-03-13"
updated: "2026-03-13"
horizon: "later"
research-needed:
  - "Detailed plugin.json schema with type array, requires shape per type, default-capabilities"
  - "AI provider schema for .orqa/providers/<name>.json"
  - "Capability routing resolution implementation (project > plugin defaults > app baseline)"
  - "Plugin installation process: wiring capabilities, skills, agent updates"
  - "Load-time filtering implementation for non-matching plugins"
  - "App MCP server baseline capabilities manifest"
relationships:
  - type: "merged-into"
    target: "IDEA-c496b2de"
  - type: "realises"
    target: "EPIC-3f65c703"
  - target: "AD-c6abc8e6"
    type: "crystallises"
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-015e8c2c"
    type: "benefits"
---
> **Surpassed 2026-03-16**: Basic plugin ecosystem concept merged into EPIC-3f65c703 via AD-c6abc8e6. Provider-aware plugin system (provider detection, capability routing, load-time filtering) split into IDEA-c496b2de as a distinct capability layer needed for multi-model AI support.

## Motivation

[RES-cd3d33bf](RES-cd3d33bf) established the design decisions for plugin-provider pairing, capability fulfilment, and plugin type taxonomy. These decisions need implementation:

- Plugin type system (array of types, extendable enum, type-specific requires shape)
- AI provider definitions in `.orqa/providers/` (identity, detection, required plugins)
- Capability routing per-project with plugin-provided defaults and app baseline
- Plugin installation that wires capabilities, skills, and agent updates as a complete package
- Load-time filtering of non-matching plugins based on active AI provider
- Bidirectional provider-plugin relationship (providers declare required plugins, plugins declare supported providers)

This is the implementation companion to [IDEA-1c7ebe74](IDEA-1c7ebe74) (sidecar-as-plugin), which would eventually merge provider definitions into plugins themselves.
