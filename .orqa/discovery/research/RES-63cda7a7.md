---
id: "RES-63cda7a7"
type: discovery-research
title: "Consistent Entity Structure"
description: "Design body templates for each artifact type to ensure consistency across planning, team, and governance artifacts."
status: completed
created: "2026-03-10"
updated: "2026-03-10"
relationships:
  - target: "EPIC-d45b4dfd"
    type: "guides"
    rationale: "Research findings informed the design of Artifact Graph SDK and Structural Integrity"
---

## Problem

Artifact frontmatter is now schema-enforced, but body structure (everything below the `---`) is completely freeform. Some types have naturally converged on consistent patterns (pillars, milestones, lessons, decisions), while others are inconsistent (epics, tasks) or nearly empty (ideas). Without body templates, new artifacts are created with whatever structure the author invents, making it harder to find information and harder for the app to eventually parse or render structured sections.

## Scope

This covers planning, team, and governance artifacts. Documentation files (`.orqa/documentation/`) are intentionally freeform — their content varies by topic and audience. Research is also intentionally freeform since investigations are domain-specific.

## Current State

| Type | Body Consistency | Pattern |
| ------ | ----------------- | --------- |
| Pillar | High | 4 standard sections |
| Milestone | High | Gate Question, Context, Epics, Completion Criteria |
| Decision | High | Decision, Rationale, Consequences |
| Lesson | High | Pattern, Fix |
| Epic | Medium | Varies between task-tracking and implementation design |
| Task | Medium | Varies between implementation and UAT |
| Idea | Low | Most have empty or minimal bodies |
| Rule | Low | Domain-specific; no universal pattern |
| Research | Low (intentional) | Varies by investigation type |

## Design Principles

1. **Templates define the minimum** — required sections that must exist. Authors can add more sections as needed.
2. **Sections serve a purpose** — every required section must answer a question that the system or users need answered.
3. **Machine-parseable** — use `## Heading` consistently so the app can eventually extract and render sections.
4. **Don't over-structure** — if a type's value comes from freeform thinking (research, rules), don't constrain it.

## Proposed Body Templates

### Pillar (already consistent — document as-is)

```markdown
## What This Pillar Means
[Narrative explanation of the principle and what it governs]

## Examples of Work That Serves This Pillar
[Concrete features, capabilities, or patterns that align]

## Anti-Patterns
[What violates this pillar — specific examples]

## Conflict Resolution
[How this pillar relates to others; when it takes priority]
```

All 4 sections required.

### Milestone (already consistent — document as-is)

```markdown
## Context
[Background, motivation, what this milestone represents]

## Epics
[Table or list of epics grouped by priority with status]

## Completion Criteria
[Checklist of conditions beyond the gate question]
```

Gate Question is now in frontmatter as `gate:`, so the body no longer needs a duplicate. Context and Epics required; Completion Criteria optional (the gate question may be sufficient).

### Epic

```markdown
## Context
[Why this epic exists, what problem it solves, what's already done]

## Implementation Design
[Data model, IPC contracts, component breakdown, approach]

## Tasks
[Checklist or summary of tasks within this epic]

## Out of Scope
[What is explicitly NOT part of this epic — prevents scope creep]
```

Context and Implementation Design required. Tasks appears naturally as work is broken down. Out of Scope is optional but encouraged.

### Task

```markdown
## What
[Problem or requirement this task addresses]

## How
[Implementation approach, files to change, key decisions]

## Verification
[How to confirm this task is done — specific checks, not just "it works"]
```

All 3 sections required. For UAT-originated tasks, "What" contains the finding reference (e.g., "F9: Null YAML values displayed") and "How" contains the root cause and fix approach.

### Idea

```markdown
## Motivation
[Why this idea matters — what problem does it solve or opportunity does it create]

## Sketch
[Initial thinking about how this could work — rough, not detailed]
```

Motivation required (an idea without a "why" isn't worth tracking). Sketch optional — some ideas are captured before any thinking has been done, and that's fine. The `research-needed` frontmatter field lists what needs investigation.

### Decision (already consistent — document as-is)

```markdown
## Decision
[What was decided — one clear statement]

## Rationale
[Why this was chosen over alternatives]

## Consequences
[What becomes easier, what becomes harder, what changes]
```

All 3 sections required. Complex decisions may add subsections (Context, Migration, Open Questions) but the core 3 must exist.

### Lesson (already consistent — document as-is)

```markdown
## Pattern
[What happened and why it was unexpected]

## Fix
[The correct approach going forward]
```

Both required. Optional: `## Git Evidence` for traceability.

### Rule (semi-structured)

Rules vary too much by domain to have a rigid template, but they should follow a common pattern:

```markdown
[Opening paragraph: one-sentence statement of the rule]

## [Domain-specific sections as needed]

## FORBIDDEN
[Anti-patterns — what must never be done]

## Related Rules
[Cross-references to other rules]
```

Opening paragraph and Related Rules required. FORBIDDEN section strongly encouraged. Everything in between is domain-specific.

### Research (intentionally freeform)

No body template. Research documents are investigations — their structure follows the question being asked. The frontmatter (`id`, `title`, `description`, `status`) provides the metadata; the body is whatever the investigation needs.

## Enforcement

Body templates are enforced at two levels in this epic:

1. **Documentation** — templates documented in artifact-framework.md, agents follow them, humans reference them
2. **Linting** — pre-commit hook checks for required `## Heading` patterns in artifact bodies, driven by template definitions in schema.json

A third level (app-assisted — artifact editor pre-populates sections from the template) is deferred to the artifact editor epic [EPIC-320d1a2f](EPIC-320d1a2f).

## Template Storage

Body templates are defined in each type's `schema.json` alongside the frontmatter schema. This keeps all structural expectations for an artifact type in one place and allows the pre-commit hook and future app editor to read templates from the same source.

## Resolved Questions

- **App rendering**: Just markdown. No collapsible/tabbed sections — keep it simple. Structured rendering is a future UX enhancement.
- **Template location**: Defined in schema.json alongside frontmatter schemas.
- **Artifact editor interaction**: Deferred to [EPIC-320d1a2f](EPIC-320d1a2f) discussion. The editor would read templates from schema.json to pre-populate sections.
