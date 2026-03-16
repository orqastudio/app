---
id: RULE-018
title: No Aliases or Hacks
description: "Fix root causes of type mismatches between layers. Never paper over with aliases, shims, or duplicate mappings."
status: active
created: 2026-03-07
updated: 2026-03-12
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: No aliases ensures structural consistency across type boundaries
  - target: RULE-010
    type: informs
    rationale: Type consistency must hold across all layers — mismatches must be fixed at the source layer
  - target: RULE-012
    type: informs
    rationale: A type mismatch is an error you own — fix it at the root, not with a workaround
  - target: RULE-020
    type: informs
    rationale: Alias entries are a form of fake data hiding real gaps between layers
  - target: RULE-005
    type: informs
    rationale: Use search_regex to find all usages of an identifier before renaming or removing it
  - target: RULE-003
    type: informed-by
---
When a value or type mismatch exists between layers, fix the root cause. Never paper over it with aliases, shims, or duplicate mappings.

## What Counts as an Alias or Hack

- Duplicate entries in type unions to handle both spellings of the same concept
- Widened type unions that let one layer tolerate a value another layer shouldn't be producing
- Normalizer or alias maps that rewrite values from one layer before passing them to another
- Fallback lookups that hide missing entries: `labels[key] ?? key`
- Duplicate entries in capability maps, enum-equivalent unions, or label maps that refer to the same concept under two different keys
- Transform/adapter layers inserted between service layers that should already agree on a schema
- Pattern-match arms that handle both the correct and incorrect variant names
- Deserialization aliases used to tolerate incorrect field names instead of fixing the source

## The Root Cause Rule

Data flows through layers. A mismatch at any layer must be fixed at the layer that **introduced** the mismatch.

| Scenario | Wrong fix | Correct fix |
|----------|-----------|-------------|
| Backend returns `"deploy"`, frontend expects `"space_deploy"` | Add `"deploy"` to the frontend type | Fix the backend serialization to produce `"space_deploy"` |
| Serialization drops a field the consumer needs | Add a null-check fallback in the consumer | Add the field to the producer's response type |
| A type union is missing a value the backend can produce | Widen the union with an alias | Fix the backend to only produce canonical values; add the single canonical value to the union |
| Two label map keys map to the same display string | Keep both keys | Remove the non-canonical key everywhere and use one canonical key |

## Required Pattern

1. **Identify which layer introduced the wrong value** — use `code_research` or `search_regex` to trace the value from its origin to the mismatch point
2. **Fix that layer** — normalize at the source
3. **Remove all downstream workarounds** — delete alias entries, revert widened unions, remove normalizer maps
4. **Verify consistency** — every layer uses the same single canonical identifier; use `search_regex` for the old alias to confirm it is gone

## One Canonical Identifier Per Concept

Every status value, type discriminator, and configuration key MUST appear exactly once across all layers of the system.

If the same concept has two identifiers in any layer, that is a bug — not a feature.

## Related Rules

- [RULE-010](RULE-010) (end-to-end-completeness) — type consistency must hold across all layers in the same commit
- [RULE-012](RULE-012) (error-ownership) — a type mismatch is an error you own; fix it, don't work around it
- [RULE-020](RULE-020) (no-stubs) — alias entries are a form of fake data hiding real gaps
- [RULE-005](RULE-005) (code-search-usage) — use `search_regex` to find all usages of an identifier before renaming or removing it
