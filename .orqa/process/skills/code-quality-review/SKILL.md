---
id: SKILL-004
title: Code Quality Review
description: |
  Code review methodology: automated checks, manual review checklists, forbidden
  pattern detection, function size limits, and structured review output format.
  Portable across any codebase with linting and testing infrastructure.
  Use when: Reviewing code for quality, running compliance checks, or producing
  structured review verdicts.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: core
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-002
    type: grounded
    rationale: Review verdicts and forbidden-pattern detection feed findings back into lessons and rules, closing the improvement loop
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
---


Code quality review methodology. This skill teaches the *review process and checklist structure* — the specific linting tools and coding standards come from the project's technology and coding-standards documentation.

## Review Protocol

### Step 1: Automated Checks

Run all available automated checks. Record PASS/FAIL for each.

Common check categories:
- Formatting (auto-formatter check mode)
- Linting (static analysis with warnings as errors)
- Type checking (compiler or type checker in strict mode)
- Unit tests
- Integration tests

### Step 2: Manual Review

Read each changed file. Evaluate against the project's coding standards and rules.

### Step 3: Lesson Check

- Search the project's lessons/known-issues for patterns matching current findings
- If a finding matches an existing lesson: note the recurrence
- If a finding is new: create a new lesson entry before the fix cycle begins

### Step 4: Produce Verdict

Structure the report with evidence.

## Review Checklist (Universal)

### Function Size

- Domain functions: within project-defined limits (typically 20-30 lines)
- Command/handler functions: within project-defined limits (typically 30-50 lines)
- No function exceeds the project's maximum — extract helpers

### Error Handling

- All functions handle errors explicitly (Result types, try/catch, error returns)
- No silent error suppression (empty catch blocks, ignore patterns)
- Error types are specific, not generic strings
- User-facing errors are helpful; internal errors go to logs

### Type Safety

- Strict type checking enabled (no escape hatches like `any`, `unsafe` without justification)
- Types at API boundaries are explicit and documented
- No type coercion that could lose data

### Stub Detection

- No functions that return hardcoded values without implementation
- No TODO/FIXME comments in committed code
- No commented-out code blocks
- No placeholder implementations

### Architecture Compliance

- Code follows the project's layered architecture
- No boundary violations (e.g., UI layer accessing database directly)
- Types consistent across layer boundaries

## Forbidden Patterns (Flag Immediately)

- Production error suppression (empty catch, silent defaults)
- Type safety escape hatches without documented justification
- Hardcoded secrets or credentials
- Direct database access from wrong layers
- Tests that don't actually assert anything

## Review Output Format

```markdown
## Code Review: [scope]

### Automated Checks
- [check]: PASS/FAIL (with output)

### Findings
#### BLOCKING
- [file:line] Description — evidence

#### WARNING
- [file:line] Description — evidence

### Lessons Logged
- New entries: [list or "none"]
- Recurrence updates: [list or "none"]
- Checked lessons: YES

### Verdict: PASS / FAIL
```

## Critical Rules

- NEVER approve code without running automated checks first
- NEVER implement fixes — send findings back to the implementer
- NEVER skip the lesson check
- NEVER declare "minor" issues as acceptable — all findings are reported
- NEVER self-approve — you verify others' work, not your own
- Always include evidence with every finding
