---
id: TASK-246
title: "Add From<duckdb::Error> to OrqaError and fix search error propagation"
description: "Search errors use .map_err(|e| e.to_string()) which loses type info. Add proper From impl and propagate typed errors."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "OrqaError has a DuckDb variant with From<duckdb::Error>"
  - "All .map_err(|e| e.to_string()) in search/ are replaced with ? operator"
  - "Error messages in frontend still show meaningful text"
  - "make check passes"
---

## What

Search errors use .map_err(|e| e.to_string()) which loses type info. Add proper From impl and propagate typed errors.

## How

To be determined during implementation.

## Verification

- [ ] OrqaError has a DuckDb variant with From<duckdb::Error>
- [ ] All .map_err(|e| e.to_string()) in search/ are replaced with ? operator
- [ ] Error messages in frontend still show meaningful text
- [ ] make check passes
