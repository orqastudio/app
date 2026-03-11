# Orqa Studio Makefile

.DEFAULT_GOAL := help

CARGO_MANIFEST := src-tauri/Cargo.toml

.PHONY: install install-sidecar \
        dev start dev-frontend dev-sidecar stop kill restart-tauri restart-vite restart status \
        build build-frontend build-sidecar \
        check fmt fmt-check clippy lint check-frontend \
        test test-rust test-frontend test-watch test-e2e \
        docs \
        index reindex calibrate \
        skills-list skills-update \
        clean help

# ── Setup ────────────────────────────────────────────────────────────────────

install: ## Install all dependencies (npm + sidecar + cargo)
	npm install
	cd sidecar && bun install
	cargo fetch --manifest-path $(CARGO_MANIFEST)

install-sidecar: ## Install sidecar dependencies
	cd sidecar && bun install

# ── Development ──────────────────────────────────────────────────────────────

dev: ## Start dev environment (spawns controller, waits for ready, exits)
	@node scripts/dev.mjs dev

start: ## Start dev controller in foreground (long-running, unified output)
	@node scripts/dev.mjs start

stop: ## Stop controller gracefully (requires manual restart to resume)
	@node scripts/dev.mjs stop

kill: ## Force-kill all OrqaStudio processes
	@node scripts/dev.mjs kill

restart-tauri: ## Restart Tauri app only — recompile Rust, Vite stays alive
	@node scripts/dev.mjs restart-tauri

restart-vite: ## Restart Vite dev server only
	@node scripts/dev.mjs restart-vite

restart: ## Restart Vite + Tauri (controller stays alive)
	@node scripts/dev.mjs restart

status: ## Show dev controller and process status
	@node scripts/dev.mjs status

dev-frontend: ## Run frontend only (Vite dev server)
	npm run dev

dev-sidecar: ## Build sidecar for development
	cd sidecar && bun run build

# ── Build ─────────────────────────────────────────────────────────────────────

build: ## Production build (cargo tauri build)
	cargo tauri build

build-frontend: ## Build frontend only
	npm run build

build-sidecar: ## Build sidecar for production
	cd sidecar && bun run build

# ── Quality ──────────────────────────────────────────────────────────────────

check: fmt-check clippy test-rust check-frontend lint test-frontend ## Run ALL checks (fmt-check + clippy + test-rust + check-frontend + lint + test-frontend)

fmt: ## Auto-format Rust code
	cargo fmt --manifest-path $(CARGO_MANIFEST)

fmt-check: ## Check Rust formatting (no changes)
	cargo fmt --manifest-path $(CARGO_MANIFEST) --check

clippy: ## Run Rust linter
	cargo clippy --manifest-path $(CARGO_MANIFEST) -- -D warnings

lint: ## Run ESLint
	npm run lint

check-frontend: ## Run svelte-check + TypeScript checks
	npm run check

# ── Testing ──────────────────────────────────────────────────────────────────

test: test-rust test-frontend ## Run all tests (Rust + frontend)

test-rust: ## Run Rust tests only
	cargo test --manifest-path $(CARGO_MANIFEST)

test-frontend: ## Run frontend tests (Vitest)
	npm run test || if [ $$? -eq 1 ] && npx vitest run 2>&1 | grep -q "No test files found"; then echo "No test files found — skipping."; else exit 1; fi

test-watch: ## Run frontend tests in watch mode
	npm run test:watch

test-e2e: ## Run E2E tests (Playwright)
	npx playwright test

# ── Documentation ────────────────────────────────────────────────────────────

docs: ## Serve documentation locally
	npx docsify serve docs/

# ── Code Search ──────────────────────────────────────────────────────────────

index: ## Index codebase for ChunkHound
	uvx chunkhound index

reindex: ## Force re-index codebase
	uvx chunkhound index --force

calibrate: ## Calibrate ChunkHound search
	uvx chunkhound calibrate

# ── Skills ───────────────────────────────────────────────────────────────────

skills-list: ## List installed skills
	npx skills list

skills-update: ## Update all skills
	npx skills update

# ── Utilities ────────────────────────────────────────────────────────────────

clean: ## Remove build artifacts
	rm -rf src-tauri/target node_modules/.vite ui/.svelte-kit build

help: ## Show all targets with descriptions
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
