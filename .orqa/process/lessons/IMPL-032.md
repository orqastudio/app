---
id: IMPL-032
title: Plugin installation must wire capabilities into agents and skills
description: "When a provider integration plugin is installed, it's not enough to register capability mappings. The plugin must also install or update agents and skills that teach the system HOW to use those capabilities. The plugin installation process is responsible for wiring everything together — capabilities, agents that use them, and skills that describe their usage patterns."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships:
  - target: IMPL-031
    type: informed-by
    rationale: "Capability configuration defines WHAT is available — this observation addresses ensuring agents and skills know HOW to use what's available"
  - target: RULE-026
    type: observes
    rationale: "Skill enforcement requires agents to load skills — plugin installation must make provider-specific skills available for loading"
  - target: RULE-040
    type: observes
    rationale: "Capability resolution feeds into agent definitions — plugin installation must update agent capabilities"
---

## Pattern

Currently, when the Claude plugin is installed:
- Hook scripts are registered (rule-engine, prompt-injector, etc.)
- Skills are symlinked into `.orqa/process/skills/`
- But agent definitions are NOT updated with provider-specific capabilities
- And there's no skill that teaches "how to use Claude Code's Read tool effectively"

A provider integration plugin should be a complete package:
1. **Capabilities** — what tools are available and how to resolve them
2. **Skills** — how to use those tools effectively (patterns, anti-patterns, provider quirks)
3. **Agent updates** — which agents gain which capabilities when this plugin is active

The installation process should handle all three, not just register hooks and hope agents figure out the rest.

## Fix

Plugin installation process needs to:
1. Register capability mappings (default-capabilities)
2. Install provider-specific skills (e.g., "claude-code-patterns" skill)
3. Update or extend agent definitions with provider-specific capabilities
4. Verify that all required capabilities have both a mapping AND a skill that describes their usage
