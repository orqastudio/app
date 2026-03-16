---
id: IDEA-111
title: "Provider-aware plugin system"
status: captured
horizon: someday
created: 2026-03-16
relationships:
  - type: informed-by
    target: EPIC-080
  - type: evolves-from
    target: IDEA-071
---

# IDEA-111: Provider-aware plugin system

Extracted from IDEA-071 (Plugin ecosystem: type system, provider integration, capability routing).

AI provider definitions in `.orqa/providers/` with provider identity, detection, and required plugins. Capability routing hierarchy: project config → plugin defaults → app baseline. Load-time plugin filtering based on active provider. Bidirectional provider-plugin relationships — providers declare required plugins, plugins declare supported providers. Needed for multi-model AI support (Claude, GPT, local models).
