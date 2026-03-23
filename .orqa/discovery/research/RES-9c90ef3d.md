---
id: RES-9c90ef3d
type: research
title: "Documentation reorganisation — book-chapter structure, how-to guides, relationship migration"
description: "Research how to reorganise .orqa/documentation/ into a book-like chapter structure, create missing how-to documentation (plugin SDK, testing, linting), and plan the relationship migration as docs move between directories."
status: completed
created: 2026-03-14
updated: 2026-03-14
relationships:
  - target: EPIC-b2ca1ea3
    type: guides
    rationale: "Auto-generated inverse of informs relationship from EPIC-b2ca1ea3"
  - target: EPIC-2a6e2567
    type: guides
    rationale: "Auto-generated inverse of informs relationship from EPIC-2a6e2567"
  - target: IDEA-93949f43
    type: spawned-by
---
## Research Questions

1. **Chapter structure**: How should `.orqa/documentation/` be reorganised into book-like chapters? What directories map to what chapters?
2. **Migration plan**: Which existing docs move where? What relationships need updating when paths change?
3. **Missing content**: What how-to guides need writing? Plugin SDK, testing frameworks, linting, artifact type creation?
4. **Relationship impact**: How many relationships reference documentation paths that will change? What's the migration strategy?
5. **project.json impact**: Which artifact config entries need path updates?

## Proposed Chapter Structure

| Chapter | Directory | Content |
|---------|-----------|---------|
| About | `about/` | Vision, core concepts, personas, journeys, governance |
| User Guide | `guide/` | Getting started, working with artifacts, exploring, delivery, process |
| Development | `development/` | Setup, architecture, coding standards, commands, dev controller, modules |
| How-To | `how-to/` | Plugin development, SDK usage, testing, linting, creating artifact types, search engine |
| Reference | `reference/` | IPC commands, SQLite schema, design system, wireframes, component inventory, artifact framework |
| Grounding | `grounding/` | Agent grounding docs (already created, stays as-is) |

## Migration Inventory

The following table maps every current documentation file to its proposed new location. DOC IDs are permanent — only the file path changes. The ID-based relationship system means relationships do not need updating when files move; only the `project.json` artifact config entries and any body text that references paths directly need updating.

### Current → Proposed Path Mapping

