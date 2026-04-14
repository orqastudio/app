---
name: Documentation first
description: Docs define intent and target state — code is changed to match docs, not the other way around
type: feedback
---

Documentation defines intent and the correct target state. Code is then changed to match the docs. When documentation lags behind code, the docs are updated first — they become the new source of truth, and then code is verified against them.

**Why:** RULE-008. Without this, docs become stale afterthoughts and the actual system behaviour is only discoverable by reading code. The documentation-first principle ensures understanding precedes action (Pillar 1).

**How to apply:** Before implementing a feature, write or update the doc/plan first. Before fixing a doc/code disagreement, decide which is correct — update the docs to reflect the intended state, then fix the code.
