---
id: EPIC-040
title: Provider Abstraction Layer
description: Refactor sidecar from Claude-specific to provider-agnostic architecture. Provider interface, Claude adapter, sdk_session_id → provider_session_id rename across 13+ files including SQLite migration.
status: done
priority: P1
created: 2026-03-07
updated: 2026-03-09
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - provider-architecture
  - provider-abstraction
docs-required: []
docs-produced:
  - .orqa/governance/decisions/AD-025.md
scoring:
  user-value: 4
  pillar-alignment: 4
  dependency-weight: 5
  effort: 4
  risk: 3
  score: 20
---
## Implementation Design

### Sidecar Changes
- `provider-interface.ts` — Provider interface defining `query()`, `resume()`, `cancelStream()`, `healthCheck()`
- `providers/claude-agent.ts` — ClaudeAgentProvider (first concrete implementation)
- `providers/index.ts` — `createProvider()` factory
- `provider.ts` — Thin facade delegating to default provider

### Cross-Stack Rename
- `sdk_session_id` → `provider_session_id` across:
  - Rust types (SidecarRequest, SidecarResponse)
  - TypeScript protocol types
  - SQLite column (migration 005)
  - All callers (13+ files)

### Neutral Protocol
- NDJSON protocol carries `ProviderEvent` types, not Claude-specific types
- Rust backend is provider-agnostic
- Claude-specific concepts encapsulated in ClaudeAgentProvider

## Produced Decision

AD-025 (Provider-Agnostic AI Integration)

## Git Evidence

- `fa8ecc7` — Pluggable sidecar, sdk→provider rename
- `34cc4b6` — Provider interface, Claude adapter
