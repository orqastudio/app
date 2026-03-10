---
id: TASK-050
title: Document governance classification schema
status: done
epic: EPIC-044
description: Updates the artifact framework documentation to formally define the three-layer classification model and agent scope categories, and updates the skill enforcement rule to reference layer-aware loading decisions.
created: 2026-03-09
updated: 2026-03-09
assignee: agent-maintainer
skills: [orqa-governance]
scope: [.orqa/documentation/product/artifact-framework.md, .orqa/governance/rules/skill-enforcement.md]
acceptance: [artifact-framework.md documents the three-layer classification (canon/project/plugin), artifact-framework.md documents agent scope categories, skill-enforcement.md updated with layer-aware loading rules]
---

## Changes Needed

1. Add "Governance Classification" section to artifact-framework.md defining:
   - Three layers (canon, project, plugin)
   - Agent scope categories (software-engineering, governance, general)
   - Which frontmatter fields carry classification

2. Update skill-enforcement.md to reference layer classification for loading decisions
