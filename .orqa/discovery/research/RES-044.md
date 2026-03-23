---
id: RES-c7b4b22f
type: research
title: Cross-cutting concerns of repository directory restructure
description: "Audit every file, config, and path reference that would need updating when reorganising the repository into backend/, ui/src/, sidecars/, and debugger/ directories."
status: completed
created: 2026-03-12
updated: 2026-03-12
sources: []
relationships:
  - target: EPIC-7b039d05
    type: guides
    rationale: "Research findings informed the design of Repository Directory Reorganisation"
---

## Research Questions

1. **Config files**: Which configuration files contain hardcoded paths to `src-tauri/`, `ui/`, `sidecar/`, or `scripts/`?
2. **Build tooling**: How do Cargo.toml workspace paths, package.json scripts, vite.config.ts, tauri.conf.json, and the Makefile need to change?
3. **Import paths**: Which TypeScript/Svelte imports use relative paths that would break? Which Rust module paths reference the current structure?
4. **Watcher configuration**: How are Vite and Cargo watchers currently configured, and what changes are needed to scope them to their new directories?
5. **Dev controller**: How does `scripts/dev.mjs` reference file paths, and what changes does moving it to `debugger/` require?
6. **Git and tooling**: How do `.gitignore`, `.gitattributes`, tsconfig.json paths, svelte.config.js, postcss.config.js, tailwind.config.ts, and other root configs reference the current structure?
7. **Documentation**: Which `.orqa/` docs reference current paths that would need updating?
8. **Symlinks**: How does the `.claude/` symlink architecture interact with the move?
9. **Tauri conf**: How does `tauri.conf.json` reference frontend dist paths and dev server URLs?
10. **Migration safety**: What is the safest order of operations to move directories without breaking the build at any intermediate step?

---

## Findings

### 1. Config Files

| File | Line/Location | Current Reference | Required Change |
|------|--------------|-------------------|-----------------|
| `Makefile` | Line 5 | `CARGO_MANIFEST := src-tauri/Cargo.toml` | `backend/src-tauri/Cargo.toml` |
| `Makefile` | Lines 21, 25 | `cd sidecar && bun install/build` | `cd sidecars/claude-agentsdk-sidecar && ...` |
| `Makefile` | ~9 refs total | `node scripts/dev.mjs` | `node debugger/dev.mjs` |
| `vite.config.ts` | Line 13 | `allow: ['ui', '.svelte-kit', 'node_modules']` | `allow: ['ui/src', '.svelte-kit', 'node_modules']` |
| `svelte.config.js` | Line 12 | `files: { src: 'ui' }` | `files: { src: 'ui/src' }` |
| `tauri.conf.json` | Line 7 | `frontendDist": "../build"` | `frontendDist": "../../build"` |
| `components.json` | Line 4 | `"css": "ui/app.css"` | `"css": "ui/src/app.css"` |
| `.gitignore` | Line 19 | `/src-tauri/target/` | `/backend/src-tauri/target/` |

**No changes needed:** `package.json` (delegates to make), `tsconfig.json` (extends .svelte-kit), `postcss.config.js`, `tailwind.config.ts`, `.chunkhound.json`, `.mcp.json`.

### 2. Import Paths & Aliases

| Category | Finding |
|----------|---------|
| Frontend `$lib/` alias | Resolved by `svelte.config.js` `src:` field — **no imports need changing** |
| Frontend relative imports | All use `$lib/` or `$app/` — no manual relative imports cross boundaries |
| Rust `mod`/`use` paths | Internal to `src-tauri/src/` — **unchanged** since the directory moves as a unit |
| Cargo.toml path deps | None — no workspace path dependencies exist |

**Key finding:** The `$lib` alias system means zero frontend import changes are needed. Update `svelte.config.js` once, everything resolves.

### 3. Watcher Configuration

| Watcher | Current Scope | Proposed Scope | Config Change |
|---------|--------------|----------------|---------------|
| Vite HMR | Watches `ui/` (via svelte.config `src: 'ui'`) | `ui/src/` | Update `svelte.config.js` |
| Vite ignore | `**/.orqa/**` | No change needed | — |
| Cargo/Tauri | Implicit — watches `src-tauri/src/` | `backend/src-tauri/src/` | Automatic (follows `tauri.conf.json` location) |
| Dev controller | `--no-watch` flag already set | No change | — |

### 4. Dev Controller (`scripts/dev.mjs`)

| Location | Current | Required Change |
|----------|---------|-----------------|
| Line 43 | `DASHBOARD_HTML = join(PROJECT_ROOT, "scripts", "dev-dashboard.html")` | `join(PROJECT_ROOT, "debugger", "dev-dashboard.html")` |
| All other paths | Use `PROJECT_ROOT` dynamically | No change needed |

Only **one hardcoded path** needs updating. All other paths are computed from PROJECT_ROOT.

### 5. Sidecar Path References (`src-tauri/src/commands/sidecar_commands.rs`)

