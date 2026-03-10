---
id: EPIC-015
title: CI/CD Pipeline & Distribution
description: Build the CI/CD pipeline with GitHub Actions, cross-platform builds, artifact signing, and auto-update for distribution.
status: draft
priority: P1
created: "2026-03-07"
updated: "2026-03-07"
milestone: MS-002
pillars:
  - PILLAR-001
depends-on: []
blocks: []
docs-required: []
docs-produced:
  - DOC-024
  - DOC-001
scoring:
  pillar: 2
  impact: 5
  dependency: 3
  effort: 4
  score: 6.3
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
