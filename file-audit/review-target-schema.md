# Review: Target Composed Schema

**File:** `targets/schema.composed.json`
**Reference:** ARCHITECTURE.md Appendix A.2, `.state/team/target-schema/schema-extraction.md`

## Verdict: PASS

All 10 acceptance criteria pass. The schema is well-constructed, valid JSON, internally consistent, and faithful to both the extraction research and the ARCHITECTURE.md target state. Minor observations are noted below but do not constitute failures.

## Acceptance Criteria

- [x] AC1: All 20 artifact types present, agent type REMOVED -- **PASS**
  - 20 types present: decision, principle-decision, rule, lesson, knowledge, doc, vision, pillar, persona, pivot, discovery-idea, discovery-research, discovery-decision, planning-idea, planning-research, planning-decision, wireframe, milestone, epic, task
  - `agent` type NOT in `artifactTypes` -- correctly moved to `removedTypes` section with reason and affected relationships listed
  - `decision` type correctly marked `deprecated: true` with reason "Use principle-decision or planning-decision instead"

- [x] AC2: principle-decision type ADDED with prefix PD -- **PASS**
  - `idPrefix: "PD"`, `idPattern: "^PD-[a-f0-9]{8}$"`
  - Source noted as "ARCHITECTURE.md (planned, not yet in any plugin)"
  - Placed in `learning` stage per ARCHITECTURE.md
  - Full field definitions, statuses, and stateCategories present

- [x] AC3: wireframe type ADDED with prefix WIRE -- **PASS**
  - `idPrefix: "WIRE"`, `idPattern: "^WIRE-[a-f0-9]{8}$"`
  - Changed from plugin's `WF` prefix per architectural decision
  - Change documented in gaps section (line 1253)

- [x] AC4: All types use 'title' not 'name' as display field -- **PASS**
  - Programmatic check: zero types have a `name` field in required or optional fields
  - All 20 types have `title` in optional fields
  - The extraction noted 6 types that use `name` in their plugin source (discovery-research, planning-research, wireframe, milestone, epic, task) -- all correctly standardized to `title` in target schema
  - Each standardization documented individually in the gaps section (lines 1254-1259)

- [x] AC5: All defaultPaths use target structure (no process/ nesting) -- **PASS**
  - Zero types have `process/` in their `defaultPath`
  - Plugin sources used `.orqa/process/decisions`, `.orqa/process/rules`, etc. -- all correctly flattened
  - Paths align with ARCHITECTURE.md Section 5.1 directory structure

- [x] AC6: All relationships have from/to constraints -- **PASS**
  - All 35 relationships have `from` and `to` arrays
  - Both are always arrays (never null/undefined/string)
  - Two deprecated relationships (`serves`, `employs`) have empty `from: []` arrays, which is correct -- the agent type was removed so there are no valid source types

- [x] AC7: Relationship types match extraction (35 unique forward keys) -- **PASS**
  - Exactly 35 relationship keys present
  - All 35 forward keys from the extraction's deduplicated inventory are accounted for
  - Multi-plugin relationships correctly merged with combined from/to arrays and source attribution (drives, informs, evolves-to, implements)
  - Bare `idea`/`research` type references from plugin sources correctly excluded from from/to arrays (documented in notes and gaps)

- [x] AC8: All statuses come from workflow state machines -- **PASS**
  - Every artifact type has `statuses`, `initialStatus`, and `stateCategories`
  - Programmatic check: every status in `statuses` appears in exactly one `stateCategories` bucket, and vice versa -- zero mismatches
  - Planning types (planning-idea, planning-research, planning-decision) correctly use their unique vocabularies (draft/evaluating/accepted, proposed/investigating/concluded, proposed/reviewing/resolved)
  - All status sets match the extraction's workflow YAML analysis

- [x] AC9: Gaps section documents known issues -- **PASS**
  - 12 gaps documented covering:
    - `bug` type referenced in relationships but has no schema (lines 1203-1232)
    - Bare `idea`/`research` references with no schema
    - `principle-decision` not yet in any plugin
    - `agent` type removal and orphaned relationships
    - Prefix changes (PLAN-AD -> PAD, WF -> WIRE) needing migration
    - Field name standardizations (name -> title) for 6 types

- [x] AC10: Valid JSON -- parseable without errors -- **PASS**
  - `JSON.parse()` succeeds with no errors
  - 1279 lines, well-structured with consistent formatting

## Issues Found

No blocking issues. The following observations are informational:

### Minor: planning-research defaultPath introduces unlisted directory (non-blocking)

`planning-research` uses `defaultPath: ".orqa/planning/research/"` but ARCHITECTURE.md Section 5.1 has no top-level `planning/` directory. The architecture tree shows `delivery/` for planning-idea and `decisions/planning/` for planning-decision. This may need clarification but is a schema design choice, not a validation error.

### Minor: discovery-decision omitted from ARCHITECTURE.md stage table (non-blocking)

ARCHITECTURE.md line 1140 lists Discovery types but does not include `discovery-decision`. The schema correctly includes it in the `discovery` stage mapping. This is an ARCHITECTURE.md oversight -- the type exists in the agile-discovery plugin and logically belongs in discovery.

### Minor: planning-decision prefix divergence (documented)

The plugin uses `PLAN-AD`, the schema changes it to `PAD`. This is intentional per the schema's `source` note and documented in the gaps section. Just noting that ARCHITECTURE.md itself never specifies the prefix, so this is an implementer judgment call.

### Positive: agent removal handled comprehensively

The agent type removal is well-handled:
- Not in `artifactTypes`
- Documented in `removedTypes` with reason and affected relationships
- `serves` and `employs` relationships kept but marked `deprecated: true` with empty `from: []`
- Gap documented explaining the orphaned relationships

### Positive: semantic categories are complete

Every relationship appears in exactly one semantic category. The `semanticCategories` section is a complete index.

## Lessons

- The schema correctly acts as a _target_ state document rather than a mirror of current plugin reality -- it applies architectural decisions (prefix changes, field standardizations, type additions/removals) that plugins haven't implemented yet
- The gaps section serves as a migration checklist -- each gap maps to work that must be done either in plugins or during artifact migration
- Merging multi-plugin relationship definitions (drives, informs, evolves-to, implements) into single entries with combined from/to arrays and multi-source attribution is a clean approach
