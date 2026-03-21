---
id: DOC-c8de180d
type: doc
title: Priority Assessment
category: concept
description: "How agents assess epic priority — project-flexible, inference-based, not formula-driven"
sort: 9
relationships: []
---

## Principle

Priority is a judgement call, not a formula. Agents read the project's priority criteria
and apply them through inference — understanding intent, context, and the current state
of the milestone. Different projects weight different things. The scoring system captures
the agent's assessment, not a mechanical calculation.

## How It Works

Each project defines its priority criteria in this document (or the equivalent in their
`.orqa/documentation/about/` directory). When an agent needs to assess an epic's
priority, it:

1. Reads this document to understand what matters right now
2. Reads the epic's context, milestone, dependencies, and scope
3. Assigns a priority band (P1 / P2 / P3) based on judgement
4. Records a brief rationale in the epic's `scoring` field

The `scoring` field on epics is freeform — projects choose their own dimension names.
What matters is that the rationale is readable and the priority band is defensible.

## Priority Bands

| Band | Meaning |
|------|---------|
| **P1** | Must be done for the milestone to succeed. Blocks other work or the milestone gate. |
| **P2** | Should be done in this milestone. Adds significant value but doesn't block the gate. |
| **P3** | Nice to have. Can carry forward to the next milestone without consequence. |

## This Project's Criteria (OrqaStudio — current state)

OrqaStudio is in alpha, working toward [MS-654badde](MS-654badde) (Dogfooding) and [MS-eea45fa8](MS-eea45fa8) (MVP). The
current priority criteria are:

### 1. Milestone Service

Does this epic directly serve the active milestone's gate question?

- **[MS-654badde](MS-654badde) gate**: "Can we use this app instead of the terminal?"
- **[MS-eea45fa8](MS-eea45fa8) gate**: "Can a new user install this and get value within 10 minutes?"

An epic that is essential for answering "yes" to the gate question is P1. An epic that
improves the answer but isn't essential is P2. An epic that doesn't affect the gate
is P3.

### 2. Blocking Status

Does this epic block other P1 or P2 work?

An epic that sits in the critical path — where other epics depend on it via
`depends-on` edges or where its deliverables are prerequisites for other work — gets
elevated priority. An epic that blocks nothing can be deferred without consequence.

### Assessment Guidance

When assigning priority, consider:
- **What breaks if this isn't done?** If the milestone can't close, it's P1.
- **What's waiting on this?** If 3 other epics depend on it, it's probably P1.
- **Would a user notice?** If it affects core workflow, P1-P2. If it's polish, P2-P3.
- **Is the project eating its own cooking?** In dogfood mode, anything that disrupts
  the development workflow is elevated because we experience it daily.

Do not assign priority based on:
- Artifact ID numbers (EPIC-e045ab6d is not more important than EPIC-3a8ad459)
- Creation date (older ≠ more urgent)
- Effort size (big ≠ important)
- Technical interest (cool ≠ necessary)

## Evolving Criteria

These criteria change as the project matures. When a milestone is completed and a new
one becomes active, reassess all epic priorities against the new gate question. An epic
that was P3 under [MS-654badde](MS-654badde) may become P1 under [MS-eea45fa8](MS-eea45fa8).

The orchestrator should trigger a priority reassessment when:
- A milestone status changes
- A new milestone becomes active
- Significant scope changes occur on multiple epics
- The user explicitly requests reprioritisation
