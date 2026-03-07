---
id: IDEA-001
title: "Multi-Provider Ecosystem"
status: captured
created: 2026-03-07
updated: 2026-03-07
pillar:
  - clarity-through-structure
promoted-to: null
research-needed:
  - Provider SDK compatibility assessment (OpenRouter, Together AI, Fireworks, Replicate)
  - Cost model research and budget prediction
  - UX for provider switching and selection
  - Local LLM viability (Ollama, air-gapped use)
tags: [providers, composability, multi-provider]
---

# Multi-Provider Ecosystem

The provider-agnostic sidecar interface supports additional providers without changing the Rust core or Svelte UI.

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
