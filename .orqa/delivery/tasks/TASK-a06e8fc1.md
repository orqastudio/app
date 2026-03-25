---
id: TASK-a06e8fc1
type: task
name: "Architecture compliance audit"
status: active
description: "Independent end-to-end review of every section of RES-d6e8ab11 against actual implementation — PASS/FAIL per section"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 5 — Final Gate"
  - target: TASK-51aa03f0
    type: depends-on
    rationale: "Content must be migrated before compliance can be audited"
acceptance:
  - "Audit report produced with PASS/FAIL per research section (all 11 sections of RES-d6e8ab11)"
  - "0 FAILs, or all FAILs have corresponding AD artifacts with user-approved variances"
  - "Audit report stored as a governance artifact in .orqa/"
---

## What

Independent end-to-end compliance audit: read every section of the architecture research document (RES-d6e8ab11) and verify that the actual implementation matches each section's recommendations. This is the final architecture review before the integration gate.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` — the FULL document, all 11 sections:
  1. Executive Summary
  2. Design Principles (P1-P7)
  3. Workflow Composition Architecture
  4. Agent Type Specialization
  5. Knowledge Plugin Architecture
  6. Programmatic Prompt Generation
  7. Deterministic State Machines
  8. Token Efficiency as Architecture
  9. Integration Points
  10. Migration Path
  11. Open Questions (resolved in AD-1ef9f57c)
- `AD-1ef9f57c` — resolved architecture decisions (answers to section 11 open questions)
- The actual codebase: `libs/`, `plugins/`, `connectors/`, `.orqa/`, `.claude/`

## Agent Role

Researcher — read-only audit producing a compliance report. Any FAIL findings that lack a corresponding user-approved AD artifact must be flagged.

## Steps

1. Read `RES-d6e8ab11.md` in full — all 11 sections
2. Read `AD-1ef9f57c` for resolved decisions
3. For EACH section (1-11):
   a. Extract the key recommendations and constraints
   b. Identify the corresponding implementation files/code/artifacts
   c. Verify the implementation matches the recommendation
   d. Mark PASS or FAIL with evidence
4. For any FAIL:
   a. Check if there is a corresponding AD artifact approving the variance
   b. If no AD artifact exists, flag it as an unresolved violation
5. Produce the audit report with:
   - Section number and title
   - Key recommendations from that section
   - PASS/FAIL verdict
   - Evidence (file paths, code references, artifact IDs)
   - For FAILs: AD artifact reference or "UNRESOLVED" flag
6. Store the audit report as a governance artifact in `.orqa/`

## Verification

- All 11 sections of RES-d6e8ab11 have a PASS/FAIL verdict in the report
- 0 UNRESOLVED FAILs (every FAIL must have a corresponding AD artifact)
- Report is stored in `.orqa/` as a trackable governance artifact
- Report includes file path evidence for every verdict
