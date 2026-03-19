---
id: PILLAR-001
title: Clarity Through Structure
description: "Making thinking, standards, and decisions visible and structured."
status: active
created: 2026-03-09
updated: 2026-03-09
gate:
  - "Does this make governance artifacts visible and manageable?"
  - "Does it produce structured knowledge (plans, decisions, rules)?"
  - "Does it enforce a workflow that ensures understanding precedes action?"
  - "Does it surface what would otherwise be hidden in files, terminal output, or people's heads?"
  - "Does the system mechanically enforce its own structural rules?"
relationships:
  - target: VISION-001
    type: upholds
  - target: IDEA-004
    type: grounded-by
  - target: IDEA-005
    type: grounded-by
  - target: IDEA-007
    type: grounded-by
  - target: IDEA-008
    type: grounded-by
  - target: IDEA-010
    type: grounded-by
  - target: IDEA-011
    type: grounded-by
  - target: IDEA-013
    type: grounded-by
  - target: IDEA-014
    type: grounded-by
  - target: IDEA-015
    type: grounded-by
  - target: IDEA-016
    type: grounded-by
  - target: IDEA-018
    type: grounded-by
  - target: IDEA-019
    type: grounded-by
  - target: IDEA-020
    type: grounded-by
  - target: IDEA-021
    type: grounded-by
  - target: IDEA-023
    type: grounded-by
  - target: IDEA-024
    type: grounded-by
  - target: IDEA-025
    type: grounded-by
  - target: IDEA-026
    type: grounded-by
  - target: IDEA-027
    type: grounded-by
  - target: IDEA-028
    type: grounded-by
  - target: IDEA-029
    type: grounded-by
  - target: IDEA-030
    type: grounded-by
  - target: IDEA-031
    type: grounded-by
  - target: IDEA-032
    type: grounded-by
  - target: IDEA-033
    type: grounded-by
  - target: IDEA-034
    type: grounded-by
  - target: IDEA-035
    type: grounded-by
  - target: IDEA-036
    type: grounded-by
  - target: IDEA-037
    type: grounded-by
  - target: IDEA-039
    type: grounded-by
  - target: IDEA-040
    type: grounded-by
  - target: IDEA-041
    type: grounded-by
  - target: IDEA-043
    type: grounded-by
  - target: IDEA-044
    type: grounded-by
  - target: IDEA-045
    type: grounded-by
  - target: IDEA-046
    type: grounded-by
  - target: IDEA-048
    type: grounded-by
  - target: IDEA-049
    type: grounded-by
  - target: IDEA-050
    type: grounded-by
  - target: IDEA-051
    type: grounded-by
  - target: IDEA-052
    type: grounded-by
  - target: IDEA-053
    type: grounded-by
  - target: IDEA-054
    type: grounded-by
  - target: IDEA-055
    type: grounded-by
  - target: IDEA-056
    type: grounded-by
  - target: IDEA-058
    type: grounded-by
  - target: IDEA-059
    type: grounded-by
  - target: IDEA-060
    type: grounded-by
  - target: IDEA-061
    type: grounded-by
  - target: IDEA-062
    type: grounded-by
  - target: IDEA-063
    type: grounded-by
  - target: IDEA-064
    type: grounded-by
  - target: IDEA-065
    type: grounded-by
  - target: IDEA-066
    type: grounded-by
  - target: IDEA-067
    type: grounded-by
  - target: IDEA-068
    type: grounded-by
  - target: IDEA-069
    type: grounded-by
  - target: IDEA-070
    type: grounded-by
  - target: IDEA-071
    type: grounded-by
  - target: IDEA-072
    type: grounded-by
  - target: IDEA-073
    type: grounded-by
  - target: IDEA-074
    type: grounded-by
  - target: IDEA-075
    type: grounded-by
  - target: IDEA-076
    type: grounded-by
  - target: IDEA-077
    type: grounded-by
  - target: IDEA-078
    type: grounded-by
  - target: IDEA-079
    type: grounded-by
  - target: IDEA-080
    type: grounded-by
  - target: IDEA-081
    type: grounded-by
  - target: IDEA-082
    type: grounded-by
  - target: IDEA-083
    type: grounded-by
  - target: IDEA-084
    type: grounded-by
  - target: IDEA-085
    type: grounded-by
  - target: IDEA-086
    type: grounded-by
  - target: IDEA-090
    type: grounded-by
  - target: IDEA-091
    type: grounded-by
  - target: IDEA-092
    type: grounded-by
  - target: IDEA-093
    type: grounded-by
  - target: IDEA-095
    type: grounded-by
  - target: IDEA-096
    type: grounded-by
  - target: IDEA-097
    type: grounded-by
  - target: IDEA-099
    type: grounded-by
  - target: IDEA-100
    type: grounded-by
  - target: IDEA-101
    type: grounded-by
  - target: IDEA-102
    type: grounded-by
  - target: IDEA-103
    type: grounded-by
  - target: IDEA-104
    type: grounded-by
  - target: IDEA-105
    type: grounded-by
  - target: IDEA-106
    type: grounded-by
  - target: IDEA-107
    type: grounded-by
  - target: IDEA-116
    type: grounded-by
  - target: IDEA-119
    type: grounded-by
  - target: IDEA-122
    type: grounded-by
  - target: IDEA-123
    type: grounded-by
  - target: IDEA-124
    type: grounded-by
---
## What This Pillar Means

Clarity Through Structure is the principle that thinking, standards, and decisions must be visible and structured — not hidden in people's heads, buried in terminal output, or scattered across incompatible files.

This pillar governs features that:

- **Make governance tangible** — Rules, agents, skills, and hooks are browsable, editable documents, not invisible config files
- **Produce structured knowledge** — Plans, decisions, and research are first-class artifacts with frontmatter, connections, and lifecycle states
- **Enforce understanding before action** — Documentation-first workflow, plan approval gates, definition of ready
- **Surface hidden information** — AI transparency (system prompts, context injection, thinking), scanner dashboards, compliance indicators

## Examples of Work That Serves This Pillar

- Artifact browser that renders `.orqa/` content as navigable documents
- Rule editor that lets users view and modify enforcement rules in-app
- System prompt transparency showing what context the AI receives
- Scanner dashboard displaying pass/fail trends and violation details
- Architecture decision records that capture why the system is built this way

## Anti-Patterns

- Features that add capability without making governance more visible
- Tools that work silently without surfacing what they do
- Hiding complexity behind automation without providing an inspection layer
- Adding features that don't produce or organize structured knowledge

## Conflict Resolution

Pillars are equal in importance. When this pillar appears to conflict with Pillar 2 (Learning Through Reflection), the conflict should be flagged to the user for resolution rather than one pillar automatically winning. Agents do not prioritise one pillar over another unilaterally.
