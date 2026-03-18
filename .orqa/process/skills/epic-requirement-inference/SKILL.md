---
id: SKILL-047
title: Plugin Artifact Usage
description: |
  How plugin-provided artifact types and relationships work in the platform.
  Teaches agents to discover what artifacts a plugin provides, what
  relationships connect them, and what constraints exist. The framework
  that every plugin's own skill+doc pair builds on.
  Use when: Working with plugin-provided artifact types, setting up a project
  with plugins, or understanding how plugins extend the artifact graph.
status: active
category: methodology
version: 2.0.0
user-invocable: true
relationships:
  - target: DOC-085
    type: synchronised-with
---

Teaches agents how plugins extend OrqaStudio's artifact graph with new types,
relationships, and views. This is the framework skill — each plugin provides
its own skill+doc pair that builds on these concepts.

## How Plugins Extend the Artifact Graph

Plugins register artifact schemas (types), relationships, and views via their
`orqa-plugin.json` manifest. At runtime, the platform merges three layers into
a single configuration that agents and the UI consume:

1. **Core types** (from `core.json`) — platform-shipped, immutable
2. **Plugin types** (from each installed plugin's `orqa-plugin.json`) — plugin-shipped, versioned
3. **Project config** (from `project.json`) — user-editable overrides and additions

Agents discover available types by reading the merged config, never by
hardcoding type keys or relationship names. If a type or relationship does not
appear in the merged config, it does not exist.

## Discovering Plugin Artifacts

Read `orqa-plugin.json` in each plugin directory under `plugins/`. The manifest
describes everything the plugin provides.

### Artifact types — `provides.schemas`

Each entry in the `provides.schemas` array defines an artifact type:

| Field | Purpose |
|-------|---------|
| `key` | Unique type identifier (e.g., `epic`, `task`, `milestone`) |
| `label` | Human-readable name |
| `idPrefix` | ID pattern prefix (e.g., `EPIC`, `TASK`) |
| `defaultPath` | Where artifacts of this type live in the filesystem |
| `frontmatter` | Required and optional frontmatter fields |
| `statusTransitions` | Map of valid state transitions from each status |

### Relationships — `provides.relationships`

Each entry in the `provides.relationships` array defines a relationship:

| Field | Purpose |
|-------|---------|
| `key` | Forward relationship name (e.g., `delivers`) |
| `inverse` | Inverse relationship name (e.g., `delivered-by`) |
| `from` | Type constraint on the source artifact |
| `to` | Type constraint on the target artifact |
| `semanticCategory` | Category: lineage, hierarchy, governance, etc. |
| `required` | Whether the relationship is required for the source type |
| `minCount` | Minimum number of targets when required |

### Navigation — `defaultNavigation`

The `defaultNavigation` section describes how the plugin's artifacts appear in
the sidebar. Navigation entries are graph filters, not filesystem paths — they
query artifacts by type and relationship.

## Working With Plugin Artifacts

### Creating artifacts

Use the plugin's `defaultPath` to determine where to create files and the
`idPrefix` to generate the correct ID pattern. For example, if a plugin defines
`defaultPath: ".orqa/delivery/epics/"` and `idPrefix: "EPIC"`, then a new epic
goes in that directory with an ID like `EPIC-001`.

Always include the required frontmatter fields defined in the plugin's schema.

### Connecting artifacts

Use the plugin's relationship definitions. Respect `from`/`to` constraints —
these are enforced by the integrity layer. For example, if a relationship
defines `from: "task"` and `to: "epic"`, do not use it from an idea to a
milestone.

Remember that relationships are bidirectional. When you add a forward
relationship on artifact A targeting artifact B, you must also add the inverse
relationship on artifact B targeting artifact A.

### Status transitions

Respect the plugin's `statusTransitions` map. Only transition an artifact to a
status that is listed as a valid target from the artifact's current status. For
example, if `statusTransitions.active` lists `["hold", "blocked", "review"]`,
then an active artifact can only move to those three states — not directly to
`completed`.

### Constraints

Some relationships have `required: true` and `minCount` values. These are
enforcement checks — the integrity scanner will flag violations. For example,
if tasks require at least one `delivers` relationship, every task must be linked
to a parent before it passes integrity.

## The Skill+Doc Pattern

Every plugin that defines artifact types SHOULD ship a paired skill and doc:

- A **skill** (e.g., `SKILL-SW-001`) explaining how agents should work with the
  plugin's artifacts — creation workflows, relationship patterns, status
  lifecycle, and common scenarios
- A **doc** (e.g., `DOC-SW-001`) explaining how humans should understand the
  plugin's artifacts — what they represent, when to use each type, and how they
  relate to each other
- The skill and doc are connected via a `synchronised-with` relationship so
  changes to one prompt a review of the other

This skill (SKILL-047) is the *framework* — it teaches the general mechanics of
plugin artifacts. Each plugin's own skill provides the *content* — the specific
types, workflows, and domain knowledge for that plugin.

## Example: Software Plugin

The software delivery plugin (`plugins/software/orqa-plugin.json`) demonstrates
the full pattern:

- **5 artifact types:** milestone, epic, task, research, wireframe
- **9 relationships:** delivers, fulfils, depends-on, realises, produces,
  yields, reports, fixes, affects
- **SKILL-SW-001** teaches agents the software delivery lifecycle — how to
  break milestones into epics, epics into tasks, and how status propagates
  up the delivery hierarchy
- **DOC-SW-001** teaches humans the same concepts in user-facing language

Other plugins follow the same pattern: define types and relationships in
`orqa-plugin.json`, then ship a skill+doc pair that teaches agents and humans
how to use them.
