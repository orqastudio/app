# Orqa Studio Makefile

.DEFAULT_GOAL := help

CARGO_MANIFEST := backend/src-tauri/Cargo.toml

.PHONY: install install-sidecar \
        build build-frontend build-sidecar \
        check format format-check lint lint-backend lint-frontend typecheck \
        test test-rust test-frontend coverage-rust coverage-frontend test-watch test-e2e \
        verify verify-links verify-integrity verify-schema verify-enforcement \
        index reindex calibrate \
        skills-list skills-update \
        clean help

# ── Setup ────────────────────────────────────────────────────────────────────

install: ## Install all dependencies (npm + sidecar + cargo)
	cd ui && npm install
	cd sidecars/claude-agentsdk-sidecar && bun install
	cargo fetch --manifest-path $(CARGO_MANIFEST)

install-sidecar: ## Install sidecar dependencies
	cd sidecars/claude-agentsdk-sidecar && bun install

# ── Build ─────────────────────────────────────────────────────────────────────

build: ## Production build (cargo tauri build)
	cd backend && cargo tauri build

build-frontend: ## Build frontend only
	cd ui && npm run build

build-sidecar: ## Build sidecar for production
	cd sidecars/claude-agentsdk-sidecar && bun run build

# ── Quality ──────────────────────────────────────────────────────────────────

check: format-check lint test-rust typecheck test-frontend ## Run ALL checks (format-check + lint + test-rust + typecheck + test-frontend)

format: ## Auto-format Rust code
	cargo fmt --manifest-path $(CARGO_MANIFEST)

format-check: ## Check Rust formatting (no changes)
	cargo fmt --manifest-path $(CARGO_MANIFEST) --check

lint: lint-backend lint-frontend ## Run all linters (backend + frontend)

lint-backend: ## Run Rust linter (clippy)
	cargo clippy --manifest-path $(CARGO_MANIFEST) -- -D warnings

lint-frontend: ## Run ESLint
	cd ui && npm run lint

typecheck: ## Run svelte-check + TypeScript checks
	cd ui && npm run check

# ── Testing ──────────────────────────────────────────────────────────────────

test: test-rust test-frontend ## Run all tests (Rust + frontend)

test-rust: ## Run Rust tests only
	cargo test --manifest-path $(CARGO_MANIFEST)

test-frontend: ## Run frontend tests (Vitest)
	cd ui && npm run test

coverage-rust: ## Run Rust tests with coverage report (cargo-llvm-cov)
	cargo llvm-cov --manifest-path $(CARGO_MANIFEST) --lib

coverage-frontend: ## Run frontend tests with coverage report
	cd ui && npm run test:coverage

test-watch: ## Run frontend tests in watch mode
	cd ui && npm run test:watch

test-e2e: ## Run E2E tests (Playwright)
	cd ui && npx playwright test

# ── Code Search ──────────────────────────────────────────────────────────────

index: ## Index codebase for ChunkHound
	uvx chunkhound index

reindex: ## Force re-index codebase
	uvx chunkhound index --force

calibrate: ## Calibrate ChunkHound search
	uvx chunkhound calibrate

# ── Skills ───────────────────────────────────────────────────────────────────

skills-list: ## List installed skills
	cd ui && npx skills list

skills-update: ## Update all skills
	cd ui && npx skills update

# ── Verification ─────────────────────────────────────────────────────────────

verify-links: ## Verify all .orqa/ cross-references and source code paths resolve
	node tools/verify-links.mjs --check-bidirectional --check-paths

verify-integrity: ## Check artifact graph integrity (links, inverses, dependencies, gates)
	cd ui && npx orqa-integrity ..

verify-schema: ## Validate all .orqa/ artifact schemas
	@echo "--- Schema validation ---"
	@bash .githooks/validate-artifacts.sh $$(find .orqa -name '*.md' ! -name 'README.md' ! -path '*/_*' | sort)

verify-enforcement: ## Check enforcement rule coverage
	node tools/verify-enforcement-rules.mjs

verify: verify-links verify-integrity verify-schema verify-enforcement ## Run all verification checks

# ── Utilities ────────────────────────────────────────────────────────────────

clean: ## Remove build artifacts
	rm -rf backend/src-tauri/target ui/node_modules/.vite ui/.svelte-kit ui/build

help: ## Show all targets with descriptions
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
