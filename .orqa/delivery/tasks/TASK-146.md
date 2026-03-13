---
id: TASK-146
title: Create systems-thinking companion skill for RULE-028
description: Create a systems-thinking skill that provides practical methodology for applying RULE-028's systems-thinking principle to real codebases.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - New skill systems-thinking created in .orqa/team/skills/
  - Covers system identification, boundary mapping, relationship tracing, pattern recognition, uniform base discovery
  - Practical examples showing the methodology applied to real scenarios
  - RULE-028 updated to reference the companion skill
  - Skill is layer canon (systems thinking is universal, not project-specific)
---
## What

[RULE-028](RULE-028) (Systems Thinking First) defines the principle and anti-patterns but lacks a companion skill that teaches the methodology — how to actually apply systems thinking to a codebase, how to trace data flows, how to identify the system boundaries, how to find the uniform base pattern.

Create a `systems-thinking` skill that provides the practical methodology. [RULE-028](RULE-028) stays as the constraint ("you must think in systems"); the skill provides the "how."

## How

1. Create `.orqa/team/skills/systems-thinking.md` with `layer: canon`
2. Write sections covering: system identification (what is the system?), boundary mapping (where does it start/end?), relationship tracing (what depends on what?), pattern recognition (is there an existing pattern?), uniform base discovery (where is the one default behaviour?)
3. Include concrete before/after examples — e.g., "patching a symptom vs. finding the root cause in a data pipeline"
4. Add a "See `systems-thinking` skill for practical methodology" line to [RULE-028](RULE-028)

## Verification

- [ ] New skill systems-thinking created in .orqa/team/skills/
- [ ] Covers system identification, boundary mapping, relationship tracing, pattern recognition, uniform base discovery
- [ ] Practical examples showing the methodology applied to real scenarios
- [ ] [RULE-028](RULE-028) updated to reference the companion skill
- [ ] Skill is layer canon (systems thinking is universal, not project-specific)
