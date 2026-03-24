---
id: RULE-5b34e7db
type: rule
title: "Agents check but do not commit"
description: "Agents must run quality checks on their code but must not commit. The orchestrator reviews agent findings and commits. This ensures code review before every commit."
status: active
created: 2026-03-24
updated: 2026-03-24
enforcement:
  - mechanism: behavioral
    message: "Agents run orqa check (format, lint, test) on their changes. Agents do NOT run git commit. The orchestrator reviews and commits."
relationships:
  - target: RULE-633e636d
    type: extends
    rationale: "Extends git workflow with agent-specific commit discipline"
  - target: RULE-532100d9
    type: extends
    rationale: "Agents implement, orchestrator coordinates — committing is coordination"
---

## The Rule

Agents MUST run quality checks on their code. Agents MUST NOT commit.

### Agent workflow
1. Write/edit code
2. Run `orqa check` (or the relevant subset: `cargo fmt`, `cargo clippy`, `npx eslint`, `npx svelte-check`)
3. Fix any failures
4. Write findings to disk (`tmp/team/<team>/task-<id>.md`)
5. Report completion to orchestrator

### Orchestrator workflow
1. Read agent findings
2. Review changes (diff, acceptance criteria)
3. Stage and commit

### Why

- Commits are review gates. An agent committing bypasses review.
- The orchestrator has the full context of what was planned vs delivered.
- Pre-commit hooks catch issues, but an agent hitting hook failures wastes tokens retrying — better to catch issues with `orqa check` before attempting to commit.
- The orchestrator can batch related changes into coherent commits with meaningful messages.

## FORBIDDEN

- Agents running `git commit`
- Agents running `git add` + `git commit`
- Agents bypassing pre-commit hooks
- Agents marking work as complete without running quality checks
