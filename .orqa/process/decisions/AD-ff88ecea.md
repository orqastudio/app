---
id: AD-ff88ecea
type: decision
title: Provider-Agnostic AI Integration
description: "The AI integration layer uses a Provider interface. Claude is the first provider via ClaudeAgentProvider, but the architecture supports pluggable providers (direct API, mock, future models) without changing the NDJSON protocol or Rust backend.\n"
status: completed
created: 2026-03-07
updated: 2026-03-13
relationships:
  - target: "RULE-92dba0cb"
    type: "enforced-by"
    rationale: "RULE-92dba0cb generalises provider-agnosticism to all agent capabilities — agents declare capabilities not tool names, enforcing the same provider-agnostic pattern at the governance layer that this AD establishes at the sidecar layer"
  - target: EPIC-ee688e85
    type: drives
---
## Decision

The sidecar AI integration is refactored from a Claude-specific implementation to a provider-agnostic architecture:

1. **Provider interface** (`provider-interface.ts`) — Defines the contract any AI provider must implement: `query()`, `resume()`, `cancelStream()`, `healthCheck()`
2. **ClaudeAgentProvider** (`providers/claude-agent.ts`) — First concrete implementation using the Claude Agent SDK
3. **Provider factory** (`providers/index.ts`) — `createProvider()` selects the appropriate provider based on configuration
4. **Neutral naming** — All "SDK"-specific terminology renamed to "provider" throughout the stack: `sdk_session_id` → `provider_session_id` across Rust types, TypeScript protocol, SQLite columns (migration 005), and all callers

## Rationale

OrqaStudio's identity is "AI-assisted clarity engine", not "Claude wrapper." Coupling the entire stack to one AI provider creates vendor lock-in and prevents:
- Using direct API calls for simpler queries (no Agent SDK overhead)
- Testing with mock providers (deterministic responses for E2E tests)
- Supporting future AI models without rewriting the sidecar
- Users choosing their preferred model

The rename from `sdk_session_id` to `provider_session_id` was done as a single atomic change (13+ files) to avoid gradual drift between old and new naming.

## Consequences

- New providers implement the `Provider` interface and register in the factory
- The NDJSON protocol between sidecar and Rust is provider-neutral — it carries `ProviderEvent` types, not Claude-specific types
- Session persistence uses `provider_session_id` — the column name in SQLite is provider-agnostic
- The Rust backend doesn't know or care which provider the sidecar is using
- Claude-specific concepts (Max subscription, Agent SDK) are encapsulated in `ClaudeAgentProvider`

## Git Evidence

- `fa8ecc7` — Provider abstraction layer: pluggable sidecar, sdk→provider rename
- `34cc4b6` — Provider abstraction layer: extract Provider interface, Claude adapter