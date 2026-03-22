---
id: IDEA-3f67dc75
title: "Release lifecycle — milestones as releases, update management, breaking change protocol, plugin compatibility"
description: "A controlled release workflow where milestones drive releases, the app manages updates with breaking change awareness, and plugins have a backwards compatibility lifecycle with deprecation and migration paths."
status: captured
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - How does a milestone gate translate to a release gate? (all P1 epics completed → release candidate?)
  - Should the mapping between milestones and releases be configurable per project?
  - What does the in-app update flow look like? Check on startup? Background check? Manual?
  - How do we surface breaking changes before the user updates? (changelog? diff? impact analysis?)
  - How do we detect which plugins are affected by a breaking change? (schema diff between versions)
  - What's the backwards compatibility lifecycle? How long do we support deprecated APIs?
  - Can we version the plugin API (orqa-plugin.json schema version) separately from the app version?
  - How do we recommend alternative plugins when one is deprecated?
  - Should the GitHub integration (IDEA-b1953a83) drive milestone→release→GitHub Release flow?
  - How do different projects configure what links to what (e.g. some use milestones as releases, others use tags)?
  - Can we generate changelogs from the artifact graph (completed epics + tasks since last release)?
relationships:
  - target: PILLAR-94b281db
    type: grounded
    rationale: A controlled release lifecycle preserves intent across versions — users and plugin authors aren't surprised by breaking changes
  - target: PERSONA-015e8c2c
    type: benefits
---

# Release Lifecycle

## Milestones as Releases

Releases should come from milestones, not arbitrary version bumps. A milestone represents a coherent body of work with gate criteria. When the gate passes (all P1 epics completed, all checks green), that's a release candidate.

But different projects have different workflows — what constitutes a "release" and what triggers it should be configurable. Some projects release per milestone. Others batch milestones into releases. The mapping between delivery artifacts and release artifacts needs to be project-configurable, not hardcoded.

## In-App Update Management

When a new version is available, the app needs to:

1. **Detect** — check for updates (startup? background? manual?)
2. **Assess** — what changed? Are there breaking changes?
3. **Surface** — show the user what's coming before they commit
4. **Migrate** — handle schema changes, config changes, plugin compatibility

The key insight: an update isn't just "new version available, click to install." It's "here's what changed, here's what it affects, here are the risks."

## Breaking Changes

A breaking change is any change that could cause existing plugins, artifacts, or workflows to stop working. This includes:

- Adding required relationships to core.json (existing artifacts won't have them)
- Removing or renaming relationship keys
- Changing from/to type constraints on relationships
- Changing the orqa-plugin.json schema
- Removing or renaming IPC commands
- Changing the artifact frontmatter format

### Detection

The app should be able to diff the current core.json against the incoming version's core.json and identify:
- New required relationships → which artifacts need updating
- Removed relationship keys → which artifacts use them
- Changed constraints → which plugins are affected
- Schema changes → which plugin manifests need updating

### Communication

Before an update applies:
- Show a breaking change report
- List affected plugins with their compatibility status
- Offer migration guidance or auto-migration where possible
- Let the user decide: update now, defer, or update with migration

## Plugin Compatibility Lifecycle

When the platform makes a breaking change:

1. **Announcement** — the breaking change is documented in the release notes with a migration guide
2. **Deprecation window** — the old behaviour continues to work for N versions (configurable), with a warning
3. **Plugin notification** — registered plugins whose manifests are affected get flagged in the registry
4. **Migration period** — plugin authors update their manifests. The app shows "update available" for affected plugins
5. **End of life** — after the deprecation window, the old behaviour is removed. Plugins that haven't updated show "incompatible with current version"
6. **Alternatives** — if a plugin is deprecated/incompatible, the app can recommend alternative plugins from the registry that provide similar functionality
7. **User choice** — the user can choose to not upgrade the app, staying on the last compatible version. They're not forced into losing plugin functionality

## Configurable Workflow Mapping

Different projects map delivery artifacts to release artifacts differently:

| Pattern | Milestone → Release | Use Case |
|---------|---------------------|----------|
| 1:1 | Each milestone is a release | Small projects, rapid iteration |
| N:1 | Multiple milestones batch into a release | Large projects, quarterly releases |
| Tagged | Releases triggered by git tags, milestones are planning only | Open-source projects |
| Continuous | Every merged PR is a release (no milestones) | SaaS, continuous deployment |

This mapping should live in `project.json` or the software plugin config — not hardcoded.

## Changelog Generation

The artifact graph already knows:
- Which epics were completed since the last release
- Which tasks delivered those epics
- Which decisions drove the work
- Which ideas were realised

A changelog could be auto-generated from this graph traversal, categorised by pillar alignment, with links back to the relevant artifacts.
