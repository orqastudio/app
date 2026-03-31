---
id: IDEA-d4b8e2a1
type: discovery-idea
title: "Email generation plugin using EmailMD"
description: "Explore EmailMD (markdown-to-email) as the basis for an OrqaStudio plugin that generates stakeholder emails from governance artifacts"
status: captured
priority: low
created: "2026-03-29"
tags:
  - plugin-idea
  - email
  - communication
  - stakeholder
relationships:
  - type: benefits
    target: PERSONA-c4afd86b
---

## Reference

https://www.emailmd.dev/

Markdown-to-email rendering tool.

## Plugin Concept

An OrqaStudio plugin that generates stakeholder communication emails from governance artifacts — milestone summaries, sprint reports, decision notifications, review requests. Uses markdown as the authoring format (consistent with .orqa/ artifacts) and EmailMD for rendering to rich email.

## Potential Features

- Generate milestone status emails from EPIC/TASK completion data
- Send decision review requests with context pulled from AD artifacts
- Sprint summary emails generated from workflow stage transitions
- Template-based email generation driven by plugin configuration (P1 compliant)
