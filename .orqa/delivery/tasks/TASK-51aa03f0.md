---
id: TASK-51aa03f0
type: task
name: "Content audit AND migration"
status: active
description: "Audit and migrate all plugin and project artifacts to correct types, statuses, and relationships for the new architecture"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 5 — Final Gate"
  - target: TASK-3c4f9bf4
    type: depends-on
    rationale: "Clean install must work before migrating content onto the new architecture"
acceptance:
  - "Every plugin's content audited AND migrated with findings documented"
  - "All artifacts have correct stage-scoped types (discovery-idea, planning-decision, etc.) — zero old generic types remain"
  - "All artifact statuses valid per their type's state machine"
  - "All relationship types valid per owning plugin's manifest declarations"
  - "No orphaned references to old plugin names, old types, or removed artifacts"
  - "Relationships trace evolution chain: discovery -> planning -> implementation -> review -> learning"
  - "orqa check validate passes with 0 errors after migration"
---

## What

Audit and migrate ALL existing content across plugins and at project level to align with the new plugin-decomposed architecture. This is not a read-only audit — artifacts must actually be updated to use correct types, statuses, and relationships.

### Audit scope

- Every artifact in every plugin must have the correct artifact type for its new owner plugin
- Every artifact's `status` must be valid per that type's state machine
- Plugin content (docs, knowledge, rules) must be accurate and up-to-date
- Project-level artifacts (`.orqa/delivery/`, `.orqa/principles/`, `.orqa/discovery/`) must use correct types and valid statuses
- Relationship types must be valid per the owning plugin's manifest declarations
- No orphaned content referencing types/plugins that no longer exist
- Relationships trace the full evolution chain through workflow stages

### Migration scope

- Existing `idea` artifacts -> `discovery-idea` type (update frontmatter `type` field)
- Existing `research` artifacts -> `discovery-research` or `planning-research` based on content
- Existing `decision` artifacts -> `discovery-decision` or `planning-decision` based on scope
- Existing `wireframe` artifacts -> move to agile-planning plugin content area
- Update all relationship types to match new plugin manifest declarations
- Update all status values to match new state machine definitions
- Ensure cross-stage relationships trace the full evolution chain

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` — architecture reference
- Plugin manifests: `plugins/*/orqa-plugin.json` — valid types, relationship types, state machines per plugin
- `.orqa/delivery/` — project delivery artifacts (epics, tasks, milestones)
- `.orqa/discovery/` — project discovery artifacts (ideas, research, decisions)
- `.orqa/principles/` — project principle artifacts (visions, pillars, etc.)
- `plugins/*/` content directories — plugin-owned content

## Agent Role

Implementer — this requires editing artifact files to correct types, statuses, and relationships.

## Steps

1. Read all plugin manifests to build a reference table: which types, statuses, and relationship types each plugin owns
2. List all artifacts across `.orqa/delivery/`, `.orqa/discovery/`, `.orqa/principles/`
3. For each artifact, verify:
   - `type` field matches a valid type from the appropriate plugin
   - `status` field is valid per the type's state machine
   - All `relationships[].type` values are valid per the owning plugin's manifest
   - No references to old plugin names or removed types
4. Migrate artifacts that need type changes:
   - `type: idea` -> `type: discovery-idea`
   - `type: research` -> `type: discovery-research` or `type: planning-research`
   - `type: decision` -> `type: discovery-decision` or `type: planning-decision`
5. Fix invalid statuses to match new state machines
6. Fix invalid relationship types
7. Remove orphaned references
8. Run `orqa check validate` to confirm 0 errors
9. Document all changes made

## Verification

- `grep -rn "^type: idea$\|^type: research$\|^type: decision$" .orqa/ --include="*.md"` — should return 0 (all migrated to stage-scoped types)
- grep for old plugin names in .orqa/ — should return 0 (no old plugin references)
- `orqa check validate` exit code 0 with 0 errors
