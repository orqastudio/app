---
id: "RULE-39169bcd"
type: rule
title: "Pillar Alignment in Documentation"
description: "Every documentation page describing a feature must include a Pillar Alignment section."
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
enforcement:
  - mechanism: behavioral
    message: "Every documentation page describing a feature must include a Pillar Alignment section; pages without it are a review FAIL"
relationships:
  - target: "AD-47c41f0c"
    type: "enforces"
---
Every documentation page that describes a feature, component, workflow, integration, or capability MUST include a "Pillar Alignment" section. This ensures all documented work traces back to the product vision and prevents scope creep from accumulating silently in the docs.

## Required Section Format

Read the active pillars from `.orqa/process/pillars/` and create a row for each one:

```markdown
## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| [Pillar title] | [How this page's topic serves this pillar, or "N/A" if it doesn't] |
| [Pillar title] | [How this page's topic serves this pillar, or "N/A" if it doesn't] |
```

Every page must serve at least one active pillar. If a page cannot justify alignment with any pillar, it is scope creep and should be flagged for removal.

## Pillar Source of Truth

Active pillars are defined as structured artifacts in `.orqa/process/pillars/PILLAR-NNN.md`. Each pillar has a `title`, `description`, and `gate` field. Read the pillar artifacts to understand what each pillar means — do not hardcode pillar names or descriptions in documentation pages.

## Pages That REQUIRE a Pillar Alignment Section

- Feature pages (.orqa/documentation/reference/)
- Architecture pages (.orqa/documentation/development/)
- Component and module documentation
- Workflow and process pages
- Any page describing a capability, component, or system behavior

## Pages That Are EXEMPT

The following page categories are exempt because they define or govern the pillars themselves, or are purely technical reference:

| Exempt Category | Examples | Reason |
|-----------------|----------|--------|
| Research pages | `.orqa/delivery/research/` | Historical investigations, not features |
| Development guidelines | Coding standards, agentic workflow, library guides | Internal process docs |
| Pillar definitions | `.orqa/process/pillars/` | These ARE the pillars |
| High-level overview pages | `.orqa/documentation/about/vision.md`, `.orqa/documentation/about/governance.md` | These reference the pillars |
| Architecture decisions | `.orqa/process/decisions/` | Individual decisions already have context |

## Alignment Descriptions

Write the alignment description as a concise sentence explaining how the page's topic directly serves the pillar. Do not write vague or generic text.

**Good** (assumes pillars "Clarity Through Structure" and "Learning Through Reflection"):

```markdown
| Clarity Through Structure | N/A |
| Learning Through Reflection | The scanner dashboard tracks pass/fail trends over time, surfacing recurring violations that feed into the lesson promotion pipeline. |
```

```markdown
| Clarity Through Structure | The rule editor allows users to view, create, and modify agent enforcement rules — making governance tangible and editable. |
| Learning Through Reflection | N/A |
```

**Bad (too vague):**

```markdown
| [Pillar title] | Makes governance better |
| [Pillar title] | Helps the system learn |
```

## When Writing or Editing Documentation

1. **New pages:** Include the Pillar Alignment section before submitting the page.
2. **Editing existing pages:** Check whether a Pillar Alignment section exists. If it is missing, add one.
3. **Cannot justify alignment:** If a page genuinely cannot be aligned to either pillar, flag it to the user as potential scope creep rather than inventing a spurious alignment.

## Placement

Place the Pillar Alignment section near the bottom of the page, after the main content but before "Related Documents". This keeps it visible but out of the way of the primary content.

## Related Rules

- [RULE-1e8a1914](RULE-1e8a1914) (vision-alignment) — governs pillar alignment for features and implementation; this rule extends the same requirement to documentation pages
- [RULE-9daf29c0](RULE-9daf29c0) (documentation-first) — documentation is the source of truth; this rule ensures docs remain aligned with the product vision
- [RULE-7b770593](RULE-7b770593) (artifact-lifecycle) — documentation gates on epics (`docs-required`, `docs-produced`) produce pages that must include pillar alignment sections
