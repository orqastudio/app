# Phase S2 — DB-as-Source-of-Truth: Executable Task Plan

**Source of truth (epic):** `.orqa/implementation/epics/EPIC-358d42a4.md` Phase S2 (lines 885-915).
**Planning session date:** 2026-04-15.
**Execution target:** Claude Code (later session). This session produces the plan only.

## Source-of-Truth Convention and Install Targets (IMPORTANT)

**Monorepo source of truth:**

- **`plugins/`** — the source of truth for plugin-provided material (both artifact content AND runtime code). Authored in-repo, versioned in git.

**Install targets (what `orqa install` produces):**

- **Artifact content** from plugins → **SurrealDB** (`source_plugin` set). No files under `.orqa/` for artifacts.
- **Runtime code** from plugins (scripts, generators, hook executables, templates, binary assets) → **`.orqa/plugins/<name>/`**. Files live on disk because they must be executed or read by subprocesses.
- **Project config** → **`.orqa/project.json`**, **`.orqa/schema.composed.json`**, **`.orqa/configs/`**.
- **User/project-authored artifacts** (decisions, lessons, project-scoped edits made at runtime) → **SurrealDB** with `source_plugin = null`.

**Post-S2 `.orqa/` canonical layout:**

```
.orqa/
├── project.json
├── schema.composed.json
├── configs/
└── plugins/
    └── <plugin-name>/    ← runtime code only, not artifact content
```

**Implications for S2:**

