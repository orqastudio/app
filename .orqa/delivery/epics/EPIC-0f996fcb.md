---
id: EPIC-0f996fcb
type: epic
title: "Canonical hook types in @orqastudio/types"
description: "Add CanonicalHookEvent, HookContext, HookResult, LoadedRule, RuleViolation to the centralised types library. All consumers import from here."
status: captured
priority: P1
relationships:
  - target: EPIC-81c336c1
    type: depends-on
    rationale: "Types must align with what the Rust engine produces"
  - target: EPIC-92de4797
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from EPIC-92de4797"
  - target: MS-654badde
    type: fulfils
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