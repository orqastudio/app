---
id: TASK-447
title: "Body template validation in integrity validator"
description: "Add integrity checks that validate artifact body sections against bodyTemplate in schema.json."
status: done
priority: P2
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-070
depends-on: []
assignee: null
skills: []
acceptance:
  - "Integrity validator checks artifact body sections against bodyTemplate in schema.json"
  - "Missing required headings or empty content under required headings is a warning"
relationships:
  - target: EPIC-070
    type: delivers
    rationale: "Body template validation ensures artifacts have complete content structure"
---

## Scope

Add a new check in @orqastudio/integrity-validator that reads bodyTemplate from schema.json and validates that artifact markdown bodies contain the required section headings with non-empty content. Also update the Rust scanner if needed.
