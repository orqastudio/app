---
id: KNOW-477f2c9c
type: knowledge
status: active
title: "OrqaStudio Agentic Workflow and Enforcement Pipeline"
description: "End-to-end: how agents do structured work, knowledge injection, schema-driven validation, three-layer enforcement stack, and the artifact graph — the complete operational model"
tier: always
created: 2026-03-29
roles: [orchestrator, implementer, reviewer, planner, governance-steward]
paths: [engine/, .orqa/, connectors/]
tags: [architecture, workflow, agents, enforcement, knowledge-injection, artifact-graph]
relationships:
  - type: synchronised-with
    target: DOC-e16aea3b
---

# OrqaStudio Agentic Workflow and Enforcement Pipeline

## Core Concept

Agents do the work. The framework ensures quality. Nothing is hardcoded — plugin schemas define what's valid, the validation engine reads those schemas, the enforcement stack reads the engine.

## The Agentic Workflow

```text
User request
    │
    ▼
Orchestrator (coordinates, delegates, reads summaries — never implements)
    │
    ▼
Specialist agents spawned with:
    ├── Role (Implementer, Reviewer, Researcher, Writer, Designer, Planner, Governance Steward)
    ├── Declared knowledge (from agent employs relationships)
    ├── Task-relevant knowledge (from ONNX semantic search)
    └── Workflow context (from active methodology stage)
    │
    ▼
Agent produces work → Independent Reviewer verifies → PASS or fix cycle
```

## Agent Role Boundaries

| Role | Permitted | Forbidden |
| ------ | --------- | --------- |
| **Orchestrator** | Coordinate, delegate, read summaries | Write implementation code |
| **Implementer** | Write code, run tests | Self-certify quality |
| **Reviewer** | Verify against AC, produce PASS/FAIL | Fix issues they find |
| **Researcher** | Read, investigate, create research artifacts | Modify non-research files |
| **Writer** | Create/edit documentation | Run system commands |
| **Planner** | Design approaches, map dependencies | Write code |
| **Designer** | Design interfaces, create component code | Run system commands |
| **Governance Steward** | Maintain `.orqa/` artifacts | Modify source code |

Every task requires an independent Reviewer PASS before acceptance. The orchestrator never self-assesses.

## Knowledge Injection

Two complementary mechanisms ensure agents have the right context:

1. **Declared knowledge** — `employs` relationships in agent definition → always loaded at spawn
2. **Semantic knowledge** — task description → ONNX search → task-relevant KNOWs → deduplicated → injected

Together: deterministic base knowledge + dynamic situational knowledge = agent has what it needs without manual enumeration.

## Schema-Driven Validation

Everything is defined by plugin schemas:

- Artifact types (what types exist)
- Required fields (what frontmatter each type must have)
- Valid statuses (what states an artifact can be in)
- Valid relationships (what types connect which artifact types)

The validation engine reads these schemas. It never hardcodes valid values. Extend a schema → enforcement follows automatically across all three layers.

## The Three-Layer Enforcement Stack

| Layer | When | What |
| ------- | ------ | ------ |
| **Layer 1: LSP** | Real-time, while editing | Schema violations, broken references, invalid statuses — red squiggles |
| **Layer 2: Behavioral rules** | At agent spawn, via prompt injection | Documentation-first, delegation boundaries, pillar alignment, honest reporting |
| **Layer 3: Pre-commit** | On `git commit` | Hard gate — ALL checks, blocks commit on any error |

Three surfaces, one pipeline. Layer 1 catches most mechanical errors before commit. Layer 2 guides agent decisions. Layer 3 catches anything that escaped Layers 1-2.

## The Artifact Graph

Nodes are artifacts (markdown + YAML frontmatter in `.orqa/`). Edges are relationships declared in frontmatter. Relationships are **forward-only** — source artifact declares the relationship, graph engine computes inverses.

The graph provides end-to-end traceability: from discovery vision through planning decisions to implementation tasks and learning lessons.

## Plugin + Three-Way Diff Model

Plugins are the canonical source. `.orqa/` holds installed copies. When a plugin updates:

1. Plugin source (new version)
2. Installed baseline (what was installed before, from `manifest.json` hashes)
3. Project copy (current `.orqa/`, may have local edits)

Three-way diff detects plugin updates AND local edits, merges intelligently.

## The Full Lifecycle

```text
1. Write artifact → LSP validates in real-time (Layer 1)
2. Agent spawned → knowledge injection (declared + semantic)
3. Agent works → follows behavioral rules (Layer 2)
4. orqa check → validation on demand
5. Pre-commit gate → hard block on any violation (Layer 3)
6. Committed and pushed
```
