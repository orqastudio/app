---
id: pillar-alignment-docs
title: "Pillar Alignment in Documentation"
description: "Every documentation page describing a feature must include a Pillar Alignment section."
scope: project
---


Every documentation page that describes a feature, component, workflow, integration, or capability MUST include a "Pillar Alignment" section. This ensures all documented work traces back to the product vision and prevents scope creep from accumulating silently in the docs.

## Required Section Format

```markdown
## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | [How this page's topic serves Pillar 1, or "N/A" if it doesn't] |
| Learning Through Reflection | [How this page's topic serves Pillar 2, or "N/A" if it doesn't] |
```

Every page must serve at least one pillar. If a page cannot justify alignment with either pillar, it is scope creep and should be flagged for removal.

## OrqaStudio's Two Pillars

**Pillar 1: Clarity Through Structure** — Making thinking, standards, and decisions visible and structured. Features that surface governance artifacts, produce structured knowledge (plans, decisions, rules), enforce documentation-first workflows, or make the invisible tangible serve this pillar.

**Pillar 2: Learning Through Reflection** — The system and its users improve over time through structured retrospection. Features that capture lessons, track metrics, feed retrospectives back into governance, accumulate knowledge across sessions, or make the process smarter with each interaction serve this pillar.

## Pages That REQUIRE a Pillar Alignment Section

- Feature pages (docs/ui/)
- Architecture pages (docs/architecture/)
- Component and module documentation
- Workflow and process pages
- Any page describing a capability, component, or system behavior

## Pages That Are EXEMPT

The following page categories are exempt because they define or govern the pillars themselves, or are purely technical reference:

| Exempt Category | Examples | Reason |
|-----------------|----------|--------|
| Research pages | `.orqa/research/` | Historical investigations, not features |
| Development guidelines | Coding standards, agentic workflow, library guides | Internal process docs |
| High-level overview pages | `docs/product/vision.md`, `docs/product/governance.md` | These define the pillars |
| Architecture decisions log | `docs/architecture/decisions.md` | Individual decisions already have context |

## Alignment Descriptions

Write the alignment description as a concise sentence explaining how the page's topic directly serves the pillar. Do not write vague or generic text.

**Good:**

```markdown
| Clarity Through Structure | N/A |
| Learning Through Reflection | The scanner dashboard tracks pass/fail trends over time, surfacing recurring violations that feed into the lesson promotion pipeline. |
```

**Good:**

```markdown
| Clarity Through Structure | The rule editor allows users to view, create, and modify agent enforcement rules — making governance tangible and editable. |
| Learning Through Reflection | N/A |
```

**Bad (too vague):**

```markdown
| Clarity Through Structure | Makes governance better |
| Learning Through Reflection | Helps the system learn |
```

## When Writing or Editing Documentation

1. **New pages:** Include the Pillar Alignment section before submitting the page.
2. **Editing existing pages:** Check whether a Pillar Alignment section exists. If it is missing, add one.
3. **Cannot justify alignment:** If a page genuinely cannot be aligned to either pillar, flag it to the user as potential scope creep rather than inventing a spurious alignment.

## Placement

Place the Pillar Alignment section near the bottom of the page, after the main content but before "Related Documents". This keeps it visible but out of the way of the primary content.
