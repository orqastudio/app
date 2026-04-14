---
name: Forward-only relationships
description: Artifacts store forward relationships only — graph computes inverses from the full set
type: project
---

Relationships are forward-only. A task stores `delivers: epic`, but the epic does NOT store `delivered-by: task`. The graph engine computes inverse relationships from the full set of forward declarations.

**Why:** Bidirectional maintenance is error-prone and was a recurring source of bugs (IMPL-0c9a5882, IMPL-023a772e). Forward-only eliminates the entire class of consistency errors.

**How to apply:** When creating or modifying artifacts, only add the forward relationship. Never manually add inverse relationships to the target artifact. The graph handles it. 32 relationship types with narrow from/to constraints — check the relationship vocabulary before using one.
