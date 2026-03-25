---
id: TASK-72d1e17e
type: task
title: "Create @orqastudio/test-config package — shared vitest + testing utilities"
description: "Extract shared test configuration (vitest config, test utilities, fixture helpers) into a standalone package. All orqastudio repos and plugins use this for consistent testing with full CI/CD integration."
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 3
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - New repo orqastudio/orqastudio-test-config exists with CI + publish workflow
  - Exports shared vitest config (base vitest.config.ts for extends)
  - "Exports test utilities: artifact fixture builders, graph builders, mock invoke"
  - "Published to GitHub Packages as @orqastudio/test-config"
  - "All orqastudio repos (integrity-validator, sdk, types) use this package"
  - CI/CD runs full test suite on every PR and push to main
relationships:
  - target: EPIC-90cb7349
    type: delivers
    rationale: Shared testing framework for the package ecosystem
---

## Scope

### Shared vitest config
- Base `vitest.config.ts` with coverage thresholds, reporter settings
- Consistent test file patterns (`**/*.test.ts`)
- Coverage targets (80%+ per module)

### Test utilities
- `createTestGraph(nodes)` — build an ArtifactGraph from minimal node descriptions
- `createTestNode(overrides)` — build an ArtifactNode with sensible defaults
- `mockInvoke(commands)` — mock Tauri invoke for store testing
- Fixture builders for common artifact types (task, epic, idea, rule, etc.)

### CI/CD
- GitHub Actions workflow template that repos can reference
- Test + lint + build on every PR
- Coverage report upload