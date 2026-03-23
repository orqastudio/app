---
id: EPIC-76e9a5da
type: epic
title: Plugin development workflow — three-tier skill architecture and template updates
description: "Established the three-tier plugin development skill architecture: base KNOW-b453410f (platform plugin development), first-party KNOW-12ed4953 (OrqaStudio-maintained plugins), and third-party KNOW-c60144c1 (community plugins). Updated plugin templates with thumbnails, README boilerplate, deactivated workflow templates, and schema compatibility."
status: completed
created: 2026-03-19
milestone: MS-654badde
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---

# EPIC-76e9a5da: Plugin Development Workflow

## What Was Done

1. **Three-tier skill architecture** — defined clear separation between platform-level plugin guidance (KNOW-b453410f), first-party plugin development workflow (KNOW-e1333874), and third-party/community plugin development (KNOW-63cc1a00). Each tier has appropriate scope and access.

2. **Plugin templates updated** — plugin template repositories now include:
   - Thumbnails per plugin folder for registry display
   - README boilerplate with badge standards
   - Deactivated GitHub Actions workflow templates (activated on first use)
   - Schema compatibility declarations in orqa-plugin.json

3. **ArtifactListItem performance fix** — the component now derives active state internally via a `path` prop instead of relying on external store subscriptions, reducing unnecessary re-renders across the navigation panel.

4. **Vite config fixes** — `fs.allow` configured for npm-linked packages so Vite can resolve files outside the project root, and `optimizeDeps.exclude` prevents Vite from pre-bundling linked packages (which would break HMR).

## Why

Plugin development is the primary extension point for OrqaStudio. Without clear guidance at each tier, plugin authors would make inconsistent choices that fragment the ecosystem. The template updates reduce boilerplate and enforce consistency from the first commit.