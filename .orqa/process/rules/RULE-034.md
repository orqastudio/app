---

id: RULE-034
title: Artifact Link Format
description: All cross-references between artifacts must use markdown link syntax with artifact IDs. Bare IDs, file paths, and web-style paths are forbidden.
status: active
created: 2026-03-11
updated: 2026-03-13
layer: core
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Cross-reference format ensures navigable, structured artifact links
  - target: RULE-032
    type: informs
    rationale: Frontmatter field values that reference artifacts must use valid IDs in the correct link format
  - target: RULE-003
    type: informs
    rationale: Artifact link format must use IDs, not file paths — config paths and link targets are separate concerns
  - type: enforces
    target: AD-036
    rationale: This rule enforces cross-linking as default — it mandates the [ID](ID) format and uses autolink-artifacts.mjs to auto-convert bare IDs to links at commit time
  - type: enforces
    target: AD-029
    rationale: This rule enforces cross-reference format consistency for the universal roles and artifact ID system
  - target: RULE-045
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RULE-045
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-007
    rationale: Migrated from scope field
  - target: AGENT-008
    type: scoped-to
    rationale: "Auto-generated inverse of scoped-to relationship from AGENT-008"
---
All artifact cross-references MUST use the format `[Display Text](ARTIFACT-ID)` where the artifact ID matches the pattern `PREFIX-NNN`. The display text is typically the artifact ID itself: `[EPIC-001](EPIC-001)`.

## Valid Artifact ID Prefixes

| Prefix | Type |
|--------|------|
| `EPIC` | Epic |
| `TASK` | Task |
| `AD` | Architecture Decision |
| `MS` | Milestone |
| `IDEA` | Idea |
| `IMPL` | Lesson |
| `RES` | Research |
| `PILLAR` | Pillar |
| `RULE` | Rule |

## Valid Formats

```markdown
See [RULE-004](RULE-004) for details.
This implements [EPIC-045](EPIC-045).
Based on [AD-029](AD-029) and [RES-036](RES-036).
```

## FORBIDDEN

```markdown
<!-- Bare IDs without link syntax — won't be detected by the renderer -->
See RULE-004 for details.

<!-- Web-style paths — don't resolve in the app -->
[Governance](/product/governance)

<!-- File paths — use artifact IDs instead -->
[Governance](.orqa/documentation/product/governance.md)
[Governance](../product/governance.md)

<!-- Wrapping links in outer parentheses — visual noise -->
([EPIC-001](EPIC-001))
```

- Bare artifact IDs without markdown link syntax
- Web-style URL paths as link targets
- Relative or absolute file paths as link targets
- Wrapping artifact links in outer parentheses or brackets

## Enforcement

- Pre-commit hook can scan for bare artifact ID patterns (`EPIC-\d+`, `RULE-\d+`, etc.) that are not wrapped in markdown link syntax
- The `orqa-documentation` skill provides authoring guidance; this rule provides the constraint

## Related Rules

- [RULE-032](RULE-032) (schema-validation) — frontmatter field values that reference artifacts must use valid IDs
- [RULE-003](RULE-003) (artifact-config-integrity) — artifact paths in config must match disk
