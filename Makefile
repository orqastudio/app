# OrqaStudio Dev Environment
#
# make install         Bootstrap the CLI then run orqa install
# make dev             Start the dev environment (Vite + Tauri + Search + MCP + LSP)
# make restart-tauri   Restart Tauri app (Vite + services stay alive)
# make restart-vite    Restart Vite dev server only
# make restart         Restart Vite + Tauri (services stay alive)
# make restart-search  Restart search server (and MCP, which depends on it)
# make restart-mcp     Restart MCP server only
# make restart-lsp     Restart LSP server only
# make stop            Stop the controller gracefully
# make kill            Force-kill all processes
#
# After install, use orqa directly for everything else:
#   orqa audit / check / test / enforce / id / version / repo / plugin / graph

.DEFAULT_GOAL := install

install: ## Bootstrap the CLI then run orqa install
	@bash scripts/install.sh

dev: ## Start the dev environment (Vite + Tauri + Search + MCP + LSP)
	@cd app && node ../tools/debug/dev.mjs dev

restart-tauri: ## Restart Tauri app only (Vite + services stay alive)
	@cd app && node ../tools/debug/dev.mjs restart-tauri

restart-vite: ## Restart Vite dev server only
	@cd app && node ../tools/debug/dev.mjs restart-vite

restart: ## Restart Vite + Tauri (controller and services stay alive)
	@cd app && node ../tools/debug/dev.mjs restart

restart-search: ## Restart search server (and MCP, which depends on it)
	@cd app && node ../tools/debug/dev.mjs restart-search

restart-mcp: ## Restart MCP server only
	@cd app && node ../tools/debug/dev.mjs restart-mcp

restart-lsp: ## Restart LSP server only
	@cd app && node ../tools/debug/dev.mjs restart-lsp

stop: ## Stop the dev controller gracefully
	@cd app && node ../tools/debug/dev.mjs stop

kill: ## Force-kill all OrqaStudio processes
	@cd app && node ../tools/debug/dev.mjs kill

status: ## Show dev controller process status
	@cd app && node ../tools/debug/dev.mjs status
