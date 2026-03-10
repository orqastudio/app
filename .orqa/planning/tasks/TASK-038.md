---
id: TASK-038
title: Null field handling and YAML display ordering
status: done
epic: EPIC-043
description: Fixes the artifact detail renderer to suppress null, empty, and invalid-date field values, and makes field render order follow YAML source order rather than a hardcoded list.
created: 2026-03-09
updated: 2026-03-09
assignee: frontend-engineer
skills: [code-search, orqa-composability]
scope: [ui/lib/components/artifact/FrontmatterHeader.svelte, ui/lib/utils/frontmatter.ts]
acceptance: [Null, empty string, and undefined YAML values are not displayed in read views, Invalid Date never shown for null/missing date fields, Fields render in the order they appear in the YAML frontmatter, Priority labels (P1/P2/P3) include a human-readable explanation, Milestone gate question renders as the last field in the detail view]
---

## Findings Addressed

- **F9**: Null YAML values displayed (promoted_to: null, deadline: Invalid Date)
- **F10**: Milestone gate question should be last
- **F11**: Epic P1/P2/P3 tags unclear in UI

## Investigation Notes

`FrontmatterHeader.svelte` has hardcoded field lists with a predefined render order. The generic "extra fields" loop (line 261) does not filter nulls. Date fields render `Invalid Date` when the value is null.

Current field order is hardcoded in the component. The fix should respect YAML source order instead, with the component simply iterating and rendering — but the YAML field order in the artifacts themselves must be audited to ensure it makes sense from a content hierarchy perspective (separate data quality task TASK-040).

## Root Cause

The renderer displays every field it finds rather than filtering meaningless values. Field order is hardcoded in the component rather than derived from the data.
