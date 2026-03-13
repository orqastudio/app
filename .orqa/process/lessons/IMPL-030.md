---
id: IMPL-030
title: "Capability fulfilment should be a user choice between native provider tools and app MCP"
description: "The system requires a set of capabilities to function (file_read, file_edit, etc.). Whether those capabilities are fulfilled by the AI provider's native tools or by the app's built-in capabilities exposed via MCP should be a user choice per capability. This also allows new provider capabilities to be used before the app has native support."
status: active
created: "2026-03-13"
updated: "2026-03-13"
maturity: observation
recurrence: 1
relationships:
  - target: IMPL-028
    type: informed-by
    rationale: "Provider schema defines capabilities — this observation addresses HOW those capabilities are fulfilled"
  - target: RULE-040
    type: observes
    rationale: "Capability resolution currently assumes one fixed mapping per provider — this observation says the mapping should be configurable"
---

## Pattern

Currently [RULE-040](RULE-040) defines a fixed capability-to-tool mapping per provider. Claude Code CLI maps `file_read` → `Read` (native tool). But there's a choice the user should be able to make:

- **Native**: `file_read` → provider's built-in `Read` tool (fast, provider-controlled)
- **App MCP**: `file_read` → app's MCP server `orqa_read` (app-controlled, consistent behaviour, additional governance)

Use cases for choosing app MCP over native:
- Consistent behaviour across providers (same file_read regardless of whether using Claude or Cursor)
- App can add governance checks (e.g., prevent reading outside project scope)
- App can add instrumentation (track what files agents access)

Use cases for choosing native over app MCP:
- Speed — no MCP roundtrip
- New provider capabilities not yet supported by app MCP
- Provider-specific features (e.g., native tool has better error handling)

The provider schema should define available native tools. The app should expose its own capabilities via MCP. The user configures which source fulfils each capability — with sensible defaults.

## Fix

Not yet determined. The provider schema needs:
1. A `native-tools` section listing what the provider can do natively
2. The app needs an MCP server exposing its built-in capabilities
3. A configuration layer (per-project or per-provider) that maps each required capability to either native or app-MCP
4. Sensible defaults: use native when available, fall back to app MCP
