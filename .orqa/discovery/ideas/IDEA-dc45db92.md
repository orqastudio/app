---
id: IDEA-dc45db92
type: discovery-idea
title: Artifact Funnel Visualisation Plugin
description: "Visual funnel showing the flow of work through the idea → research → epic → task pipeline, with counts, conversion rates, and stage duration metrics at each step. Implemented as a plugin using the Artifact Graph SDK."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: later
research-needed:
  - "What funnel visualisation patterns work best for non-linear flows (ideas can skip stages, tasks can block)?"
  - "How to handle artifacts that don't follow the standard funnel (e.g., epics created directly without an idea)?"
  - "What metrics matter most at each funnel stage? (count, avg duration, drop-off rate, promotion rate)"
  - "Should the funnel be interactive (click a stage to see the artifacts in it)?"
  - "How does this integrate with the plugin system (AD-0dfa4d52 future work)?"
  - "What seed data format works for the plugin to consume from the Artifact Graph SDK?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---
## Motivation

OrqaStudio's artifact lifecycle defines a natural funnel:

```
Ideas (captured) → Ideas (exploring) → Ideas (shaped) → Epics (draft) → Epics (ready) → Tasks (in-progress) → Tasks (done)
```

But there's no way to see this flow visually. Questions like "how many ideas actually become epics?", "where do things get stuck?", and "what's our idea-to-delivery conversion rate?" require manually counting artifacts.

A funnel visualisation makes the pipeline tangible — showing where work accumulates, where it drops off, and how quickly it flows through each stage. This serves both pillars: Clarity Through Structure (making the invisible pipeline visible) and Learning Through Reflection (identifying process bottlenecks over time).

Research artifacts fit into this as a parallel enrichment track — ideas spawn research, research informs epic design. The funnel can show research as a side channel feeding into the main flow.

## Sketch

- Classic funnel diagram: wide at top (ideas), narrow at bottom (completed tasks)
- Each stage shows: count of artifacts, average time in stage, conversion rate to next stage
- Colour coding: green (healthy flow), amber (accumulating), red (stalled)
- Click a stage to expand into the list of artifacts at that status
- Side channel showing research docs feeding into the epic stage
- Time selector to view funnel state at different points (last week, last month, all time)
- Built as a plugin consuming `get_artifacts_by_type`, `get_graph_stats`, and frontmatter timestamps from the Artifact Graph SDK