---
id: TASK-077
title: Broken link styling and path validation
description: "Style unresolvable artifact IDs as broken links with warning colour and broken-link icon. Validate docs-required/docs-produced paths against disk during scan."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-083
assignee: frontend-engineer
skills:
  - svelte5-best-practices
  - tailwind-design-system
scope:
  - ui/lib/components/artifact/ArtifactLink.svelte
  - ui/lib/components/artifact/FrontmatterHeader.svelte
acceptance:
  - "ArtifactLink renders broken state when artifactGraph.resolve(id) returns undefined"
  - "Broken links use broken-link Lucide icon and app warning colour token"
  - "docs-required/docs-produced paths validated against disk — broken paths flagged visually"
  - "Working links unchanged in appearance"
---
