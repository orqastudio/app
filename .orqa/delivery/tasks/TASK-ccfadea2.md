---
id: TASK-ccfadea2
type: task
title: "Download ONNX embedding model for dev environment"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-1358323e
    type: delivers
---

# TASK-ccfadea2: ONNX Model Download

## Acceptance Criteria

1. BGE-small-en-v1.5 model downloaded to dev environment model directory
2. Model NOT bundled in the app binary (too large for distribution)
3. MCP server's semantic search initialises the model from the local path
4. Clear error message if model not found (with download instructions)
5. Production model download is a separate concern (IDEA to be logged)

## Notes

The app already has a model download pipeline in `search/embedder.rs` (`ensure_model_exists`).
This task just ensures it's available for the MCP server's use in the dev environment.
