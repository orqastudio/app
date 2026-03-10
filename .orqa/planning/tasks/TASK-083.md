---
id: TASK-083
title: "Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout"
description: "Replace ArtifactLink prefix routing, ArtifactNav pendingArtifactId workaround, and AppLayout watch init with SDK-based patterns."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-076
assignee: frontend-engineer
skills:
  - svelte5-best-practices
  - orqa-store-patterns
scope:
  - ui/lib/components/artifact/ArtifactLink.svelte
  - ui/lib/components/navigation/ArtifactNav.svelte
  - ui/lib/components/artifact/FrontmatterHeader.svelte
  - ui/lib/components/layout/AppLayout.svelte
acceptance:
  - "ArtifactLink uses artifactGraph.resolve(id) for navigation"
  - "ArtifactNav removes isTree guard — auto-select works for flat AND tree types"
  - "FrontmatterHeader uses SDK resolve to determine if a value is a valid artifact link"
  - "AppLayout watcher integration replaced with SDK auto-refresh"
  - "ARTIFACT_ID_RE regex in FrontmatterHeader removed — SDK determines linkability"
---
