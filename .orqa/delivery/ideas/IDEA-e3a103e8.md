---
id: IDEA-e3a103e8
title: "Improve graph query performance for large artifact graphs"
type: idea
status: captured
description: "Graph queries (graph_query, graph_resolve, etc.) may become slow as the artifact count grows (currently 1422 nodes). Investigate caching, indexing, and query optimization strategies."
pillars:
  - PILLAR-569581e0
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: PILLAR-569581e0
    type: grounds
    rationale: "Faster graph queries directly improve the structured thinking workflow — every orchestrator action depends on responsive graph traversal"
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: "Alex (The Lead) depends on responsive graph queries for every orchestrator coordination action — slow queries degrade the structured thinking workflow"
---

## Context

The artifact graph is the backbone of OrqaStudio's structured thinking process. Every orchestrator action — task discovery, dependency checking, relationship traversal, integrity validation — issues graph queries. As the graph grows (currently 1422 nodes across rules, tasks, epics, ideas, decisions, lessons, knowledge, agents, and pillars), query latency becomes a bottleneck for the entire coordination workflow.

The current architecture is file-based: each query triggers a scan of markdown files on disk, parsing YAML frontmatter and filtering results. This approach is simple and correct, but does not scale gracefully.

## Research Questions

1. **Caching strategies** — Can parsed frontmatter be cached in memory after the first scan? What invalidation strategy works when files change on disk (file watcher, timestamp check, hash-based)?
2. **In-memory indexing** — Would building an in-memory index (by type, status, ID) at startup and maintaining it incrementally improve query throughput enough to justify the complexity?
3. **Query optimization** — Are there hot paths (e.g., `graph_query({ type: "task", status: "in-progress" })`) that account for most query volume? Could targeted indexes for common query patterns provide the best cost/benefit ratio?
4. **Architecture evaluation** — Does the current file-based scanning architecture need to be supplemented with a persistent index (e.g., SQLite, DuckDB), or is an in-memory cache sufficient for the foreseeable graph size (up to ~5000 nodes)?
5. **Benchmark baseline** — What are the current query latencies for common operations (full scan, type filter, ID resolve, relationship traversal) so that improvement can be measured?

## Expected Outcome

A research document with benchmark data, a recommended caching/indexing strategy, and a clear recommendation on whether the file-based scanning architecture is sufficient or needs supplementation. The outcome should inform whether this becomes an epic or whether simple caching is enough to address the concern.