| Current Path | ID | Current Section | Proposed Chapter | New Path | Notes |
|---|---|---|---|---|---|
| `architecture/core-architecture.md` | (no ID) | architecture | development | `development/core-architecture.md` | No DOC id assigned |
| `architecture/enforcement.md` | DOC-4db3a417 | architecture | development | `development/enforcement.md` | |
| `architecture/error-taxonomy.md` | DOC-dcf96a52 | architecture | reference | `reference/error-taxonomy.md` | Reference material |
| `architecture/governance-bootstrap.md` | DOC-d498eac8 | architecture | development | `development/governance-bootstrap.md` | Dev-facing internals |
| `architecture/ipc-commands.md` | DOC-3c65a1e3 | architecture | reference | `reference/ipc-commands.md` | Reference catalog |
| `architecture/lessons.md` | DOC-1ab26625 | architecture | development | `development/lesson-pipeline.md` | |
| `architecture/mcp-host.md` | DOC-7290a3df | architecture | development | `development/mcp-host.md` | |
| `architecture/plugin-architecture.md` | (no ID) | architecture | development | `development/plugin-architecture.md` | No DOC id assigned |
| `architecture/project-configuration.md` | DOC-78ed20f0 | architecture | reference | `reference/project-configuration.md` | Config reference |
| `architecture/rust-modules.md` | DOC-2f60e3a2 | architecture | development | `development/rust-modules.md` | |
| `architecture/search-engine.md` | DOC-c1d1f212 | architecture | development | `development/search-engine.md` | |
| `architecture/setup-wizard.md` | DOC-dacb6315 | architecture | development | `development/setup-wizard.md` | |
| `architecture/sqlite-schema.md` | DOC-f9c6149e | architecture | reference | `reference/sqlite-schema.md` | Reference material |
| `architecture/streaming-pipeline.md` | DOC-eda128cd | architecture | development | `development/streaming-pipeline.md` | |
| `architecture/sub-agents.md` | DOC-e5ff4baf | architecture | development | `development/sub-agents.md` | |
| `architecture/svelte-components.md` | DOC-52bbfba5 | architecture | development | `development/svelte-components.md` | |
| `architecture/tool-definitions.md` | DOC-a184e87f | architecture | development | `development/tool-definitions.md` | |
| `architecture/wireframe-serving.md` | DOC-fe637e80 | architecture | development | `development/wireframe-serving.md` | |
| `development/artifact-graph-sdk.md` | DOC-e6f39c35 | development | development | stays in `development/` | No move needed |
| `development/coding-standards.md` | DOC-4b4fbc0f | development | development | stays in `development/` | No move needed |
| `development/commands.md` | DOC-357f8d7c | development | development | stays in `development/` | No move needed |
| `development/contributing.md` | DOC-aae5a443 | development | development | stays in `development/` | No move needed |
| `development/dev-controller.md` | DOC-46ffe1d2 | development | development | stays in `development/` | No move needed |
| `development/getting-started.md` | DOC-2a7f1063 | development | development | stays in `development/` | No move needed |
| `guide/getting-started.md` | DOC-b5714e7e | guide | guide | stays in `guide/` | No move needed |
| `process/artifact-workflow.md` | DOC-c4b4b8b7 | process | guide | `guide/artifact-workflow.md` | User-facing process |
| `process/content-governance.md` | DOC-3e7ea7d2 | process | about | `about/content-governance.md` | Product-level framing |
| `process/delegation.md` | DOC-c158f4a6 | process | reference | `reference/delegation.md` | Reference lookup table |
| `process/metrics.md` | DOC-b513c6b2 | process | development | `development/metrics.md` | Internal metrics |
| `process/orchestration.md` | DOC-b10607c0 | process | development | `development/orchestration.md` | Internal architecture |
| `process/team.md` | DOC-abd30340 | process | about | `about/team.md` | Who the agents are |
| `process/workflow.md` | DOC-7c66f103 | process | guide | `guide/workflow.md` | User-facing workflow |
| `product/artifact-framework.md` | DOC-01ddd8aa | product | reference | `reference/artifact-framework.md` | Definitive reference |
| `product/glossary.md` | DOC-1b0a39a1 | product | about | `about/glossary.md` | |
| `product/go-to-market.md` | DOC-0339700d | product | about | `about/go-to-market.md` | |
| `product/governance.md` | DOC-1bc9d0b9 | product | about | `about/governance.md` | |
| `product/information-architecture.md` | DOC-87d0a2de | product | about | `about/information-architecture.md` | |
| `product/journeys.md` | DOC-db530ffe | product | about | `about/journeys.md` | |
| `product/mvp-specification.md` | DOC-434e00e1 | product | about | `about/mvp-specification.md` | |
| `product/personas.md` | DOC-ffd47d19 | product | about | `about/personas.md` | |
| `product/priority-assessment.md` | DOC-c8de180d | product | about | `about/priority-assessment.md` | |
| `product/roadmap.md` | DOC-044 | product | about | `about/roadmap.md` | |
| `product/vision.md` | VISION-7781f555 | product | about | `about/vision.md` | |
| `ui/brand-identity.md` | DOC-bc65f7a8 | ui | reference | `reference/brand-identity.md` | |
| `ui/design-system.md` | DOC-03f1dff2 | ui | reference | `reference/design-system.md` | |
| `ui/enforcement-panel.md` | DOC-c400fae0 | ui | reference | `reference/enforcement-panel.md` | |
| `ui/interaction-patterns.md` | DOC-9c29d60c | ui | reference | `reference/interaction-patterns.md` | |
| `ui/lesson-dashboard.md` | DOC-d2d193de | ui | reference | `reference/lesson-dashboard.md` | |
| `ui/responsive-behavior.md` | DOC-05278b9c | ui | reference | `reference/responsive-behavior.md` | |
| `wireframes/artifact-browser.md` | DOC-99d7fbe5 | wireframes | reference | `reference/wireframes/artifact-browser.md` | Subfolder under reference |
| `wireframes/conversation-view.md` | DOC-c816d78d | wireframes | reference | `reference/wireframes/conversation-view.md` | |
| `wireframes/core-layout.md` | DOC-91c7dc26 | wireframes | reference | `reference/wireframes/core-layout.md` | |
| `wireframes/dashboard.md` | DOC-2c8605a0 | wireframes | reference | `reference/wireframes/dashboard.md` | |
| `wireframes/settings-onboarding.md` | DOC-53b07f55 | wireframes | reference | `reference/wireframes/settings-onboarding.md` | |
| `grounding/artifact-principles.md` | DOC-bfdf76ba | grounding | grounding | stays in `grounding/` | No move needed |
| `grounding/code-principles.md` | DOC-4aec151e | grounding | grounding | stays in `grounding/` | No move needed |
| `grounding/design-principles.md` | DOC-6b733de8 | grounding | grounding | stays in `grounding/` | No move needed |
| `grounding/product-purpose.md` | DOC-1e0dcc5d | grounding | grounding | stays in `grounding/` | No move needed |
| `grounding/research-principles.md` | DOC-3749bfac | grounding | grounding | stays in `grounding/` | No move needed |

