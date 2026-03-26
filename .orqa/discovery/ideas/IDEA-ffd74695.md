---
id: IDEA-ffd74695
type: discovery-idea
title: Wireframing plugin — native wireframe creation within OrqaStudio
description: A plugin that enables creating and editing wireframes natively in the app instead of maintaining static markdown wireframe documents.
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: later
research-needed: []
relationships:
  - target: PILLAR-2acd86c1
    type: grounded
  - target: PERSONA-2721ae35
    type: benefits
---

## Motivation

Wireframes in OrqaStudio are currently static markdown documents in `.orqa/documentation/wireframes/`. They are text descriptions of layouts, not interactive or visual. Maintaining these docs by hand is slow and they drift from reality quickly. A native wireframing plugin would allow wireframes to be created, edited, and linked directly to features and epics — keeping them alive as living artifacts rather than stale documents.

## Sketch

A plugin-provided artifact type (`WIRE-NNN`) with a canvas editor embedded in the artifact panel. Wireframes reference epics and features via the standard relationship graph. Could integrate with the Designer role to allow AI-assisted wireframe generation from feature descriptions.