---
id: "TASK-c9d9baab"
type: "task"
title: "Real-time schema validation diagnostics"
description: "Wire plugin schema validation into the LSP diagnostic pipeline so that frontmatter violations produce real-time editor squiggles with precise line/column positions."
status: "ready"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
relationships:
  - target: "EPIC-3a3e5aea"
    type: "delivers"
    rationale: "Real-time schema validation is the core diagnostic capability"
  - target: "TASK-47225043"
    type: "depends-on"
    rationale: "Schema validation requires plugin schemas to be ingested first"
---

# Real-Time Schema Validation Diagnostics

## What to Implement

`validate_file()` already calls `check_json_schema()` which delegates to `orqa_validation::checks::schema`. With plugin schemas ingested (TASK-47225043), this path will produce real diagnostics. The remaining work is ensuring line-level positioning is accurate.

### Steps

1. **Verify `check_json_schema()` produces diagnostics** once artifact types are available — the code path exists but has never been exercised with real types in the LSP context.

2. **Improve diagnostic positioning** — currently `check_json_schema()` returns diagnostics at line 0. Map schema validation errors to the actual frontmatter line where the invalid field appears.

3. **Distinguish error severity** — required field missing = Error, invalid enum value = Error, additional property = Warning.

4. **Test with real plugin schemas** — validate against the TypeScript plugin's `orqa-plugin.json` artifact type definitions.

## Acceptance Criteria

- [ ] Invalid status values produce Error diagnostics at the correct frontmatter line
- [ ] Missing required fields produce Error diagnostics
- [ ] Additional/unknown properties produce Warning diagnostics
- [ ] Diagnostic messages include the field name and expected values
- [ ] Diagnostics update in real-time as the user types
- [ ] No `unwrap()` / `expect()` / `panic!()` in new code
- [ ] `make lint-backend` passes with zero warnings
