---
id: RULE-130f1f63
title: Data Integrity
description: "All artifact cross-references must resolve, pipeline relationships must have bidirectional inverses, and integrity checks run on every commit."
status: active
created: 2026-03-13
updated: 2026-03-14
enforcement:
  - "event: file"
  - ".orqa/**/*.md"
  - "event: file"
  - ".orqa/**/*.md"
  - orqa-governance
relationships:
  - target: AD-d8ea4d2b
    type: enforces
  - target: AD-a76663db
    type: enforces
  - target: AGENT-ff44f841
    type: observed-by
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

- [RULE-a764b2ae](RULE-a764b2ae) (schema-validation) — schema validation is complementary to link verification
- [RULE-2f7b6a31](RULE-2f7b6a31) (artifact-cross-references) — cross-reference format rules enforced by link verification
- [RULE-633e636d](RULE-633e636d) (git-workflow) — pre-commit hook enforcement mechanism
