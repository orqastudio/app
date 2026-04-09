# OrqaStudio Dev Environment — Bootstrap Only
#
# make install     Bootstrap the CLI then run orqa install
#
# After install, use orqa directly for everything:
#   orqa debug         Start dev environment
#   orqa check         Run all quality checks
#   orqa test          Run all tests
#   orqa build         Production build
#   orqa daemon        Manage validation daemon
#   orqa --help        Full command list

.DEFAULT_GOAL := install

install: ## Bootstrap the CLI then run orqa install
	@bash scripts/install.sh

link: ## Relink the CLI onto PATH (fast, use after reboot)
	@cd libs/types && npx tsc
	@cd cli && npx tsc && npm link
