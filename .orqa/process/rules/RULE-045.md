---
id: RULE-045
title: Data Integrity
description: All artifact cross-references must resolve, pipeline relationships must have bidirectional inverses, and integrity checks run on every commit.
status: active
created: "2026-03-13"
updated: "2026-03-13"
layer: core
scope: []
relationships:
  - type: grounded
    target: PILLAR-001
    rationale: Data integrity ensures the artifact graph is trustworthy and navigable
  - type: informs
    target: RULE-032
    rationale: Schema validation is one layer of integrity; link verification is another
  - type: informs
    target: RULE-034
    rationale: Cross-reference format rules are enforced by link verification
  - type: informs
    target: RULE-013
    rationale: Pre-commit enforcement is the mechanism for commit-time integrity checks
  - type: enforces
    target: AD-036
    rationale: Bidirectional link verification ensures every cross-link target resolves — directly enforcing cross-linking as a reliable default
  - type: enforces
    target: AD-042
    rationale: verify-pipeline-integrity.mjs enforces that knowledge pipeline stage transitions have proper bidirectional relationship edges
---

All artifact cross-references must resolve to existing artifacts. Pipeline relationships must have bidirectional inverses. These constraints are enforced at commit time and can be verified manually.

## Link Resolution (NON-NEGOTIABLE)

Every cross-reference in `.orqa/` artifacts must resolve:

1. **Frontmatter references** — fields like `epic`, `milestone`, `depends-on`, `pillars` must point to existing artifacts
2. **Body text links** — `[DISPLAY](ARTIFACT-ID)` link targets must exist
3. **Relationship targets** — every `target` in a `relationships` array must be a valid artifact ID

## Bidirectional Inverses (NON-NEGOTIABLE)

For every relationship `A --type--> B`, the artifact `B` must have the corresponding inverse relationship `inverse-type --> A`.

| Type | Inverse |
|------|---------|
| `observes` | `observed-by` |
| `grounded` | `grounded-by` |
| `practices` | `practiced-by` |
| `enforces` | `enforced-by` |
| `verifies` | `verified-by` |
| `informs` | `informed-by` |

One-sided relationships indicate a broken graph edge. The pre-commit hook blocks commits that introduce asymmetric relationships.

## Enforcement

### Pre-commit (automatic)

The `.githooks/pre-commit` hook runs on every commit that includes `.orqa/` files:

- `tools/verify-links.mjs --staged --check-bidirectional` — checks staged files for broken links and missing inverses
- `tools/verify-pipeline-integrity.mjs --staged` — checks staged files for pipeline consistency

### Manual (full scan)

```bash
make verify-links      # Full link verification across all .orqa/ files
make verify-integrity  # Pipeline integrity check
make verify            # Both
```

## FORBIDDEN

- Committing artifacts with broken cross-references
- Committing relationships without bidirectional inverses
- Bypassing integrity checks with `--no-verify`
- Phantom artifact IDs (referencing IDs that were never created as real artifacts)

## Related Rules

- [RULE-032](RULE-032) (schema-validation) — schema validation is complementary to link verification
- [RULE-034](RULE-034) (artifact-cross-references) — cross-reference format rules enforced by link verification
- [RULE-013](RULE-013) (git-workflow) — pre-commit hook enforcement mechanism
