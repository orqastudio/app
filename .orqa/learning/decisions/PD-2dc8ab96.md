---
id: "PD-2dc8ab96"
type: principle-decision
title: "Lint suppression exceptions require decision audit trail"
description: "Every lint suppression annotation (#[allow(clippy::...)], eslint-disable, @ts-ignore) must reference an accepted decision artifact. Pre-commit scanner validates the mapping. Vendored components are exempt."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: "2026-04-13"
relationships:
  - type: drives
    target: EPIC-e24086ed
    rationale: "Lint suppression audit trail decision drove the code quality audit and enforcement alignment epic"
---

## Decision

Every lint suppression annotation in the codebase must include a decision reference comment on the same line (or adjacent comment block):

```rust
#[allow(clippy::too_many_lines)] // PD-2dc8ab96: Tauri app builder wiring
```

```svelte
<!-- eslint-disable rule-name --> <!-- PD-2dc8ab96: shadcn pattern requirement -->
```

```typescript
// eslint-disable-next-line @typescript-eslint/no-explicit-any // AD-NNN: justification
```

### Requirements

1. Every suppression annotation must reference an accepted `AD-NNN` decision
2. The referenced decision must exist and have `status: accepted`
3. The decision must document the justification in an "Approved Exceptions" section
4. New exceptions require creating or updating a decision artifact

### Exempt Paths

These paths are exempt because they contain vendored/generated code whose lint suppressions we don't control:

| Path Pattern | Reason |
| ------------- | -------- |
| `ui/src/lib/components/ui/**` | shadcn-svelte vendored components |
| `.svelte-kit/**` | SvelteKit generated types |
| `node_modules/**` | Third-party dependencies |
| `target/**` | Rust build output |

### Enforcement

**Current:** Pre-commit scanner (`.githooks/validate-lint-suppressions.mjs`) checks staged files.

**Future:** Linter integration plugins (clippy plugin, eslint plugin) that consume the same `// AD-NNN` pattern. See [IDEA-df1d0829](IDEA-df1d0829).

### Acknowledged Tech Debt

The pre-commit scanner is a stopgap enforcement mechanism. The long-term design is linter plugins that:

- Parse `// AD-NNN` references natively
- Validate against decision artifacts at lint time (not just commit time)
- Provide IDE-level feedback (red squiggly on suppressions without decision refs)

This is acceptable because:

- The `// AD-NNN` comment pattern is the stable interface — both the scanner and future plugins consume it
- The scanner is a single file (~80 lines) with no dependencies beyond Node.js
- Migration to plugins means deleting the scanner and adding plugin config, not refactoring data

### Approved Exceptions

*No exceptions currently in use. The codebase has zero lint suppression annotations in non-vendored code.*

| Function/Location | File | Rule Suppressed | Justification |
| ------------------- | ------ | ----------------- | --------------- |
| *(none)* | | | |

## Rationale

- Lint rules exist for a reason — suppressing them without justification accumulates silent tech debt
- Decision artifacts provide audit trail, review history (via git), and periodic reassessment
- The `// AD-NNN` pattern is language-agnostic and tooling-agnostic — works with any linter
- Vendored code exemption prevents fighting upstream library decisions

## Consequences

- Every `#[allow(...)]`, `// eslint-disable`, `@ts-ignore`, `@ts-expect-error` in non-exempt paths must have a decision reference
- Developers must create or update a decision before adding a suppression
- Pre-commit blocks commits with unreferenced suppressions
- Periodic audit: if exceptions accumulate (>5 per decision), consider whether the lint rule needs reconfiguration rather than more exceptions
