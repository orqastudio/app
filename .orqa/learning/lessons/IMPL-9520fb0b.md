---

id: IMPL-9520fb0b
type: lesson
title: Project-level prioritization system needed to enforce priority principles automatically
description: "Priority should be set at the project level with configurable dimensions and automatic classification. For OrqaStudio: data integrity and filling gaps in process/principles are always CRITICAL. Tools that improve/enforce agentic workflow principles take precedence over feature work."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships:
  - type: cautions
    target: EPIC-a60f5b6b
    rationale: "Project-level prioritization system needed for enforcement — cautions enforcement foundations"
---

## Pattern

Priority is currently assessed per-epic using freeform `scoring` dimensions. There's no project-level declaration of what matters most — each prioritization decision starts from scratch. For OrqaStudio, the user established a clear principle: data integrity and process/principle gap-filling are always CRITICAL, and tools that improve the agentic workflow come before feature work. But this principle lives in conversation, not in the project configuration.

## Fix

Create a project-level priority configuration in `project.json` or a dedicated artifact that declares:

1. **Priority dimensions** with weights (configurable per project)
2. **Automatic classification rules** (e.g., "if touches data integrity → CRITICAL")
3. **Priority ordering principles** (e.g., "enforcement tooling before features")
4. The orchestrator reads this config when assessing epic priority instead of making ad-hoc judgements
