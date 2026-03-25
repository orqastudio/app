---
id: IDEA-54ea251a
type: idea
title: "Improve CLI integrity scanner to match app's Rust checks"
description: "The CLI verify-links.mjs only checks markdown link syntax and relationship entries. The app's Rust integrity scan checks all frontmatter reference fields (skills, docs-required, docs-produced, milestone, epic, etc.), dependency violations, supersession symmetry, research gaps, and planning placement. The CLI should have parity."
status: captured
created: 2026-03-14
updated: 2026-03-14
horizon: next
research-needed:
  - "Should the CLI scanner be a Node port of the Rust checks, or should we expose the Rust scanner via a CLI binary?"
  - "Which checks are most valuable to run at commit time vs full scan time?"
relationships:
  - target: EPIC-90cb7349
    type: realises
    rationale: "CLI scanner parity idea drove the integrity validator package extraction"
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---
## Motivation

During UAT, the app's integrity scan found 54 errors that the CLI `verify-links.mjs` reported as 0 errors. The gap is significant:

| Check | App (Rust) | CLI (Node) |
|-------|-----------|------------|
| Broken links (markdown) | Yes | Yes |
| Broken refs (frontmatter fields) | Yes | No |
| Missing inverses | Yes | Yes |
| Dependency violations | Yes | No |
| Supersession symmetry | Yes | No |
| Research gaps | Yes | No |
| Planning placement | Yes | No |
| Circular dependencies | Yes | No |
| Milestone gates | Yes | No |
| Idea promotion validity | Yes | No |

The pre-commit hook runs CLI tools, so these errors slip through undetected.