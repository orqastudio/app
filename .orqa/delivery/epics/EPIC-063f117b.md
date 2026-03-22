---
id: EPIC-063f117b
type: epic
title: GitHub Packages CI/CD pipeline — all 7 packages publishing
description: Established CI/CD pipeline for all 7 OrqaStudio npm packages via GitHub Packages. SHA-based versioning, npm ci for reproducible installs, caret dependencies for flexibility. All packages publish on push to main.
status: completed
created: 2026-03-19
milestone: MS-654badde
relationships:
  - target: MS-654badde
    type: fulfils
---

# EPIC-063f117b: GitHub Packages CI/CD Pipeline

## What Was Done

1. **All 7 packages publishing** — @orqastudio/types, @orqastudio/sdk, @orqastudio/svelte-components, @orqastudio/graph-visualiser, @orqastudio/integrity-validator, and plugin packages all publish to GitHub Packages.

2. **SHA-based versions** — each publish uses a version derived from the git SHA, ensuring every build is uniquely identifiable and traceable to a specific commit.

3. **npm ci everywhere** — CI pipelines use `npm ci` instead of `npm install` for reproducible, lockfile-driven installs.

4. **Caret dependencies** — inter-package dependencies use caret ranges (`^x.y.z`) so downstream consumers get compatible updates automatically.

## Why

A reliable package pipeline is infrastructure — without it, every consumer of OrqaStudio libraries is manually linking or copy-pasting. Published packages with deterministic versions let the app, plugins, and external consumers depend on stable, versioned artifacts.
