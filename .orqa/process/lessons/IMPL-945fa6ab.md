---
id: IMPL-945fa6ab
type: lesson
title: "Orchestrator bypassed OrqaStudio's enforcement system in favor of raw platform hooks"
description: "When adding enforcement for RULE-532100d9, the orchestrator created a raw Claude Code Stop hook in .claude/hooks/ instead of using the artifact graph enforcement system. This bypasses the very system we're building."
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: observation
relationships:
  - target: AD-f9034c99
    type: teaches
    rationale: "This lesson directly triggered the decision that rule promotion requires enforcement through the artifact graph"
  - target: EPIC-915291e7
    type: cautions
    rationale: "Auto-generated inverse of informs relationship from EPIC-915291e7"
---
## Pattern

When promoting IMPL-85add0f1 (permission-seeking) to RULE-532100d9, the orchestrator needed to add enforcement. Instead of working through the artifact graph's enforcement system (enforcement entries on rules, consumed by the plugin and app), it created a raw `.claude/hooks/block-permission-seeking.sh` — a platform-specific hook that bypasses the entire governance system.

This is the equivalent of hardcoding a database query instead of using the repository pattern. The enforcement system exists specifically so that enforcement is:
1. Declared in the artifact graph (traceable, auditable)
2. Consumed by the Rust application layer (app context)
3. Consumed by the Claude plugin (CLI context)
4. Version-controlled alongside the rules they enforce

A raw Claude hook is none of these. It's invisible to the artifact graph, doesn't work in the app, and isn't governed by the same lifecycle.

## Principle

All enforcement MUST flow through OrqaStudio's enforcement system. The artifact graph is the single source of truth for what is enforced, how, and where. Platform-specific hooks are implementation details consumed by the enforcement engine — never created directly.