| Line | Current Path | Required Change |
|------|--------------|-----------------|
| 16 | `Path::new("sidecar/dist/sidecar.js")` | `"sidecars/claude-agentsdk-sidecar/dist/sidecar.js"` |
| 22 | `Path::new("../sidecar/dist/sidecar.js")` | `"../../sidecars/claude-agentsdk-sidecar/dist/sidecar.js"` |
| 28 | `Path::new("src-tauri/test-sidecar/echo.cjs")` | `"backend/src-tauri/test-sidecar/echo.cjs"` |
| 34 | `Path::new("test-sidecar/echo.cjs")` | No change (relative to src-tauri/) |
| 40 | Fallback error message | Update path string |

### 6. Documentation Path References

**~40 files** in `.orqa/` reference current paths. Major categories:

| Pattern | Replacement | Est. Occurrences |
|---------|------------|------------------|
| `src-tauri/` | `backend/src-tauri/` | ~80 |
| `sidecar/` | `sidecars/claude-agentsdk-sidecar/` | ~15 |
| `scripts/dev.mjs` | `debugger/dev.mjs` | ~10 |
| `ui/lib/` | `ui/src/lib/` | ~30 |
| `ui/routes/` | `ui/src/routes/` | ~5 |

Automatable via `sed` find-replace on `.md` files in `.orqa/`.

### 7. Symlinks (`.claude/`)

`.claude/` symlinks point to `.orqa/` (unchanged). No symlink changes needed.

---

## Migration Order

### Recommended Sequence

**Phase 1: Documentation** (zero code risk)
- Find-replace all path references in `.orqa/` documentation
- Commit: "[EPIC-7b039d05](EPIC-7b039d05): Update documentation paths for directory reorganisation"

**Phase 2: Move sidecar** (lowest dependency)
- `git mv sidecar sidecars/claude-agentsdk-sidecar`
- Update Makefile (3 changes), sidecar_commands.rs (5 paths)
- Verify: `make lint-backend && make test-rust`

**Phase 3: Move backend** (medium dependency)
- `git mv src-tauri backend/src-tauri`
- Update Makefile (1 change), tauri.conf.json (1 change), .gitignore (1 change)
- Verify: `make lint-backend && make test-rust`

**Phase 4: Nest frontend source** (low dependency)
- Move `ui/` contents into `ui/src/`
- Update svelte.config.js, components.json, vite.config.ts (3 changes)
- Verify: `make typecheck && make test-frontend`

**Phase 5: Move dev controller** (self-contained)
- `git mv scripts debugger` (or individual files)
- Update Makefile (9 changes), dev.mjs (1 path)
- Verify: `make dev && make kill`

**Phase 6: Integration test**
- `make check` (all quality gates)
- `make dev` + manual smoke test
- `make build` (production build)

### Verification Between Phases

| Phase | Check | Command |
|-------|-------|---------|
| 2 | Rust compiles with sidecar paths | `make lint-backend` |
| 3 | Cargo finds manifest | `cargo metadata --manifest-path backend/src-tauri/Cargo.toml` |
| 4 | Frontend checks pass | `make typecheck && make test-frontend` |
| 5 | Controller starts | `make dev && sleep 5 && make kill` |
| 6 | Full integration | `make check && make build` |

---

## Risk Assessment

### High Risk

| Area | Risk | Mitigation |
|------|------|-----------|
| Sidecar path discovery | Sidecar fails to spawn if paths wrong | Test immediately: `make dev`, verify sidecar connects |
| Cargo manifest location | All cargo commands fail | CARGO_MANIFEST variable centralizes; one update |

### Medium Risk

| Area | Risk | Mitigation |
|------|------|-----------|
| SvelteKit `src:` field | Frontend build breaks | Test `npm run check` immediately after |
| tauri.conf.json `frontendDist` | Tauri can't find built frontend | Test `make build` |

### Low Risk

| Area | Risk | Mitigation |
|------|------|-----------|
| Documentation paths | Docs become inaccurate | Automated find-replace, spot-check |
| Dev controller dashboard | 404 on dashboard HTML | One path to fix |

### Rollback Strategy

Each phase is an independent commit. To rollback any phase: `git revert <commit>`. Phases are designed so each can be reverted independently without affecting other phases.

---

## Summary

| Category | Changes | Effort |
|----------|---------|--------|
| Config files | 8 files, ~15 line changes | 15 min |
| Makefile | 1 file, ~12 line changes | 10 min |
| Rust code | 1 file, 5 path changes | 30 min (with testing) |
| JS code | 1 file, 1 path change | 5 min |
| Documentation | ~40 files, ~140 replacements | 30-45 min (automated) |
| .gitignore | 1 file, 2 pattern changes | 5 min |
| **Total** | **~50 files** | **~2-3 hours** |

**Key insight:** The codebase is well-structured for this reorganisation. The `$lib` alias system, centralised CARGO_MANIFEST variable, and PROJECT_ROOT pattern mean most paths resolve through a single config point. The heaviest lift is documentation updates, which are automatable.