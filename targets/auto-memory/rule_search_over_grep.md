---
name: Semantic search before grep
description: RULE-005 — use semantic search to find code by concept before falling back to file-level grep
type: feedback
---

Use semantic search to find code and artifacts by concept before falling back to exact-match grep. Semantic search finds what you mean; grep finds what you type.

**Why:** RULE-005. The search engine (ONNX embeddings + DuckDB) exists specifically to enable concept-level code discovery. Grep misses renamed functions, aliased imports, and semantically equivalent patterns.

**How to apply:** When looking for code or artifacts, start with `search_semantic` or `search_research`. Only fall back to `search_regex` when you need exact identifiers.
