---
id: KNOW-8c98ea98
type: knowledge
title: Restructuring Methodology
description: |
  Safe, incremental refactoring methodology: one change at a time, verify after
  each step, preserve behavior, no temporary files. Covers module extraction,
  type unification, store consolidation, and scope assessment.
  Use when: Cleaning up architectural debt, reorganizing modules, consolidating
  patterns, or performing any structural code change.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-a1b2c3d4
    type: synchronised-with
---


Methodology for safe, incremental restructuring of codebases. This skill teaches the *process* of refactoring — the domain-specific patterns (e.g., which modules exist, what the architecture looks like) come from the project's architecture skills.

## Core Principles

### One Change at a Time

- Make a single, well-defined refactoring step
- Verify with the project's check/test command after each step
- Only then proceed to the next step
- If a step breaks something, revert it before trying an alternative

### No Temporary Files

- Never create "temporary" bridge files or compatibility shims
- Refactor in place — rename, move, restructure, but never duplicate
- If a refactoring requires a temporary state, it should be small enough to complete in one step

### Preserve Behavior

- Refactoring changes structure, not behavior
- Every refactoring step must be behavior-preserving (tests pass before and after)
- If behavior needs to change, that is a feature change, not a refactoring

## Refactoring Patterns

### Module Extraction

When a module grows too large:

1. Identify a cohesive set of functions/types to extract
2. Create the new module file in the appropriate directory
3. Move the items to the new module
4. Update public exports
5. Fix all import paths across the codebase
6. Verify: lint + test

### Type Unification

When types or error handling is inconsistent:

1. Audit all types/error types in the module
2. Design a unified type that covers all variants
3. Implement conversion traits where needed
4. Replace ad-hoc patterns with the unified type
5. Verify: lint + test

### Store / State Consolidation

When related state is scattered:

1. Identify state that multiple consumers read or modify
2. Create a centralized state container
3. Move data fetching into the container
4. Update consumers to read from the container
5. Verify: lint + test

### Component Extraction

When a UI component exceeds size limits:

1. Identify a self-contained section of template + logic
2. Create a new component in the appropriate directory
3. Define inputs via props — display components never fetch data directly
4. Replace the inline section with the new component
5. Verify: lint + test

## Scope Assessment

Before starting, assess the scope:

| Scope | Estimate | Action |
|-------|----------|--------|
| **Small** | < 30 minutes | Rename, extract function, fix inconsistency — proceed immediately |
| **Medium** | 30 min - 2 hours | Extract module, consolidate types — plan steps first |
| **Large** | > 2 hours | Restructure hierarchy, change data flow — write a plan document, get approval |

## Critical Rules

- NEVER refactor and add features in the same change — separate concerns
- NEVER leave the codebase in a broken state between steps
- NEVER create temporary compatibility layers — refactor cleanly or don't refactor
- NEVER refactor code you don't understand — use code search first
- Always run the project's full check suite after completing a refactoring session
- If tests fail after a refactoring step, fix the refactoring, not the tests
- Document the rationale for structural changes in commit messages