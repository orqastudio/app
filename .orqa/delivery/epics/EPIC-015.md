---
id: EPIC-015
title: "CI/CD Pipeline & Distribution"
description: "Build the CI/CD pipeline with GitHub Actions, cross-platform builds, artifact signing, and auto-update for distribution."
status: captured
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: MS-002
    type: fulfils
    rationale: Epic belongs to this milestone
---
## Why P1

Can't ship to users without a build pipeline and update mechanism.

## Tasks

- [ ] GitHub Actions: PR checks (`make check` on all platforms)
- [ ] GitHub Actions: build artifacts on merge to main (pre-release)
- [ ] GitHub Actions: build release on tag push (stable)
- [ ] Platform matrix: Windows (x64), macOS (x64, arm64), Linux (x64)
- [ ] Artifact signing with Tauri updater keys
- [ ] Semantic versioning: tauri.conf.json + Cargo.toml + package.json sync
- [ ] Auto-update via `tauri-plugin-updater` with GitHub Releases
- [ ] Update channel selector in Settings (pre-release / stable)

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
