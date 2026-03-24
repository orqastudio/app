---
id: IDEA-0d7e84de
type: idea
title: "Competitor research and market analysis agent for the software plugin"
description: "Add a specialised agent and supporting knowledge to the software project plugin that performs competitor research and market analysis as part of the discovery phase. The agent would research competing products, analyse market positioning, identify differentiators, and feed findings into the idea and epic pipeline."
status: captured
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: PILLAR-cdf756ff
    type: grounded
    rationale: "Learning Through Reflection — market intelligence becomes a structured input to the discovery process rather than ad-hoc research"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Clarity Through Structure — competitor and market data is captured as structured artifacts, not scattered notes"
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: "Gives product owners structured market intelligence for prioritisation"
---

## The Idea

The software project plugin (`plugins/software/`) should include a **market research agent** with specialised knowledge for:

- **Competitor analysis** — identifying competing products, analysing their features, strengths, weaknesses, pricing, and positioning
- **Market landscape mapping** — understanding the market segment, target personas, and unmet needs
- **Differentiator identification** — surfacing what makes (or could make) the product unique
- **Trend analysis** — tracking relevant technology and market trends that affect product direction

## How It Would Work

1. A new agent definition in the software plugin with `market-researcher` role
2. Supporting knowledge artifacts covering research methodology, analysis frameworks (e.g., SWOT, Porter's Five Forces adapted for software), and structured output formats
3. Research artifacts produced as standard `.orqa/discovery/research/` documents that feed into the idea pipeline
4. Integration with web search tools for gathering current market data

## Why This Belongs in the Software Plugin

Market analysis is universal to software projects but not to all OrqaStudio project types. A governance-only project or documentation project wouldn't need it. The software plugin is the right home because it already provides the software development methodology layer.
