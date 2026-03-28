---
id: IDEA-2aa5581e
type: discovery-idea
title: Sidecar-specific subagent mapping on agents
description: "Agent subagent mappings are implementation-specific to the sidecar, not part of the universal agent model. The agent definition should declare capabilities and roles; how those map to subagent tool calls is a sidecar concern."
status: archived
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "How should sidecar-specific configuration be separated from the universal agent model?"
  - "Should subagent mappings live in a sidecar config file rather than agent frontmatter?"
  - "Does this overlap with the provider-agnostic capabilities model (RULE-8abcbfd5)?"
relationships:
  - target: EPIC-f7055984
    type: realises
    rationale: "Captured during EPIC-f7055984 UAT as Finding #18"
  - target: PILLAR-a6a4bbbb
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

Agent definitions currently include subagent mapping details that are specific to the Claude Code sidecar implementation. These mappings don't apply to other providers or contexts. The universal agent model should be provider-agnostic — capabilities and roles are universal, but how those translate to concrete subagent calls is a sidecar implementation detail.
