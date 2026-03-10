---
id: TASK-077
title: Broken link styling and path validation
description: Style unresolvable artifact IDs as broken links with warning colour and broken-link icon. Validate docs-required/docs-produced paths against disk during scan.
status: done
scope:
  - ui/lib/components/artifact/ArtifactLink.svelte
  - ui/lib/components/artifact/FrontmatterHeader.svelte
created: "2026-03-10"
updated: "2026-03-10"
epic: EPIC-048
depends-on:
  - TASK-083
assignee: AGENT-002
skills:
  - SKILL-030
  - SKILL-031
acceptance:
  - ArtifactLink renders broken state when artifactGraph.resolve(id) returns undefined
  - Broken links use broken-link Lucide icon and app warning colour token
  - docs-required/docs-produced paths validated against disk — broken paths flagged visually
  - Working links unchanged in appearance
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
