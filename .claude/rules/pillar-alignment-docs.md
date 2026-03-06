---
scope: project
---

# Pillar Alignment in Documentation (MANDATORY)

Every documentation page that describes a feature, component, workflow, integration, or capability MUST include a "Pillar Alignment" section. This ensures all documented work traces back to the product vision and prevents scope creep from accumulating silently in the docs.

## Required Section Format

```markdown
## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | [How this page's topic serves Pillar 1, or "N/A" if it doesn't] |
| Process Governance | [How this page's topic serves Pillar 2, or "N/A" if it doesn't] |
```

Every page must serve at least one pillar. If a page cannot justify alignment with either pillar, it is scope creep and should be flagged for removal.

## Orqa Studio's Two Pillars

**Pillar 1: Self-Learning Loop** — The system improves over time. Features that capture lessons, track metrics, feed retrospectives back into governance, accumulate knowledge across sessions, or make the system smarter with each interaction serve this pillar.

**Pillar 2: Process Governance** — Standards, rules, and workflows are visible, enforceable, and manageable. Features that define rules, manage agents, run scanners, enforce quality gates, track architecture decisions, or surface compliance status serve this pillar.

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
| Self-Learning Loop | The scanner dashboard tracks pass/fail trends over time, surfacing recurring violations that feed into the lesson promotion pipeline. |
| Process Governance | N/A |
```

**Good:**

```markdown
| Self-Learning Loop | N/A |
| Process Governance | The rule editor allows users to view, create, and modify agent enforcement rules — making governance tangible and editable. |
```

**Bad (too vague):**

```markdown
| Self-Learning Loop | Helps the system learn |
| Process Governance | Makes governance better |
```

## When Writing or Editing Documentation

1. **New pages:** Include the Pillar Alignment section before submitting the page.
2. **Editing existing pages:** Check whether a Pillar Alignment section exists. If it is missing, add one.
3. **Cannot justify alignment:** If a page genuinely cannot be aligned to either pillar, flag it to the user as potential scope creep rather than inventing a spurious alignment.

## Placement

Place the Pillar Alignment section near the bottom of the page, after the main content but before "Related Documents". This keeps it visible but out of the way of the primary content.
