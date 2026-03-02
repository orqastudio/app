# Forge Makefile

.PHONY: dev build check test docs index reindex calibrate skills-list skills-update

dev:
	cargo tauri dev

build:
	cargo tauri build

check:
	cargo fmt --check
	cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
	cargo test --manifest-path src-tauri/Cargo.toml
	npm run check
	npm run lint
	npm run test

test:
	cargo test --manifest-path src-tauri/Cargo.toml
	npm run test

docs:
	npx docsify serve docs/

index:
	uvx chunkhound index

reindex:
	uvx chunkhound index --force

calibrate:
	uvx chunkhound calibrate

skills-list:
	npx skills list

skills-update:
	npx skills update
