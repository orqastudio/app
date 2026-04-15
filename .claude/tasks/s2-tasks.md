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
- [ ] `--on-conflict=merge`: three-way merge between (base = record's stored original from plugin source or last import), (ours = current SurrealDB state), (theirs = incoming file). On conflict that cannot auto-merge, write both versions to a conflict file in `.state/import-conflicts/<migration_id>/<artifact_id>.conflict.md` and FAIL the import with that record flagged; do not commit partial merges.
- [ ] Default read from `project.json` `import.onConflict` if present, else `upsert`; CLI flag overrides config.
- [ ] Re-running import on the same directory is a no-op under both policies (content hash unchanged → skip).
- [ ] Reports per-file status: CREATED / UPDATED / SKIPPED / MERGED / CONFLICT with reason.
- [ ] Integration test: seed SurrealDB from fixture A, import fixture B (diverges on 3 records) under both policies — verify upsert overwrites all 3, merge auto-merges non-conflicting fields and flags the true conflicts.

**Reviewer checks:**
- Run both policies against a fixture with hand-engineered conflicts; confirm upsert loses the in-DB change on conflicted records (documented behaviour) and merge preserves both side changes where possible.
- Verify `version` increments on UPDATE and MERGED outcomes; unchanged on SKIPPED.
- Confirm no partial writes on CONFLICT — the transaction is all-or-nothing per artifact.

**Open question for Bobbi on this task (Q2-followup):** For three-way merge, what is the "base" version used as the merge base when the artifact is user-authored (no plugin source)? Options: (a) the artifact's last-imported `content_hash` stored alongside the record; (b) a snapshot taken at each import; (c) fall back to two-way merge (no base) when unavailable. Default proposal: (a) with (c) as fallback when no base is recorded.

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

The manifest (`orqa-plugin.json`) already declares per-path intent via the `content` mapping; extend the schema if needed so each entry explicitly declares `target: "surrealdb"` or `target: "runtime"` (see open question Q-A below).

**Files to READ:** `engine/plugin/src/installer.rs:53-129`, `cli/src/lib/content-lifecycle.ts` (for plugin-manifest → source-path mapping logic — keep the parser, redirect targets), `daemon/src/watcher.rs` plugin branch, sample `plugins/**/orqa-plugin.json` manifests.
**Files to MODIFY:** `engine/plugin/src/installer.rs` (two install sinks: SurrealDB ingest for artifact entries, copy-to-`.orqa/plugins/<name>/` for runtime entries); `daemon/src/routes/plugins.rs:208-250`; possibly new `engine/plugin/src/ingest.rs`; watcher `plugins/` handler routes edits through the same splitter. Update the plugin manifest JSON schema if new `target` field is introduced.

**AC:**
- [ ] After `orqa plugin install <name>`, every manifest entry classified as artifact is present in SurrealDB with `source_plugin = <name>` — read directly from `plugins/<name>/**`.
- [ ] Every manifest entry classified as runtime is present under `.orqa/plugins/<name>/` with byte-identical content to the source.
- [ ] Editing a file under monorepo `plugins/<name>/` triggers re-ingest (artifact entries) AND re-copy (runtime entries) via the watcher.
- [ ] Re-installing the same plugin is idempotent (content-hash skip on both sinks).
- [ ] Uninstalling a plugin removes SurrealDB records where `source_plugin = <name>` AND removes `.orqa/plugins/<name>/` (see Q9 for user-edited artifact semantics).
- [ ] Enforcement rules from the plugin are inserted into `enforcement_rule` table.
- [ ] No new files created under `.orqa/<artifact-dir>/` (discovery, implementation, learning, planning, workflows, documentation) by any install or watcher path.
- [ ] Only files under `.orqa/plugins/<name>/` are created for plugin runtime code.

**Reviewer checks:**
- Install a fixture plugin declaring one artifact file and one runtime file. Verify: SurrealDB has one record with `source_plugin = 'fixture'`; `.orqa/plugins/fixture/<runtime-file>` exists; no other `.orqa/` files touched.
- Edit both files under `plugins/fixture/`, wait for watcher debounce, confirm SurrealDB `content_hash` updated AND `.orqa/plugins/fixture/` file re-copied.
- Uninstall, verify both install targets cleared.
- Grep the install code path for any write under `.orqa/` outside `.orqa/plugins/` — must be zero.

**Open question raised by this task (add to Q-list):** Q-A — how does the manifest distinguish artifact entries from runtime entries? Current `content` block has `source`/`target` paths; do we add a `target: "surrealdb" | "runtime"` field, infer by file extension, or by directory convention (e.g., everything under `plugin/artifacts/` is artifact, everything under `plugin/runtime/` is runtime)?

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

### TASK-S2-12 — Plug SurrealDB LIVE SELECT into `event_bus`

**What (gated on TASK-S2-01 PASS):** Subscribe to `LIVE SELECT * FROM artifact` inside the daemon; on each notification, publish a typed event onto the existing `event_bus`.

**Files to READ:** `daemon/src/event_bus.rs`, `daemon/src/main.rs:310` (existing broadcaster pattern), SurrealDB live-query API.
**Files to MODIFY:** new `daemon/src/surreal_live.rs`; register in `main.rs`; extend event type enum if needed.

**AC:**
- [ ] LIVE subscription starts on daemon boot alongside SurrealDB init; reconnect on error.
- [ ] CREATE/UPDATE/DELETE notifications each produce a distinct event type on `event_bus`.
- [ ] Backpressure behaviour documented — if `event_bus` buffer full, oldest event drops (existing 10k-buffer semantics).

**Reviewer checks:**
- Issue POST /artifacts → observe event on event_bus within 200ms.
- Kill SurrealDB mid-run (if feasible), confirm reconnect or clean failure log.

**Deps:** TASK-S2-01 PASS; TASK-S2-04/05/06 (writes exist to trigger events).

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

**Reviewer checks:**
- Run on a copy of the real repo `.orqa/`; verify move (not copy) by checking source dirs are gone.
- Confirm the three canonical entries are the ONLY survivors.

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
- [ ] Refuses to delete archives younger than N days (N is configurable via flag `--min-age-days`, default: 30; open question Q5 asks Bobbi for the default).
- [ ] Records cleanup in `.state/migrations/<migration_id>.json` as `archive_cleaned_at`.

**Reviewer checks:**
- Run without `--confirm` → dry-run only, no filesystem changes.
- Attempt cleanup on a recent archive → refused with age-gate message.
- Attempt with `--min-age-days 0 --confirm` → proceeds.

**Deps:** TASK-S2-14 (no longer depends on TASK-S2-02 since export is deferred to S3).

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
| `project.json` | TASK-S2-14 only |
| Frontend artifact store | TASK-S2-13 only |

Parallel-safe pairs: almost everything between different CLI commands, different engine modules, and frontend. The serialised files above are the only true contention.

---

## Sequencing Plan (Waves)

Each wave is a set of tasks safe to execute in parallel. A wave completes fully (all Reviewer PASS) before the next begins.

**Wave 1 — Foundation, read-only risk**
- TASK-S2-01 (LIVE SELECT probe)
- TASK-S2-03 (orqa import with user-selectable conflict policy — no dep on -02)
- TASK-S2-07 (CLI install stops writing `.orqa/`)
- TASK-S2-09 (migrate storage ingest)
- (TASK-S2-02 export DEFERRED to S3 per Bobbi)

**Wave 2 — Write-path flip**
- TASK-S2-04 (flip PUT)
- TASK-S2-08 (engine install → SurrealDB + `.orqa/plugins/` — depends on -07)

**Wave 3 — New CRUD endpoints**
- TASK-S2-05 (POST /artifacts — depends on -04)

**Wave 4 — Continue CRUD + verification**
- TASK-S2-06 (DELETE /artifacts — depends on -05)
- TASK-S2-10 (migrate verify — depends on -09)

**Wave 5 — Live updates pipeline**
- TASK-S2-12 (event_bus integration — depends on -01, -04/-05/-06)

**Wave 6 — Cutover**
- TASK-S2-14 (cutover — depends on -04, -05, -06, -08, -09, -10)

**Wave 7 — Post-cutover cleanup and frontend**
- TASK-S2-11 (rollback — depends on -14)
- TASK-S2-13 (frontend live updates — depends on -12)
- TASK-S2-15 (remove `.orqa/` watcher branch — depends on -14)
- TASK-S2-16 (verify flip — depends on -14)

**Wave 8 — Retention**
- TASK-S2-17 (cleanup archive — depends on -14, -02)

---

## Risk Log

| # | Risk | Severity | Mitigating task(s) |
|---|------|----------|--------------------|
| R1 | SurrealDB 3.x embedded LIVE SELECT may have the same kind of bug as the depth-range traversal gap | High | TASK-S2-01 probes first; if FAIL, stop before TASK-S2-12, escalate to Bobbi for decision (defer item 8 or switch to server mode) |
| R2 | Source-of-truth flip is one-way — bad cutover loses user edits if rollback fails | Critical | TASK-S2-11 (rollback) lands before TASK-S2-14 (cutover) is ever run; TASK-S2-10 verify gates cutover. Note: `orqa export` escape hatch deferred to S3 per Bobbi Q1 — rollback + verify are the only safety net in S2. This raises R2 from High to Critical |
| R3 | Removing `.orqa/` watcher while a bug keeps writing to `.orqa/` would silently lose writes | Medium | TASK-S2-15 is gated on TASK-S2-14; add a pre-removal assertion that `.orqa/` contains no artifact files |
| R4 | Plugin install changes (TASK-S2-07/-08) could leave plugins half-installed if interrupted between TS copy and engine ingest | Medium | Engine ingest is idempotent (content-hash); document recovery as "re-run install" in TASK-S2-08 findings |
| R5 | Frontend live updates (TASK-S2-13) could thrash the UI on bulk migrate | Low | TASK-S2-13 adds debounce; migrate runs while app is closed per Bobbi's dogfood pattern |
| R6 | `version` field contention on concurrent PUTs from multiple clients | Medium | Document: S2 assumes single-writer (one app instance). Multi-writer optimistic-lock is a separate epic |
| R7 | Archive step (TASK-S2-14) move-not-copy could fail mid-move leaving `.orqa/` in a broken hybrid state | High | TASK-S2-14 writes manifest BEFORE moving; implements a two-phase commit (copy → verify → unlink original). Update AC accordingly during execution if copy-then-delete is safer than rename |

---

## Open Questions for Bobbi

Surface before any S2 execution begins. Do NOT answer autonomously.

1. ~~`orqa export` scope.~~ **RESOLVED (Bobbi, 2026-04-15):** Export deferred to S3. TASK-S2-02 removed from scope.

2. ~~`orqa import` conflict resolution.~~ **RESOLVED (Bobbi, 2026-04-15):** Either upsert-and-bump or three-way merge, user-selectable. TASK-S2-03 updated to expose `--on-conflict=upsert|merge` with default configurable in `project.json`. **Follow-up question:** for merge, what's the base version when there's no plugin source? (See TASK-S2-03 Q2-followup — proposal: stored last-imported content_hash with two-way fallback.)

3. **`orqa migrate storage` idempotency.** Should the ingest phase be safely re-runnable in production (TASK-S2-09 says yes) — or is it one-shot-per-project, with re-runs requiring `--force`?

4. **LIVE SELECT vs SSE bridge.** If TASK-S2-01 probe FAILS, options are: (a) defer scope item 8 to post-MVP and keep polling; (b) switch to SurrealDB server mode just for live queries; (c) poll SurrealDB inside the daemon at ~500ms and publish diffs to event_bus. Which do you prefer as a fallback?

5. **Archive location and retention.** `.state/archive/orqa-files/` per the epic. Do you want the archive retained indefinitely, or pruned automatically N days post-cutover?

6. **`orqa verify` output shape.** Today it runs `enforce`, `version check`, `repo license`, `repo readme`. After TASK-S2-16, should those sub-checks still run (unchanged) alongside the new SurrealDB consistency check, or does "flip to SurrealDB consistency" replace them entirely?

7. **Watcher coverage during migration.** During TASK-S2-09 ingest, the watcher is still watching `.orqa/` — any user edit would trigger sync. Should the watcher be paused during migration, or is the content-hash skip sufficient?

8. **Multi-writer posture.** R6 above: S2 assumes one writer (one app instance). Is that acceptable for the MVP beta, or do you want optimistic-lock groundwork in S2?

9. **Plugin uninstall semantics.** TASK-S2-08 AC says uninstall removes all `source_plugin = <name>` artifacts. Is this correct? Some plugin artifacts may have been user-edited; do they vanish on uninstall or get orphaned?

10. **`manifest.json`, `prompt-registry.json` fate.** The filesystem inventory found these at `.orqa/` root. Are they config (stay), artifact-derived (archive), or generated (regenerate from SurrealDB)?

11. **Q-A (raised by TASK-S2-08) — Manifest classification for artifact vs runtime code.** How does `orqa-plugin.json` distinguish plugin material that becomes SurrealDB artifacts from material that is installed as runtime code to `.orqa/plugins/<name>/`? Three options: (a) add an explicit `target: "surrealdb" | "runtime"` field per `content` entry; (b) infer by directory convention (e.g., `artifacts/` subtree → SurrealDB, `runtime/` subtree → file copy); (c) infer by file type (`.md` with frontmatter → SurrealDB, everything else → runtime). Preference and the migration path for existing plugin manifests (16 per the session context)?

---

## Execution Notes for the Next Session

- Every task runs through: Implementer subagent → write findings file → Reviewer subagent (PASS/FAIL each AC) → orchestrator commits.
- Do NOT begin Wave N+1 until every task in Wave N has Reviewer PASS. Follow-ups go to a backlog file, not into the current wave.
- Commit at wave boundaries with a `feat(surreal-s2-waveN): …` message.
- After Rust changes: rebuild and restart daemon before the next task in the same wave.
- Re-read this plan at session start — resume from the first unfinished task in the first incomplete wave.

**End of plan.**
