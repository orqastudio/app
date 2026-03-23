---
id: IDEA-c496b2de
type: idea
title: "Provider-aware plugin system"
status: captured
horizon: someday
created: 2026-03-16
relationships:
  - type: merged-from
    target: IDEA-1287dd52
  - target: PILLAR-94b281db
    type: grounded
  - target: PERSONA-a4b15450
    type: benefits
---

# IDEA-c496b2de: Provider-aware plugin system

Extracted from IDEA-1287dd52 (Plugin ecosystem: type system, provider integration, capability routing).

AI provider definitions in `.orqa/providers/` with provider identity, detection, and required plugins. Capability routing hierarchy: project config → plugin defaults → app baseline. Load-time plugin filtering based on active provider. Bidirectional provider-plugin relationships — providers declare required plugins, plugins declare supported providers. Needed for multi-model AI support (Claude, GPT, local models).