---
id: TASK-489
title: Add grounding/ to project.json + create chapter READMEs
description: "Register the grounding/ directory in the project.json artifact config so it is scannable. Create README.md files for each new chapter directory (about, how-to, reference) with icon, label, and description frontmatter for the nav tree."
status: completed
priority: P2
scoring:
  impact: 2
  urgency: 2
  complexity: 1
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - grounding/ added to project.json artifact config and resolves to an existing directory
  - "README.md created for about/ chapter with icon, label, description frontmatter"
  - "README.md created for how-to/ chapter with icon, label, description frontmatter"
  - "README.md created for reference/ chapter with icon, label, description frontmatter"
  - All chapters scannable in the artifact browser after changes
relationships:
  - target: EPIC-075
    type: delivers
    rationale: Chapter registration and nav metadata for the reorganised documentation structure
  - target: TASK-486
    type: depends-on
---
