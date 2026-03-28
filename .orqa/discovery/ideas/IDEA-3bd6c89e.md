---
id: IDEA-3bd6c89e
type: discovery-idea
title: Multi-Provider Ecosystem
description: Support additional AI providers through the provider-agnostic sidecar interface without changing the Rust core or Svelte UI.
status: archived
created: 2026-03-07
updated: 2026-03-13
research-needed:
  - "Provider SDK compatibility assessment (OpenRouter, Together AI, Fireworks, Replicate)"
  - Cost model research and budget prediction
  - UX for provider switching and selection
  - "Local LLM viability (Ollama, air-gapped use)"
relationships:
  - target: EPIC-2f1648f5
    type: realises
  - target: PILLAR-a6a4bbbb
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
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
