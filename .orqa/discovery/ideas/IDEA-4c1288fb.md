---
id: IDEA-4c1288fb
type: discovery-idea
title: Local ONNX-based intent interpreter for artifact graph traversal pre-processing
description: Small local-run ONNX fine-tuned LLM that pre-processes user prompts before the orchestrator receives them. Interprets intent to help the orchestrator decide how to crawl the artifact graph. Does not crawl itself — acts as a lightweight intent classifier fine-tuned on the core artifacts. Optionally extends to include a local fine-tuning pipeline that ingests process artifacts from all locally installed projects.
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "What ONNX models are suitable for intent classification at this scale? (Phi-3-mini, TinyLlama, etc.)"
  - "What training data format would work? How do core artifacts map to training examples?"
  - "What is the inference latency budget? Must complete before the prompt reaches the orchestrator."
  - "How does this interact with the existing ONNX Runtime infrastructure in the search engine?"
  - "What does the fine-tuning pipeline look like? Hardware requirements, training time, data ingestion."
  - "How should the intent classifier output be structured? (graph traversal hints, artifact type weights, scope signals)"
  - "What is the fallback when the model is not available or not trained? (degrade gracefully to current prompt-injector)"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-2721ae35
    type: benefits
---

## Motivation

The current prompt-injector uses regex pattern matching to detect artifact references and observation intent. This works for explicit references (TASK-NNN, EPIC-NNN) but misses semantic intent. When a user says "I need to refactor the session handling", the injector doesn't know to surface sessions-related ADs, IPC patterns, or repository skills.

A small fine-tuned model sitting between the user and the orchestrator solves this:

1. **Intent classification**: "refactor session handling" → {area: persistence, artifacts: [AD-75bb14ae, AD-859ed163], skills: [orqa-repository-pattern, orqa-domain-services]}
2. **Graph traversal hints**: Instead of the orchestrator reading every artifact to find relevance, the intent model provides starting nodes for the graph walk
3. **Reduced orchestrator context load**: The model pre-selects what matters, keeping the orchestrator focused on coordination

## Sketch

**Architecture:**

```text
User prompt → [Intent Model (local ONNX)] → enriched prompt + graph hints → Orchestrator
```

The model is optional. When available:

- Runs locally via ONNX Runtime (already in the project for search embeddings)
- Adds structured metadata to the prompt: relevant artifact types, suggested skills, scope signals
- Does NOT replace the orchestrator's judgment — provides input to accelerate graph traversal

When not available:

- Falls back to current regex-based prompt injector
- No degradation in correctness, just slower graph discovery

**Fine-tuning pipeline (extended):**

- **Training data**: Core artifacts (rules, skills, agent definitions) mapped to intent categories
- **Per-project extension**: When the pipeline is active, it ingests all `.orqa/` process artifacts from every locally installed project, building a project-aware intent classifier
- **Auto-retraining**: When process artifacts change significantly (new rules, restructured skills), the pipeline detects staleness and suggests retraining
- **Hardware gating**: Only runs on machines with sufficient compute. Detects GPU availability and storage. Falls back to pre-trained base model on resource-constrained machines.

**Intent model output format:**

```json
{
  "intent": "refactor",
  "domain": ["persistence", "domain-services"],
  "suggested_artifacts": ["AD-75bb14ae", "AD-859ed163"],
  "suggested_skills": ["orqa-repository-pattern", "orqa-domain-services"],
  "scope_signal": "backend-only",
  "confidence": 0.87
}
```

The orchestrator treats this as advisory — high-confidence suggestions are loaded automatically, low-confidence ones are logged but not acted on.

## Open Questions

- **Should this be a plugin?** The intent model could be packaged as an OrqaStudio plugin rather than a core feature. This would make it opt-in, keep the core lightweight, and allow different intent models to compete (official vs community). The plugin would hook into `UserPromptSubmit` and inject graph traversal hints — same mechanism as the current prompt-injector but with ML-based intent classification instead of regex.
