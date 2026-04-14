---
name: Dogfood mode
description: RULE-009 — enforcement gaps discovered during development are immediately CRITICAL priority
type: feedback
---

Enforcement gaps discovered during OrqaStudio's own development are immediately CRITICAL priority. This is dogfood mode — the product must use its own tools and governance, and any failure in that loop is the highest-priority fix.

**Why:** RULE-009. OrqaStudio is built using OrqaStudio. If the governance system fails during its own development, it will fail for users too. Dogfooding is not optional — it is a foundational design constraint.

**How to apply:** When you discover that OrqaStudio's rules, enforcement, or governance aren't working during development, treat it as CRITICAL. Don't work around it — fix the system.
