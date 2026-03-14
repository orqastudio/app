---
id: TASK-413
title: "Add bidirectional relationship checking to graph-guardian.mjs"
description: "Extend the PostToolUse graph-guardian hook to check for missing bidirectional relationship inverses when .orqa/ artifacts are written. Return warnings for one-sided relationships so the agent can fix them immediately."
status: todo
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-064
depends-on: []
assignee: null
skills:
  - SKILL-011
  - SKILL-020
acceptance:
  - "After Write/Edit on .orqa/**/*.md files, graph-guardian checks for bidirectional inverses"
  - "For each relationship A --type--> B, checks if B has inverse-type --> A"
  - "Missing inverses returned as warnings in additionalContext (not blocking)"
  - "Warning includes the exact relationship entry that should be added to the target artifact"
  - "Non-.orqa/ files are ignored (no performance impact on code writes)"
  - "Relationship type/inverse mapping matches RULE-045 table"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: "Core task — write-time integrity enforcement for graph consistency"
  - target: RULE-045
    type: enforces
    rationale: "RULE-045 declared write-time enforcement entries that this task implements mechanically"
  - target: IMPL-055
    type: informed-by
    rationale: "IMPL-055 identified the need for write-time integrity checking"
---

## Scope

### graph-guardian.mjs Changes

Add bidirectional check function:

1. Parse frontmatter of the just-written file
2. Extract `relationships` array
3. For each relationship `{ target, type }`:
   a. Resolve target to file path (e.g., `RULE-001` → `.orqa/process/rules/RULE-001.md`)
   b. Read target file's frontmatter
   c. Check for inverse relationship pointing back
   d. If missing: add to warnings list
4. Return warnings as additionalContext

### Inverse Mapping

```js
const INVERSE_MAP = {
  'observes': 'observed-by',
  'grounded': 'grounded-by',
  'practices': 'practiced-by',
  'enforces': 'enforced-by',
  'verifies': 'verified-by',
  'informs': 'informed-by',
  'delivers': 'delivered-by',
  // Reverse mappings
  'observed-by': 'observes',
  'grounded-by': 'grounded',
  'practiced-by': 'practices',
  'enforced-by': 'enforces',
  'verified-by': 'verifies',
  'informed-by': 'informs',
  'delivered-by': 'delivers',
};
```

### Performance

- Only runs on `.orqa/**/*.md` writes (guard at top of function)
- Single-file check per write — reads only the target files referenced in relationships
- Typical artifact has 3-8 relationships — reads 3-8 files per check
- Total: ~50ms per artifact write
