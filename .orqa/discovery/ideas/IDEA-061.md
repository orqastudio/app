---
id: IDEA-061
title: Plugin architecture for verification data collection
description: "Plugins that provide skill + hooks + data collection for enforcement tooling. Each plugin closes the enforcement-verification loop: the same plugin that enforces also provides metrics for automated verification."
status: captured
created: 2026-03-12
updated: 2026-03-13
horizon: later
research-needed:
  - Plugin data collection API design — how plugins expose metrics endpoints
  - VER-NNN record schema and storage (file vs SQLite based on volume)
  - Parameter comparison engine — how automated pass/fail works
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---

## Motivation

[AD-042](AD-042) defines enforcement tooling as plugins providing three capabilities: skill (how the tool works), data collection (metrics for automated verification), and enforcement (hooks). OrqaStudio core provides the interfaces; plugins provide the specifics. This idea covers building those interfaces.
