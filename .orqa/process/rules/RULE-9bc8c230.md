---
id: RULE-9bc8c230
type: rule
title: Behavioral Rule Enforcement Plan
description: "Defines enforcement strategies for rules that cannot be mechanically checked by linters, hooks, or tooling. Every behavioral rule has a defined enforcement mechanism: prompt injection, output validation, knowledge injection, or session hooks."
status: active
created: 2026-03-13
updated: 2026-03-13
enforcement:
  - mechanism: behavioral
    message: "Every behavioral rule maps to a defined enforcement strategy (prompt injection, output validation, knowledge injection, or session hooks); the orchestrator injects this rule as the authoritative reference"
  - mechanism: tool
    command: "orqa enforce"
    description: "Pipeline integrity tool verifies behavioral rule enforcement coverage"
relationships:
  - target: AD-f9034c99
    type: enforces
---
Rules that cannot be enforced by linters, hooks, or automated tooling still need a defined enforcement mechanism. Every behavioral rule maps to one of four strategies, and each strategy has a concrete implementation path.

## Enforcement Strategies

### Strategy 1: Prompt Injection

Rule content is injected into the agent's context at delegation time. The orchestrator includes the rule's constraints in the delegation prompt, making them part of the agent's active instructions.

| Rule | What is injected |
|------|-----------------|
| [RULE-532100d9](RULE-532100d9) | Delegation boundaries — orchestrator coordinates, doesn't implement |
| [RULE-5e03e67b](RULE-5e03e67b) | Search usage — prefer semantic search over Grep/Glob |
| [RULE-c71f1c3f](RULE-c71f1c3f) | Make targets — use make commands, not raw cargo/npm |
| [RULE-22783309](RULE-22783309) | ID semantics — IDs are identifiers, not priority rankings |
| [RULE-b2753bad](RULE-b2753bad) | Required reading — load governing docs before implementation |
| [RULE-deab6ea7](RULE-deab6ea7) | Knowledge loading — load knowledge before starting work |
| [RULE-8035e176](RULE-8035e176) | Structure before work — artifacts must exist before implementation |
| [RULE-df24948b](RULE-df24948b) | Context management — minimize orchestrator context window usage |

**Implementation**: The orchestrator's delegation template includes these rules by reference. The companion plugin's prompt injector (`prompt-injector.ts`) auto-injects relevant rule IDs when task artifacts are referenced.

### Strategy 2: Output Validation

Post-hoc checks on agent output for compliance signals. After an agent completes work, its output is checked for required sections and forbidden language.

| Rule | What is validated |
|------|------------------|
| [RULE-878e5422](RULE-878e5422) | Honest reporting — check for "What Is NOT Done" section in completion reports |
| [RULE-551bde31](RULE-551bde31) | Lessons learned — check for IMPL entries mentioned in review output |
| [RULE-e120bb70](RULE-e120bb70) | No deferred deliverables — check completion reports for deferral language ("handled by EPIC-NNN", "wired up later") |
| [RULE-303c1cc8](RULE-303c1cc8) | Plan compliance — check plan structure for required sections (Architectural Compliance, Systems Architecture Checklist) |

**Implementation**: A `PostToolUse` hook or stop hook scans agent output for compliance markers. Initially implemented as orchestrator self-checks; automated via plugin hooks as patterns stabilize.

### Strategy 3: Knowledge Injection

Domain knowledge is loaded into agent context before work begins on relevant files. The enforcement engine auto-injects knowledge based on file paths being modified.

| Rule | When injected |
|------|--------------|
| [RULE-65973a88](RULE-65973a88) | AD compliance — architecture knowledge injected when modifying cross-boundary code |
| [RULE-9daf29c0](RULE-9daf29c0) | Documentation first — documentation knowledge injected when creating new features |
| [RULE-3eccebf3](RULE-3eccebf3) | Enforcement before code — governance knowledge injected when modifying rules/lessons |
| [RULE-d90112d9](RULE-d90112d9) | Systems thinking — systems-thinking knowledge injected on all implementation work |
| [RULE-4d4f540d](RULE-4d4f540d) | UAT process — uat-process knowledge injected during review/testing phases |

**Implementation**: [RULE-f9d0279c](RULE-f9d0279c) defines the path-to-knowledge injection map. The companion plugin's `PostToolUse` hook on Write/Edit triggers knowledge injection.

### Strategy 4: Session Hooks

Plugin hooks that trigger at session boundaries (start, end, stop) to enforce workflow rules.

| Rule | When checked |
|------|-------------|
| [RULE-633e636d](RULE-633e636d) | Git workflow — session-start checks for stashes and untracked files; session-end verifies all changes committed |
| [RULE-e352fd0a](RULE-e352fd0a) | Session management — session-end checks for uncommitted changes and writes session state |

**Implementation**: The companion plugin's `SessionStart` hook (`session-start.sh`) and `Stop` hook (`stop-checklist.sh`) enforce these checks.

## Coverage Summary

| Category | Rule Count | Strategy |
|----------|-----------|----------|
| Prompt injection | 8 rules | Delegation template + plugin injector |
| Output validation | 4 rules | Stop hook + orchestrator self-check |
| Knowledge injection | 5 rules | [RULE-f9d0279c](RULE-f9d0279c) enforcement entries + plugin PostToolUse |
| Session hooks | 2 rules | Plugin SessionStart + Stop hooks |
| **Total behavioral** | **19 rules** | |

## Verification

To verify behavioral enforcement coverage:
1. Run `node app/tools/verify-pipeline-integrity.mjs` — reports rules without enforcement chains
2. Run `node app/tools/verify-enforcement-rules.mjs` — reports agent capability compliance
3. Cross-reference: every rule in this plan should appear in the pipeline integrity tool's enforcement chain data

## Related Rules

- [RULE-f9d0279c](RULE-f9d0279c) (knowledge-injection) — implements Strategy 3 via path-to-knowledge mapping
- [RULE-7f416d7d](RULE-7f416d7d) (tooling-ecosystem) — distinguishes linter-enforceable from behavioral rules
- [RULE-6083347d](RULE-6083347d) (dogfood-mode) — enforcement gap priority on self-enforcing products
