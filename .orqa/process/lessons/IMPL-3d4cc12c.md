---
id: "IMPL-3d4cc12c"
title: "Out of Scope sections created without user verification — RULE-e120bb70 violated"
description: "The orchestrator wrote Out of Scope sections on epics without presenting them to the user for approval. RULE-e120bb70 requires every scope reduction to be a user decision. This is a self-compliance-only rule (no mechanical enforcement) and was violated twice in the same session — first on EPIC-e37794bf, then on EPIC-4e6e9eae."
status: "review"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "observation"
recurrence: 2
relationships: []
---
## Pattern

When creating epics, the orchestrator decides what is "out of scope" and writes it into the epic without asking the user. This violates [RULE-e120bb70](RULE-e120bb70)'s principle that scope reductions are user decisions. The rule exists but is self-compliance only — no tooling flags when an Out of Scope section is created without an approval step.

## Fix

Two layers:
1. **Planning methodology**: Update [RULE-303c1cc8](RULE-303c1cc8) or the `planning` skill to require that Out of Scope sections are presented to the user for explicit approval before being committed. The orchestrator should present proposed scope exclusions and ask: "Should any of these be in scope?"
2. **Mechanical enforcement**: The prompt-submit hook (IMPL-8d666f0c) or a plan-review step could detect when Out of Scope is written to an epic and prompt for user verification. Alternatively, the gap audit tool could flag epics with Out of Scope sections that lack a corresponding user approval marker.
