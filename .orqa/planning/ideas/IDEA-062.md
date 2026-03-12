---
id: IDEA-062
title: "Project initialization — tooling detection and plugin suggestion"
description: "During project setup, detect existing tooling (automated scan + user interview), suggest verification plugins, recommend options if none exist. Mature plugin system could offer to generate plugins automatically."
status: captured
created: "2026-03-12"
updated: "2026-03-12"
pillars: [PILLAR-001]
research-needed:
  - "Extend project-inference skill to detect linters, monitoring tools, CI/CD"
  - "User interview flow design for metrics/monitoring tooling"
  - "Plugin marketplace/registry concept for matching detected tools to plugins"
  - "AI-assisted plugin generation feasibility — how much can be automated from the plugin template"
promoted-to: null
---

## Motivation

[AD-042](AD-042) defines three discovery paths during project initialization: automated detection of codebase tooling, user interview about metrics/monitoring tools, and recommendation of options based on project context. This turns OrqaStudio setup into a guided process that connects the project to the verification pipeline from day one.
