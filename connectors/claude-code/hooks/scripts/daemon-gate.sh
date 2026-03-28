#!/usr/bin/env bash
# OrqaStudio connector — Daemon health gate
#
# Runs on UserPromptSubmit to ensure the daemon is reachable before any work.
# The daemon provides rule enforcement, graph validation, and content services.
# Without it, rules are not mechanically enforced — work must not proceed.
#
# Exit codes:
#   0 — daemon healthy, proceed normally
#   2 — daemon unreachable, block the interaction

set -euo pipefail

# ─── Port Resolution ─────────────────────────────────────────────────────────
# ORQA_PORT_BASE is the direct daemon health port (not a base for an offset).
# This matches daemon/src/health.rs resolve_port() — default is 9120.
DAEMON_PORT="${ORQA_PORT_BASE:-9120}"

# ─── Health Check ───────────────────────────────────────────────────────────
# Use a short timeout (2s) to avoid blocking the user experience.
if curl -sf --max-time 2 "http://127.0.0.1:${DAEMON_PORT}/health" > /dev/null 2>&1; then
  exit 0
fi

# ─── Daemon Not Reachable — Block ──────────────────────────────────────────
BLOCK_MSG="OrqaStudio daemon is not running. Rule enforcement requires the daemon.\\n\\n"
BLOCK_MSG="${BLOCK_MSG}Start it with: orqa daemon start\\n"
BLOCK_MSG="${BLOCK_MSG}Or run: orqa-validation daemon --project-root \$(pwd) &\\n\\n"
BLOCK_MSG="${BLOCK_MSG}Daemon expected on port ${DAEMON_PORT} (ORQA_PORT_BASE, default 9120).\\n\\n"
BLOCK_MSG="${BLOCK_MSG}To work without the daemon, disable the OrqaStudio plugin."
printf '{"hookSpecificOutput":{"permissionDecision":"deny"},"systemMessage":"%s"}' "$BLOCK_MSG" >&2
exit 2
