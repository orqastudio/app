---
id: TASK-299
title: README alignment audit
description: "Every artifact directory README should describe what the artifact type IS, its role in the pipeline, and be consistent in format. Must reflect new directory structure."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - Every artifact directory has a README.md
  - Each README describes the artifact type and its pipeline role
  - "README frontmatter is consistent (icon, label, description, sort)"
  - READMEs reflect new directory structure
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-292
    type: depends-on
  - target: TASK-349
    type: depended-on-by
---

## What

Audit and align all artifact directory READMEs for consistency and pipeline awareness.

## How

1. List all artifact directories under process/, delivery/, documentation/
2. For each: check README exists, has correct frontmatter, describes pipeline role
3. Create missing READMEs
4. Update existing READMEs for consistency and pipeline language
5. Ensure frontmatter icons, labels, descriptions are appropriate

## Verification

- Every artifact directory has a README
- All READMEs follow consistent format
- Pipeline role is described in each README
