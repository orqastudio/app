---
id: IDEA-003
title: "Entry Modes & Directed Onboarding"
description: "AI-assisted onboarding flows beyond chat guidance, with each mode supporting new projects and existing work adaptation."
status: completed
created: 2026-03-07
updated: 2026-03-13
research-needed:
  - UX research for each entry mode flow
  - Domain-agnostic template design
  - Mode detection accuracy testing
relationships:
  - target: RES-013
    type: informed-by
    rationale: "Tool display research informed entry mode UX design"
  - target: RES-012
    type: informed-by
    rationale: "First-run setup wizard research informed onboarding flow design"
  - target: EPIC-035
    type: evolves-into
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-001
    type: grounded-by
---
## Candidate Items

- Problem mode — guided diagnosis flow with root cause mapping
- Idea mode — validation flow with feasibility exploration
- Goal mode — planning flow with gap analysis
- Chaos mode — triage flow with situation mapping
- Existing work assessment — AI scans folder structure, identifies contents (docs, code, config, READMEs), and proposes an .orqa/ hierarchy that reflects what already exists rather than imposing a template. Discovery-based, not requirements-based.
- Mode convergence — all paths produce same artifact structure
- Domain-agnostic templates — personal planning, healthcare, research, consulting
- Mode detection — AI suggests most appropriate entry mode
