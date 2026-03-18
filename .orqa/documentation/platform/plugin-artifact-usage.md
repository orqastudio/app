---
id: DOC-085
title: Plugin Artifact Usage Guide
description: "How plugin-provided artifact types and relationships extend the platform graph. Reference for understanding and working with any plugin's artifacts."
category: concept
created: 2026-03-18
updated: 2026-03-18
relationships:
  - target: SKILL-047
    type: synchronised-with
---

# Plugin Artifact Usage Guide

Plugins extend the artifact graph by registering new artifact types, relationships, views, and widgets. This guide explains how plugin-provided artifacts work and how to use them effectively.

## How Plugins Extend the Graph

The platform defines core artifact types (idea, decision, rule, lesson, skill, agent, doc, pillar, persona, vision, pivot, bug) in `core.json`. Plugins add domain-specific types on top.

At runtime, three layers merge:
1. **Core** (`core.json`) — platform types and relationships, always present
2. **Plugins** (`orqa-plugin.json` per plugin) — domain-specific types, relationships, views
3. **Project** (`project.json`) — navigation configuration and project metadata

The merged result is what agents and the UI see. No artifact types or relationships are hardcoded in any code path.

## Reading a Plugin Manifest

Every plugin has an `orqa-plugin.json` file. The key sections:

### `provides.schemas`

Each entry defines an artifact type:

| Field | Purpose |
|-------|---------|
| `key` | Type identifier (e.g., `epic`, `task`) |
| `label` / `plural` | Display names |
| `icon` | Lucide icon name |
| `defaultPath` | Where artifacts of this type live on disk |
| `idPrefix` | ID pattern (e.g., `EPIC` → `EPIC-001`) |
| `frontmatter.required` | Fields that must be present |
| `statusTransitions` | Valid status changes for this type |

### `provides.relationships`

Each entry defines a connection type:

| Field | Purpose |
|-------|---------|
| `key` / `inverse` | Forward and inverse relationship names |
| `from` / `to` | Type constraints (e.g., task→epic only) |
| `semantic` | Category (hierarchy, dependency, lineage, etc.) |
| `constraints` | Required counts, status transition rules |

### `provides.skills`

Skills the plugin ships — loaded when agents work with plugin artifacts.

## Working With Plugin Artifacts

### Creating Artifacts

Use the plugin's `defaultPath` and `idPrefix` to determine where to create files and what ID pattern to use. Respect `frontmatter.required` for mandatory fields.

### Connecting Artifacts

Use the plugin's relationship definitions. Always respect `from`/`to` constraints. Always create bidirectional relationships (forward + inverse).

### Status Transitions

Only transition to states listed as valid from the current state in `statusTransitions`. Some relationships have `statusRules` that propose transitions automatically (e.g., "epic moves to review when all delivering tasks are completed").

## The Skill+Doc Pattern

Every plugin that defines artifact types SHOULD ship with:
- A **skill** explaining how agents should work with the artifacts
- A **doc** explaining how humans should understand the artifacts
- Connected via `synchronised-with` relationship

The skill teaches agents; the doc teaches humans. They cover the same ground but in different ways.

## Example: Software Plugin

The software plugin (`plugins/software/`) defines 5 types: milestone, epic, task, research, wireframe. It ships 9 custom relationships (delivers, fulfils, depends-on, realises, produces, yields, reports, fixes, affects).

- **SKILL-SW-001** teaches agents the software delivery lifecycle
- **DOC-SW-001** teaches humans the same concepts

Any new plugin should follow this pattern.
