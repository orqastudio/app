# Review: Migration Tasks Phases 4-5

## Verdict: FAIL

7 issues found. 3 are critical misalignments between the task list and ARCHITECTURE.md that would produce incorrect implementation if followed as-is.

---

## Acceptance Criteria

### 1. Every GAP-* from the plugin gap analysis has a corresponding task

**PASS** (with caveat)

All 33 GAP-* IDs from phase2-02-plugin-gaps.md have corresponding tasks or explicit dispositions in the GAP Coverage Verification table (lines 1354-1394). Two gaps (GAP-AGENT-1, GAP-AGENT-2) are explicitly deferred to Phase 7, which is acceptable since Phase 7 handles AGENT-*.md removal.

**Caveat:** GAP-ROLE-3 ("Roles predefine knowledge composition") is listed as "No action in Phase 5" with no task in any phase. This is an architectural concern (roles predefine knowledge composition at definition time vs. runtime) that should at minimum be a design decision task like P5-30/P5-31, or explicitly documented as intentional behavior. Currently it falls through the cracks.

### 2. Every boundary violation from the connector gap analysis has a task

**PASS**

All 8 boundary violations from phase2-03-connector-gaps.md are covered (lines 1396-1407):
- prompt-injector.ts (MAJOR) -> P4-PRE-1, P4-01
- knowledge-injector.ts (MAJOR) -> P4-PRE-2, P4-02
- save-context.ts (MODERATE) -> P4-PRE-3, P4-03
- impact-check.ts (MINOR) -> P4-PRE-5, P4-04
- shared.ts MCP IPC (MODERATE) -> P4-05
- artifact-bridge.ts (LEGACY) -> P4-06
- connector-setup.ts (TRANSITIONAL) -> P4-07
- session-start.sh (MODERATE) -> P4-PRE-4, P4-08

### 3. Plugin rename (agile-workflow -> agile-methodology) is explicit

**PASS**

P5-29 is a dedicated task for this rename with thorough AC including directory rename, manifest name update, all cross-references, and grep verification.

### 4. Every plugin manifest gets updated with purpose, stage_slot, affects_schema, affects_enforcement

**FAIL**

Two critical misalignments between the task list and ARCHITECTURE.md:

**Issue A: `triggers_recomposition` vs `affects_schema` + `affects_enforcement`**

ARCHITECTURE.md A.6 (line 1275-1276) mandates TWO boolean fields:
```json
"affects_schema": true | false,
"affects_enforcement": true | false,
```

The task list invents a DIFFERENT field: `triggers_recomposition: boolean`. This is not in ARCHITECTURE.md. The tasks P5-02 through P5-18 all set `triggers_recomposition` instead of the architecturally-defined `affects_schema` and `affects_enforcement`.

This means:
- The `affects_enforcement` field is completely absent from all tasks. Plugins like `rust`, `typescript`, `coding-standards`, and `githooks` that have `affects_enforcement: yes` in ARCHITECTURE.md (line 1303-1309) would not get this field.
- The field name itself diverges from the architecture document.

**Issue B: `purpose` field type mismatch**

ARCHITECTURE.md A.6 (line 1273) defines `purpose` as an **array** to support multi-purpose plugins:
```json
"purpose": ["knowledge", "infrastructure"]
```

The task list uses `purpose` as a **scalar string** with a separate `dual_purpose` array (P5-10, P5-13). The `dual_purpose` field is not in ARCHITECTURE.md. Multi-purpose plugins like `rust` and `typescript` would get `purpose: "domain-knowledge"` + `dual_purpose: ["domain-knowledge", "infrastructure"]` instead of the architecturally-defined `purpose: ["knowledge", "infrastructure"]`.

**Issue C: `coding-standards` miscategorized**

ARCHITECTURE.md table (line 1307) says `coding-standards` has:
- Purpose: `infrastructure`
- Affects enforcement: `yes`

Task P5-14 sets:
- `category: "domain-knowledge"`
- `purpose: "domain-knowledge"`
- `triggers_recomposition: false`

This directly contradicts the architecture.

### 5. All duplicate artifacts (GAP-DUP-*) have resolution tasks

**PASS**

- GAP-DUP-1 -> P5-19 (AGENT-ae63c406 dedup)
- GAP-DUP-2 -> P5-20 (3 knowledge pairs in agile-workflow)
- GAP-DUP-3 -> P5-21 (duplicate knowledge_declaration IDs)
- GAP-DUP-4 -> P5-22 (duplicate declarations in software-kanban)

All four have atomic tasks with testable AC.

### 6. Missing files (GAP-MISS-*) have fix or removal tasks

**PASS**

- GAP-MISS-1 -> P5-23 (KNOW-3f307edb)
- GAP-MISS-2 -> P5-24 (review-checklist.md)

Both have create-or-remove decision logic with clear AC.

### 7. Schema field standardization (name -> title) is a task

**PASS**

P5-25 covers this with specific files listed and grep-based verification.

### 8. Installation constraint enforcement implementation is included

**PASS**

- One-methodology rule -> P5-26
- One-per-stage rule -> P5-27
- Definition vs non-definition -> P5-28

All three have implementation tasks with test criteria.

### 9. All tasks are atomic with testable AC

