---
id: PILLAR-002
title: Learning Through Reflection
description: The system and its users improve over time through structured retrospection.
status: active
created: 2026-03-09
updated: 2026-03-09
gate:
  - Does this capture lessons, discoveries, or patterns?
  - Does it track metrics or outcomes that show improvement (or regression)?
  - Does it feed retrospectives back into the governance framework?
  - Does it accumulate knowledge across sessions so each cycle starts from a better position?
relationships:
  - type: grounded-by
    target: RULE-009
    rationale: Inverse of grounded relationship from RULE-009
  - type: grounded-by
    target: RULE-011
    rationale: Inverse of grounded relationship from RULE-011
  - type: grounded-by
    target: RULE-015
    rationale: Inverse of grounded relationship from RULE-015
  - type: grounded-by
    target: RULE-017
    rationale: Inverse of grounded relationship from RULE-017
  - type: grounded-by
    target: RULE-029
    rationale: Inverse of grounded relationship from RULE-029
  - type: grounded-by
    target: RULE-030
    rationale: Inverse of grounded relationship from RULE-030
  - type: grounded-by
    target: AD-016
    rationale: Inverse of grounded relationship from AD-016
  - type: grounded-by
    target: AD-027
    rationale: Inverse of grounded relationship from AD-027
  - type: grounded-by
    target: AD-042
    rationale: Inverse of grounded relationship from AD-042
  - type: grounded-by
    target: IMPL-011
    rationale: Inverse of grounded relationship from IMPL-011
  - type: grounded-by
    target: IMPL-012
    rationale: Inverse of grounded relationship from IMPL-012
  - type: grounded-by
    target: SKILL-004
    rationale: Inverse of grounded relationship from SKILL-004
  - type: grounded-by
    target: SKILL-006
    rationale: Inverse of grounded relationship from SKILL-006
  - type: grounded-by
    target: SKILL-018
    rationale: Inverse of grounded relationship from SKILL-018
  - type: grounded-by
    target: SKILL-025
    rationale: Inverse of grounded relationship from SKILL-025
  - type: grounded-by
    target: SKILL-048
    rationale: Inverse of grounded relationship from SKILL-048
  - type: grounded-by
    target: SKILL-033
    rationale: Inverse of grounded relationship from SKILL-033
  - type: grounded-by
    target: SKILL-035
    rationale: Inverse of grounded relationship from SKILL-035
---
## What This Pillar Means

Learning Through Reflection is the principle that the system and its users get smarter with every cycle. Mistakes are documented, patterns are extracted, and governance artifacts are updated so the same problem doesn't recur.

This pillar governs features that:

- **Capture lessons** — Implementation lessons (IMPL entries) are created when unexpected behaviours are discovered
- **Track metrics** — Pass/fail rates, coverage trends, violation recurrence are measured over time
- **Feed retrospectives back** — Lessons promote to rules, rules promote to scanners, scanners promote to hard blocks
- **Accumulate knowledge** — Session continuity, cross-session search, handoff summaries preserve context

## Examples of Work That Serves This Pillar

- Lesson management with recurrence tracking and promotion pipeline
- Session analytics showing trends across conversations
- Post-session hooks that capture lessons automatically
- Automated promotion suggestions when a lesson recurs enough
- Scanner dashboard with historical trend charts
- Cross-project pattern detection surfacing recurring lessons

## Anti-Patterns

- Features that produce output without capturing what was learned
- One-off fixes without documenting the pattern for future avoidance
- Tools that reset state between sessions instead of accumulating knowledge
- Skipping retrospectives or lesson documentation because "it's done now"

## Relationship to Pillar 1

This pillar complements Pillar 1 (Clarity Through Structure). The learning loop operates on structured, visible governance artifacts — if artifacts aren't structured and visible, there is nothing for the reflection process to improve. The two pillars are deeply intertwined in practice.

## Conflict Resolution

Pillars are equal in importance. When this pillar appears to conflict with Pillar 1 (Clarity Through Structure), the conflict should be flagged to the user for resolution rather than one pillar automatically winning. Agents do not prioritise one pillar over another unilaterally.

