---
id: IDEA-c3d5e7f9
title: "Research ort crate upgrade to 2.0.0-rc.12+"
type: idea
status: captured
description: "Investigate upgrading ort from 2.0.0-rc.11 to rc.12 (ONNX Runtime 1.24) for better DirectML support and Blackwell GPU compatibility"
pillars: []
created: 2026-03-23
updated: 2026-03-23
---

## Context

The search engine uses `ort 2.0.0-rc.11` with DirectML for hardware-accelerated embeddings. We encountered an `E_INVALIDARG` error on the NVIDIA RTX 5060 Ti (Blackwell architecture) — the `/embeddings/Add_1` node fails in DirectML's kernel. Currently working around this by targeting the AMD Radeon 890M iGPU via `device_id(1)`.

## Research Questions

1. Does `ort 2.0.0-rc.12` (ONNX Runtime 1.24) fix the Blackwell DirectML issue?
2. What breaking changes exist between rc.11 and rc.12?
3. Is there a stable 2.0.0 release timeline?
4. Would the upgrade allow us to use the discrete NVIDIA GPU for better performance?

## Expected Outcome

Decision on whether to upgrade ort, and if so, whether we can switch back to the default device (NVIDIA dGPU) for better inference performance.
