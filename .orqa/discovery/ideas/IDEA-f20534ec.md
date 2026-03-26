---
id: IDEA-f20534ec
type: discovery-idea
title: "Status bar connectivity should reflect daemon health"
status: captured
description: The app's status bar 'connected' indicator should represent live connectivity to the daemon (orqa-validation HTTP server), not just a static label. Show connected/disconnected/degraded based on periodic health checks.
created: 2026-03-23
relationships:
  - target: PERSONA-c4afd86b
    type: benefits
    rationale: "Serves the primary developer persona"
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: Clarity Through Structure — making system health visible
---

## Observation

The status bar shows "connected" but this doesn't reflect actual daemon connectivity. If the daemon is down, the app has no access to the artifact graph, search, or validation — but the UI doesn't indicate this.

## What It Should Do

- Periodic health check against the daemon endpoint (e.g. `GET /health` every 10s)
- **Connected** — daemon responds, artifact count available
- **Disconnected** — daemon unreachable, show clear warning
- **Degraded** — daemon responds but search index is stale or graph has violations

The status bar is the first thing users see — it should be the source of truth for system health.
