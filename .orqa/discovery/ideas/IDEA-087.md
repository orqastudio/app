---
id: IDEA-087
title: Linter integration plugins for lint suppression governance
description: "Replace the pre-commit lint suppression scanner with native linter plugins (clippy plugin, ESLint plugin) that validate AD-NNN decision references at lint time, providing IDE-level feedback. Software-project-specific plugin."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "What is the clippy plugin API? Can custom lints validate comments adjacent to #[allow] annotations?"
  - "What is the ESLint plugin API for validating eslint-disable comments?"
  - "Can these plugins read .orqa/ decision artifacts to validate references at lint time?"
  - "Should this be a standalone plugin or part of the software project type preset?"
  - "How does this interact with IDE integration — can suppression violations show as warnings in the editor?"
relationships:
  - target: AD-047
    type: crystallises
    rationale: "AD-047 defines the // AD-NNN pattern these plugins would consume"
  - target: PILLAR-002
    type: grounded
  - target: PERSONA-001
    type: benefits
---

## Motivation

The pre-commit scanner (`.githooks/validate-lint-suppressions.mjs`) enforces that lint suppressions reference decision artifacts. This works but only catches violations at commit time. Linter plugins would catch them at edit time in the IDE, providing immediate feedback.

The `// AD-NNN` comment pattern is designed as the stable interface — both the scanner and future plugins consume it. Migration means deleting the scanner and configuring the plugins, not changing the data model.

## Sketch

Software-project-specific plugin providing:
- Clippy lint: `orqa::suppression_requires_decision` — validates `#[allow]` has `// AD-NNN`
- ESLint rule: `orqa/suppression-requires-decision` — validates `eslint-disable` has `// AD-NNN`
- Both read `.orqa/process/decisions/` to verify referenced decisions exist and are accepted
