---
id: DOC-bf70068c
type: doc
title: "Thinking Mode: Documentation"
domain: reference
description: "The user wants docs written, updated, or organised — capturing knowledge for humans, not building features."
category: platform
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: KNOW-bf70068c
    type: synchronised-with
---

## What This Mode Is

Documentation Mode is active when the user wants documentation written, updated, or organised. This is about capturing knowledge for humans browsing the app, not building features. The agent writes markdown artifacts, not code.

Documentation Mode enforces the documentation-first principle (RULE-ec9462d8 (documentation-first implementation)): docs define intent and the correct target state. Code is then changed to match the docs. When documentation lags behind code, the docs are updated first — they become the new source of truth, and then code is changed to match.

---

## When It Activates

The orchestrator routes here when the user's request is about knowledge capture for humans rather than system execution.

Typical signals:

- "document how X works"
- "update the README for this module"
- "write a guide for Y"
- "the docs are out of date"
- "add documentation for the new command"
- "write the platform doc for this feature"
- "the doc page is missing from the app"

---

## What the Agent Needs

The writer role handles documentation work and needs:

- Documentation standards (`orqa-documentation` knowledge, RULE-ec9462d8 (documentation-first implementation))
- The current state of the feature being documented — read the code and existing artifacts first
- The correct doc schema: `type: doc`, appropriate `category`, relationships to referenced artifacts
- The `synchronised-with` relationship pattern for knowledge/doc pairs

Documentation artifacts belong in `.orqa/documentation/` with the appropriate subdirectory for their category (platform, development, reference, about).

---

## How It Connects to the Thinking Framework

Documentation Mode is both a standalone mode and a downstream output of other modes:

- **Implementation Mode** produces code → Documentation Mode captures what was built
- **Planning Mode** produces task artifacts → Documentation Mode captures design decisions as docs
- **Learning Loop Mode** captures lessons → Documentation Mode may produce the human-readable doc for a new rule

When documentation reveals that the code disagrees with the documented design, that is a **debugging or review signal** — route the discrepancy appropriately before updating the docs.

---

## Governance

- RULE-ec9462d8 (documentation-first implementation): docs define intent; code follows
- Doc artifacts use `type: doc` with a `category` field
- Knowledge/doc pairs use the `synchronised-with` relationship to stay linked
- Platform docs (this category) describe the platform itself and are not user-editable — they are refined through the learning loop
