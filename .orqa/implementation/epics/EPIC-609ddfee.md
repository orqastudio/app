---
id: EPIC-609ddfee
type: epic
title: Plugin development workflow — three-tier skill architecture and template updates
description: "Established the three-tier plugin development skill architecture: base KNOW-2f38309a (platform plugin development), first-party KNOW-46f68631 (OrqaStudio-maintained plugins), and third-party KNOW-1a4f41f7 (community plugins). Updated plugin templates with thumbnails, README boilerplate, deactivated workflow templates, and schema compatibility."
status: archived
created: 2026-03-19
milestone: MS-b1ac0a20
relationships:
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---

# EPIC-609ddfee: Plugin Development Workflow

## What Was Done

1. **Three-tier skill architecture** — defined clear separation between platform-level plugin guidance (KNOW-2f38309a), first-party plugin development workflow (KNOW-e6fee7a0), and third-party/community plugin development (KNOW-1b7fa054). Each tier has appropriate scope and access.

2. **Plugin templates updated** — plugin template repositories now include:
   - Thumbnails per plugin folder for registry display
   - README boilerplate with badge standards
   - Deactivated GitHub Actions workflow templates (activated on first use)
   - Schema compatibility declarations in orqa-plugin.json

3. **ArtifactListItem performance fix** — the component now derives active state internally via a `path` prop instead of relying on external store subscriptions, reducing unnecessary re-renders across the navigation panel.

4. **Vite config fixes** — `fs.allow` configured for npm-linked packages so Vite can resolve files outside the project root, and `optimizeDeps.exclude` prevents Vite from pre-bundling linked packages (which would break HMR).

## Why

Plugin development is the primary extension point for OrqaStudio. Without clear guidance at each tier, plugin authors would make inconsistent choices that fragment the ecosystem. The template updates reduce boilerplate and enforce consistency from the first commit.
