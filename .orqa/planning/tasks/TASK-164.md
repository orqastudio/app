---
id: TASK-164
title: Audit artifact group README files for accuracy
description: Verify that all README.md files in .orqa/ artifact directories have accurate descriptions, icons, labels, and sort metadata reflecting the current state of each group.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on: []
docs:
  - .orqa/documentation/product/artifact-framework.md
skills:
  - orqa-governance
  - orqa-documentation
  - orqa-artifact-audit
scope:
  - Audit every README.md in .orqa/ artifact directories (planning/, governance/, team/, documentation/)
  - Verify description text accurately reflects the group's current contents and purpose
  - Verify icon, label, and sort frontmatter are appropriate
  - Update any stale or inaccurate descriptions
  - Ensure consistency with the five-layer taxonomy (core/project/plugin/community/user)
acceptance:
  - Every artifact directory README.md has been reviewed
  - Descriptions match the current purpose and contents of each group
  - No references to deprecated terminology (canon, plugin as old meaning)
  - All frontmatter fields (icon, label, description, sort) are present and accurate
---

## What

The artifact browser uses README.md frontmatter (icon, label, description, sort) as the primary metadata source for navigation entries. Stale or inaccurate README descriptions degrade the browsing experience. This task ensures all READMEs reflect the current state after the layer taxonomy rename and structural changes.

## How

1. List all directories under `.orqa/` that contain a README.md
2. For each README, compare its description to the actual contents of the directory
3. Update any descriptions that are stale, inaccurate, or reference deprecated terminology
4. Verify icon choices are appropriate for the group's purpose
5. Ensure sort values produce a logical ordering in the sidebar

## Verification

- [ ] Every `.orqa/**/README.md` has been reviewed
- [ ] No README references "canon" layer (should be "core")
- [ ] Descriptions match actual directory contents
- [ ] App sidebar shows correct labels and descriptions from README metadata
