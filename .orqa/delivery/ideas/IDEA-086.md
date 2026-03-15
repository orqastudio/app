---
id: IDEA-086
title: "Test coverage enforcement plugin"
description: "A plugin that enforces test coverage thresholds at configurable granularity — per milestone gate, per epic completion, or per task acceptance. Integrates with cargo tarpaulin (Rust) and Vitest coverage (frontend) to gate artifact status transitions on coverage targets."
status: captured
created: "2026-03-13"
updated: "2026-03-13"
pillars:
  - PILLAR-001
  - PILLAR-002
horizon: someday
research-needed:
  - "What coverage tool outputs can be parsed? tarpaulin JSON, Vitest JSON, lcov, cobertura?"
  - "How should thresholds be configured — per-project in project.json, per-milestone, per-epic?"
  - "Should coverage enforcement be a milestone gate (blocking MS completion), epic gate (blocking epic done), or both?"
  - "How does this interact with the integrity engine? New IntegrityCategory for coverage violations?"
  - "Can coverage deltas be tracked per-commit to prevent regressions?"
  - "Plugin vs core feature: is this universal enough for core, or better as an official plugin?"
relationships:
  - target: IDEA-004
    type: informs
    rationale: "CI/CD integration would consume coverage data from the same pipeline"
  - target: IDEA-087
    type: informed-by
    rationale: "Auto-generated inverse of informed-by relationship from IDEA-087"
---
## Motivation

RULE-029 requires 80%+ test coverage but enforcement is manual — agents check coverage during review but there's no automated gate. A plugin could make coverage a first-class integrity check, blocking status transitions when coverage drops below threshold. This turns coverage from a guideline into an enforceable constraint, consistent with the Knowledge Maturity Pipeline philosophy of enforcement through visibility.