### Summary by Movement Type

| Action | Count | Sections affected |
|--------|-------|------------------|
| No move (stays in same directory) | 12 | development (6), guide (1), grounding (5) |
| Move from `architecture/` | 18 | → development (13), → reference (5) |
| Move from `product/` | 11 | → about (11) |
| Move from `ui/` | 6 | → reference (6) |
| Move from `wireframes/` | 5 | → reference/wireframes/ (5) |
| Move from `process/` | 6 | → guide (2), → about (2), → development (1), → reference (1) |
| **Total files** | **58** | |

### New Directories Required

The following directories must be created:

- `about/` (replacing `product/` which becomes `about/`)
- `how-to/` (new — empty at first, populated by how-to guides)
- `reference/` (new top-level, consolidating `ui/`, `wireframes/`, and reference-class architecture docs)
- `reference/wireframes/` (subfolder within reference)

The `architecture/` and `process/` directories are dissolved — all content moves out.

## Missing Content (How-To Guides to Write)

The `how-to/` chapter is entirely new and needs these guides written:

| Guide | Path | Audience | Priority |
|-------|------|----------|----------|
| Plugin development — SDK overview | `how-to/plugin-sdk.md` | Plugin authors | P1 |
| Plugin development — plugin.json schema | `how-to/plugin-json.md` | Plugin authors | P1 |
| Plugin development — hooks and commands | `how-to/plugin-hooks.md` | Plugin authors | P1 |
| Plugin development — skills and agents | `how-to/plugin-skills.md` | Plugin authors | P2 |
| Writing Rust tests | `how-to/testing-rust.md` | Contributors | P1 |
| Writing frontend tests (Vitest) | `how-to/testing-frontend.md` | Contributors | P1 |
| Writing E2E tests (Playwright) | `how-to/testing-e2e.md` | Contributors | P2 |
| Using the linting framework | `how-to/linting.md` | Contributors | P2 |
| Creating custom artifact types | `how-to/custom-artifact-types.md` | Power users / plugin authors | P2 |
| Using the search engine | `how-to/search-engine-usage.md` | Agents and users | P2 |

These guides do not exist in any form. They must be written from scratch. P1 guides should be created before the reorganisation epic is marked done.

## Relationship Impact

### Why DOC-ID References Are Stable

Relationships between artifacts use DOC-NNN IDs, not file paths. The relationship graph (`target: DOC-3c65a1e3`, etc.) will not break when files move. **No relationship frontmatter needs updating** when documentation files are reorganised.

### What DOES Need Updating

**1. `project.json` artifact config** (definite changes required):

The current `docs` group in `project.json` lists 7 paths:

```json
{ "key": "guide", "path": ".orqa/documentation/guide" },
{ "key": "product", "path": ".orqa/documentation/product" },
{ "key": "architecture", "path": ".orqa/documentation/architecture" },
{ "key": "development", "path": ".orqa/documentation/development" },
{ "key": "process", "path": ".orqa/documentation/process" },
{ "key": "ui", "path": ".orqa/documentation/ui" },
{ "key": "wireframes", "path": ".orqa/documentation/wireframes" }
```

After reorganisation, these become:

