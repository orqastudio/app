---
id: IMPL-044
title: Idea promotion should automatically surface related ideas for bundling
description: "When promoting an idea to an epic, the orchestrator should automatically scan all other ideas for thematic overlap and ask the user whether to bundle them into the same epic. Currently this requires manual prompting — the user had to explicitly ask 'see if there are any other ideas relevant to the dashboard'."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships: []
---
## Pattern

During idea promotion (shaped → promoted), the orchestrator creates the epic from a single idea without checking whether other ideas address the same domain or could be bundled. The user has to manually ask "see if there are any other ideas relevant to the dashboard." This is exactly the kind of graph traversal the system should do automatically.

## Fix

Add a step to the idea promotion procedure in [RULE-004](RULE-004): before creating the epic, scan all ideas with `status: captured | exploring | shaped` for thematic overlap with the idea being promoted. Present any matches to the user and ask whether to:
1. Bundle them into the same epic (promote together)
2. Keep them separate (promote independently later)
3. Ignore (no real overlap)

This leverages the artifact graph — ideas share pillars, reference similar concepts, or have overlapping `research-needed` items. The scan can use keyword matching on titles/descriptions and shared pillar references.