- Plugin-sourced artifacts in the old `.orqa/` tree are **derived** — re-ingest from `plugins/` → SurrealDB on install. They do NOT need migration; TASK-S2-09 skips them.
- Runtime code from plugins must be **copied** (not linked — plugins must work in shipped installations) from monorepo `plugins/<name>/` into `.orqa/plugins/<name>/` at install time.
- The file watcher on **monorepo `plugins/`** STAYS post-S2: source edits trigger SurrealDB re-ingest of artifact content AND refresh of runtime code under `.orqa/plugins/<name>/` (dogfood dev loop).
- The watcher on **`.orqa/` artifact dirs** GOES AWAY. The watcher does NOT watch `.orqa/plugins/` (that's install output; source edits go through the monorepo `plugins/` branch).
- `.state/archive/orqa-files/` archives **user/project-authored artifacts only** from the old `.orqa/` layout. Plugin-derived artifact files don't need archiving.

**Classification for migration (TASK-S2-09):** each existing `.orqa/**/*.md` file is classified as user-authored, plugin-derived (artifact), or runtime-code. Plan defers to Bobbi on ambiguous cases — see Open Questions Q2 and Q10.

## Context Summary

### What S1 delivered (do NOT rebuild)

- SurrealDB embedded at `.state/surreal/` with schema (artifact, relates_to, enforcement_rule tables).
- `engine/graph/src/sync.rs` — `bulk_sync`, `sync_file`, `delete_artifact_by_path`, `check_file_hash` (SHA-256 content hashing). Reusable for import/migrate ingestion.
- `daemon/src/surreal_queries.rs` — `list_artifacts`, `search_artifacts`, `trace_descendants`, `find_siblings`, `find_orphans`, totals/health aggregates.
- `GET /artifacts` already uses SurrealDB fast-path with HashMap fallback (the S2 write-path template).
- HTTP routes: `/graph/parity`, `/graph/trace/{id}`, `/graph/siblings/{id}`, `/graph/orphans`.
- Warm-start skip on daemon restart; per-file sync from watcher.

### Known S1-carried blocker (does not block S2)

- SurrealDB 3.x embedded does not support depth-range traversal (`.{1..N}`). Only affects `/graph/health` metrics. **Out of scope for S2.**

### Code topology relevant to S2

| Area | Today | Path |
|------|-------|------|
| `orqa` CLI | TypeScript (Node), HTTP to daemon | `cli/src/commands/*.ts` |
| Artifact PUT handler | Writes markdown, reloads graph | `daemon/src/routes/artifacts.rs:302-364` |
| File write for frontmatter edits | Line-by-line rewrite | `engine/validation/src/auto_fix.rs:123-164` |
| Artifact graph (HashMap) | Authoritative today | `engine/types/src/types/graph.rs:26-32` |
| Plugin install (CLI side) | Copies md into `.orqa/` | `cli/src/lib/content-lifecycle.ts:123-193` |
| Plugin install (engine side) | Copies tree to `plugins/` | `engine/plugin/src/installer.rs:53-129` |
| File watcher | Watches `.orqa/` AND `plugins/` | `daemon/src/watcher.rs:45,232-240` |
| SSE push | `event_bus` broadcast, 10k buffer | `daemon/src/event_bus.rs` |
| `orqa verify` | Runs enforce/version/license/readme | `cli/src/commands/verify.ts:14-36` |
| `orqa migrate` | Status migrations (NOT storage) | `cli/src/commands/migrate.ts:284-369` |

---

## Atomic Tasks

Each task is sized for one implementer context window. Format: **ID / What / Files / AC / Reviewer checks / Deps**.

### TASK-S2-01 — Probe SurrealDB LIVE SELECT in embedded mode

**What:** Before committing to scope item 8, verify that `LIVE SELECT` works against the embedded SurrealDB 3.x build. The depth-range bug proves embedded-mode features can be incomplete.

**Files to READ:** `engine/graph-db/src/lib.rs`, `daemon/src/graph_state.rs`, `surrealdb` crate version in `Cargo.toml`.
**Files to MODIFY:** new `engine/graph-db/examples/live_probe.rs` (standalone binary).

**AC:**
- [ ] Example binary subscribes to `LIVE SELECT * FROM artifact` against `.state/surreal/`.
- [ ] Binary triggers a CREATE and a DELETE; both notifications arrive.
- [ ] Result documented in a new file `.state/investigation/S2-live-select-probe.md` with verdict PASS/FAIL and observed latency.

**Reviewer checks:**
- Run the example against a scratch DB; confirm both events printed.
- If FAIL: verify the report lists SurrealDB version, error text, and whether a server-mode workaround is feasible.

**Deps:** none.

---

### TASK-S2-02 — CLI `orqa export` — **DEFERRED to Phase S3** (per Bobbi, 2026-04-15)

**Status:** NOT in S2 scope. Export is tied to automatic git history (Phase S3). The S2 plan proceeds without an export escape hatch for cutover; rollback and verify (TASK-S2-10, -11) are the safety net instead.

**Impact on other tasks:**
- TASK-S2-03 (`orqa import`) no longer has an export-based round-trip test; use a hand-authored fixture corpus for its idempotency test.
- TASK-S2-17 (archive cleanup) no longer gates on "export has been run since cutover." Replace that pre-condition with a simpler gate: archive cleanup only runs after the user explicitly confirms, and only deletes archives older than N days (N is open question Q5).

**Removed from sequencing:** no change to wave structure beyond dropping this task.

---

### TASK-S2-03 — CLI `orqa import` (markdown tree → SurrealDB, user-selectable conflict policy)

**What:** Implement `orqa import --path <dir>`. Per Bobbi (2026-04-15), conflict behaviour is user-selectable between **upsert-and-bump** and **three-way merge**. The CLI exposes `--on-conflict=upsert|merge` (default: `upsert`). Default can also be set in `project.json` under an `import.onConflict` key.

**Files to READ:** `engine/graph/src/sync.rs:bulk_sync`, `sync_file`; any existing diff/merge helpers in `engine/` (e.g. content-lifecycle's three-way diff that was removed in TASK-S2-07 — salvage the algorithm if still in git history).
**Files to MODIFY:** new `cli/src/commands/import.ts`; new daemon route `POST /artifacts/import` (accepts path + conflict policy, invokes engine ingest against target dir, streams progress); new `engine/graph/src/merge.rs` if three-way merge requires new code.

**AC:**
- [ ] `--on-conflict=upsert`: if a record exists with different content, overwrite and bump `version`, write new `content_hash`. No data loss for the incoming file; existing record's prior version is overwritten.
- [ ] `--on-conflict=merge`: three-way merge between (base), (ours = current SurrealDB state), (theirs = incoming file). Base resolution (resolved Q2-followup, 2026-04-15):
      - If the record has a `source_plugin` set, use the plugin manifest ledger content-hash as base.
      - Else if the import payload includes a `base_snapshot` entry for this artifact (forward-compat hook for S3 `orqa export`), use that.
      - Else: the record has no known base. DO NOT silently downgrade — instead, collect these records up front and surface them to the user BEFORE any merge runs, with bulk-control options: `--no-base-action=take-theirs | keep-ours | review-each | fail` (default `review-each` in interactive mode, `fail` in non-interactive mode).
- [ ] On conflict that cannot auto-merge, write both versions to a conflict file in `.state/import-conflicts/<migration_id>/<artifact_id>.conflict.md` and FAIL the import with that record flagged; do not commit partial merges.
- [ ] Import payload parser accepts-and-ignores a `base_snapshot` field (future-proof for S3 export format); warns if present but unused (i.e. no matching records).
- [ ] Default read from `project.json` `import.onConflict` if present, else `upsert`; CLI flag overrides config.
- [ ] Re-running import on the same directory is a no-op under both policies (content hash unchanged → skip).
- [ ] Reports per-file status: CREATED / UPDATED / SKIPPED / MERGED / CONFLICT with reason.
- [ ] Integration test: seed SurrealDB from fixture A, import fixture B (diverges on 3 records) under both policies — verify upsert overwrites all 3, merge auto-merges non-conflicting fields and flags the true conflicts.

**Reviewer checks:**
- Run both policies against a fixture with hand-engineered conflicts; confirm upsert loses the in-DB change on conflicted records (documented behaviour) and merge preserves both side changes where possible.
- Verify `version` increments on UPDATE and MERGED outcomes; unchanged on SKIPPED.
- Confirm no partial writes on CONFLICT — the transaction is all-or-nothing per artifact.

**Q2-followup resolved (Bobbi, 2026-04-15):** No-base records are surfaced to the user up front (count + sample list); user picks a bulk policy (`take-theirs | keep-ours | review-each | fail`) before any merge commits. Interactive default: `review-each`. Non-interactive default: `fail`. Forward-compat: import parser accepts-and-ignores a `base_snapshot` field so future `orqa export` output works here without a format change.

**Deps:** none (fixture-based tests; no longer depends on TASK-S2-02).

---

### TASK-S2-04 — Flip `PUT /artifacts/:id` to SurrealDB-first write

**What:** Change `update_artifact` so SurrealDB is the authoritative write target. File write to `.orqa/` is removed; the HashMap is updated from the SurrealDB record (not from disk).

**Files to READ:** `daemon/src/routes/artifacts.rs:302-364`, `engine/validation/src/auto_fix.rs:123-164`, `engine/graph/src/sync.rs`.
**Files to MODIFY:** `daemon/src/routes/artifacts.rs` (PUT handler); `engine/graph/src/sync.rs` (expose a `write_artifact_record` helper if not already present); the HashMap update path in `daemon/src/graph_state.rs`.

**AC:**
- [ ] PUT handler writes to SurrealDB first, returns 503 if SurrealDB unavailable (no silent fallback to file write).
- [ ] HashMap entry refreshed from the SurrealDB record post-write (not from disk).
- [ ] `version` and `updated_at` fields increment on every PUT.
- [ ] `content_hash` recomputed and stored.
- [ ] File-write path via `auto_fix.rs:update_artifact_field` is removed from the PUT call site. (The `auto_fix` helper itself stays for now if other callers exist; mark it for deletion as a follow-up.)
- [ ] Existing PUT integration tests pass without file-watcher-induced reloads.

**Reviewer checks:**
- Grep for `update_artifact_field` callers — confirm PUT no longer calls it.
- Run PUT with SurrealDB disabled: expect 503, not silent success.
- Confirm no `std::fs::write` under `.orqa/` remains in the PUT path.

**Deps:** none (S1 provides SurrealDB).

---

### TASK-S2-05 — Implement `POST /artifacts` (SurrealDB write)

**What:** Currently deferred per `daemon/src/routes/artifacts.rs:5`. Implement create-artifact that inserts into SurrealDB, writes relationship edges, updates HashMap from the new record.

**Files to READ:** TASK-S2-04 output; `engine/graph/src/sync.rs` insert patterns.
**Files to MODIFY:** `daemon/src/routes/artifacts.rs`; route registration in `lib.rs` and `health.rs`.

**AC:**
- [ ] Request body validated against the artifact JSON schema (reuse composed schema loader).
- [ ] Duplicate ID returns 409.
- [ ] Created artifact visible in `GET /artifacts` immediately (read-your-writes).
- [ ] Relationship edges from frontmatter `relationships:` block are created as `relates_to` edges.
- [ ] Integration test: POST → GET by ID → GET /graph/trace/{id} succeeds.

**Reviewer checks:**
- Verify edge creation by querying SurrealDB directly after POST.
- Confirm request validation rejects missing required fields.

**Deps:** TASK-S2-04 (establishes the SurrealDB write helper).

---

### TASK-S2-06 — Implement `DELETE /artifacts/:id` (SurrealDB + edge cleanup)

**What:** Implement DELETE. Must cascade-delete `relates_to` edges where the artifact is source or target, then remove the record, then refresh HashMap.

**Files to MODIFY:** `daemon/src/routes/artifacts.rs`; route registration.

**AC:**
- [ ] Returns 404 if artifact does not exist.
- [ ] All `relates_to` edges involving the ID are deleted (both directions).
- [ ] HashMap entry removed.
- [ ] Deletion event published on `event_bus` (prepares for TASK-S2-12).
- [ ] `/graph/orphans` does NOT report newly-orphaned siblings as inconsistent (edges are cleaned).

**Reviewer checks:**
- Create artifact A with edges to B, DELETE A, query SurrealDB for any remaining edge with A as source or target — must be zero.
- Confirm 404 path covered by test.

**Deps:** TASK-S2-05 (serialises on `artifacts.rs`).

---

### TASK-S2-07 — CLI install: stop copying plugin markdown into `.orqa/`

**What:** Remove the TS-side `copyPluginContent()` writes into `.orqa/` subdirs. Plugins still get their source tree copied to `plugins/<name>/` (engine side); that stays in TASK-S2-08.

**Files to READ:** `cli/src/lib/content-lifecycle.ts:123-193`, `cli/src/commands/install.ts:569`.
**Files to MODIFY:** `cli/src/lib/content-lifecycle.ts` (remove `.orqa/` target writes but keep the manifest parse — it feeds TASK-S2-08); update callers in `cli/src/commands/install.ts`.

**AC:**
- [ ] Installing a plugin does NOT create any new files under `.orqa/`.
- [ ] Three-way-diff logic for user edits is deleted (no longer needed — SurrealDB holds truth).
- [ ] Existing install tests updated; removed tests noted in the task findings.

**Reviewer checks:**
- Run `orqa plugin install` against a fixture plugin on a clean checkout; assert no new `.orqa/**` files.
- Confirm `plugins/<name>/` still receives the source copy (not in scope of this task but must not regress).

**Deps:** none.

---

### TASK-S2-08 — Engine install: split plugin source into SurrealDB (artifacts) + `.orqa/plugins/` (runtime code)

**What:** `plugins/` (monorepo) is the source of truth. On install, the engine splits each plugin's material into two install targets: artifact content → SurrealDB, runtime code → `.orqa/plugins/<name>/`. The watcher on monorepo `plugins/` re-runs both paths on source edits (dogfood dev loop).

**Manifest classification (resolved Q11/Q-A, 2026-04-15):** each `content` entry in `orqa-plugin.json` gets an explicit `target: "surrealdb" | "runtime"` field. Installer default: omitted + `.md` path → `surrealdb` (lint warn); omitted + non-`.md` path → `runtime` (lint warn). Schema validates `target` is one of the two literals. A one-shot migration script walks all 16 existing manifests, classifies each entry by current install destination (`.orqa/<artifact-dir>/*` → `surrealdb`; everywhere else → `runtime`), writes `target` back, and commits.

**Files to READ:** `engine/plugin/src/installer.rs:53-129`, `cli/src/lib/content-lifecycle.ts` (for plugin-manifest → source-path mapping logic — keep the parser, redirect targets), `daemon/src/watcher.rs` plugin branch, sample `plugins/**/orqa-plugin.json` manifests.
**Files to MODIFY:** `engine/plugin/src/installer.rs` (two install sinks: SurrealDB ingest for artifact entries, copy-to-`.orqa/plugins/<name>/` for runtime entries); `daemon/src/routes/plugins.rs:208-250`; possibly new `engine/plugin/src/ingest.rs`; watcher `plugins/` handler routes edits through the same splitter. Update the plugin manifest JSON schema if new `target` field is introduced.

**AC:**
- [ ] After `orqa plugin install <name>`, every manifest entry classified as artifact is present in SurrealDB with `source_plugin = <name>` — read directly from `plugins/<name>/**`.
- [ ] Every manifest entry classified as runtime is present under `.orqa/plugins/<name>/` with byte-identical content to the source.
- [ ] Editing a file under monorepo `plugins/<name>/` triggers re-ingest (artifact entries) AND re-copy (runtime entries) via the watcher.
- [ ] Re-installing the same plugin is idempotent (content-hash skip on both sinks).
- [ ] Uninstalling a plugin removes SurrealDB records where `source_plugin = <name>` AND removes `.orqa/plugins/<name>/`. Uninstall is a two-step handshake: engine reports what *would* be removed (including any user-edited artifacts flagged), then frontend confirms via UI (TASK-S2-18), then engine executes the destructive step. CLI `orqa plugin uninstall <name>` takes `--force` for headless/scripted runs that bypass the UI.
- [ ] Enforcement rules from the plugin are inserted into `enforcement_rule` table.
- [ ] No new files created under `.orqa/<artifact-dir>/` (discovery, implementation, learning, planning, workflows, documentation) by any install or watcher path.
- [ ] Only files under `.orqa/plugins/<name>/` are created for plugin runtime code.
- [ ] `orqa-plugin.json` schema requires `target: "surrealdb" | "runtime"` per `content` entry; omitted values default per the lint-warn rule above.
- [ ] Manifest migration script ported all 16 existing manifests; `git diff` shows only `target` additions, no other changes.
- [ ] Lint runs on install: warnings emitted for any entry relying on the default.
- [ ] Integration test: manifest with malformed `target` value is rejected at install time with a clear error.

**Reviewer checks:**
- Install a fixture plugin declaring one artifact file (`target: surrealdb`) and one runtime file (`target: runtime`). Verify: SurrealDB has one record with `source_plugin = 'fixture'`; `.orqa/plugins/fixture/<runtime-file>` exists; no other `.orqa/` files touched.
- Edit both files under `plugins/fixture/`, wait for watcher debounce, confirm SurrealDB `content_hash` updated AND `.orqa/plugins/fixture/` file re-copied.
- Uninstall, verify both install targets cleared.
- Grep the install code path for any write under `.orqa/` outside `.orqa/plugins/` — must be zero.
- Inspect every `plugins/**/orqa-plugin.json` post-migration — every `content` entry has an explicit `target`.

**Deps:** TASK-S2-07 (must run after TS stops writing into `.orqa/` artifact dirs).

---

### TASK-S2-09 — CLI `orqa migrate storage` — ingestion phase (Phase 1 of Section 7.2)

**What:** New subcommand `orqa migrate storage`. First stage only: scan existing `.orqa/` markdown files, classify each as **user/project-authored** or **plugin-derived** (via frontmatter `source_plugin` or by matching against installed plugin manifests), insert ONLY user/project-authored records into SurrealDB. Plugin-derived artifacts are left to TASK-S2-08's install path to re-ingest from `plugins/`. No git init, no archive, no cutover yet.

**Files to READ:** EPIC-358d42a4.md Section 7.2; `engine/graph/src/sync.rs:bulk_sync`; `plugins/**/orqa-plugin.json` for manifest-based classification.
**Files to MODIFY:** new `cli/src/commands/migrate-storage.ts`; new daemon route `POST /admin/migrate/storage/ingest`; extend `bulk_sync` with a classification filter OR wrap it.

**AC:**
- [ ] Command scans `.orqa/` recursively; for each markdown file reports: source classification (user/plugin/unknown), action (INSERT/SKIP/FLAG).
- [ ] User/project-authored artifacts are inserted into SurrealDB with `source_plugin = null`.
- [ ] Plugin-derived artifacts are SKIPPED (not inserted) with a note that re-ingest will happen on next `orqa install`.
- [ ] Unknown-classification artifacts are FLAGGED in the report and NOT inserted until Bobbi resolves them.
- [ ] Idempotent — re-running against a populated SurrealDB performs zero writes.
- [ ] On completion, daemon log records a migration_id, start/end timestamps, counts per classification.
- [ ] Writes a report to `.state/migrations/<migration_id>.json` with per-file outcome and classification.

**Reviewer checks:**
- Run on a fresh checkout, confirm SurrealDB count == user-authored file count (NOT total file count).
- Confirm flagged/unknown files list is surfaced for Bobbi before any cutover.
- Re-run, confirm zero new writes.

**Deps:** none (reuses S1 `bulk_sync`; classification logic added here).

---

### TASK-S2-10 — CLI `orqa migrate storage --verify` (Phase 4 of Section 7.2)

**What:** Verification step — compare health metrics and sample queries between the SurrealDB state and a snapshot of the pre-migration HashMap. Exit non-zero on any delta.

**Files to MODIFY:** `cli/src/commands/migrate-storage.ts` (add `--verify` subflag); new daemon route `GET /admin/migrate/storage/verify`; extend `surreal_queries.rs` if needed for sample counts.

**AC:**
- [ ] Verify compares: total artifact count, count per artifact_type, count per status, orphan count, edge count.
- [ ] Reports each delta with expected/actual; exit 0 only if all zero.
- [ ] Runs a sample of 20 random traceability queries (artifact → pillar) and compares results.

**Reviewer checks:**
- Force a delta (delete one SurrealDB record), re-run verify — must exit non-zero with that record flagged.

**Deps:** TASK-S2-09.

---

### TASK-S2-11 — CLI `orqa migrate storage --rollback` (Section 7.3)

**What:** Rollback restores pre-migration `.orqa/` files from `.state/archive/orqa-files/` and reverts `project.json` storage block.

**Files to MODIFY:** `cli/src/commands/migrate-storage.ts` (`--rollback`); a helper to safely restore the archive.

**AC:**
- [ ] Only runs if `.state/archive/orqa-files/<migration_id>/` exists.
- [ ] Restores every file; reports restored count.
- [ ] Wipes SurrealDB artifact + relates_to tables (keeps schema) so next `orqa migrate storage` starts clean.
- [ ] Refuses to run if cutover has not yet happened (no archive to restore).

**Reviewer checks:**
- Full cycle: migrate ingest → cutover → rollback → confirm files back and SurrealDB empty of artifact records.

**Deps:** TASK-S2-14 (cutover — rollback inverts it).

---

### TASK-S2-12 — Publish artifact change events to `event_bus` (LIVE SELECT with poll-and-diff fallback)

**What:** Publish typed artifact change events to the existing `event_bus` so the SSE layer can push them to the frontend. Implementation path is decided by the TASK-S2-01 probe outcome:

- **If probe PASS:** subscribe to `LIVE SELECT * FROM artifact` inside the daemon; each notification becomes a typed event.
- **If probe FAIL:** **poll-and-diff fallback** (resolved by Bobbi Q4, 2026-04-15) — poll `SELECT id, content_hash, updated_at FROM artifact` every 500ms, compare against a previous snapshot held in a daemon-owned cache, emit CREATE/UPDATE/DELETE events for the deltas.

**Files to READ:** `daemon/src/event_bus.rs`, `daemon/src/main.rs:310`, SurrealDB live-query API (PASS path), TASK-S2-01 probe findings.
**Files to MODIFY:** new `daemon/src/surreal_live.rs` (PASS path) OR new `daemon/src/surreal_poller.rs` (FAIL path); register in `main.rs`; extend event type enum if needed.

**AC (shared, both paths):**
- [ ] Subscription or poller starts on daemon boot alongside SurrealDB init; reconnect or restart on error.
- [ ] CREATE / UPDATE / DELETE notifications each produce a distinct event type on `event_bus`.
- [ ] Backpressure: if `event_bus` buffer full, oldest event drops (existing 10k-buffer semantics).

**AC — poll-and-diff fallback only (if probe FAIL):**
- [ ] Poll interval configurable via `project.json` `live_updates.poll_interval_ms`, default 500.
- [ ] Every log line from the poller prefixed with `[surreal-poll-fallback]` so operators can grep.
- [ ] Module-level doc comment in `surreal_poller.rs` states: "TEMPORARY WORKAROUND for SurrealDB embedded LIVE SELECT. Remove when TASK-S2-19 is resolved."
- [ ] Diff cache size bounded (LRU or full-snapshot depending on artifact count); documented memory cost.
- [ ] `GET /admin/live-updates/status` returns `{ mode: "live" | "poll-fallback", last_tick_ms, events_emitted_since_boot }` for observability.

**Reviewer checks:**
- POST /artifacts → observe event on event_bus within 200ms (PASS path) or 600ms (FAIL path).
- Kill SurrealDB mid-run, confirm reconnect or clean failure log.
- If FAIL path: grep daemon logs for `[surreal-poll-fallback]` prefix, confirm it fires.
- If FAIL path: verify TASK-S2-19 tracking artifact exists and is linked from code comments.

**Deps:** TASK-S2-01 (outcome decides implementation path); TASK-S2-04/05/06 (writes exist to trigger events).

---

### TASK-S2-13 — Frontend subscription to live artifact events

**What:** Frontend (Svelte/Tauri) subscribes to the SSE stream for artifact events and updates its artifact store in place.

**Files to READ:** `daemon/src/routes/streaming.rs` (existing SSE); the frontend artifact store module (find under `app/` or `src/`).
**Files to MODIFY:** frontend store; any polling code that must be retired.

**AC:**
- [ ] Artifact list UI updates without refresh when a POST/PUT/DELETE hits the daemon.
- [ ] Polling of `GET /artifacts` for live refresh is removed (initial load only).
- [ ] Reconnect-on-SSE-drop logic implemented with exponential backoff.

**Reviewer checks:**
- Run two app instances against the same daemon, edit in one, confirm update in the other.
- Disconnect network briefly, confirm reconnect.

**Deps:** TASK-S2-12.

---

### TASK-S2-14 — CLI `orqa migrate storage --cutover` (Phase 5 of Section 7.2)

**What:** The one-way cutover. Archives current `.orqa/` artifact files to `.state/archive/orqa-files/<migration_id>/`, then restructures `.orqa/` to config-only (`project.json`, `schema.composed.json`, `configs/`, `plugins-installed.json`). Updates `project.json` storage block.

**Files to MODIFY:** `cli/src/commands/migrate-storage.ts` (`--cutover`); ensures `.state/archive/` dir exists; writes a cutover receipt.

**AC:**
- [ ] Refuses to run until `--verify` has returned clean in the current migration_id.
- [ ] Moves (not copies) `.orqa/discovery`, `documentation`, `implementation`, `learning`, `planning`, `workflows` into `.state/archive/orqa-files/<migration_id>/`.
- [ ] `.orqa/` post-cutover contains only: `project.json`, `schema.composed.json`, `configs/`, and `plugins/` (runtime-code install target from TASK-S2-08).
- [ ] Writes `.state/archive/orqa-files/<migration_id>/manifest.json` listing every moved file.
- [ ] `project.json` gains `storage: { backend: "surrealdb", migration_id: "..." }`.
- [ ] `.orqa/prompt-registry.json` is NOT archived/moved by cutover — it is live runtime output of the knowledge-injection pipeline and belongs to TASK-S2-24 (deferred). Cutover must leave it in place.

**Reviewer checks:**
- Run on a copy of the real repo `.orqa/`; verify move (not copy) by checking source dirs are gone.
- Confirm the three canonical entries are the ONLY survivors.
- Confirm `.orqa/prompt-registry.json` is still present post-cutover.

**Deps:** TASK-S2-09, TASK-S2-10, TASK-S2-04, TASK-S2-05, TASK-S2-06, TASK-S2-08 (all writes must flow through SurrealDB before archive).

---

### TASK-S2-15 — Remove `.orqa/` branch from file watcher (keep `plugins/` branch)

**What:** After cutover, `.orqa/` no longer holds artifacts, so watching it is waste and risks phantom reloads. Delete the `.orqa/` branch of the watcher; keep the plugin source branch untouched.

**Files to MODIFY:** `daemon/src/watcher.rs:45,388-396,416` (remove `.orqa/` registration + handler branch).

**AC:**
- [ ] Watcher no longer registers `.orqa/` as a watched directory.
- [ ] Plugin source directories still watched; generator subprocess trigger still functions.
- [ ] Graph reload is no longer triggered by `.orqa/` edits (should be impossible post-cutover anyway — verify).
- [ ] Integration test: touch a file under `.state/archive/orqa-files/` — no watcher event, no reload.

**Reviewer checks:**
- Grep for `.orqa` in `watcher.rs` — only legitimate references (e.g. comments describing what was removed) remain.
- Plugin watch path test still passes.

**Deps:** TASK-S2-14.

---

### TASK-S2-16 — Flip `orqa verify` to check SurrealDB consistency

**What:** Replace the enforce-via-files path with a SurrealDB consistency check: graph integrity, orphan count within tolerance, enforcement rules present, no duplicate IDs.

**Files to READ:** `cli/src/commands/verify.ts:14-36`.
**Files to MODIFY:** `cli/src/commands/verify.ts`; new daemon route `GET /admin/verify/storage` if helpers not already sufficient.

**AC:**
- [ ] `orqa verify` returns 0 on a healthy SurrealDB, non-zero on any detected inconsistency.
- [ ] No file comparison logic remains.
- [ ] Output lists: total artifacts, edges, orphans, per-plugin counts.

**Reviewer checks:**
- Inject a dangling edge (source artifact deleted, edge kept) — verify flags it.
- Inject a duplicate ID — verify flags it.

**Deps:** TASK-S2-14 (verify against the post-cutover state).

---

### TASK-S2-17 — `.state/archive/` retention + cleanup command

**What:** Implement `orqa migrate storage --cleanup-archive <migration_id>` to delete the archived `.orqa/` files after user confirms. Since `orqa export` is deferred to S3 (per Bobbi Q1), the gate is age-based and user-confirm, not export-based.

**Files to MODIFY:** `cli/src/commands/migrate-storage.ts` (`--cleanup-archive`).

**AC:**
- [ ] Requires explicit `--confirm` flag.
- [ ] Dry-run by default; prints a full summary of what will be deleted.
- [ ] Refuses to delete archives younger than 90 days (configurable via `--min-age-days`, default: 90; resolved by Bobbi 2026-04-15 on Q5).
- [ ] Records cleanup in `.state/migrations/<migration_id>.json` as `archive_cleaned_at`.

**Reviewer checks:**
- Run without `--confirm` → dry-run only, no filesystem changes.
- Attempt cleanup on a recent archive → refused with age-gate message.
- Attempt with `--min-age-days 0 --confirm` → proceeds.

**Deps:** TASK-S2-14 (no longer depends on TASK-S2-02 since export is deferred to S3).

---

### TASK-S2-19 — Tracking artifact for SurrealDB LIVE SELECT embedded gap

**What:** File a tracked issue artifact documenting the SurrealDB embedded LIVE SELECT limitation (if TASK-S2-01 probe FAILED) and the poll-and-diff workaround introduced in TASK-S2-12. The artifact lives in the project graph so it surfaces in orphan checks, health reports, and review rituals until the workaround is retired.

**Only runs if TASK-S2-01 probe FAILED** — skip otherwise.

**Files to MODIFY:** new artifact under `.orqa/implementation/lessons/` or the closest matching type once the post-S2 graph structure is live (likely a `LESSON-*` or `ISSUE-*` record). At S2-time (pre-SurrealDB cutover) file it as markdown under `.orqa/learning/lessons/` following existing conventions. Post-cutover, it exists as a SurrealDB record with type `lesson` or `technical-debt`.

**AC:**
- [ ] Artifact filed with: SurrealDB version probed, observed failure mode, workaround location in code (`daemon/src/surreal_poller.rs`), retire-condition (upstream fix or version upgrade that passes the probe).
- [ ] Artifact status: `open` with priority `high`.
- [ ] `TASK-S2-12` findings link to this artifact ID.
- [ ] `surreal_poller.rs` module doc comment links to this artifact ID.
- [ ] Graph relationship: artifact `relates_to` TASK-S2-12 as `blocks_resolution_of`.
- [ ] Added to the project's retro review list for quarterly re-check.

**Reviewer checks:**
- Artifact exists and is queryable via `GET /artifacts/<id>`.
- Code comment grep in `surreal_poller.rs` returns the artifact ID.
- Relationship edge present.

**Deps:** TASK-S2-01 (outcome), TASK-S2-12 (needs code location to link to).

---

### TASK-S2-18 — Plugin uninstall UI confirmation flow

**What:** Two-step uninstall handshake in the app frontend. When the user triggers uninstall, the daemon returns a preview of what will be removed (artifact count, user-edited artifact list, runtime files). The frontend displays a confirmation dialog; only on explicit confirm does the daemon execute removal.

**Files to READ:** TASK-S2-08 output (engine uninstall backend); existing frontend plugin-management UI; SSE event types for the confirmation response.
**Files to MODIFY:** new daemon route `GET /plugins/:name/uninstall-preview` (returns preview, no side effects); extend existing uninstall route to require a `confirm_token` from the preview; new frontend `UninstallDialog` component; update the plugin-list page.

**AC:**
- [ ] Clicking "uninstall" in the UI first calls `uninstall-preview`, never deletes anything.
- [ ] Dialog displays: plugin name, total artifacts that will be removed, count of user-edited artifacts, a scrollable list of artifact titles, and runtime file count.
- [ ] User-edited artifacts are visually flagged (distinct styling) and require an additional "I understand these edits will be lost" checkbox before confirm is enabled.
- [ ] Confirm button is disabled until both checkbox AND confirm-text-match (e.g. typing the plugin name) are satisfied.
- [ ] Cancel or ESC leaves everything intact.
- [ ] CLI `orqa plugin uninstall <name>` without `--force` prints the same preview text and prompts on stdin; with `--force` skips the prompt (for scripted workflows).
- [ ] `confirm_token` from preview expires after 2 minutes to prevent stale confirmations.

**Reviewer checks:**
- Trigger uninstall preview, verify zero database mutations before confirm.
- Flag a user edit, confirm the checkbox gate works.
- Time out the confirm_token (mock clock), confirm the delete route 409s.
- Run CLI with `--force` against a test plugin; confirm no prompt shown.

**Deps:** TASK-S2-08 (engine uninstall backend), TASK-S2-13 (frontend SSE infrastructure for live preview counts if artifact count is dynamic).

---

### TASK-S2-20 — Add `version` field to artifact schema + bump helper (optimistic-lock groundwork)

**What:** Extend the SurrealDB artifact schema with a `version: int` column (default `1`) and a `updated_at: datetime` column. Introduce a single `bump_version(record)` helper in `engine/graph-db/` that every writer calls immediately before commit. MVP enforcement is OFF: feature flag `ORQA_OPTIMISTIC_LOCK` defaults `false`; when `false`, writers still bump but skip the pre-update version check. When `true` (future stream), writers include `IF version = $expected` in the update and return 409 on mismatch.

**Files to READ:** `engine/graph-db/src/schema.rs` (or wherever DEFINE FIELD lives), `daemon/src/surreal_queries.rs`, `engine/graph/src/sync.rs:17,251-266` (content-hash helper, parallel pattern).
**Files to MODIFY:** `engine/graph-db/src/schema.rs` — add `version`, `updated_at` fields; `engine/graph-db/src/writers.rs` (new) — `bump_version()` helper + optional enforcement check; `daemon/src/surreal_queries.rs` — writers use helper; schema migration note in `orqa migrate storage` (existing markdown-ingested rows start at `version: 1`).

**AC:**
- [ ] Schema defines `version: int DEFAULT 1` and `updated_at: datetime DEFAULT time::now()` on the artifact table.
- [ ] `bump_version()` helper increments `version` and sets `updated_at = time::now()` atomically with the write.
- [ ] Every writer path (PUT, POST, DELETE soft-delete, plugin install ingest, migrate ingest) calls the helper — enforced by test.
- [ ] `ORQA_OPTIMISTIC_LOCK=false` (default): version is written but not checked; no 409 paths exercised.
- [ ] `ORQA_OPTIMISTIC_LOCK=true`: writer with a stale `expected_version` returns HTTP 409 (test with mocked stale value). This path exists but is unreachable in MVP.
- [ ] Migration ingest (TASK-S2-09) sets `version: 1` on every imported artifact.
- [ ] Zero user-visible behaviour change in MVP — confirmed by existing PUT/POST integration tests passing unchanged.

**Reviewer checks:**
- `SELECT version, updated_at FROM artifact LIMIT 5` after any write — values populated and monotonically increasing per record.
- Grep for direct SurrealDB writes bypassing the helper — none found.
- Toggle flag, run stale-version test — 409 returned only when flag is on.
- Default config confirms flag is off.

**Deps:** None (foundational). Should land in Wave 1 or 2 so later writer tasks consume the helper.

---

### TASK-S2-21 — Tracking artifact for deferred optimistic-lock enforcement

**What:** File a tracked issue artifact documenting that `ORQA_OPTIMISTIC_LOCK` ships OFF in MVP, the conditions under which it should be flipped ON (self-hosted tier, multi-client scenarios, or first observed silent-overwrite incident), and the UI work required at that point (conflict dialog / stale-reload UX). Keeps the deferred enforcement visible in orphan checks, health reports, and quarterly retros.

**Files to MODIFY:** new artifact under `.orqa/learning/lessons/` pre-cutover; post-cutover lives as SurrealDB `lesson` or `technical-debt` record.

**AC:**
- [ ] Artifact filed with: rationale for deferring, flag location in code, retire-conditions, UI-design open questions.
- [ ] Status: `open`, priority: `medium`.
- [ ] Graph relationship: artifact `relates_to` TASK-S2-20 as `blocks_resolution_of`.
- [ ] Code comment in `engine/graph-db/src/writers.rs` (or flag definition site) links to this artifact ID.
- [ ] Added to project retro review list for quarterly re-check.

**Reviewer checks:**
- Artifact exists and is queryable via `GET /artifacts/<id>`.
- Code comment grep returns the artifact ID.
- Relationship edge present.

**Deps:** TASK-S2-20 (needs flag location to link to).

---

### TASK-S2-22 — Migrate `manifest.json` into SurrealDB as `plugin_installation` records

**What:** Replace the on-disk `.orqa/manifest.json` installation ledger with a SurrealDB record type `plugin_installation`. Each record: `plugin_name`, `version`, `installed_at`, `manifest_hash`, and a sub-list of installed files with `{path, source_hash, installed_hash, target: "surrealdb" | "runtime", artifact_id?}`. Plugin install writes the record in the same transaction as the artifact ingest + runtime-code copy. Uninstall queries by `plugin_name` to enumerate what to remove. Verify queries by `plugin_name` to detect drift. Add a `orqa plugin list` CLI command that renders the table for debug (replacing `cat .orqa/manifest.json`).

**Files to READ:** `.orqa/manifest.json` (current shape), `cli/src/commands/install.ts:569` (`copyPluginContent` + ledger write), `cli/src/commands/verify.ts` (how ledger is consumed), `engine/plugin/src/installer.rs:53-129` (Rust install path), TASK-S2-08 output (new split-install flow).
**Files to MODIFY:** `engine/graph-db/src/schema.rs` — add `plugin_installation` table; `engine/plugin/src/installer.rs` — write record instead of JSON; `daemon/src/routes/plugins.rs` — expose read/list; `cli/src/commands/plugin-list.ts` (new); `cli/src/commands/verify.ts` — consume from daemon, not filesystem; TASK-S2-09 migration — port existing `.orqa/manifest.json` entries into records on first run; delete `.orqa/manifest.json` after successful port.

**AC:**
- [ ] Schema defines `plugin_installation` with `plugin_name` as primary key, `version`, `installed_at`, `manifest_hash`, and nested `files` array.
- [ ] Plugin install writes the record atomically with artifact ingest and runtime copy; no ledger entry without a completed install, and vice versa.
- [ ] `orqa plugin list` prints: plugin name, version, install date, file count (artifact + runtime), drift status.
- [ ] `orqa verify` drift detection reads from the daemon, not the filesystem.
- [ ] `orqa migrate storage` ports every existing `.orqa/manifest.json` entry into SurrealDB with `source_hash` / `installed_hash` preserved.
- [ ] After successful migration, `.orqa/manifest.json` is moved to `.state/archive/orqa-files/` (not deleted — part of the general archive).
- [ ] Uninstall uses `plugin_installation` record as the source of truth for what to remove.
- [ ] Integration test: install → verify record exists → uninstall → verify record gone → re-install → verify record recreated with new `installed_at`.

**Reviewer checks:**
- `SELECT * FROM plugin_installation` after a test install returns the expected shape.
- Rename `.orqa/manifest.json` out of the way post-migration — `orqa verify` still works.
- Kill the daemon mid-install — no half-written ledger (transaction atomicity).
- `orqa plugin list` output matches what a test plugin installed.

**Deps:** TASK-S2-08 (split install), TASK-S2-09 (migrate pipeline runs the port), TASK-S2-20 (writers use bump helper — plugin_installation records participate).

---

### TASK-S2-23 — Delete `.orqa/prompt-registry.json` — **WITHDRAWN from Wave 1** (per Bobbi, 2026-04-16)

**Status:** WITHDRAWN. `.orqa/prompt-registry.json` is NOT an unused stub. It is the live output of the knowledge-injection pipeline.

**Rationale:**
- **Writer:** `cli/src/lib/prompt-registry.ts:generatePromptRegistry()` — reads all installed plugin manifests, collects `knowledge_declarations`, and writes merged JSON to `.orqa/prompt-registry.json`. Called from `cli/src/commands/install.ts` and `cli/src/commands/plugin.ts` at `orqa install` / `orqa plugin install/update`.
- **Reader:** `daemon/src/knowledge.rs:get_declared_knowledge()` — reads the file at POST /knowledge request time to return declared knowledge entries for the detected agent role. Degrades gracefully if file absent, but knowledge injection silently goes dark.
- The file is currently empty (`knowledge: []`) only because no installed plugin declares `knowledge_declarations`. The pipeline itself is live code.

**See findings:** `.state/findings/s2-wave1/TASK-S2-23.findings.md`

**Impact on other tasks:** None for Wave 1. TASK-S2-14 cutover MUST NOT archive or delete `.orqa/prompt-registry.json` — it is config-adjacent runtime output, not an artifact file. The cutover's move list covers only the artifact directories (`discovery`, `documentation`, `implementation`, `learning`, `planning`, `workflows`).

**Where the work went:** Moved to TASK-S2-24 (deferred, knowledge layer migration).

---

### TASK-S2-24 — Migrate knowledge-injection pipeline (prompt-registry) to SurrealDB

**Status:** DEFERRED — not in Wave 1-8 scope. Runs when the broader knowledge layer is migrated to SurrealDB (tentatively Phase S3 or a dedicated knowledge-layer epic — Bobbi to slot).

**What:** Replace file-based `.orqa/prompt-registry.json` with SurrealDB records. `generatePromptRegistry()` writes records to a new `knowledge_declaration` table with `source_plugin`. `get_declared_knowledge()` queries records for the agent role. Delete the JSON file and all file-based code paths once the DB path is live.

**Files to READ:**
- `cli/src/lib/prompt-registry.ts`
- `cli/src/commands/install.ts:21,629`
- `cli/src/commands/plugin.ts:29`
- `daemon/src/knowledge.rs:267,651,694,734`
- `cli/__tests__/agent-spawner.test.ts:29`

**Files to MODIFY:** all of the above plus new `knowledge_declaration` table in `engine/graph-db/src/schema.rs`.

**AC (sketch — refine at schedule time):**
- [ ] `knowledge_declaration` table defined in SurrealDB schema with columns: `id`, `source_plugin`, `role`, `content_file`, `content`.
- [ ] `generatePromptRegistry()` writes records to `knowledge_declaration` instead of JSON file.
- [ ] `get_declared_knowledge()` queries `knowledge_declaration` records; no filesystem access for prompt-registry.
- [ ] `.orqa/prompt-registry.json` deleted, all references removed (`grep -r prompt-registry` returns zero hits post-task).
- [ ] POST /knowledge integration test: install plugin declaring knowledge → query records → verify returned knowledge matches declaration.
- [ ] Plugin uninstall removes `source_plugin = <name>` records, matching TASK-S2-08 pattern.

**Reviewer checks (sketch):**
- Run integration test end-to-end.
- Confirm zero filesystem reads of `prompt-registry.json` post-task (strace or grep-based).
- Confirm graceful empty state when no plugin declares knowledge (returns `[]`, no error).

**Deps:** TASK-S2-08 (plugin install split pattern — knowledge migration mirrors the same ingest flow).

---

### TASK-S2-25 — Wave 1 post-commit cleanup (fixture hygiene + non-blocking followups)

**Status:** SCHEDULED — runs immediately after the Wave 1 commit lands, before Wave 2 begins. Contains the non-blocking followups surfaced during Wave 1 review cycles.

**What:** Three small cleanups, bundled because they are each too small for standalone tasks but collectively worth a short focused pass:

1. **Fixture hygiene for TASK-S2-09 integration tests.** `daemon/tests/routes_admin_migrate.rs` tests C1 and C2 currently post ingest requests at the fixture root directly, so each test run writes a fresh JSON report to `tests/fixtures/s2-09-migrate/.state/migrations/`. The fixture root accumulates these reports (28 present at Wave 1 review time). C3 already correctly uses `tempfile::tempdir()` for isolation — apply the same pattern to C1 and C2. Post-fix, the `.state/` subtree under the fixture dir should never be git-tracked (also add a `tests/fixtures/**/.state/` entry to `.gitignore` if not already present).

2. **Three-way merge route-level integration test.** TASK-S2-03's three-way merge algorithm (`engine/graph/src/merge.rs`) has strong unit-test coverage but no HTTP-level integration test exercising `POST /artifacts/import --on-conflict=merge` with a hand-authored conflict fixture. Add one end-to-end test using a divergent fixture to prove the route path is wired correctly.

3. **Review cycle-2 doc comment hygiene.** `engine/graph/src/writers.rs` module doc (introduced in S2-20 cycle 2) should be spot-verified after S2-03 cycle 2 landed — check the doc still accurately describes that every writer routes through `bump_version()`.

**Files to READ:**
- `daemon/tests/routes_admin_migrate.rs` (C1, C2, C3 tests)
- `tests/fixtures/s2-09-migrate/` (fixture tree)
- `tests/fixtures/s2-03-import/fixture-merge-conflict/` (existing merge-conflict fixture)
- `engine/graph/src/writers.rs` (module doc)
- `.gitignore`

**Files to MODIFY:**
- `daemon/tests/routes_admin_migrate.rs` (update C1, C2 to use `tempfile::tempdir()`)
- `.gitignore` (add `tests/fixtures/**/.state/` if missing)
- `daemon/tests/routes_import.rs` (new integration test for merge path — or a new test file if cleaner)
- `engine/graph/src/writers.rs` (doc comment spot fix if needed)

**AC:**
- [ ] C1 and C2 tests in `routes_admin_migrate.rs` use tempdirs; no writes to `tests/fixtures/s2-09-migrate/.state/` during test runs.
- [ ] `tests/fixtures/**/.state/` gitignored.
- [ ] `tests/fixtures/s2-09-migrate/.state/` removed from the working tree (delete the accumulated reports).
- [ ] New merge-path integration test exists, passes, and actually exercises a three-way merge via HTTP (not just a merge call via the engine layer).
- [ ] `engine/graph/src/writers.rs` module doc accurately reflects the post-cycle-2 state.
- [ ] Full `cargo test -p orqa-daemon` + `cargo clippy` still green.

**Reviewer checks:**
- Run the full daemon test suite twice in a row — no fixture-dir pollution between runs.
- New merge integration test actually fails when the algorithm is temporarily broken (mutation test).
- `git ls-files tests/fixtures/s2-09-migrate/.state/` returns empty.

**Deps:** Wave 1 must be committed first — this task assumes the landed shape of S2-03, S2-09, and S2-20.

---

## Collision Map

Tasks that touch the same file and MUST serialize:

| File | Tasks |
|------|-------|
| `daemon/src/routes/artifacts.rs` | TASK-S2-04, 05, 06 — serialise in this order |
| `daemon/src/lib.rs` + `daemon/src/health.rs` (route registration) | TASK-S2-02, 05, 06, 09, 10 — each adds routes; last-to-merge rebases |
| `cli/src/commands/migrate-storage.ts` | TASK-S2-09, 10, 11, 14, 17 — serialise all |
| `cli/src/lib/content-lifecycle.ts` | TASK-S2-07 only (TASK-S2-08 reads but does not modify) |
| `engine/plugin/src/installer.rs` | TASK-S2-08 only |
| `daemon/src/watcher.rs` | TASK-S2-15 only |
| `engine/graph/src/sync.rs` | TASK-S2-04 (helper extraction); TASK-S2-08, 09 read-only |
| `daemon/src/event_bus.rs` | TASK-S2-12 only |
| `daemon/src/main.rs` | TASK-S2-12 (live subscription boot) |
| `.orqa/` filesystem layout | TASK-S2-14 (archive/restructure), TASK-S2-17 (cleanup) — serialise |
| `.orqa/plugins/` runtime-code install target | TASK-S2-08 only (created by install); TASK-S2-14 leaves it intact |
| `orqa-plugin.json` schema (if extended for target field) | TASK-S2-08 only |
| `engine/graph-db/src/schema.rs` | TASK-S2-20 (version/updated_at); TASK-S2-22 (plugin_installation table); TASK-S2-24 (knowledge_declaration table, deferred) — serialise |
| `engine/graph-db/src/writers.rs` (new) | TASK-S2-20 creates; TASK-S2-04, 05, 06, 08, 09, 22 call the helper |
| `cli/src/commands/verify.ts` | TASK-S2-16 (SurrealDB consistency flip); TASK-S2-22 (manifest read-from-daemon) — serialise |
| `cli/src/commands/install.ts` | TASK-S2-07 (stop copying into `.orqa/`); TASK-S2-22 (ledger write path) — serialise |
| `.orqa/manifest.json` | TASK-S2-22 (ported then archived) |
| `.orqa/prompt-registry.json` | TASK-S2-24 (deferred migration to SurrealDB — do NOT delete in S2) |
| `project.json` | TASK-S2-14 only |
| Frontend artifact store | TASK-S2-13 only |
| `.gitignore` | TASK-S2-25 only |
| `daemon/tests/routes_admin_migrate.rs` | TASK-S2-09 (created); TASK-S2-25 (fixture tempdirs — Wave 1.5 runs alone post-commit) — serialise |
| `daemon/tests/routes_import.rs` | TASK-S2-03 (created); TASK-S2-25 (merge route integration test may add to it) — serialise |
| `engine/graph/src/writers.rs` | TASK-S2-20 creates; TASK-S2-03 extends (import_upsert, import_merge_write route through bump_version); TASK-S2-25 (module doc hygiene) — serialise |

Parallel-safe pairs: almost everything between different CLI commands, different engine modules, and frontend. The serialised files above are the only true contention.

---

## Sequencing Plan (Waves)

Each wave is a set of tasks safe to execute in parallel. A wave completes fully (all Reviewer PASS) before the next begins.

**Wave 1 — Foundation, read-only risk** ✅ LANDED 2026-04-16
- TASK-S2-01 (LIVE SELECT probe) — **PASS**
- TASK-S2-03 (orqa import with user-selectable conflict policy — no dep on -02) — **PASS** (cycle 2)
- TASK-S2-07 (CLI install stops writing `.orqa/`) — **PASS**
- TASK-S2-09 (migrate storage ingest) — **PASS** (cycle 2)
- TASK-S2-20 (version field + bump helper — lands before any writer consumes it) — **PASS** (cycle 2)
- ~~TASK-S2-23~~ WITHDRAWN — prompt-registry.json is live code, not a stub; knowledge-layer migration deferred to TASK-S2-24
- (TASK-S2-02 export DEFERRED to S3 per Bobbi)

Wave 1 landed 2026-04-16 with 5 PASS (S2-01, S2-03, S2-07, S2-09, S2-20). S2-23 withdrawn. Post-wave cleanup scheduled as TASK-S2-25.

**Wave 1.5 — Post-Wave-1 cleanup (single implementer, runs before Wave 2)**
- TASK-S2-25 (fixture hygiene + merge-route integration test + doc comment hygiene)

**Wave 2 — Write-path flip**
- TASK-S2-04 (flip PUT)
- TASK-S2-08 (engine install → SurrealDB + `.orqa/plugins/` — depends on -07)
- TASK-S2-22 (manifest.json → `plugin_installation` records — depends on -08, -09, -20)

**Wave 3 — New CRUD endpoints**
- TASK-S2-05 (POST /artifacts — depends on -04)

**Wave 4 — Continue CRUD + verification**
- TASK-S2-06 (DELETE /artifacts — depends on -05)
- TASK-S2-10 (migrate verify — depends on -09)

**Wave 5 — Live updates pipeline**
- TASK-S2-12 (event_bus integration, LIVE or poll-fallback — depends on -01, -04/-05/-06)
- TASK-S2-19 (tracking artifact for LIVE SELECT gap — runs only if -01 FAILED; depends on -01, -12)

**Wave 6 — Cutover**
- TASK-S2-14 (cutover — depends on -04, -05, -06, -08, -09, -10)

**Wave 7 — Post-cutover cleanup and frontend**
- TASK-S2-11 (rollback — depends on -14)
- TASK-S2-13 (frontend live updates — depends on -12)
- TASK-S2-15 (remove `.orqa/` watcher branch — depends on -14)
- TASK-S2-16 (verify flip — depends on -14)

**Wave 8 — Retention and plugin UX**
- TASK-S2-17 (cleanup archive — depends on -14)
- TASK-S2-18 (uninstall UI confirmation flow — depends on -08, -13)
- TASK-S2-21 (tracking artifact for deferred optimistic-lock enforcement — depends on -20)

---

## Risk Log

| # | Risk | Severity | Mitigating task(s) |
|---|------|----------|--------------------|
| R1 | SurrealDB 3.x embedded LIVE SELECT may have the same kind of bug as the depth-range traversal gap | High | TASK-S2-01 probes first. **Fallback resolved (Bobbi Q4, 2026-04-15): option C (poll-and-diff inside daemon)** if probe FAILS. TASK-S2-12 implements both paths; TASK-S2-19 files a tracking artifact so the workaround gets retired when SurrealDB ships the fix |
| R2 | Source-of-truth flip is one-way — bad cutover loses user edits if rollback fails | Critical | TASK-S2-11 (rollback) lands before TASK-S2-14 (cutover) is ever run; TASK-S2-10 verify gates cutover. Note: `orqa export` escape hatch deferred to S3 per Bobbi Q1 — rollback + verify are the only safety net in S2. This raises R2 from High to Critical |
| R3 | Removing `.orqa/` watcher while a bug keeps writing to `.orqa/` would silently lose writes | Medium | TASK-S2-15 is gated on TASK-S2-14; add a pre-removal assertion that `.orqa/` contains no artifact files |
| R4 | Plugin install changes (TASK-S2-07/-08) could leave plugins half-installed if interrupted between TS copy and engine ingest | Medium | Engine ingest is idempotent (content-hash); document recovery as "re-run install" in TASK-S2-08 findings |
| R5 | Frontend live updates (TASK-S2-13) could thrash the UI on bulk migrate | Low | TASK-S2-13 adds debounce; migrate runs while app is closed per Bobbi's dogfood pattern |
| R6 | `version` field contention on concurrent PUTs from multiple clients | Medium | **Resolved (Bobbi Q8, 2026-04-15):** TASK-S2-20 adds `version` + `updated_at` fields and a bump helper that every writer uses; enforcement stays OFF behind `ORQA_OPTIMISTIC_LOCK=false` in MVP (last-write-wins preserved). TASK-S2-21 files a tracking artifact for the deferred enforcement. Groundwork lands now so self-hosted/cloud tiers can flip the flag without schema migration |
| R7 | Archive step (TASK-S2-14) move-not-copy could fail mid-move leaving `.orqa/` in a broken hybrid state | High | TASK-S2-14 writes manifest BEFORE moving; implements a two-phase commit (copy → verify → unlink original). Update AC accordingly during execution if copy-then-delete is safer than rename |

---

## Open Questions for Bobbi

Surface before any S2 execution begins. Do NOT answer autonomously.

1. ~~`orqa export` scope.~~ **RESOLVED (Bobbi, 2026-04-15):** Export deferred to S3. TASK-S2-02 removed from scope.

2. ~~`orqa import` conflict resolution.~~ **RESOLVED (Bobbi, 2026-04-15):** Either upsert-and-bump or three-way merge, user-selectable. TASK-S2-03 updated to expose `--on-conflict=upsert|merge` with default configurable in `project.json`. **Follow-up RESOLVED (Bobbi, 2026-04-15):** No-base records surfaced up front with bulk-control options (`take-theirs | keep-ours | review-each | fail`); `review-each` is interactive default, `fail` is non-interactive default. Import parser accepts-and-ignores `base_snapshot` field for forward-compat with S3 `orqa export`.

3. ~~`orqa migrate storage` idempotency.~~ **RESOLVED (Bobbi, 2026-04-15):** Safely re-runnable. TASK-S2-09 AC already aligns; no `--force` flag required.

4. ~~LIVE SELECT vs SSE bridge.~~ **RESOLVED (Bobbi, 2026-04-15):** Option C — poll SurrealDB inside the daemon at ~500ms, diff, publish to event_bus. Fallback applies only if TASK-S2-01 probe FAILS. Requirements:
   - The poll-and-diff implementation must be clearly marked as a **temporary workaround**, not long-term design.
   - A tracking artifact must be filed (see new TASK-S2-19) so this gets revisited when SurrealDB embedded LIVE SELECT is fixed.
   - Every log line from the poller should tag itself so operators can grep for it.

5. ~~Archive location and retention.~~ **RESOLVED (Bobbi, 2026-04-15):** Prune at 90 days. TASK-S2-17 default `--min-age-days 90`.

6. ~~`orqa verify` output shape.~~ **RESOLVED (Bobbi, 2026-04-15):** SurrealDB consistency check **replaces `enforce` only**. `version check`, `repo license`, and `repo readme` keep running unchanged. Post-S2 `orqa verify` runs: SurrealDB consistency + version check + license + readme.

7. ~~Watcher coverage during migration.~~ **RESOLVED (Bobbi, 2026-04-15):** Pause the watcher during `orqa migrate storage` ingest. TASK-S2-09 calls `POST /watcher/pause` before ingest and `POST /watcher/resume` after success (or after rollback on error). Daemon restart defaults watcher to `running` — never stuck paused. Rationale: clean SSE event stream, deterministic migration semantics, small control-surface cost. Add pause/resume endpoints as part of TASK-S2-09 (or spin out a sub-task if scope grows).

8. ~~Multi-writer posture.~~ **RESOLVED (Bobbi, 2026-04-15):** Option C — add `version: int` field to the SurrealDB artifact schema in S2 and have every writer bump it on update, but do NOT enforce version-mismatch 409s in MVP (last-write-wins stays in effect). Enforcement ships disabled behind `ORQA_OPTIMISTIC_LOCK=false`. See new **TASK-S2-20** for the schema + bump-helper foundation and **TASK-S2-21** for the tracking artifact that keeps the deferred enforcement visible until it's turned on.

9. ~~Plugin uninstall semantics.~~ **RESOLVED (Bobbi, 2026-04-15):** Vanish — but only after a UI confirmation flow shows the user what will be lost (list of artifacts, flag of any user-edited ones). Uninstall blocked until user explicitly confirms. See new TASK-S2-18 for the UI flow.

10. ~~`manifest.json`, `prompt-registry.json` fate.~~ **RESOLVED (Bobbi, 2026-04-15):** (A2) `manifest.json` becomes a SurrealDB record type `plugin_installation` written atomically with the plugin install transaction; existing 56KB JSON is migrated in. A `plugin install list` CLI command replaces the `cat .orqa/manifest.json` debug path. (B1) `prompt-registry.json` is deleted after a grep confirms nothing writes to it; if a writer exists, fix it to target SurrealDB first, then delete. See new **TASK-S2-22** (manifest migration to SurrealDB) and **TASK-S2-23** (prompt-registry deletion). **UPDATE (Bobbi, 2026-04-16):** S2-23 WITHDRAWN — `prompt-registry.json` has a live writer (`cli/src/lib/prompt-registry.ts:generatePromptRegistry()`) and reader (`daemon/src/knowledge.rs:get_declared_knowledge()`); the empty-file state was misleading. Knowledge-layer migration deferred to **TASK-S2-24** (Phase S3 / dedicated knowledge-layer epic).

11. ~~Q-A (raised by TASK-S2-08) — Manifest classification for artifact vs runtime code.~~ **RESOLVED (Bobbi, 2026-04-15):** Option (a) — explicit `target: "surrealdb" | "runtime"` field per `content` entry in `orqa-plugin.json`. Convenience default: if `target` is omitted AND the path ends in `.md`, installer defaults to `surrealdb` and emits a lint warning; otherwise defaults to `runtime` with lint warning. Strict validation (no defaults) can be tightened later. Migration: one-shot script in TASK-S2-08 walks all 16 existing manifests, classifies each entry by its current install destination (`.orqa/<artifact-dir>/*` → `surrealdb`; everywhere else → `runtime`), writes the `target` back, commits the updated manifests.

---

## Execution Notes for the Next Session

- Every task runs through: Implementer subagent → write findings file → Reviewer subagent (PASS/FAIL each AC) → orchestrator commits.
- Do NOT begin Wave N+1 until every task in Wave N has Reviewer PASS. Follow-ups go to a backlog file, not into the current wave.
- Commit at wave boundaries with a `feat(surreal-s2-waveN): …` message.
- After Rust changes: rebuild and restart daemon before the next task in the same wave.
- Re-read this plan at session start — resume from the first unfinished task in the first incomplete wave.

**End of plan.**
