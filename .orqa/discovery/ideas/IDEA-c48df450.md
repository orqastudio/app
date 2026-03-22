---
id: IDEA-c48df450
title: Production ONNX model download — first-run installer for embedding models
description: The ONNX embedding model (BGE-small-en) is too large to bundle with the app. Production environments need a first-run download flow that fetches the model on install, with progress indication, retry logic, and offline fallback.
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-c48df450: Production Model Download

First-run installer that downloads the ONNX embedding model when the app is installed in production. The existing `ensure_model_exists` pipeline in `search/embedder.rs` already handles download with progress — this idea is about wrapping it in a user-facing setup flow with proper UX (progress bar, retry, offline mode).
