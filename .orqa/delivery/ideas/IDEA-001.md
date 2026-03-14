---
id: IDEA-001
title: Multi-Provider Ecosystem
description: |
  Support additional AI providers through the provider-agnostic sidecar interface without changing the Rust core or Svelte UI.
status: delivered
created: 2026-03-07
updated: 2026-03-13
pillars:
  - PILLAR-001
research-needed:
  - Provider SDK compatibility assessment (OpenRouter, Together AI, Fireworks, Replicate)
  - Cost model research and budget prediction
  - UX for provider switching and selection
  - Local LLM viability (Ollama, air-gapped use)
promoted-to: EPIC-040
relationships:
  - target: RES-009
    type: informed-by
    rationale: Provider architecture research informed multi-provider ecosystem design
  - target: RES-027
    type: informed-by
    rationale: Provider abstraction layer research informed sidecar refactoring approach
  - target: DOC-036
    type: documented-by
    rationale: Referenced in documentation page Artifact Framework
  - target: DOC-044
    type: documented-by
    rationale: Referenced in documentation page Roadmap
---
## Candidate Items

- Third-party AI cloud provider research — OpenRouter, Together AI, Fireworks, Replicate
- Direct Anthropic API provider — Rust-native HTTP, pay-per-token
- Direct OpenAI-compatible API provider — OpenAI, Azure OpenAI, compatible endpoints
- Gemini Developer API provider
- OpenAI Agents SDK sidecar — second agent runtime
- Google ADK sidecar — third agent runtime
- Ollama / local LLM provider — offline/air-gapped use
- Budget & billing prediction — usage tracking and cost prediction
- Multi-provider cost optimisation — route work to cheapest capable provider
- Provider selection in project config
