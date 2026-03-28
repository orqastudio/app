---
id: TASK-d39e416a
type: task
name: "Migrate connector runConnectorSetup() to declarative capabilities"
status: archived
description: Replace the connector's bespoke runConnectorSetup() with declarative capability entries in its orqa-plugin.json manifest. The framework handles symlinks, MCP/LSP aggregation, and root file management.
relationships:
  - target: EPIC-8b01ee51
    type: delivers
    rationale: Phase 3 — universal plugin capabilities
  - target: TASK-2d03d1a3
    type: depends-on
    rationale: Needs universal capability types and framework support first
acceptance:
  - "connectors/claude-code/orqa-plugin.json declares symlinks, aggregates, rootFiles"
  - "runConnectorSetup() is removed or reduced to minimal lifecycle callback"
  - "orqa plugin refresh produces the same .claude/ structure as before"
  - "orqa plugin refresh produces the same .mcp.json as before"
  - "make check passes"
---
