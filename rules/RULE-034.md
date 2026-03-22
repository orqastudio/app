---
id: "RULE-2f7b6a31"
type: "rule"
title: "Artifact Link Format"
description: "All cross-references between artifacts must use markdown link syntax with artifact IDs. Bare IDs, file paths, and web-style paths are forbidden."
status: "active"
created: "2026-03-11"
updated: "2026-03-13"
enforcement: "pre-commit hook — pre-commit can scan staged .orqa files for bare artifact ID patterns not wrapped in markdown link syntax; orqa validate checks link format on .orqa artifacts"
relationships:
  - target: "AD-d8ea4d2b"
    type: "enforces"
---
All artifact cross-references MUST use the format `[Display Text](ARTIFACT-ID)` where the artifact ID matches the pattern `PREFIX-NNN`. The display text is typically the artifact ID itself: `[EPIC-e045ab6d](EPIC-e045ab6d)`.

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
See [RULE-7b770593](RULE-7b770593) for details.
This implements [EPIC-be023ed2](EPIC-be023ed2).
Based on [AD-774cc3d0](AD-774cc3d0) and [RES-a6311b1b](RES-a6311b1b).
```

## FORBIDDEN

```markdown
<!-- Bare IDs without link syntax — won't be detected by the renderer -->
See RULE-7b770593 for details.

<!-- Web-style paths — don't resolve in the app -->
[Governance](/product/governance)

<!-- File paths — use artifact IDs instead -->
[Governance](.orqa/documentation/about/governance.md)
[Governance](../about/governance.md)

<!-- Wrapping links in outer parentheses — visual noise -->
([EPIC-e045ab6d](EPIC-e045ab6d))
```

- Bare artifact IDs without markdown link syntax
- Web-style URL paths as link targets
- Relative or absolute file paths as link targets
- Wrapping artifact links in outer parentheses or brackets

## Enforcement

- Pre-commit hook can scan for bare artifact ID patterns (`EPIC-\d+`, `RULE-\d+`, etc.) that are not wrapped in markdown link syntax
- The `orqa-documentation` skill provides authoring guidance; this rule provides the constraint

## Related Rules

- [RULE-a764b2ae](RULE-a764b2ae) (schema-validation) — frontmatter field values that reference artifacts must use valid IDs
- [RULE-6c0496e0](RULE-6c0496e0) (artifact-config-integrity) — artifact paths in config must match disk
