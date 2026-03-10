---
id: DOC-023
title: Contributing
description: Guide for contributors covering setup, workflow, and conventions for the OrqaStudio codebase.
created: "2026-03-07"
updated: "2026-03-07"
---

# Contributing to OrqaStudio™

**Date:** 2026-03-07

Thank you for your interest in contributing to OrqaStudio. This guide covers the workflow, expectations, and quality standards for contributions.

---

## Before You Start

1. Read the [Getting Started](getting-started.md) guide to set up your development environment
2. Read the [Architecture Overview](architecture-overview.md) to understand how the system fits together
3. Read the [Coding Standards](coding-standards.md) to understand the quality expectations

---

## Finding Work

- **Issues:** Check the [GitHub Issues](https://github.com/orqastudio/orqastudio-app/issues) for open tasks. Issues labelled `good first issue` are suitable for new contributors.
- **Discussions:** Open a [GitHub Discussion](https://github.com/orqastudio/orqastudio-app/discussions) if you have an idea but are unsure how it fits the project.
- **Roadmap:** See `.orqa/documentation/product/roadmap.md` for the project direction. Features not on the roadmap should be discussed before implementation.

---

## Contribution Workflow

### 1. Fork and Branch

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/<your-username>/orqastudio-app.git
cd orqastudio-app

# Create a branch from main
git checkout -b <type>/<short-description>
```

Branch naming convention:

| Prefix | Use for |
|--------|---------|
| `feat/` | New features |
| `fix/` | Bug fixes |
| `docs/` | Documentation changes |
| `refactor/` | Code restructuring without behaviour change |
| `test/` | Test additions or fixes |

### 2. Make Changes

- Keep commits focused — one logical change per commit
- Write descriptive commit messages explaining *why*, not just *what*
- Follow the [Coding Standards](coding-standards.md)

### 3. Verify Quality

Run the full check suite before pushing:

```bash
make check
```

This runs formatting, linting, type checking, and tests for both Rust and frontend. All checks must pass. See [Development Commands](commands.md) for individual targets.

### 4. Submit a Pull Request

- Push your branch to your fork
- Open a pull request against `main` on the upstream repository
- Fill in the PR template describing your changes, motivation, and how to test them
- Link any related issues

### 5. Review Process

- A maintainer will review your PR for correctness, style, and alignment with project direction
- Feedback is collaborative — we aim for constructive discussion
- Once approved, a maintainer will merge your PR

---

## Quality Standards

All contributions must meet these standards:

### Rust

- `cargo fmt --check` passes (zero formatting issues)
- `cargo clippy -- -D warnings` passes (zero warnings)
- `cargo test` passes (all tests green)
- No `unwrap()` or `expect()` in production code
- Functions under 50 lines

### Frontend

- `npm run check` passes (svelte-check + TypeScript)
- `npm run lint` passes (ESLint)
- `npm run test` passes (Vitest)
- Svelte 5 runes only — no Svelte 4 patterns
- Strict TypeScript — no `any` types

### Both

- New functionality has tests
- No TODO comments in committed code
- No commented-out code

---

## Types of Contributions

### Bug Reports

Open an issue with:

- Steps to reproduce
- Expected behaviour
- Actual behaviour
- Platform and version information

### Feature Requests

Open a discussion with:

- Problem description — what are you trying to do?
- Proposed solution — how do you think it should work?
- Alternatives considered

Feature requests that align with the [product vision](../product/vision.md) and serve at least one of the two pillars (Clarity Through Structure or Learning Through Reflection) are more likely to be accepted.

### Documentation

Documentation improvements are always welcome. Project documentation in `.orqa/documentation/` is licensed under CC BY 4.0. See `docs/LICENSE` for terms.

### Code

Code contributions should be discussed in an issue or discussion first for anything beyond small bug fixes. This avoids wasted effort on changes that may not align with the project direction.

---

## Licensing

By contributing to OrqaStudio, you agree that your contributions will be licensed under the project's existing licenses:

- **Code:** Apache License 2.0 (see [LICENSE](../../LICENSE))
- **Documentation:** Creative Commons Attribution 4.0 (see [docs/LICENSE](../LICENSE))

---

## Code of Conduct

Be respectful, constructive, and collaborative. We are building a tool to bring clarity to complex problems — we aim to practice the same clarity in our interactions.

---

## Related Documents

- [Getting Started](getting-started.md) — Environment setup
- [Architecture Overview](architecture-overview.md) — System design
- [Coding Standards](coding-standards.md) — Quality rules
- [Development Commands](commands.md) — Build, test, and lint commands
