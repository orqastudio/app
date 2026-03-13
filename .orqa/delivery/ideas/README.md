---
role: artifacts
label: "Ideas"
description: "Candidate features that need research and validation before promotion."
icon: "lightbulb"
sort: 1
---

# Ideas

Ideas capture "what if we..." and "we should eventually..." moments. They are the entry point to the delivery pipeline — unvalidated candidates that need research before they can become epics.

## Pipeline Role

Ideas are the **intake stage** of the delivery pipeline:

```
Idea → Research → Epic → Task → Verification
```

An idea cannot skip directly to an epic. It must pass through `exploring` (investigation) and `shaped` (clear scope and approach) before promotion. This gate prevents half-formed ideas from consuming implementation capacity.

## Lifecycle

```
captured → exploring → shaped → promoted (or archived)
```

- **Captured**: Initial idea recorded with pillar alignment and `research-needed` items
- **Exploring**: User approves investigation; research begins
- **Shaped**: Research complete; scope and approach clearly defined
- **Promoted**: Graduated to an `EPIC-NNN`; `promoted-to` field set
