---
id: "PD-af88bb69"
type: principle-decision
title: "Composability Principle"
description: "External integrations connect through provider-agnostic interfaces. Provider-specific logic lives in swappable sidecar processes."
status: completed
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
relationships:
  - target: "EPIC-05ae2ce7"
    type: "drives"
---

## Decision

External integrations (AI providers, MCP servers) connect through provider-agnostic interfaces. The Rust core speaks a neutral `ProviderEvent` protocol; provider-specific logic lives in swappable sidecar processes. Phase 1 implements one provider (Agent SDK for Max subscription). Future providers implement the same interface without changing the core.

## Rationale

Extends the composability design principle from the Alvarez project. Decoupling the AI provider from the core application means: (1) switching providers requires only a new sidecar implementation, (2) supporting multiple providers simultaneously is architecturally possible, (3) the Rust core and Svelte UI are tested independently of any provider, (4) if Anthropic releases a Rust SDK, the sidecar can be replaced with a native implementation.

## Consequences

The `ProviderEvent` enum must be stable and provider-neutral. The sidecar protocol (stdin/stdout NDJSON) is the contract — any process that speaks it can be a provider. Provider selection and configuration surfaces in OrqaStudio's settings UI.
