---
id: KNOW-477f2c9c
type: knowledge
title: Agentic Workflow and Enforcement Pipeline
summary: "This is the master reference for how OrqaStudio works as a system. It describes the complete pipeline from user request to committed code, covering agent delegation, knowledge injection, artifact graph traversal, schema-driven validation, and the three-layer enforcement stack."
description: |
  The complete OrqaStudio execution model: how user requests become agent-delegated work,
  how knowledge flows into agent context via declared and semantic injection, how the
  artifact graph connects everything, and how the three-layer enforcement stack (LSP,
  behavioral rules, pre-commit) validates quality. Use when: understanding the system
  architecture, designing new enforcement, configuring agent knowledge, adding artifact
  types, or explaining how OrqaStudio works.
status: active
created: 2026-03-24
updated: 2026-03-24
category: architecture
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-e16aea3b
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
  - target: KNOW-22783288
    type: references
    rationale: "CLI architecture is the developer interface into this workflow"
  - target: KNOW-dd5062c9
    type: realises
    rationale: "Shared validation engine implements enforcement Layers 1 and 3"
  - target: KNOW-1f4aba8f
    type: crystallises
    rationale: "Three-layer enforcement model is the detailed enforcement breakdown"
  - target: KNOW-586bfa9a
    type: references
    rationale: "Knowledge auto-injection is the detailed injection mechanism"
---

## System Model

```text
User request → Orchestrator → Agent spawned with knowledge → Work → Reviewer → Commit
```text

Orchestrator coordinates (never implements). Each agent receives: declared knowledge (employs relationships), semantic knowledge (ONNX search), and governing documentation.

## Agent Roles

| Role | May Do | May NOT Do |
| ------ | -------- | ----------- |
| Orchestrator | Coordinate, delegate, query graph | Write code, run tests |
| Implementer | Write/edit code, run builds | Self-certify quality |
| Reviewer | Run checks, read files | Edit files, fix issues |
| Researcher | Read, search | Modify files |
| Writer | Write docs | Run shell commands |

## Knowledge Injection

- **Declared**: Agent frontmatter `employs` relationships loaded at spawn (deterministic)
- **Semantic**: ONNX embedding search on task description (dynamic, deduplicated)

## Artifact Graph

- **Nodes**: Markdown files with YAML frontmatter in `.orqa/`
- **Edges**: `relationships` array in frontmatter
- **Types**: Defined by plugin schemas
- **Query**: `orqa graph --type rule --status active`

## Three-Layer Enforcement

| Layer | When | What | Character |
| ------- | ------ | ------ | ----------- |
| LSP | Real-time | Schema validation (status, types, refs) | Deterministic |
| Behavioral rules | Agent spawn | Judgement rules (doc-first, boundaries) | Requires judgement |
| Pre-commit | On commit | All schema + lint + test | Hard gate |

## Plugin Architecture

Plugins are source of truth. `.orqa/` contains installed copies. Three-way diff on refresh detects both plugin updates and local edits.

| Content | Plugin Location | Installed To |
| --------- | ---------------- | ------------- |
| Schemas | `orqa-plugin.json` | Read by validation engine |
| Agents/Knowledge/Rules | `agents/`, `knowledge/`, `rules/` | `.orqa/process/` |
| Docs | `docs/` | `.orqa/documentation/` |

## Doc + Knowledge Pairing

Every doc has a paired knowledge artifact linked by `synchronised-with`. Doc = human-facing, Knowledge = agent-facing. Updated together.

## FORBIDDEN

- Hardcoding types/statuses outside plugin schemas
- Validation logic outside the shared engine
- Bypassing pre-commit with `--no-verify`
- Spawning agents without knowledge injection
- Orchestrator writing code; Reviewer fixing issues
- Updating doc without updating paired knowledge