```json
{ "key": "guide", "path": ".orqa/documentation/guide" },
{ "key": "about", "path": ".orqa/documentation/about" },
{ "key": "development", "path": ".orqa/documentation/development" },
{ "key": "how-to", "path": ".orqa/documentation/how-to" },
{ "key": "reference", "path": ".orqa/documentation/reference" },
{ "key": "grounding", "path": ".orqa/documentation/grounding" }
```

The old keys `product`, `architecture`, `process`, `ui`, and `wireframes` are removed. New keys `about`, `how-to`, and `reference` are added. The `grounding` key is added (was missing from current config — grounding directory exists on disk but is not in project.json).

**2. Body text path references** (require manual updates after move):

152 lines across 71 files reference documentation paths directly in body text (not in frontmatter relationship targets). These are in:

- Rules: 42 references (e.g., `RULE-b49142be` references `.orqa/documentation/development/coding-standards.md`, `RULE-7b770593` references `.orqa/documentation/about/roadmap.md`)
- Agent definitions: ~20 references (e.g., `writer.md` references `.orqa/documentation/reference/`, `.orqa/documentation/development/`)
- Epic and task bodies: ~50 references
- Research documents: ~40 references

These body text references will not break the app (they are not machine-parsed) but will be stale documentation. They should be updated in the same commit as the file moves, or in a follow-up pass immediately after.

**Most-referenced paths that will change:**

| Old Path | New Path | References affected |
|----------|----------|---------------------|
| `.orqa/documentation/product/roadmap.md` | `.orqa/documentation/about/roadmap.md` | ~8 |
| `.orqa/documentation/product/artifact-framework.md` | `.orqa/documentation/reference/artifact-framework.md` | ~12 |
| `.orqa/documentation/product/vision.md` | `.orqa/documentation/about/vision.md` | ~6 |
| `.orqa/documentation/development/coding-standards.md` | stays | 0 |
| `.orqa/documentation/architecture/ipc-commands.md` | `.orqa/documentation/reference/ipc-commands.md` | ~4 |
| `.orqa/documentation/product/priority-assessment.md` | `.orqa/documentation/about/priority-assessment.md` | ~2 |

**3. Missing from current `project.json`**: The `grounding/` directory exists on disk and contains 5 files (DOC-1e0dcc5d through DOC-3749bfac) but is not listed in the `artifacts` config. These files are visible via the file system but not scanned by the artifact browser. This is a pre-existing gap that should be fixed as part of the reorganisation.

### Relationship Count

Total DOC-NNN relationship references across all `.orqa/` artifacts: **187 references** across **100 files**. None of these need updating for the reorganisation (IDs are stable). Only the 152 body-text path references across 71 files need updating.

## Findings Summary

1. **The `architecture/` and `process/` sections are dissolved.** Their content is redistributed across `development/` (architecture internals), `reference/` (reference catalogs), `about/` (product framing), and `guide/` (user-facing workflow). This is the biggest structural change.

2. **The `product/` section becomes `about/`.** All 11 files move with minimal content changes. The section label shifts from a technical ownership label ("product docs") to a reader-orientation label ("about this product").

3. **The `ui/` and `wireframes/` sections consolidate into `reference/`.** UI specs and wireframes are reference material, not a separate top-level category. Wireframes become a subfolder `reference/wireframes/`.

4. **The `grounding/` directory exists but is not in `project.json`.** Fix this during the reorganisation — add `grounding` to the artifact config.

5. **`how-to/` is entirely new.** 10 guides need writing; 3 are P1 (plugin SDK, Rust tests, frontend tests).

6. **ID-based relationships are safe.** The 187 relationship references to DOC-NNN IDs will survive the file moves unchanged. Only body-text path references (152 lines in 71 files) need a sweep after the move.

7. **Two files have no DOC id**: `architecture/core-architecture.md` and `architecture/plugin-architecture.md`. They should be assigned IDs before moving (to allow relationship tracking going forward). Next available appears to be DOC-7d811a5d+ (audit confirms DOC-b5714e7e is the highest currently assigned; check for gaps).

8. **The `development/` directory expands significantly.** It absorbs 13 architecture files. A README with clear section structure will be important to maintain navigability.

9. **Migration should be atomic per directory.** Move one source directory at a time, update `project.json`, verify the scanner picks up the new paths, then move the next. Do not do all moves in a single commit unless the full path update sweep is also in that commit.