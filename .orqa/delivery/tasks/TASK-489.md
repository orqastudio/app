---
id: TASK-489
title: "Add grounding/ to project.json + create chapter READMEs"
description: "Register the grounding/ directory in the project.json artifact config so it is scannable. Create README.md files for each new chapter directory (about, how-to, reference) with icon, label, and description frontmatter for the nav tree."
status: todo
priority: P2
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-075
depends-on:
  - TASK-486
acceptance:
  - grounding/ added to project.json artifact config and resolves to an existing directory
  - README.md created for about/ chapter with icon, label, description frontmatter
  - README.md created for how-to/ chapter with icon, label, description frontmatter
  - README.md created for reference/ chapter with icon, label, description frontmatter
  - All chapters scannable in the artifact browser after changes
relationships:
  - target: EPIC-075
    type: delivers
    rationale: Chapter registration and nav metadata for the reorganised documentation structure
  - target: EPIC-075
    type: belongs-to
    rationale: Task belongs to this epic
---
