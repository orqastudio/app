---
id: IDEA-f0caec95
title: Internal routing with back/forward/history navigation
description: Add proper internal routing to the app so users can navigate back/forward through their browsing history like a web browser. Enables natural navigation patterns and reduces cognitive load when exploring the artifact graph.
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: next
research-needed:
  - How to implement client-side routing in a Tauri app without a URL bar
  - History stack design — what constitutes a navigation entry
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

Currently navigating the app is one-directional — clicking an artifact link takes you there but there's no way to go back. This forces users to mentally track where they came from and manually navigate back. A browser-style history stack with back/forward would make exploration natural.
