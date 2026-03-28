---
id: IDEA-18da61df
type: discovery-idea
title: Multi-Provider Model Routing
description: "Route different agent roles to different AI providers/models based on task type. Planning done by one model (potentially local/fine-tuned), implementation by another (cloud provider like Claude). Enables cost optimization, latency reduction, and specialized model selection per role."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: someday
research-needed:
  - "Which agent roles benefit most from specialized models vs general-purpose?"
  - "What is the latency/quality tradeoff for local fine-tuned planning models vs cloud models?"
  - "How does the Agent SDK support multiple provider backends?"
  - "What model routing strategies exist (static per-role, dynamic based on task complexity)?"
  - "How to handle context sharing between models when different roles work on the same task?"
relationships:
  - target: PILLAR-a6a4bbbb
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

OrqaStudio's universal agent model already separates work into distinct roles (Planner, Implementer, Reviewer, etc.). Each role has different requirements — planning needs broad reasoning but not code generation speed, while implementation needs strong code capabilities. Routing each role to the optimal model enables:

1. **Cost optimization** — use cheaper/local models for planning, expensive models for implementation
2. **Latency reduction** — local fine-tuned models for fast planning iterations
3. **Specialization** — models tuned for specific tasks (code review, documentation, architecture)
4. **Provider independence** — the app already aims to be provider-agnostic (AD-23e27cf5), this extends that to per-role granularity

## Sketch

- Per-role model configuration in project settings (or per-agent in agent YAML)
- Provider registry supporting multiple simultaneous providers (Claude, local GGUF, OpenAI, etc.)
- Model routing table: `{ role: "planner", provider: "local-llama", model: "planning-7b" }`
- Fallback chain: if local model is unavailable, fall back to cloud provider
- Context serialization between providers when handing off between roles
