---
id: "IMPL-c726abc2"
type: lesson
title: "Out of Scope sections created without user verification — RULE-8ee65d73 violated"
description: "The orchestrator wrote Out of Scope sections on epics without presenting them to the user for approval. RULE-8ee65d73 requires every scope reduction to be a user decision. This is a self-compliance-only rule (no mechanical enforcement) and was violated twice in the same session — first on EPIC-82dd0bd2, then on EPIC-a60f5b6b."
status: archived
created: "2026-03-13"
updated: "2026-03-13"
maturity: "observation"
recurrence: 2
relationships: []
---

## Pattern

When creating epics, the orchestrator decides what is "out of scope" and writes it into the epic without asking the user. This violates [RULE-8ee65d73](RULE-8ee65d73)'s principle that scope reductions are user decisions. The rule exists but is self-compliance only — no tooling flags when an Out of Scope section is created without an approval step.

## Fix

Two layers:

1. **Planning methodology**: Update [RULE-dccf4226](RULE-dccf4226) or the `planning` skill to require that Out of Scope sections are presented to the user for explicit approval before being committed. The orchestrator should present proposed scope exclusions and ask: "Should any of these be in scope?"
2. **Mechanical enforcement**: The prompt-submit hook (IMPL-6a8f9612) or a plan-review step could detect when Out of Scope is written to an epic and prompt for user verification. Alternatively, the gap audit tool could flag epics with Out of Scope sections that lack a corresponding user approval marker.
