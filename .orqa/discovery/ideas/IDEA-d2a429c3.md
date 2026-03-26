---
id: IDEA-d2a429c3
type: discovery-idea
title: Make sidecars transportable as plugins
description: "Currently sidecars (AI provider integrations like the Claude Code CLI bridge) are native app code, while plugins are the extension mechanism. If sidecars were themselves transportable as plugins, the provider definition and its integration plugin would be a single artefact — eliminating the need for separate provider config in the app and requirement declarations in the plugin."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "What parts of the current sidecar are app-native vs extractable?"
  - "How would a sidecar-as-plugin be loaded and initialised differently from a regular plugin?"
  - "What security/trust model applies to a plugin that controls AI provider communication?"
  - "How does this interact with the five-layer trust model (core/plugin/community/user)?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

The system currently has two configuration surfaces for AI provider integration:

1. **Provider definitions** — native to the app, describing capabilities, detection, and required plugins
2. **Provider integration plugins** — installed in `.orqa/plugins/`, containing hooks, skills, and agents for the provider

This duality exists because the sidecar (the bridge between AI provider and Rust backend) is native app code, not a plugin. If the sidecar itself were transportable as a plugin, the provider definition would ship with it — one artefact, one configuration, one install step.

This would also make it possible for third parties to create provider integrations without modifying app source code.