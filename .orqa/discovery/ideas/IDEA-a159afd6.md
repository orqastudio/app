---
id: IDEA-a159afd6
type: discovery-idea
title: "Split unified logger into separate metrics and logger libraries"
description: "Separate the current unified logging library into two distinct libraries — one for structured metrics/telemetry and one for operational logging. Allows consumers to use each independently and enables richer metrics collection without polluting log output."
status: captured
created: 2026-03-26
updated: 2026-03-26
relationships:
  - target: "PILLAR-c9e0a695"
    type: grounded
    rationale: "Structured metrics make system behavior visible and measurable"
  - target: "PERSONA-477971bf"
    type: benefits
    rationale: "Practitioners need observability into system cost and performance"
---

## Context

The current architecture uses a unified logger library for both operational logging and metrics/telemetry. This is sufficient for debugging during pre-release, but as the system matures the two concerns have different requirements:

- **Metrics** need structured data, time-series storage, aggregation, and dashboard integration (token usage, agent costs, cache hit rates, enforcement pass/fail rates)
- **Logging** needs human-readable output, log levels, contextual tracing, and rotation

## Proposed Approach

Split into two Rust library crates:
- `libs/logger` — operational logging (info, warn, error, debug, trace)
- `libs/metrics` — structured telemetry (counters, gauges, histograms, token tracking)

Both consumed independently by daemon, app, CLI, and connectors. Connectors currently using custom telemetry endpoints would switch to the metrics library.
