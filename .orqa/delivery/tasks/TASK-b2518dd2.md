---
id: TASK-b2518dd2
type: task
title: "Plugin hook injection mechanism — plugins declare behavioral rules and mode templates"
description: "Implement a mechanism by which plugins declare behavioral rules and mode templates in their manifests, and the connector aggregates these at runtime to build the injected system context."
status: captured
created: 2026-03-21
updated: 2026-03-21
acceptance:
  - orqa-plugin.json manifest schema supports a hooks or injection section for behavioral rules and mode templates
  - The connector reads each installed plugin's manifest at startup and aggregates declared behavioral rules
  - Mode templates from plugins are merged into the connector's runtime context injection
  - No behavioral rules or mode templates are hardcoded in the connector itself
  - orqa validate passes on all plugin manifests after schema update
  - Integration test: a new plugin with a behavioral rule results in that rule appearing in injected context
relationships:
  - target: EPIC-6967c7dc
    type: delivers
---

## What

Plugins should be able to declare their own behavioral rules and mode templates in their `orqa-plugin.json` manifests. The connector aggregates these declarations at runtime and injects them into the system context. Currently, behavioral rules and mode templates are either hardcoded in the connector or maintained separately, creating drift as plugins evolve.

## Why

The plugin system's promise is that plugins are self-describing and self-contained. Behavioral context (rules, templates) that a plugin contributes to the agent's reasoning should be declared in the plugin's manifest, not duplicated or hardcoded elsewhere. This is a prerequisite for TASK-dc8d94c3 (removing hardcoded plugin-specific rules from the connector).

## How

1. Define the manifest schema extension — what does the `injection` or `hooks` section look like in `orqa-plugin.json`?
2. Update the manifest schema (JSON Schema or equivalent) to include the new section
3. Update the connector's runtime aggregation logic to read this section from each installed plugin
4. Build the merged context injection from all plugin contributions
5. Update existing plugins (software, claude, governance) to declare their behavioral rules in their manifests
6. Remove any connector-side copies of these declarations

## Verification

1. `orqa validate` passes on all updated plugin manifests
2. A test plugin with a declared behavioral rule produces that rule in the injected runtime context
3. No behavioral rules remain hardcoded in the connector hook scripts
4. TASK-dc8d94c3 is unblocked
