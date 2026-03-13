---
role: artifacts
label: "Research"
description: "Investigation artifacts that validate ideas and inform architectural decisions."
icon: "flask-conical"
sort: 2
---

# Research

Research documents capture investigations, design explorations, and architecture spikes. They are the bridge between an idea and a decision — turning "we should look into this" into "here is what we found and what we recommend."

## Pipeline Role

Research sits between the delivery pipeline and the knowledge maturity pipeline:

```
Delivery:  Idea → Research → Epic
Process:   Research → Decision (AD-NNN)
```

A research document's findings feed two outputs: an epic's `research-refs` field (the design was informed by this) and an architecture decision (the conclusion is now a durable principle). Research documents are permanent records — when surpassed, they are marked `status: surpassed` and `surpassed-by`, never deleted.

## Lifecycle

```
draft → complete → surpassed
```

- **Draft**: Investigation in progress
- **Complete**: Findings documented; recommendation made
- **Surpassed**: Newer research supersedes this document

## Structure

Each document includes: the question being investigated, findings, options evaluated with trade-offs, and a concrete recommendation.
