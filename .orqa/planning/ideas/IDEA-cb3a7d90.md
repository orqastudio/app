---
id: IDEA-cb3a7d90
type: planning-idea
title: "Evaluate FastFlowLM for local NPU inference"
description: "Research FastFlowLM as a candidate local inference backend for the sidecar architecture. The goal is to give OrqaStudio users an offline-capable LLM path that runs on commodity NPU hardware instead of requiring remote API calls or heavyweight GPU runtimes."
status: captured
priority: P3
created: 2026-04-11
updated: 2026-04-11
horizon: later
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Local inference is a core availability property — OrqaStudio should work offline and on air-gapped machines, not only when the API is reachable."
---

## Source

User research lead, captured 2026-04-11 in conversation: "look into https://fastflowlm.com/ for local NPU inference".

## What

[FastFlowLM](https://fastflowlm.com/) is a local LLM inference runtime targeted at NPUs (neural processing units) on commodity hardware — Apple Silicon, AMD Ryzen AI, Intel Core Ultra, and similar integrated accelerators. The lead is: evaluate it as a plug-in sidecar backend for OrqaStudio, alongside or instead of the existing Claude / cloud API paths.

## Why it's relevant

OrqaStudio's sidecar architecture is explicitly plugin-composable — each inference backend is a plugin that registers with the daemon and receives routed requests. Today the first-party sidecar is `claude-agent-sdk`, which requires an Anthropic API key and a network connection. A local NPU backend would:

- Unblock offline use for practitioners on planes, in secure environments, or on metered connections.
- Cut latency for embedding-style tasks where a small local model is enough.
- Give the "Bring Your Own Inference" story a concrete working example beyond Claude.
- Let the devtools demonstrate multi-sidecar routing (route embeddings locally, route agent work to Claude).

## What to investigate

- **Hardware coverage**: what NPUs does FastFlowLM actually accelerate? Apple M-series, Ryzen AI, Intel? Fallback to CPU when no NPU is present?
- **Model catalogue**: what models ship, and are they good enough for agent work? The baseline is "good enough for embeddings + a local small model for cheap tasks" — not "good enough to replace Claude Opus".
- **API surface**: OpenAI-compatible HTTP? Native SDK? An OpenAI-compatible endpoint drops straight into the sidecar plugin model with no custom glue.
- **License**: open source or proprietary? Redistribution model? Impact on the BSL-1.1 licensing of OrqaStudio.
- **Packaging**: how do users install it — single binary, Docker image, npm, platform package manager? Each delivery path has install-time constraints the sidecar plugin needs to handle.
- **Performance envelope**: tokens/s on common hardware (M2 Pro, Ryzen AI 9, Intel Core Ultra), VRAM vs NPU memory, cold-start time.
- **Comparison**: how does it compare to `ollama`, `llama.cpp`, `mlx-lm`, `lmdeploy`, `vLLM`? Each of these already has working integrations in the wider ecosystem — FastFlowLM has to earn its slot.

## Decision criteria

- Must be installable without root privileges.
- Must expose an HTTP API (ideally OpenAI-compatible) so the sidecar plugin is a thin wrapper, not a full SDK rewrite.
- Must degrade gracefully when no NPU is detected (CPU fallback acceptable; hard failure is not).
- Must not require CUDA or vendor-specific GPU runtimes to function at baseline.

## Relationship to existing work

- Slots into the sidecar plugin architecture captured in `project_sidecar_architecture.md` (user memory) and PD-09fc4e65.
- Would satisfy the "local inference" branch of the inference routing work.
- The existing ONNX path (see `project_onnx_model.md` user memory — BGE-small-en-v1.5) is the embeddings equivalent of this idea. FastFlowLM is the generative equivalent.

## Not in scope

- Replacing Claude as the primary agent backend. Claude remains the default; FastFlowLM is additive.
- Building a model store or a fine-tuning pipeline. If FastFlowLM ships models, we use them; we don't host them.
- Bundling FastFlowLM binaries with the OrqaStudio install. Users install it themselves; the plugin detects and connects.
