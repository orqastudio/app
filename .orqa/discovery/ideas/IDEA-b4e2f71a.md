---
id: IDEA-b4e2f71a
type: discovery-idea
status: captured
title: Product management utility plugins inspired by VizTools
created: 2026-03-28T17:30:00.000Z
---

# Product Management Utility Plugins

## Source

Inspired by [viztools.app](https://viztools.app/) — 16 supplementary tools for product managers. Product management is a key target persona for OrqaStudio.

## Plugin Opportunities

These tools consume governance data and produce formatted output — exactly what plugin views/widgets enable.

### High-value integrations (read from artifact graph)

1. **PRD Generator** — generates one-pager from epic artifacts (title, scope, acceptance criteria, relationships)
2. **QBR Agenda Generator** — pulls from milestone status, completed epics, active metrics
3. **Sprint Retro Board** — captures feedback that feeds into lesson artifacts (IMPL-*)
4. **Project Brief Generator** — composes from vision + pillars + active epics
5. **ROI Calculator** — attaches cost-benefit analysis to epics/milestones

### Standalone utilities (useful alongside governance)

1. **Meeting Cost Calculator** — context for planning decisions
2. **SaaS Metrics Calculator** — feeds into milestone tracking
3. **A/B Test Significance Calculator** — links to research artifacts
4. **Presentation Pacing Calculator** — useful for QBR prep
5. **1-on-1 Agenda Builder** — structured agendas with action items

## Implementation Pattern

Each tool is a plugin that:

- Declares `provides.views` or `provides.widgets` in its manifest
- Reads from the artifact graph via engine APIs
- Presents a purpose-built view in the app UI
- Optionally writes back (retro → lessons, PRD → epic)

## Priority

Post-migration. These are value-add plugins that demonstrate the plugin system's extensibility for the product management persona.
