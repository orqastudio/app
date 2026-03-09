---
id: test-engineering
layer: canon
title: "Test Engineering"
name: test-engineering
description: |
  Test engineering methodology: test organisation, coverage requirements, mock
  boundaries, test writing standards, and the discipline of testing behavior
  not implementation. Portable across backend and frontend test frameworks.
  Use when: Writing tests, setting up test infrastructure, defining mock
  boundaries, or enforcing test coverage requirements.
version: 1.0.0
tags: [testing, test-engineering, coverage, mocking, tdd]
user-invocable: true
---


Test engineering methodology. This skill teaches *how to think about testing* — the specific test frameworks, commands, and file locations come from the project's testing skill.

## Test Organisation Principles

| Test Type | Scope | What It Verifies |
|-----------|-------|-----------------|
| **Unit tests** | Single module/function | Logic, edge cases, error handling |
| **Integration tests** | Cross-module flows | Boundaries work together correctly |
| **Component tests** | UI components in isolation | Rendering, interactions, state |
| **E2E tests** | Full user flows | The whole system works for the user |

Tests should be colocated with the code they test (not in a separate tree), unless the project convention says otherwise.

## Mock Boundaries

**Mock ONLY at the adapter/boundary layer.** Never mock domain logic, orchestrators, or internal functions.

| Correct | Forbidden |
|---------|-----------|
| Mock a repository trait for testing without a database | Mock domain logic or internal functions |
| Mock a file system trait for testing without disk | Mock private methods |
| Mock an API client for testing without network | Mock the thing you're testing |
| Mock IPC bridge in frontend component tests | Mock internal state management |

## Coverage Requirements

- Backend domain logic: highest coverage (80%+)
- Command/handler layer: moderate coverage (70%+)
- Frontend stores: high coverage (80%+) — all state transitions
- Frontend components: moderate coverage (60%+) — key interactions
- E2E: all primary user flows covered

## What to Test

- All public functions in domain logic
- All error paths and edge cases
- All API/command handlers (input validation, error responses)
- All state transitions in stores (loading -> loaded, loading -> error)
- User interactions (click, type, keyboard) — not implementation details

## What NOT to Test

- Framework glue (command registration wiring, route configuration)
- Trivial getters/setters with no logic
- Third-party library internals
- Type definitions and struct declarations (the compiler handles these)

## Test Writing Standards

| Standard | Why |
|----------|-----|
| Test names describe behavior | `rejects_empty_name`, not `test_new` |
| Each test verifies one behavior | Split tests with multiple unrelated assertions |
| Tests must be independent | No shared mutable state between tests |
| Tests must be deterministic | No flaky tests from timing, randomness, or external deps |
| Arrange-Act-Assert pattern | Setup, perform action, verify result |
| Clean up artifacts in teardown | File system, temp databases, etc. |

## Test-Driven Development

For new features:

1. Write a failing test that describes the expected behavior
2. Write the minimum code to make the test pass
3. Refactor while keeping tests green
4. Repeat

For bug fixes:

1. Write a failing test that reproduces the bug
2. Fix the bug
3. Verify the test passes (this becomes the regression test)

## Critical Rules

- NEVER write tests that depend on execution order
- NEVER write tests that pass by coincidence
- NEVER leave failing tests in the codebase — fix them or delete with justification
- NEVER mock the thing you're testing — only mock its dependencies
- Every bug fix must include a regression test
- Use the project's test runner commands, not raw framework commands
