---
name: orqa-validate
description: "Validate governance artifacts against the composed plugin schema via the OrqaStudio daemon."
user-invocable: true
---

# Validate Artifacts

Run schema validation on all governance artifacts in `.orqa/`. Reports frontmatter errors, invalid relationships, status violations, and missing required fields.

## Usage

```bash
orqa validate
```

Validation checks:

- **Schema compliance** -- required fields, valid types, correct ID prefixes
- **Relationship integrity** -- valid relationship types, correct from/to constraints, bidirectional consistency
- **Status validity** -- status values match the artifact type's state machine
- **Graph consistency** -- referenced targets exist, no orphaned relationships

Use `orqa enforce --fix` for auto-remediation of common issues.
