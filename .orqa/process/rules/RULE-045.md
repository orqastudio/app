---
id: RULE-045
title: Data Integrity
description: "All artifact cross-references must resolve, pipeline relationships must have bidirectional inverses, and integrity checks run on every commit."
status: active
created: 2026-03-13
updated: 2026-03-14
layer: core
enforcement:
  - "event: file"
  - ".orqa/**/*.md"
  - "event: file"
  - ".orqa/**/*.md"
  - orqa-governance
relationships:
  - target: RES-056
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from RES-056
  - target: TASK-413
    type: enforced-by
    rationale: Auto-generated inverse of enforced-by relationship from TASK-413
  - target: IMPL-058
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-058
  - target: AGENT-008
    type: enforces
    rationale: "Auto-generated inverse of scoped-to relationship from AGENT-008"
  - target: EPIC-059
    type: informs
  - target: IMPL-055
    type: observed-by
  - target: IMPL-068
    type: observed-by
  - target: PILLAR-001
    type: grounded
  - target: RULE-013
    type: informs
  - target: RULE-032
    type: informs
  - target: RULE-034
    type: informs
  - target: AD-036
    type: enforces
  - target: AD-042
    type: enforces
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
| `delivers` | `delivered-by` |
| `delivers` | `delivered-by` |
| `observes` | `observed-by` |
| `grounded` | `grounded-by` |
| `grounded-by` | `grounded` |
| `enforces` | `enforced-by` |
| `enforces` | `enforced-by` |
| `informs` | `informed-by` |
| `enforces` | `enforced-by` |
| `informs` | `informed-by` |

One-sided relationships indicate a broken graph edge. The pre-commit hook blocks commits that introduce asymmetric relationships.

## Enforcement

### Write-time (automatic — enforcement engine)

When any `.orqa/**/*.md` file is written or edited, the enforcement engine (consumed by the Claude plugin in CLI context, the Rust app in app context) injects a graph integrity reminder. This catches missing bidirectional inverses at the moment of creation — before more artifacts are built on top of broken edges.

The enforcement entries on this rule declare:
- `event: file` / `action: inject` — reminds the agent to check bidirectional inverses
- `event: file` / `action: inject` / `skills: [orqa-governance]` — loads governance patterns

### Pre-commit (automatic)

The `.githooks/pre-commit` hook runs on every commit that includes `.orqa/` files:

- `tools/verify-links.mjs --staged --check-bidirectional` — checks staged files for broken links and missing inverses
- `tools/verify-pipeline-integrity.mjs --staged` — checks staged files for pipeline consistency

This is the hard gate — commits with broken links or missing inverses are blocked.

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