**PASS** (with minor note)

All 55 tasks have:
- Clear "What" descriptions
- Specific files modified/created
- Checkbox-style acceptance criteria
- Reviewer checks

Minor note: Some tasks are very large (P4-09 moves 10 knowledge files across 5 manifests, P5-25 modifies schemas across all plugins, P5-29 is a multi-file rename). These are still atomic in the sense of having one goal, but they are not small. An implementer could reasonably complete each in one context window.

### 10. Nothing deferred

**FAIL**

Two items from ARCHITECTURE.md Phase 5 have no corresponding task:

**Missing Task A: "Declare content installation targets (where in `.orqa/` hierarchy)"**

ARCHITECTURE.md Phase 5 item 4 (line 774) says manifests should declare where content installs in the `.orqa/` hierarchy. No task addresses this. The current manifests have `content.mappings` objects, but there is no task to standardize or verify these against the architecture.

**Missing Task B: GAP-ROLE-3 resolution**

GAP-ROLE-3 ("Roles predefine knowledge composition") is dismissed with "No action in Phase 5" and has no task in any phase. Since the task list claims to be exhaustive, this should either be a design decision task (like P5-30/P5-31) or have an explicit justification for why no action is needed.

---

## Issues Found

### CRITICAL: Field name mismatch — `triggers_recomposition` vs `affects_schema` + `affects_enforcement`

**Location:** All of P5-01 through P5-18 (schema definition + all manifest updates)

ARCHITECTURE.md A.6 defines `affects_schema` and `affects_enforcement` as separate booleans. The task list replaces both with a single `triggers_recomposition` boolean that is not in the architecture. This means:

1. P5-01 (schema definition) would create a schema with the wrong field name
2. P5-02 through P5-18 would all set the wrong field
3. The `affects_enforcement` dimension is completely lost — plugins like `rust`, `typescript`, `coding-standards`, `core`, and `githooks` that affect enforcement would not be marked

**Fix:** Replace all `triggers_recomposition` references with `affects_schema` and `affects_enforcement`. Update each task's AC to set both fields per the ARCHITECTURE.md table at lines 1293-1310.

### CRITICAL: `purpose` field should be an array, not a scalar

**Location:** P5-01, P5-10, P5-13, P5-33

ARCHITECTURE.md defines `purpose` as an array. The task list uses a scalar `purpose` + invented `dual_purpose` field. Multi-purpose plugins (`rust`, `typescript`, `core`) would get the wrong structure.

**Fix:** Make `purpose` an array in the schema (P5-01). For single-purpose plugins, use a single-element array. Remove `dual_purpose` from P5-10, P5-13, P5-33.

### CRITICAL: `coding-standards` plugin miscategorized

**Location:** P5-14

ARCHITECTURE.md line 1307 says `coding-standards` has purpose `infrastructure` with `affects_enforcement: yes`. Task P5-14 sets `category: "domain-knowledge"`, `purpose: "domain-knowledge"`, contradicting the architecture.

**Fix:** Update P5-14 to set `category: "infrastructure"`, `purpose: ["infrastructure"]`, `affects_enforcement: true`.

### HIGH: Stage slot naming inconsistency

**Location:** P5-03 through P5-08

The task list uses contribution-point-style names for `stage_slot` values:
- `learning-pipeline`, `discovery-artifacts`, `planning-methodology`, `documentation-standards`, `review-process`, `implementation-workflow`

ARCHITECTURE.md table (lines 1293-1310) uses shorter names:
- `learning`, `discovery`, `planning`, `documentation`, `review`, `implementation`

These may both be valid (the contribution points in the methodology workflow YAML use the long form), but the task list should specify which naming convention is canonical, and all tasks should be consistent with it. Currently the task list contradicts the ARCHITECTURE.md table.

### MEDIUM: Missing task for "Declare content installation targets"

**Location:** ARCHITECTURE.md Phase 5 item 4 (line 774)

No task addresses declaring where plugin content installs in the `.orqa/` hierarchy. This is a documented Phase 5 requirement.

### MEDIUM: GAP-ROLE-3 unresolved

**Location:** GAP Coverage Verification table, line 1383

GAP-ROLE-3 is dismissed without a task or design decision. Should be at minimum a design decision task.

### LOW: `githooks` affects_enforcement not captured

**Location:** P5-17

ARCHITECTURE.md says `githooks` has `affects_enforcement: yes` (line 1309). Even with the current `triggers_recomposition` field, P5-17 sets it to `false`. If `affects_enforcement` is added per the fix above, P5-17 must set it to `true`.

---

## Lessons

1. **Always cross-reference field names against the architecture document.** The `triggers_recomposition` vs `affects_schema`/`affects_enforcement` divergence is the kind of drift that compounds — every downstream task inherits the wrong field name, and the implementation would need a second migration to fix.

2. **Check type signatures, not just field existence.** The `purpose` field exists in both ARCHITECTURE.md and the task list, but with different types (array vs scalar). A surface-level check would miss this.

3. **The gap coverage verification table at the end of the task list is valuable** — it makes cross-referencing mechanical. But it can give false confidence if the tasks themselves have incorrect content (the table says P5-14 addresses GAP-TAX-1 for coding-standards, but the task content contradicts the architecture).
