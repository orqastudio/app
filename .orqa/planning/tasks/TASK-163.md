---
id: TASK-163
title: Classify artifacts as core vs project-specific and standardise layer taxonomy
description: Audit every governance artifact to determine whether it is truly core (ships with every project) or OrqaStudio-specific (dogfood). Rename layer values, standardise scope, plan template distribution, and identify user-facing documentation gaps.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Audit all layer:canon artifacts and classify as truly-core vs OrqaStudio-specific
  - Rename layer enum from canon/project/plugin to core/project/community (with future user)
  - Standardise scope values across all artifact types
  - Add layer field to decision and other schemas where missing
  - Identify which artifacts need generalisation for template distribution
  - Document user-facing documentation gaps and plan structure
acceptance:
  - Every artifact has been evaluated and correctly classified
  - Layer enum renamed in all schemas and all artifact frontmatter
  - Scope values standardised across rules, skills, and agents
  - Research document RES-037 updated with classification results
  - List of artifacts needing generalisation for core templates produced
  - User-facing documentation plan documented
---

## What

The `.orqa/` governance system serves two roles simultaneously:
1. **Platform governance** — rules, skills, and agents that ship with OrqaStudio and apply to every project
2. **Project governance** — artifacts specific to OrqaStudio's own development (dogfooding)

These are currently conflated under `layer: canon`. This task separates them, renames the taxonomy, and identifies what needs to happen to make core artifacts truly portable.

See [RES-037](RES-037) for the full investigation and proposals.

## Deliverables

### 1. Classification Audit
For every `layer: canon` artifact, answer:
- Is the constraint universal? Is the content universal?
- Does it reference OrqaStudio-specific paths, components, or patterns?
- Would it make sense in a non-software project?
- Classification: `core` (ships as-is), `core-needs-generalisation` (constraint is universal but content is OrqaStudio-specific), or `project` (OrqaStudio-only)

### 2. Schema Changes
- Rename `canon` → `core` in all `schema.json` files
- Add `community` as a value (replacing `plugin`)
- Add `user` as a future-reserved value
- Standardise `scope` enum to `universal | software | governance`
- Add `layer` field to decision schema

### 3. Artifact Updates
- Update every artifact's `layer` field from `canon` to `core` (or reclassify to `project`)
- Update every artifact's `scope` field to use new standardised values
- Update RULE-026, orchestrator, and other governance docs that reference layer values

### 4. Documentation Plan
- Identify where user-facing docs should live
- List the documentation pages needed (getting started, core concepts, layer system, customisation)
- Create placeholder structure

## How

1. Start with RES-037 proposals
2. Build a spreadsheet/table of every artifact with current layer, proposed layer, and rationale
3. Update schemas first (so validation catches any misses)
4. Batch-update all artifact frontmatter
5. Update governance docs that reference layer values
6. Create user-facing documentation structure

## Verification

- [ ] No artifact has `layer: canon` — all renamed to `core` or reclassified
- [ ] No artifact has `layer: plugin` — all renamed to `community` or removed
- [ ] All schemas use consistent layer and scope enums
- [ ] Pre-commit hook validates against updated schemas
- [ ] Governance docs (RULE-026, orchestrator, CLAUDE.md) reference new layer names
