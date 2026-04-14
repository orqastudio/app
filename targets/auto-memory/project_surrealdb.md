---
name: SurrealDB storage migration
description: Replacing markdown-based .orqa/ artifact system with SurrealDB as source of truth, three deployment tiers
type: project
---

Current focus: replacing the markdown-based artifact system with SurrealDB as the source of truth. The `.orqa/` directory currently holds 1,748 markdown files scanned into an in-memory HashMap — every file change triggers a full rebuild (~3.3s). SurrealDB eliminates this with native graph queries.

**Plan:** `.orqa/planning/PLAN-storage-migration.md`
**PoC crate:** `engine/graph-db/` — standalone SurrealDB proof-of-concept (in progress via Claude Code)
**Three tiers:** local (embedded SurrealKV + SQLite), self-hosted (SurrealDB server + Postgres), cloud-hosted
**Git becomes background infrastructure** — automatic commits for version history/audit, invisible to user
**P7 revised:** "Resolved Workflow Is a Record" — SurrealDB record, exportable as file via `orqa export`

**How to apply:** New storage-related work should align with this plan. Markdown files become the distribution/export format, not the source of truth.
