---
id: TASK-a8eb165a
type: task
name: "Create agile-discovery plugin"
status: active
description: "Create plugins/agile-discovery/ as a stage-definition plugin for discovery-stage artifacts filling the discovery-artifacts contribution point"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
  - target: TASK-cd118e75
    type: depends-on
    rationale: "Plugin renames must complete before moving schemas from agile-methodology and software-kanban"
  - target: TASK-9495bc0f
    type: depends-on
    rationale: "agile-methodology skeleton must be renamed before creating contribution that targets it"
acceptance:
  - "plugins/agile-discovery/ exists with valid orqa-plugin.json"
  - "Defines artifact types: vision, persona, pillar, pivot, discovery-idea, discovery-research, discovery-decision"
  - "Contribution fills discovery-artifacts point of agile-methodology workflow"
  - "vision, persona, pillar, pivot, idea schemas and workflows no longer exist in agile-methodology"
  - "research schema and workflow no longer exist in software-kanban"
  - "Relationship types support intra-stage evolution chain: discovery-idea -> discovery-research -> discovery-decision"
  - "Vision -> pillar and vision -> pivot relationships defined"
  - "Cross-stage relationship types declared for discovery-decision -> planning-idea / planning-decision handoff"
  - "npx tsc --noEmit passes for libs/cli"
---

## What

Create `plugins/agile-discovery/` as a new stage-definition plugin for the discovery stage. This plugin defines all discovery-stage artifact types and fills the `discovery-artifacts` contribution point of the agile-methodology workflow.

Discovery-stage artifacts have different semantics from their planning-stage counterparts. A `discovery-idea` is a high-level product concept, while a `planning-idea` is a scoped implementation approach. Same concept name, different lifecycle and context.

## Knowledge Needed

- Read `plugins/agile-methodology/` (after TASK-1 rename) for:
  - `schemas/` directory: vision, persona, pillar, pivot, idea schemas to move
  - `workflows/` directory: vision.workflow.yaml, persona.workflow.yaml, pillar.workflow.yaml, pivot.workflow.yaml, idea.workflow.yaml to move
- Read `plugins/software-kanban/` (after TASK-1 rename) for:
  - `workflows/research.workflow.yaml` to move (rename type to discovery-research)
- Read `plugins/agile-methodology/workflows/agile-methodology.workflow.yaml` (after TASK-2 rename) for the contribution point structure
- Read `.orqa/connectors/claude-code/injector-config.json` for relationship type definitions
- Read RES-d6e8ab11 for the three-layer model and stage-scoped type rationale

## Agent Role

Implementer — create new plugin, move schemas/workflows, create new artifact types, define relationships.

## Steps

1. Create `plugins/agile-discovery/` directory
2. Create `plugins/agile-discovery/orqa-plugin.json` manifest:
   - `name`: `@orqastudio/plugin-agile-discovery`
   - NO `core:*` exclusive role
   - `provides.schemas`: vision, persona, pillar, pivot, discovery-idea, discovery-research, discovery-decision
   - `provides.workflows`: standalone workflows for each type + discovery contribution
   - `provides.relationships`: intra-stage and cross-stage relationship types
   - `content` mappings for `orqa install`
3. Create `plugins/agile-discovery/schemas/` directory and move schemas:
   - Move vision, persona, pillar, pivot schemas from `plugins/agile-methodology/schemas/`
   - Move idea schema from `plugins/agile-methodology/schemas/`, rename type to `discovery-idea`
   - Move research schema from `plugins/software-kanban/schemas/`, rename type to `discovery-research`
   - Create `discovery-decision` schema (high-level strategic decisions)
4. Create `plugins/agile-discovery/workflows/` directory and move workflows:
   - Move vision.workflow.yaml, persona.workflow.yaml, pillar.workflow.yaml, pivot.workflow.yaml from agile-methodology
   - Move idea.workflow.yaml from agile-methodology, rename type to `discovery-idea`
   - Move research.workflow.yaml from software-kanban, rename type to `discovery-research`
   - Create discovery-decision.workflow.yaml
   - Create discovery.contribution.workflow.yaml that fills `discovery-artifacts` point of `agile-methodology`
5. Define relationship types in the manifest:
   - Intra-stage: discovery-idea -> discovery-research (evolves-to), discovery-research -> discovery-decision (evolves-to)
   - Vision relationships: vision -> pillar (drives), vision -> pivot (pivots)
   - Cross-stage joins: discovery-decision -> planning-idea (hands-off-to), discovery-decision -> planning-decision (informs)
6. Remove moved schemas/workflows from agile-methodology and software-kanban manifests
7. Run `npx tsc --noEmit` in `libs/cli`

## Verification

- `test -d plugins/agile-discovery && echo PASS || echo FAIL`
- `test -f plugins/agile-discovery/orqa-plugin.json && echo PASS || echo FAIL`
- `grep -c 'discovery-idea\|discovery-research\|discovery-decision\|vision\|persona\|pillar\|pivot' plugins/agile-discovery/orqa-plugin.json` should be >= 7
- `test -f plugins/agile-discovery/workflows/discovery.contribution.workflow.yaml && echo PASS || echo FAIL`
- `grep 'discovery-artifacts' plugins/agile-discovery/workflows/discovery.contribution.workflow.yaml | wc -l` should be >= 1
- `test ! -f plugins/agile-methodology/workflows/idea.workflow.yaml && echo PASS || echo FAIL`
- `test ! -f plugins/agile-methodology/workflows/vision.workflow.yaml && echo PASS || echo FAIL`
- `cd libs/cli && npx tsc --noEmit`
