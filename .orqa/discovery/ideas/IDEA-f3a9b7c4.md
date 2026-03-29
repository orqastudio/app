---
id: IDEA-f3a9b7c4
type: discovery-idea
title: "Scan codebase for governance-enforceable patterns beyond code quality"
description: "Identify non-code artifacts in the codebase that could benefit from plugin-based mechanical enforcement via the generator/contributor pattern"
status: captured
priority: medium
created: "2026-03-29"
tags:
  - enforcement
  - plugins
  - governance
  - automation
---

## Context

The universal enforcement pattern (rules → plugin generator → composed config → `orqa check`) applies to any mechanical enforcement, not just code linting. Scan the current codebase for governance-related patterns that could become enforcement plugins.

## Candidates to Investigate

- **LICENSE files** — consistent format, required fields, correct license type across all packages
- **README structure** — required sections (description, install, usage, contributing), consistent format across packages/plugins
- **Package.json standards** — required fields, version consistency, license field, repository field
- **Cargo.toml standards** — required metadata, consistent edition/license across workspace members
- **Changelog format** — consistent format, required sections per release
- **Git commit messages** — conventional commits, required scope, length limits
- **API documentation** — required sections for public APIs
- **Security policy** — SECURITY.md existence and required content
- **Code of conduct** — existence and required content

## Key Principle

These are currently hardcoded in the CLI (e.g., `cli/src/lib/license.ts`, `cli/src/lib/readme.ts`). They should be moved out of the CLI into configurable enforcement plugins that users can customize via rules. The CLI just runs `orqa check` — what gets checked comes from installed plugins and user-defined rules.

## Approach

1. Audit the current codebase for recurring structural patterns in non-code files
2. Identify what's currently hardcoded in the CLI that should be plugin-configurable
3. For each pattern, assess: can it be expressed as rules + mechanical check?
4. If yes, design the enforcement plugin (generator + rule format + check command)
5. Prioritize by: frequency of manual checking today, cost of getting it wrong, ease of automation
