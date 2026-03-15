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
  - Are discovered enforcement gaps acted on immediately, not deferred?
relationships:
  - type: grounded-by
    target: RULE-009
    rationale: Dogfood mode creates a feedback loop where using the app reveals improvements needed in the app itself
  - type: grounded-by
    target: RULE-011
    rationale: Enforcement before code ensures lessons from recurring mistakes are encoded as rules or skills before the fix
  - type: grounded-by
    target: RULE-015
    rationale: Honest reporting requires review agents to document lessons and recurrence tracking in their output
  - type: grounded-by
    target: RULE-017
    rationale: Lessons learned rule defines the full learning loop from lesson capture through promotion to enforcement
  - type: grounded-by
    target: RULE-029
    rationale: Testing standards create measurable quality baselines that track improvement or regression over time
  - type: grounded-by
    target: RULE-030
    rationale: UAT process requires systemic analysis of findings and encoding process improvements as lessons
  - type: grounded-by
    target: AD-016
    rationale: Onboarding strategy captures learning from new user experiences to improve the setup workflow
  - type: grounded-by
    target: AD-027
    rationale: Domain-agnostic vision positions the product as a clarity engine that learns and improves across domains
  - type: grounded-by
    target: AD-042
    rationale: Knowledge maturity pipeline formalises governance as a learning loop from observation through enforcement
  - type: grounded-by
    target: IMPL-011
    rationale: Lesson on systemic investigation captures the pattern of analysing root causes before fixing symptoms
  - type: grounded-by
    target: IMPL-012
    rationale: Lesson on encoding improvements ensures process learnings are captured in governance artifacts, not lost
  - type: grounded-by
    target: SKILL-004
    rationale: Code quality review skill includes lesson documentation as a mandatory part of review output
  - type: grounded-by
    target: SKILL-006
    rationale: Diagnostic methodology captures root cause analysis patterns that accumulate debugging knowledge
  - type: grounded-by
    target: SKILL-018
    rationale: Testing patterns skill captures test engineering knowledge that improves quality verification over time
  - type: grounded-by
    target: SKILL-025
    rationale: QA verification skill captures functional testing patterns that feed back into quality improvement
  - type: grounded-by
    target: SKILL-048
    rationale: Research methodology skill captures investigation patterns that improve how the team gathers knowledge
  - type: grounded-by
    target: SKILL-033
    rationale: Test engineering skill captures testing methodology that tracks coverage and quality trends
  - type: grounded-by
    target: SKILL-035
    rationale: UAT process skill captures the structured testing methodology that produces both fixes and process improvements
  - target: AD-043
    type: grounded-by
    rationale: Auto-generated inverse of grounded-by relationship from AD-043
  - target: IMPL-041
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-041
  - target: AD-044
    type: grounded-by
    rationale: Auto-generated inverse of grounded-by relationship from AD-044
  - target: RULE-046
    type: grounded-by
    rationale: Auto-generated inverse of grounded-by relationship from RULE-046
  - target: IMPL-048
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-048
  - target: PILLAR-003
    type: informs
    rationale: Auto-generated inverse of informs relationship from PILLAR-003
  - target: IMPL-050
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-050
  - target: DOC-036
    type: documented-by
    rationale: Referenced in documentation page Artifact Framework
  - target: AGENT-003
    type: grounded
    rationale: Orchestrator is grounded by this pillar — Learning Through Reflection is injected as foundational context
  - target: DOC-064
    type: informs
    rationale: Learning Through Reflection pillar is distilled into the product-purpose grounding document — inverse of informed-by on DOC-064
  - target: IDEA-095
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IDEA-095"
  - target: DOC-036
    type: informs
    rationale: Artifact promotion pipelines ensure knowledge compounds over time
  - target: DOC-041
    type: informs
    rationale: Journey 5 (Learning Loop) directly operationalises the learning loop
  - target: DOC-043
    type: informs
    rationale: Persona pain points (repeated mistakes, context loss) are what the learning loop addresses
  - target: DOC-020
    type: informs
    rationale: brokenRefs and orphans methods surface structural integrity issues for ongoing monitoring
  - target: DOC-002
    type: informs
    rationale: Process gates inject reflective thinking prompts at workflow transitions
  - target: DOC-004
    type: informs
    rationale: Governance bootstrap is the first turn of the learning loop
  - target: DOC-006
    type: informs
    rationale: Lessons are the core mechanism for capturing mistakes and promoting them to rules
  - target: DOC-029
    type: informs
    rationale: Process metrics quantify whether the learning loops are working
  - target: DOC-071
    type: informs
    rationale: Plugin skills carry institutional knowledge forward across sessions
  - target: DOC-010
    type: informs
    rationale: Lesson pipeline modules implement the learning loop in the backend
  - target: DOC-011
    type: informs
    rationale: Semantic search enables contextual understanding to reduce repeated mistakes
  - target: DOC-014
    type: informs
    rationale: Token counts and compliance violations feed session metrics and lesson pipeline
  - target: DOC-015
    type: informs
    rationale: Sub-agent turn logs contribute to session analytics and improvement
  - target: DOC-016
    type: informs
    rationale: Lessons directory and LessonStore make the lessons pipeline tangible in the UI
  - target: DOC-017
    type: informs
    rationale: Tool call sequences feed into the lesson pipeline via process_state
  - target: DOC-025
    type: informs
    rationale: Promotion pipeline ensures knowledge compounds and mistakes are not repeated
  - target: DOC-072
    type: informs
    rationale: Skills injected by plugins carry institutional knowledge forward across sessions
  - target: DOC-005
    type: informs
    rationale: Lesson commands implement the lesson capture and promotion pipeline
  - target: DOC-055
    type: informs
    rationale: Lesson dashboard is the primary UI for the learning loop
  - target: DOC-009
    type: informs
    rationale: Project settings track governance artifact counts showing governance maturity
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
