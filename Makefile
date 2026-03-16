.PHONY: setup verify verify-integrity verify-rust verify-types verify-sdk verify-app test-rust build-ui

# First-time setup: install deps, build libs, link everything
setup:
	bash scripts/link-all.sh

# Run all verification checks
verify: verify-integrity verify-rust verify-types verify-sdk verify-app

# Artifact integrity (runs orqa-integrity on the dev environment root)
verify-integrity:
	cd app/ui && npx orqa-integrity ../..

# Rust backend tests
verify-rust: build-ui
	cd app/backend/src-tauri && cargo test

# TypeScript type checking
verify-types:
	cd libs/types && npx tsc --noEmit

verify-sdk:
	cd libs/sdk && npx tsc --noEmit

verify-app:
	cd app/ui && npx svelte-check --threshold warning

# Rust clippy lint
lint-rust: build-ui
	cd app/backend/src-tauri && cargo clippy -- -D warnings

# Rust format check
fmt-rust:
	cd app/backend/src-tauri && cargo fmt --check

# Build UI (required before Rust compilation)
build-ui:
	cd app/ui && npm run build

# Run all Rust tests
test-rust: build-ui
	cd app/backend/src-tauri && cargo test
