---
id: IMPL-031
title: Capability configuration is per-project with plugin-provided defaults
description: "Capability routing (native vs app-MCP) is configured per-project because plugin selection is per-project. Provider integration plugins should ship sane defaults for their capability mappings. Plugins can also ADD capabilities to the system that the app doesn't natively support, extending the capability vocabulary when a provider offers something new."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships:
  - target: IMPL-030
    type: informed-by
    rationale: "Capability fulfilment model needs a configuration layer — this decides where it lives and how defaults work"
  - target: IMPL-027
    type: informed-by
    rationale: "Plugin type system — ai-provider-integration plugins are the source of capability defaults"
  - target: IMPL-032
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-032"
---
## Pattern

Three layers of capability configuration:

1. **App baseline** — the capabilities the app can fulfil via its own MCP server. This is the floor.
2. **Plugin defaults** — when an ai-provider-integration plugin is installed, it declares default mappings for its capabilities (e.g., "use native Read for file_read"). It can also ADD new capabilities the app doesn't have (e.g., a provider supports `notebook_edit` natively before the app does).
3. **Project config** — the user can override any mapping per-project. "In this project, route file_edit through app MCP for governance."

Resolution order: project config > plugin defaults > app baseline.

This means:
- A new provider plugin can work immediately by providing its own defaults
- The app doesn't need to know about every provider's capabilities upfront
- Users retain control over routing decisions per-project
- New capabilities from providers extend the system vocabulary rather than being blocked by it

## Fix

The capability system needs:
1. App baseline capabilities defined somewhere (app-native MCP server manifest)
2. Plugin default capabilities in plugin config (`default-capabilities` section)
3. Project-level overrides in `project.json` (`capability-routing` section or similar)
4. A resolution function that merges all three layers
