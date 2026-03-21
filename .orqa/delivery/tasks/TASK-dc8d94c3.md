---
id: TASK-dc8d94c3
type: task
title: "Remove hardcoded plugin-specific rules from connector — move to owning plugins"
description: "Move epic/task sync rules to the software plugin and governance-specific rules to the governance plugin. The connector must not own logic that belongs to a specific plugin domain."
status: captured
created: 2026-03-21
updated: 2026-03-21
acceptance:
  - Epic/task sync behavioral rules are declared in the software plugin manifest or hooks, not in the connector
  - Governance-specific behavioral rules are declared in the governance plugin, not in the connector
  - The connector contains no hardcoded references to software-plugin or governance-plugin specific logic
  - All moved rules are accessible at runtime via the plugin hook injection mechanism (TASK-b2518dd2)
  - orqa validate passes on the connector and all affected plugins
  - End-to-end behavior is unchanged — rules are applied at the same points in the workflow
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-b2518dd2
    type: depends-on
---

## What

The connector currently contains behavioral rules that are specific to individual plugins (software, governance). These rules should live in the owning plugin and be injected by the plugin hook injection mechanism. This decouples the connector from plugin-specific concerns.

## Scope

- **Epic/task sync rules** — currently in the connector, belong in the software plugin
- **Governance rules** — currently in the connector, belong in the governance plugin

## Why

The connector is a bridge, not a plugin owner. Rules about how epic/task sync works belong in the software plugin that defines that workflow. Rules about governance artifact structure belong in the governance plugin. Keeping them in the connector creates coupling and means plugin changes require connector changes.

Blocked by TASK-b2518dd2 — the plugin hook injection mechanism must exist before this migration can happen cleanly.

## How

1. Identify all plugin-specific rules currently hardcoded in the connector (search connector hooks and system prompt files)
2. For each rule, determine the owning plugin
3. Declare each rule in the owning plugin's manifest using the injection schema from TASK-b2518dd2
4. Remove the hardcoded rule from the connector
5. Verify end-to-end that rules are still applied at runtime

## Verification

1. `search_regex` in connector source returns no matches for software-specific or governance-specific logic
2. Software plugin manifest declares the epic/task sync rules
3. Governance plugin manifest declares the governance rules
4. Runtime behavior test: both rule sets are present in injected context when both plugins are installed
5. `orqa validate` passes on all affected artifacts
