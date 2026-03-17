---
id: TASK-465
title: Define Governance Steward agent
description: Create a specialist agent definition for all .orqa/ artifact creation and maintenance. The orchestrator delegates governance work to this agent instead of writing artifacts directly.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - governance-steward.md created in .orqa/process/agents/ with full frontmatter
  - Agent has grounded-by relationships to artifact-principles and product-purpose grounding docs
  - "Agent system prompt includes graph discipline (bidirectional relationships, schema compliance, pillar alignment)"
  - "Agent skills include orqa-governance, orqa-documentation, orqa-schema-compliance, migration-tooling"
  - "Agent capabilities include file_read, file_write, file_edit, file_search, content_search"
  - RULE-001 updated to include Governance Steward in the delegation table
  - Orchestrator prompt updated to reference Governance Steward for all .orqa/ artifact work
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 2 — specialist agent for governance integrity
  - target: IMPL-055
    type: informed-by
    rationale: Missing graph integrity lesson drove the need for a specialist governance agent
  - target: TASK-463
    type: depends-on
  - target: SKILL-011
    type: informed-by
  - target: SKILL-037
    type: informed-by
  - target: TASK-466
    type: depended-on-by
  - target: SKILL-011
    type: informed-by
  - target: SKILL-037
    type: informed-by
---

## Scope

### Agent Definition

Create `.orqa/process/agents/governance-steward.md` with:

**Purpose section** (grounding):
- You maintain the integrity of the artifact graph
- Every artifact you create has correct frontmatter, bidirectional relationships, pillar alignment, and schema compliance
- You are the reason the orchestrator can coordinate without worrying about graph discipline
- If an artifact has a one-sided relationship, you have failed

**Skills**:
- orqa-governance — artifact conventions
- orqa-documentation — writing standards
- orqa-schema-compliance — frontmatter validation
- migration-tooling — relationship backfill and verification

**Capabilities**:
- file_read, file_write, file_edit, file_search, content_search, code_search_regex

**Grounding** (via grounded-by relationships):
- artifact-principles grounding doc
- product-purpose grounding doc

### Rule Updates

Update RULE-001's delegation table to include:

| Role | Purpose | Boundary |
|------|---------|----------|
| **Governance Steward** | Create and maintain .orqa/ artifacts | Does NOT coordinate or delegate |

Update the orchestrator's "What You MUST Delegate" to include governance artifacts.
