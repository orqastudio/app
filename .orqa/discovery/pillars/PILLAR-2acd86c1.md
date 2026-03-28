---
id: "PILLAR-2acd86c1"
type: pillar
title: "Learning Through Reflection"
description: "The system and its users improve over time through structured retrospection."
status: active
created: "2026-03-09"
updated: "2026-03-09"
gate:
  - "Does this capture lessons, discoveries, or patterns?"
  - "Does it track metrics or outcomes that show improvement (or regression)?"
  - "Does it feed retrospectives back into the governance framework?"
  - "Does it accumulate knowledge across sessions so each cycle starts from a better position?"
  - "Are discovered enforcement gaps acted on immediately, not deferred?"
relationships:
  - target: "VISION-4893db55"
    type: "upholds"

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
