---

id: IMPL-8d666f0c
type: lesson
title: User prompt input should infer intent and auto-record observations
description: "When a user submits a prompt, the system should infer intent — if the prompt contains an observation worth recording, the system should automatically capture it as an IMPL entry without requiring the user to explicitly say 'observation:' or ask for it to be recorded."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships: []
---
## Pattern

Users naturally embed observations, feedback, and process insights within their prompts. Currently the orchestrator only records these when the user explicitly prefixes with "observation:" or asks for it. The system should infer when a prompt contains a recordable observation and capture it automatically — the user's suggestion of a "prompt input hook" points to this being a pre-processing step on every user message.

## Fix

Implement intent inference on user prompt input — either as:
1. **A prompt-submit hook** in the plugin system that analyses user input before/after it reaches the agent
2. **Orchestrator behavior** where the orchestrator checks every user message for observation-like content and auto-records it
3. **App-level preprocessing** where the UI layer detects observation patterns and creates IMPL entries

The hook approach (option 1) is most composable — it runs on `user-prompt-submit` events and can classify intent without modifying the orchestrator or app code. The user specifically suggested "prompt input hook?" as the mechanism.
