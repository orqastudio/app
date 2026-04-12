---
id: "EPIC-d668a2ae"
type: epic
title: "Schema-driven CLI validator — zero hardcoded types or keys"
description: "Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule."
status: archived
created: 2026-03-18T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# EPIC-d668a2ae: Schema-driven CLI validator — zero hardcoded types or keys

Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule.
