---
id: "EPIC-3f01a10f"
type: "epic"
title: "Canonical hook types in @orqastudio/types"
description: "Add CanonicalHookEvent, HookContext, HookResult, LoadedRule, RuleViolation to the centralised types library. All consumers import from here."
status: "captured"
priority: "P1"
relationships:
  - target: "EPIC-347a8c3d"
    type: "depends-on"
    rationale: "Types must align with what the Rust engine produces"
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---
# Canonical Hook Types

Add to `libs/types/src/`:
- `CanonicalHookEvent` type (PreAction, PostAction, PromptSubmit, etc.)
- `HookContext` interface (event, projectDir, toolName, toolInput, etc.)
- `HookResult` interface (action, messages, violations, knowledgeToInject)
- `LoadedRule` interface (parsed rule with enforcement entries)
- `RuleViolation` interface (ruleId, action, message)
- Ensure Rust types in libs/validation match these contracts