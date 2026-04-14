---
name: Investigate systemically before fixing
description: Understand the pattern across the system before fixing individual instances
type: feedback
---

When a problem is found, investigate systemically across the codebase before fixing the individual instance. The same pattern likely exists in multiple places.

**Why:** IMPL-series — multiple lessons showed that fixing one instance without checking for the same pattern elsewhere leads to repeated discovery of the same issue across sessions.

**How to apply:** When you find a bug or anti-pattern, grep/search for the same pattern across the codebase before implementing a fix. Fix all instances together, or document the scope for planned remediation.
