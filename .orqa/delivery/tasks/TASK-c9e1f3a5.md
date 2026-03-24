---
id: TASK-c9e1f3a5
type: task
name: "Migrate connector runConnectorSetup() to declarative capabilities"
status: completed
description: Replace the connector's bespoke runConnectorSetup() with declarative capability entries in its orqa-plugin.json manifest. The framework handles symlinks, MCP/LSP aggregation, and root file management.
relationships:
  - target: EPIC-d4a8c1e5
    type: delivers
    rationale: Phase 3 — universal plugin capabilities
  - target: TASK-b8d0e2f4
    type: depends-on
    rationale: Needs universal capability types and framework support first
acceptance:
  - "connectors/claude-code/orqa-plugin.json declares symlinks, aggregates, rootFiles"
  - "runConnectorSetup() is removed or reduced to minimal lifecycle callback"
  - "orqa plugin refresh produces the same .claude/ structure as before"
  - "orqa plugin refresh produces the same .mcp.json as before"
  - "make check passes"
---
