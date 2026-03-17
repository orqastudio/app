---
id: RULE-046
title: Behavioral Rule Enforcement Plan
description: "Defines enforcement strategies for rules that cannot be mechanically checked by linters, hooks, or tooling. Every behavioral rule has a defined enforcement mechanism: prompt injection, output validation, skill injection, or session hooks."
status: active
created: 2026-03-13
updated: 2026-03-13
layer: project
relationships:
  - target: RES-056
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-056
  - target: IMPL-050
    type: informed-by
  - target: PILLAR-002
    type: grounded
  - target: PILLAR-003
    type: grounded
  - target: PILLAR-002
    type: informs
  - target: PILLAR-003
    type: informs
---
Rules that cannot be enforced by linters, hooks, or automated tooling still need a defined enforcement mechanism. Every behavioral rule maps to one of four strategies, and each strategy has a concrete implementation path.

## Enforcement Strategies

### Strategy 1: Prompt Injection

Rule content is injected into the agent's context at delegation time. The orchestrator includes the rule's constraints in the delegation prompt, making them part of the agent's active instructions.

| Rule | What is injected |
|------|-----------------|
| [RULE-001](RULE-001) | Delegation boundaries — orchestrator coordinates, doesn't implement |
| [RULE-005](RULE-005) | Search usage — prefer semantic search over Grep/Glob |
| [RULE-007](RULE-007) | Make targets — use make commands, not raw cargo/npm |
| [RULE-016](RULE-016) | ID semantics — IDs are identifiers, not priority rankings |
| [RULE-023](RULE-023) | Required reading — load governing docs before implementation |
| [RULE-026](RULE-026) | Skill loading — load skills before starting work |
| [RULE-027](RULE-027) | Structure before work — artifacts must exist before implementation |
| [RULE-036](RULE-036) | Context management — minimize orchestrator context window usage |

**Implementation**: The orchestrator's delegation template includes these rules by reference. The companion plugin's prompt injector (`prompt-injector.mjs`) auto-injects relevant rule IDs when task artifacts are referenced.

### Strategy 2: Output Validation

Post-hoc checks on agent output for compliance signals. After an agent completes work, its output is checked for required sections and forbidden language.

| Rule | What is validated |
|------|------------------|
| [RULE-015](RULE-015) | Honest reporting — check for "What Is NOT Done" section in completion reports |
| [RULE-017](RULE-017) | Lessons learned — check for IMPL entries mentioned in review output |
| [RULE-019](RULE-019) | No deferred deliverables — check completion reports for deferral language ("handled by EPIC-NNN", "wired up later") |
| [RULE-022](RULE-022) | Plan compliance — check plan structure for required sections (Architectural Compliance, Systems Architecture Checklist) |

**Implementation**: A `PostToolUse` hook or stop hook scans agent output for compliance markers. Initially implemented as orchestrator self-checks; automated via plugin hooks as patterns stabilize.

### Strategy 3: Skill Injection

Domain knowledge is loaded into agent context before work begins on relevant files. The enforcement engine auto-injects skills based on file paths being modified.

| Rule | When injected |
|------|--------------|
| [RULE-002](RULE-002) | AD compliance — architecture skills injected when modifying cross-boundary code |
| [RULE-008](RULE-008) | Documentation first — documentation skills injected when creating new features |
| [RULE-011](RULE-011) | Enforcement before code — governance skills injected when modifying rules/lessons |
| [RULE-028](RULE-028) | Systems thinking — systems-thinking skill injected on all implementation work |
| [RULE-030](RULE-030) | UAT process — uat-process skill injected during review/testing phases |

**Implementation**: [RULE-042](RULE-042) defines the path-to-skill injection map. The companion plugin's `PostToolUse` hook on Write/Edit triggers skill injection.

### Strategy 4: Session Hooks

Plugin hooks that trigger at session boundaries (start, end, stop) to enforce workflow rules.

| Rule | When checked |
|------|-------------|
| [RULE-013](RULE-013) | Git workflow — session-start checks for stashes and untracked files; session-end verifies all changes committed |
| [RULE-039](RULE-039) | Session management — session-end checks for uncommitted changes and writes session state |

**Implementation**: The companion plugin's `SessionStart` hook (`session-start.sh`) and `Stop` hook (`stop-checklist.sh`) enforce these checks.

## Coverage Summary

| Category | Rule Count | Strategy |
|----------|-----------|----------|
| Prompt injection | 8 rules | Delegation template + plugin injector |
| Output validation | 4 rules | Stop hook + orchestrator self-check |
| Skill injection | 5 rules | [RULE-042](RULE-042) enforcement entries + plugin PostToolUse |
| Session hooks | 2 rules | Plugin SessionStart + Stop hooks |
| **Total behavioral** | **19 rules** | |

## Verification

To verify behavioral enforcement coverage:
1. Run `node tools/verify-pipeline-integrity.mjs` — reports rules without enforcement chains
2. Run `node tools/verify-enforcement-rules.mjs` — reports agent capability compliance
3. Cross-reference: every rule in this plan should appear in the pipeline integrity tool's enforcement chain data

## Related Rules

- [RULE-042](RULE-042) (skill-injection) — implements Strategy 3 via path-to-skill mapping
- [RULE-043](RULE-043) (tooling-ecosystem) — distinguishes linter-enforceable from behavioral rules
- [RULE-009](RULE-009) (dogfood-mode) — enforcement gap priority on self-enforcing products
