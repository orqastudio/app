---
id: EPIC-4e92b6f0
type: epic
title: Core Methodology Plugin
description: Build the first-party methodology plugin that ships the 7 thinking modes, 4 entry modes, and opinionated structured thinking workflows as composable plugin contributions. This is the reference implementation for the plugin framework and the content layer that makes OrqaStudio opinionated.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: next
scoring:
  impact: 5
  urgency: 3
  complexity: 4
  dependencies: 4
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Stream 4 — the methodology plugin is what makes OrqaStudio opinionated, not just a framework
  - target: EPIC-c8f1a7d3
    type: depends-on
    rationale: Requires composition pipeline to be operational before plugin content can load
---

## Context

OrqaStudio is an opinionated clarity engine. The methodology — the 7 thinking modes, the entry modes, the structured thinking workflows — is the product. This plugin is what ships that methodology. The framework is useless without content; this plugin is the content.

## Acceptance Criteria

- [ ] Plugin manifest defines all 7 thinking modes with activation signals and quality gates
- [ ] Plugin manifest defines 4 entry modes (Problem, Idea, Goal, Chaos) with detection heuristics
- [ ] Structured thinking workflows for each mode (Research → report, Planning → plan, etc.)
- [ ] Plugin loads cleanly through the composition pipeline
- [ ] A new user can reach a thinking mode within 10 minutes without knowing the framework
- [ ] Plugin serves as documented reference implementation for third-party plugin authors
