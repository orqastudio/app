---
id: TASK-132
title: "Implement settings view and semantic code search"
description: "Built the settings view for provider configuration and model selection, and set up the ONNX embeddings engine with DuckDB vector search."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-030
depends-on: []
scope:
  - Implement settings view with provider config fields
  - Implement model selection dropdown
  - Set up ONNX embeddings engine for code search
  - Integrate DuckDB for vector similarity search
acceptance:
  - Settings view allows provider configuration and model selection
  - Code search produces relevant results via semantic similarity
  - Settings persist across app restarts
---
## What

Built the settings view for AI provider configuration and model selection, and integrated the ONNX embeddings engine with DuckDB for semantic code search.

## How

Implemented the settings UI with form fields backed by settings commands, and set up the native search engine in `src-tauri/src/search/` using the `ort` crate for ONNX inference and DuckDB for vector storage.

## Verification

Provider config and model selection persist across restarts, and semantic code search returns relevant results via vector similarity.
